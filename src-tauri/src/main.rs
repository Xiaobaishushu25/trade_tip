// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #![feature(error_generic_member_access)]

mod app_errors;
mod dtos;
mod entities;
mod service;
mod utils;

use std::sync::atomic::AtomicBool;
use std::sync::{Mutex};
use log::{error, info};
use tokio::task::JoinHandle;
use crate::entities::init_db_coon;
use crate::service::command::handle::init_cache;
use crate::service::command::tauri_command::{add_stock_info, get_response, query_all_groups, query_groups_by_code, query_stock_info, query_stocks_by_group_name, create_group, update_stock_groups, remove_stock_from_group, update_stock_hold, query_stocks_day_k_limit, query_live_stocks_data,update_live_state};
use crate::service::curd::update_all_day_k;
use crate::service::http::{init_http};
///是否需要实时更新
pub static UPDATEING: AtomicBool = AtomicBool::new(true);
pub static NOTICE: Mutex<Option<String>> = Mutex::new(None);
pub struct MyState{
    // live_state:AtomicBool,
    live_task:Mutex<Option<JoinHandle<()>>>
}
impl MyState{
    pub fn new() -> Self{
        Self{
            // live_state:AtomicBool::new(false),
            live_task:Mutex::new(None)
        }
    }
    // pub fn update_live_state(&self,state:bool){
    //     self.live_state.store(state,std::sync::atomic::Ordering::Relaxed);
    // }
    // pub fn get_live_state(&self) -> bool {
    //     self.live_state.load(std::sync::atomic::Ordering::Relaxed)
    // }
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
#[tokio::main]
async fn main() {
    // println!("开始测试");
    // init_db_coon().await;
    // init_http().await;
    // tokio::spawn(async {
    //     println!("开始测试全部分组");
    //     query_data("全部".into()).await;
    // });
    // println!("开始测试持有分组");
    // let _  = query_data("持有".into()).await;
    // println!("结束测试");
    init_app().await;
    tauri::Builder::default()
        // .manage(MyState{task:Mutex::new(None)})
        .manage(MyState::new())
        // .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        // .setup(|app|{
        //     let window = tauri::window::WindowBuilder::new(app, "tool")
        //         .build()?;
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
            update_stock_groups,
            remove_stock_from_group,
            update_stock_hold,
            query_stocks_day_k_limit,
            query_live_stocks_data,
            update_live_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
async fn init_app() {
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
    init_db_coon().await;
    init_http().await;
    match update_all_day_k().await{
        Ok(_)=>{
            info!("更新日线数据成功");
            NOTICE.lock().unwrap().replace("更新日线数据成功".to_string());
        },
        Err(e)=>{
            error!("更新日线数据失败:{}",e);
            NOTICE.lock().unwrap().replace(format!("更新日线数据失败:{}",e.to_string()));
        }
    };
    init_cache().await;
}
