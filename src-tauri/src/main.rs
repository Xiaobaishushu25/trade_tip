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
use crate::config::config::Config;
use crate::entities::init_db_coon;
use crate::service::command::tauri_command::{
    add_stock_info, create_group, delete_all_transaction_records, delete_graphic_by_group_id,
    delete_graphic_by_id, delete_group, delete_transaction_record_by_primary, exit_app, get_config,
    get_response, query_all_groups, query_box, query_graphic_by_code, query_groups_by_code,
    query_intraday_chart_img, query_live_stock_data_by_code, query_live_stocks_data_by_group_name,
    query_stock_info, query_stocks_by_group_name, query_stocks_day_k_limit,
    query_transaction_records, query_transaction_records_by_code, read_save_transaction_records,
    remove_stock_from_group, save_graphic, save_transaction_records, update_groups,
    update_live_state, update_stock_groups, update_stock_hold, update_transaction_record,
};
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::init_http;
use log::{error, info};
use std::collections::HashMap;
use std::env;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, LazyLock, Mutex};
use tauri::Manager;
use tokio::task::JoinHandle;

///是否需要实时更新
pub static UPDATEING: AtomicBool = AtomicBool::new(true);
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
    // pub fn update_config(&self,config:Config){
    //     match save_config(&self.config){
    //         Ok(_)=>{
    //             info!("save config success");
    //             *self.config = config;
    //         }
    //         Err(e)=>{
    //             error!("save config failed:{}",e.to_string());
    //         }
    //     }
    //     // if let Ok(_) = save_config(&config){
    //     //     info!("save config success");
    //     //     *self.config = config;
    //     // }else {  }
    //     // *self.config = config;
    //
    // }
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
    tokio::spawn(async {
        info!("update stock info start");
        update().await;
    });
    // init_app().await;
    let state = MyState::new().await;
    info!("ui start");

    // let config = Config::load();
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
        // .manage(ConfigState::new().await)
        .manage(Mutex::new(Config::load().await))
        // .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        // .setup(|app|{
        //     info!("{:?}",app.app_handle().path().app_config_dir().unwrap()); //"C:\\Users\\Xbss\\AppData\\Roaming\\com.xbss.trade-tip"
        //     let main_window = app.get_webview_window("main").unwrap();
        //     // main_window.as_ref().window().restore_state(StateFlags::all())
        //     let tool_window = app.get_webview_window("tool").unwrap();
        //     tokio::spawn(async move{
        //         // let mut result = main_window.restore_state(StateFlags::all());
        //         // info!("restore main window state result:{:?}",result);
        //         // if result.is_err(){
        //         //     error!("restore main window state error:{}",result.err().unwrap().to_string());
        //         // };
        //         main_window.show().unwrap();
        //         //  result = tool_window.restore_state(StateFlags::all());
        //         // if result.is_err(){
        //         //     error!("restore tool window state error:{}",result.err().unwrap().to_string());
        //         // };
        //     });
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
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    info!("ui end");
}
async fn init_app() {
    //todo 日志配置应该不需要放在外面的文件夹中，应该打包进二进制。
    log4rs::init_file("./data/log4rs.yaml", Default::default()).unwrap();
    // log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();
    init_db_coon().await;
    init_http().await;
}
async fn update() {
    match crate::service::curd::update_all_day_k().await {
        Ok(_) => {
            info!("更新日线数据成功");
            NOTICE
                .lock()
                .unwrap()
                .replace("更新日线数据成功".to_string());
        }
        Err(e) => {
            error!("更新日线数据失败:{}", e);
            NOTICE
                .lock()
                .unwrap()
                .replace(format!("更新日线数据失败:{}", e.to_string()));
        }
    };
}
