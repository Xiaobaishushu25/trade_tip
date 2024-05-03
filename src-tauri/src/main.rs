// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #![feature(error_generic_member_access)]

mod app_errors;
mod dtos;
mod entities;
mod service;
mod utils;

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use log::{error, info};
use tauri::Manager;
use tauri_plugin_window_state::{StateFlags, WindowExt};
// use tauri_plugin_window_state::{StateFlags, WindowExt};
use tokio::task::JoinHandle;
use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::service::command::tauri_command::{add_stock_info, get_response, query_all_groups, query_groups_by_code, query_stock_info, query_stocks_by_group_name, create_group,delete_group, update_stock_groups, remove_stock_from_group, update_stock_hold, query_stocks_day_k_limit, query_live_stocks_data, update_live_state, query_graphic_by_code, save_graphic, query_box, query_live_stock_data_by_code, delete_graphic_by_id, delete_graphic_by_group_id, exit_app, update_groups};
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::curd::update_all_day_k;
use crate::service::http::{init_http};
///是否需要实时更新
pub static UPDATEING: AtomicBool = AtomicBool::new(true);
// pub static NOTICE: Mutex<Option<Vec<String>>> = Mutex::new(None);
pub static NOTICE: Mutex<Option<String>> = Mutex::new(None);
pub struct MyState{
    // live_state:AtomicBool,
    live_task:Mutex<Option<JoinHandle<()>>>,
    history_close_price:Mutex<HashMap<String,Arc<Vec<f64>>>>
}
impl MyState{
    pub async fn new() -> Self{
        let result = get_close_prices(None).await;
        let close_prices = match result {
            Ok(data) => {
                info!("初始化缓存成功");
                data
            }
            Err(e) => {
                error!("初始化缓存失败:{}",e.to_string());
                // NOTICE.lock().unwrap().unwrap().
                HashMap::new()
            }
        };
        Self{
            live_task:Mutex::new(None),
            history_close_price:Mutex::new(close_prices)
        }
    }
    pub fn update_history_close_price(&self,code:String,close_prices:Arc<Vec<f64>>){
        self.history_close_price.lock().unwrap().insert(code,close_prices);
        // self.history_close_price.insert(code,close_price);
    }
    fn abort_task(&self){
        if let Some(task) = self.live_task.lock().unwrap().take(){
            info!("hava task cancel");
            task.abort();
        }
    }
    pub fn set_task(&self,task:JoinHandle<()>){
        self.abort_task();
        *self.live_task.lock().unwrap() = Some(task);
    }
}
pub async fn get_close_prices(single_code:Option<&str>) ->AppResult<HashMap<String,Arc<Vec<f64>>>> {
    if single_code.is_none() {
        let codes = StockInfoCurd::query_all_only_code().await?;
        let mut map = HashMap::with_capacity(codes.len());
        for code in codes {
            map.insert(code.clone(),Arc::new(StockDataCurd::query_only_close_price_by_nums(&code,60).await?));
        }
        Ok(map)
    }else {
        let code = single_code.unwrap();
        let close_price = StockDataCurd::query_only_close_price_by_nums(code,60).await?;
        Ok(HashMap::from([(code.to_string(),Arc::new(close_price))]))
    }
}
#[tokio::main]
async fn main() {
    init_app().await;
    tokio::spawn(async {
        info!("update start");
        update().await;
    });
    // init_app().await;
    let state = MyState::new().await;
    info!("ui start");
    
    tauri::Builder::default()
        // .manage(MyState{task:Mutex::new(None)})
        .manage(state)
        // .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app|{
            // let window = tauri::window::WindowBuilder::new(app, "tool")
            //     .build()?;
            // app.get_window("main").
            // let main_window = app.get_window("main").unwrap();
            // let tool_window = app.get_window("tool").unwrap();
            info!("{:?}",app.app_handle().path().app_config_dir().unwrap()); //"C:\\Users\\Xbss\\AppData\\Roaming\\com.xbss.trade-tip"
            let main_window = app.get_webview_window("main").unwrap();
            let tool_window = app.get_webview_window("tool").unwrap();
            tokio::spawn(async move{
                let mut result = main_window.restore_state(StateFlags::all());
                if result.is_err(){
                    error!("restore main window state error:{}",result.err().unwrap().to_string());
                };
                main_window.show().unwrap();
                 result = tool_window.restore_state(StateFlags::all());
                if result.is_err(){
                    error!("restore tool window state error:{}",result.err().unwrap().to_string());
                };
            });
            // let result = main_window.restore_state(StateFlags::all());
            // if result.is_err(){
            //     error!("restore state error:{}",result.err().unwrap().to_string());
            // }
            // let main_window = app.get_window("tool").unwrap();
            // // let main_window = app.get_webview_window("main").unwrap().get_window();
            // let result = main_window.restore_state(StateFlags::all());
            // if result.is_err(){
            //     error!("restore state error:{}",result.err().unwrap().to_string());
            // }
            Ok(())
        })
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
            query_live_stocks_data,
            query_live_stock_data_by_code,
            update_live_state,
            query_graphic_by_code,
            save_graphic,
            query_box,
            delete_graphic_by_id,
            delete_graphic_by_group_id,
            update_groups,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    info!("ui end");
}
async fn init_app() {
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
    init_db_coon().await;
    init_http().await;
}
async fn update(){
    match crate::service::curd::update_all_day_k().await{
        Ok(_)=>{
            info!("更新日线数据成功");
            NOTICE.lock().unwrap().replace("更新日线数据成功".to_string());
        },
        Err(e)=>{
            error!("更新日线数据失败:{}",e);
            NOTICE.lock().unwrap().replace(format!("更新日线数据失败:{}",e.to_string()));
        }
    };
}
