// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app_errors;
mod service;
mod entities;
mod dtos;

use log::{error, info};
use crate::entities::init_db_coon;
use crate::service::http::{init_http};
use crate::service::command::command::{get_response,add_stock_info,query_stock_info};

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
            query_stock_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
async fn init_app(){
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
    init_db_coon().await;
    init_http().await;
}

