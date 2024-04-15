use log::{error, info};
use crate::entities::prelude::StockInfo;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::REQUEST;

#[tauri::command]
pub async fn get_response(url: String) -> Result<String,String> {
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
#[tauri::command]
pub async fn add_stock_info(code: String,name:String) -> Result<StockInfo,String> {
    let stock_info = StockInfo::new(code, name);
    match StockInfoCurd::insert(stock_info).await{
        Ok(stock_info)=>{
            info!("插入成功:{:?}",stock_info);
            Ok(stock_info)
        },
        Err(e)=>{
            error!("插入失败:{}",e);
            Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn query_stock_info() -> Result<Vec<StockInfo>,String> {
    match StockInfoCurd::find_all().await{
        Ok(stock_infos)=>{
            info!("查询所有信息成功:{:?}",stock_infos);
            Ok(stock_infos)
        },
        Err(e)=>{
            error!("查询失败:{}",e);
            Err(e.to_string())
        }
    }
}