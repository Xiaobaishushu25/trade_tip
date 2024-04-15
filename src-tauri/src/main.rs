// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::{WindowBuilder};

mod app_errors;
mod service;
mod entities;
mod dtos;

use log::{error, info};
use crate::entities::init_db_coon;
use crate::service::http::{init_http, REQUEST};
// use tauri_plugin_http::reqwest;
// use tauri_plugin_http::reqwest::header::HeaderMap;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
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
        .invoke_handler(tauri::generate_handler![greet,get_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
async fn init_app(){
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
    init_db_coon().await;
    init_http().await;
}
#[tauri::command]
async fn get_response(url: String) -> Result<String,String> {
    return match REQUEST.get().unwrap().get(&url).await{
        Ok(response)=>{
            info!("ok");
            Ok(response.text().await.unwrap())
        },
        Err(e)=>{
            error!("http请求错误:{}",e);
            Err(e.to_string())
        }
    }
    // println!("{:?}", url);
    // let response = reqwest::blocking::get(url).unwrap();
    // let client = reqwest::Client::new();
    // let mut header_map = HeaderMap::new();
    // // header_map.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Safari/537.36 Edg/91.0.864.41".parse().unwrap());
    // header_map.insert("User-Agent", "Apifox/1.0.0 (https://apifox.com)".parse().unwrap());
    // // header_map.insert("Accept", "*/*".parse().unwrap());
    // // header_map.insert("Host", "echarts.apache.org".parse().unwrap());
    // // header_map.insert("Connection", "keep-alive".parse().unwrap());
    //
    // let x = client.get(url).headers(header_map).send().await.unwrap();
    // // let x = reqwest::get(url).await.unwrap();
    // println!("{:?}", x);
    // // let x1 = x.json().await.unwrap();
    // x.text().await.unwrap()
    // // let response = reqwest::get(url).unwrap();
    // // return response.text().unwrap();
}
