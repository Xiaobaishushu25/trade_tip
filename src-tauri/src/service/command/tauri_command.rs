use log::{error, info};
use crate::entities::prelude::{StockData, StockGroup, StockGroups, StockInfo};
use crate::service::command::handle_data::handle_new_stock;
use crate::service::curd::group_stock_relation_curd::{GroupStockRelationCurd, MoreStockInfo};
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_group_curd::StockGroupCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::REQUEST;

#[tauri::command]
pub async fn get_response(url: String) -> Result<String,String> {
    return match REQUEST.get().unwrap().get(&url).await{
        Ok(response)=>{
            // info!("ok");
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
            // info!("插入成功:{:?}",stock_info);
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
            // info!("查询所有信息成功:{:?}",stock_infos);
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
            // info!("查询所有分组成功:{:?}",stock_groups);
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
                // // info!("查询持有分组成功:{:?}",more_infos);
                Ok(more_infos)
            },
            Err(e)=>{
                error!("查询失败:{}",e);
                Err(e.to_string())
            }
        }
    }
    match GroupStockRelationCurd::query_stocks_by_group_name(name).await{
        Ok(more_infos)=>{
            // // info!("根据分组名查询成功:{:?}",more_infos);
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
    match GroupStockRelationCurd::query_groups_by_stock_code(code).await{
        Ok(stock_groups)=>{
            // info!("根据股票代码查询分组成功:{:?}",stock_groups);
            Ok(stock_groups)
        },
        Err(e)=>{
            error!("查询失败:{}",e);
            Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn create_group(name:String) -> Result<i32,String> {
    match StockGroupCurd::insert(StockGroup::new(name)).await{
        Ok(stock_group)=>{
            // info!("插入分组成功:{:?}",stock_group);
            Ok(stock_group.index)
        },
        Err(e)=>{
            error!("插入分组失败:{}",e);
            Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn delete_group(name:String) -> Result<i32,String> {
    match StockGroupCurd::delete_by_name(name).await{
        Ok(count)=>{
            // info!("删除分组成功:{:?}",count);
            Ok(count)
        },
        Err(e)=>{
            error!("删除分组失败:{}",e);
            Err(e.to_string())
        }
    }
}
/// 更新股票的分组信息（前端需保证is_new==true和group_names长度为0不同时出现）
/// 如果是一个新的股票，则先插入股票信息表，在更新分组表
/// 如果不是一个新的股票，但是分组名为空，则先删除分组关系表，再删除股票信息表相关信息。若分组名不空，直接更新分组表
#[tauri::command]
pub async fn update_stock_groups(is_new:bool, code:String, name:String, group_names:Vec<String>) -> Result<(),String> {
    info!("is_new:{},code:{},name:{},group_names:{:?}",is_new,code,name,group_names);
    if is_new{
        match handle_new_stock(&code,&name).await{
            Ok(_)=>{
                info!("创建成功:{:?}",code);
            },
            Err(e)=>{
                error!("创建失败:{}",e);
                return Err(e.to_string());
            }
        }
        // match GroupStockRelationCurd::insert(GroupStockR::new("全部".into(),code.clone())).await{
        //     Ok(_) => { Ok(()) }
        //     Err(e) => {
        //         error!("插入失败:{}",e);
        //         return Err(e.to_string());
        //     }
        // }
    }else if group_names.is_empty(){//全部没选上
        match GroupStockRelationCurd::delete_by_stock_code(code.clone()).await{
            Ok(_)=>{
                // info!("删除分组关系成功。");
                match StockInfoCurd::delete_by_code(code.clone()).await{
                    Ok(_)=>{
                        // info!("删除股票信息成功。");
                        return Ok(());
                    },
                    Err(e)=>{
                        error!("删除股票信息失败:{}",e);
                    }
                }
            }
            Err(e)=>{
                error!("删除分组关系失败:{}",e);
            }
        }
    };
    match GroupStockRelationCurd::update_groups_by_code(code, group_names).await{
        Ok(_)=>{
            // info!("更新分组成功。");
            Ok(())
        },
        Err(e)=>{
            error!("更新分组失败:{}",e);
            Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn remove_stock_from_group(code:String,group_name:String) -> Result<(),String> {
    match GroupStockRelationCurd::delete_by_code_and_group_name(code, group_name).await{
        Ok(_)=>{
            // info!("删除成功。");
            Ok(())
        },
        Err(e)=>{
            error!("删除失败:{}",e);
            Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn update_stock_hold(code:String,hold:bool) -> Result<(),String> {
    match StockInfoCurd::update_hold_by_code(code, hold).await{
        Ok(_)=>{
            // info!("更新成功。");
            Ok(())
        },
        Err(e)=>{
            error!("更新失败:{}",e);
            Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn query_stocks_day_k_limit(code:String) -> Result<Vec<StockData>,String> {
    match StockDataCurd::query_by_nums(&code, 1000).await{
        Ok(stock_data_list)=>{
            // info!("查询成功:{:?}",stock_datas);
            Ok(stock_data_list)
        },
        Err(e)=>{
            error!("查询股票信息失败失败:{}",e);
            Err(e.to_string())
        }
    }
}
