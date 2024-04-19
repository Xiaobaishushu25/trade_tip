use log::{error, info};
use crate::entities::prelude::{StockGroup, StockGroups, StockInfo};
use crate::service::curd::group_stock_relation_curd::{GroupStockRelationCurd, MoreStockInfo};
use crate::service::curd::stock_group_curd::StockGroupCurd;
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

#[tauri::command]
pub async fn query_all_groups() -> Result<Vec<StockGroup>,String> {
    match StockGroupCurd::find_all().await{
        Ok(stock_groups)=>{
            info!("查询所有分组成功:{:?}",stock_groups);
            Ok(stock_groups)
        },
        Err(e)=>{
            error!("查询失败:{}",e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn query_stocks_by_group_name(name:String) -> Result<Vec<MoreStockInfo>,String> {
    if name=="持有"{
        return match StockInfoCurd::find_all_hold().await{
            Ok(more_infos)=>{
                info!("查询持有分组成功:{:?}",more_infos);
                Ok(more_infos)
            },
            Err(e)=>{
                error!("查询失败:{}",e);
                Err(e.to_string())
            }
        }
    }
    match GroupStockRelationCurd::find_stocks_by_group_name(name).await{
        Ok(more_infos)=>{
            info!("根据分组名查询成功:{:?}",more_infos);
            Ok(more_infos)
        },
        Err(e)=>{
            error!("查询失败:{}",e);
            Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn query_groups_by_code(code:String) -> Result<Vec<String>,String> {
    match GroupStockRelationCurd::find_groups_by_stock_code(code).await{
        Ok(stock_groups)=>{
            info!("根据股票代码查询分组成功:{:?}",stock_groups);
            Ok(stock_groups)
        },
        Err(e)=>{
            error!("查询失败:{}",e);
            Err(e.to_string())
        }
    }
}
