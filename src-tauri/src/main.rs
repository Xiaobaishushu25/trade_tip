// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app_errors;
mod dtos;
mod entities;
mod service;

use crate::entities::init_db_coon;
use crate::service::command::command::{
    add_stock_info, get_response, query_all_groups, query_groups_by_code, query_stock_info,
    query_stocks_by_group_name,create_group,update_stock_groups,remove_stock_from_group,
    update_stock_hold
};
use crate::service::http::init_http;
use log::{error, info};

#[tokio::main]
async fn main() {
    init_app().await;
    tauri::Builder::default()
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
            update_stock_hold
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
async fn init_app() {
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
    init_db_coon().await;
    init_http().await;
}
