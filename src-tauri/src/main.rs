// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #![feature(error_generic_member_access)]

mod app_errors;
mod cache;
mod config;
mod dtos;
mod entities;
mod service;
mod utils;

use crate::app_errors::AppResult;
use crate::cache::intraday_chart_cache::IntradayChartCache;
use crate::config::config::{Config, DataConfig};
use crate::entities::init_db_coon;
use crate::service::command::tauri_command::{
    add_stock_info, create_group, delete_all_transaction_records, delete_graphic_by_group_id,
    delete_graphic_by_id, delete_group, delete_transaction_record_by_primary, exit_app, get_config,
    get_response, query_all_groups, query_box, query_graphic_by_code, query_groups_by_code,
    query_intraday_chart_img, query_live_stock_data_by_code, query_live_stocks_data_by_group_name,
    query_stock_info, query_stocks_by_group_name, query_stocks_day_k_limit,
    query_transaction_records, query_transaction_records_by_code, read_save_transaction_records,
    remove_stock_from_group, save_graphic, save_transaction_records, update_groups,
    update_live_state, update_stock_groups, update_stock_hold, update_transaction_record, judge_can_t,
    save_config,update_data_config,query_all_positions,update_position,
};
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::{init_http, start_data_server};
use log::{error, info, LevelFilter};
use log4rs::config::{Appender, Root};
use std::collections::HashMap;
use std::{env, panic};
use std::sync::atomic::{AtomicBool};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use tauri::{Manager};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

///是否需要实时更新
pub static UPDATEING: AtomicBool = AtomicBool::new(true);
///是否开市，这个本来是打算用于K线图界面是否绘制最新数据K线的，但是如果处于交易日的中午，用这个无法绘制当天的最新数据，所以暂时还没用上。
pub static IS_MARKET_OPEN: AtomicBool = AtomicBool::new(true);
// pub static NOTICE: Mutex<Option<Vec<String>>> = Mutex::new(None);
pub static NOTICE: Mutex<Option<String>> = Mutex::new(None);
pub static CURRENT_DIR: LazyLock<String> = LazyLock::new(|| {
    let current_dir = &env::current_dir().unwrap();
    current_dir.to_string_lossy().to_string()
});
pub struct MyState {
    // live_state:AtomicBool,
    live_task: Mutex<Option<JoinHandle<()>>>,
    history_close_price: Mutex<HashMap<String, Arc<Vec<f64>>>>,
    // config: Config
}
impl MyState {
    pub async fn new() -> Self {
        let result = get_close_prices(None).await;
        let close_prices = match result {
            Ok(data) => {
                info!("初始化缓存成功");
                data
            }
            Err(e) => {
                error!("初始化缓存失败:{}", e.to_string());
                // NOTICE.lock().unwrap().unwrap().
                HashMap::new()
            }
        };
        Self {
            live_task: Mutex::new(None),
            history_close_price: Mutex::new(close_prices),
        }
    }
    pub fn update_history_close_price(&self, code: String, close_prices: Arc<Vec<f64>>) {
        self.history_close_price
            .lock()
            .unwrap()
            .insert(code, close_prices);
        // self.history_close_price.insert(code,close_price);
    }
    fn abort_task(&self) {
        if let Some(task) = self.live_task.lock().unwrap().take() {
            info!("hava task cancel");
            task.abort();
        }
    }
    pub fn set_task(&self, task: JoinHandle<()>) {
        self.abort_task();
        *self.live_task.lock().unwrap() = Some(task);
    }
}
pub async fn get_close_prices(
    single_code: Option<&str>,
) -> AppResult<HashMap<String, Arc<Vec<f64>>>> {
    if single_code.is_none() {
        let codes = StockInfoCurd::query_all_only_code().await?;
        let mut map = HashMap::with_capacity(codes.len());
        for code in codes {
            map.insert(
                code.clone(),
                Arc::new(StockDataCurd::query_only_close_price_by_nums(&code, 60).await?),
            );
        }
        Ok(map)
    } else {
        let code = single_code.unwrap();
        let close_price = StockDataCurd::query_only_close_price_by_nums(code, 60).await?;
        Ok(HashMap::from([(code.to_string(), Arc::new(close_price))]))
    }
}
#[tokio::main]
async fn main() {
    init_app().await;

    let config = Config::load().await;
    let data_config = config.data_config.clone();
    let (send,recv) = mpsc::channel::<()>(1);
    init_data_server_daily_position(recv,data_config).await;

    let state = MyState::new().await;
    info!("tauri start...");
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            error!("the app is already running, args: {:?}, cwd: {}", args, cwd);
            let _ = app.get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_dialog::init())
        // .manage(MyState{task:Mutex::new(None)})
        .manage(state)
        .manage(IntradayChartCache::new())
        // .manage(Mutex::new(Config::load().await))
        .manage(Mutex::new(config))
        .manage(send)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        // .setup(|app| {
        //     let app_handle: &AppHandle = app.app_handle();
        //     let config = app_handle.state::<Mutex<Config>>();
        //     let data_config = config.lock().unwrap().data_config.clone();
        //     let (send,recv) = mpsc::channel::<()>(1);
        //     app.manage(send);
        //     tokio::spawn(async move{
        //         tokio::spawn(async move{
        //             info!("update daily info start...");
        //             if let Err(e) = service::curd::update_all_day_k(data_config.use_ak_share).await{
        //                 error!("update daily info failed:{}",e)
        //             };
        //         });
        //         if let Err(e) = start_data_server(&data_config,recv).await{
        //             error!("start data server error:{}",e);
        //         };
        //     });
        //     info!("setup ok.");
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            get_response,
            add_stock_info,
            query_stock_info,
            query_all_groups,
            query_stocks_by_group_name,
            query_groups_by_code,
            create_group,
            delete_group,
            update_stock_groups,
            remove_stock_from_group,
            update_stock_hold,
            query_stocks_day_k_limit,
            query_live_stocks_data_by_group_name,
            query_live_stock_data_by_code,
            update_live_state,
            query_graphic_by_code,
            save_graphic,
            query_box,
            delete_graphic_by_id,
            delete_graphic_by_group_id,
            update_groups,
            query_intraday_chart_img,
            read_save_transaction_records,
            query_transaction_records,
            update_transaction_record,
            query_transaction_records_by_code,
            delete_transaction_record_by_primary,
            delete_all_transaction_records,
            save_transaction_records,
            get_config,
            update_data_config,
            save_config,
            judge_can_t,
            query_all_positions,
            update_position,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    info!("ui end");
}
async fn init_app() {
    init_logger().await;
    //done 日志配置应该不需要放在外面的文件夹中，应该打包进二进制。不然第一次启动时无法找到日志配置文件，导致无法启动。
    // let config_content = include_str!("../data/log4rs.yaml");
    // println!("config_content:{}", config_content);
    // let config = serde_yml::from_str::<RawConfig>(config_content).unwrap();
    // println!("config:{:?}", config);
    // log4rs::init_raw_config(config).unwrap();
    // log4rs::init_file("./data/log4rs.yaml", Default::default()).unwrap();
    init_db_coon().await;
    init_http().await;
}
async fn init_logger() {
    // 创建一个控制台 appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {h({l})} {f} line:{L} - {m}{n}")))
        .build();
    // 创建一个文件 appender
    let file = FileAppender::builder()
        .append(true)
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {h({l})} {f} line:{L} - {m}{n}")))
        .build("log/server.log")
        .unwrap();
    // 构造日志配置
    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(LevelFilter::Error))).build("server", Box::new(file)))
        // .logger(Logger::builder().build("tao::platform_impl::windows::event_loop::runner", LevelFilter::Off))
        .build(Root::builder().appender("stdout").appender("server").build(LevelFilter::Info))
        .unwrap();
    // 初始化日志
    log4rs::init_config(config).unwrap();
    // 设置panic hook
    // panic::set_hook(Box::new(|info| {
    //     error!("Panic occurred: {:?}", info);
    // }));
    // 设置panic hook
    panic::set_hook(Box::new(|info| {
        if let Some(location) = info.location() {
            // 打印 panic 信息和发生 panic 的位置
            error!(
                "Panic occurred at {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }
        // 处理panic payload，检查是否为某个具体的错误类型
        if let Some(payload) = info.payload().downcast_ref::<String>() {
            // 如果payload是字符串类型，直接打印
            error!("Panic message: {}", payload);
        } else if let Some(payload) = info.payload().downcast_ref::<&str>() {
            // 如果是&str，直接打印
            error!("Panic message: {}", payload);
        } else {
            // 其他情况，打印更通用的信息
            error!("Panic occurred with unknown payload: {:?}", info.payload());
        }
    }));
}
async fn init_data_server_daily_position(recv:Receiver<()>,data_config:DataConfig){
    let can_handle_futures = data_config.use_ak_share;
    tokio::spawn(async move{
        if let Err(e) = start_data_server(&data_config,recv).await{
            error!("start data server error:{}",e);
        };
    });
    //todo 下面这俩更新如果出错了在前端页面是没有提醒的，如果想要emit一个错误事件到前端的话，有可能存在前端页面还没渲染完就emit了，就收不到了；
    // 如果弄一个本地变量保存错误信息，前端页面渲染完成后主动来invoke读取这个变量判断是否出错应该可以，但是目前懒得搞。
    tokio::spawn(async move{
        info!("update daily info start...");
        if let Err(e) = service::curd::update_all_day_k(can_handle_futures,false).await{
            error!("update daily info failed:{:?}.",e)
        }
        info!("update daily info end.");
        // match service::curd::update_all_day_k(can_handle_futures).await {
        //     Ok(failure) => {
        //         if failure {
        //             error!("first update daily info failed, 20s later will retry again.");
        //             sleep(Duration::from_secs(20)).await;
        //             match service::curd::update_all_day_k(can_handle_futures).await{
        //                 Ok(failure) =>{
        //
        //                 }
        //                 Err(e) => {error!("second update stock info failed:{:?}",e)}
        //             }
        //         }
        //     }
        //     Err(e) => {error!("update stock info failed:{:?}",e)}
        // }
    });
    tokio::spawn(async {
        info!("update position start");
        if let Err(e) = service::curd::update_all_position().await{
            error!("update position failed:{}",e)
        }
    });
}
// ///判断是否开市,先发起一个请求，然后sleep 1秒，再发起一个请求，如果两个请求的返回值不一样，则证明开市了，否则证明闭市了。
// async fn judge_market_open() {
//     let response1 = REQUEST.get().unwrap().get("https://qt.gtimg.cn/q=sz159992").await.unwrap();
//     let string1 = response1.text().await.unwrap();
//     sleep(Duration::from_secs(1)).await;
//     let response2 = REQUEST.get().unwrap().get("https://qt.gtimg.cn/q=sz159992").await.unwrap();
//     let string2 = response2.text().await.unwrap();
//     if string1==string2 {
//         println!("market is closed");
//         IS_MARKET_OPEN.store(false, Ordering::Relaxed);
//     }else {
//         println!("market is open");
//         IS_MARKET_OPEN.store(true, Ordering::Relaxed);
//     }
// }
