use std::collections::HashMap;
use std::sync::Arc;
use anyhow::anyhow;
use crate::app_errors::AppResult;
use crate::dtos::stock_dto::StockLiveData;
use crate::entities::{init_db_coon};
use crate::entities::prelude::StockInfo;
use crate::entities::table::{create_table_with_dyn_name};
use crate::service::curd::group_stock_relation_curd::GroupStockRelationCurd;
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::{init_http, REQUEST};
use crate::utils::stock_util::{calculate_ago_with_num, compute_mul_ma, compute_single_ma};

///处理并保存股票数据
/// 需要先根据code创建表，然后处理日线数据（主要是计算ma60）
pub(crate) async fn handle_and_save_stock_data(create_need:bool,code:&str) ->AppResult<()>{
// async fn handle_and_save_stock_data(create_need:bool,code:&str,num:i32,date:Option<String>) ->AppResult<()>{
    if create_need { let _ = create_table_with_dyn_name(code).await?; }
    // let _ = create_table_with_dyn_name(code).await?;
    // let mut stock_data = REQUEST.get().unwrap().get_stock_day_data(&code, calculate_day_num()).await?;
    //不需要那么多的数据，再减三百，因为有大量不是交易日的日期。
    let mut stock_data = REQUEST.get().unwrap().get_stock_day_data(&code, calculate_ago_with_num(2020, 1, 1)-300).await?;
    // let mut stock_data = REQUEST.get().unwrap().get_stock_day_data(&code, num).await?;
    //stock_data中的ma60是None，手动计算一下。
    let closes = stock_data.iter().map(|item| item.close).collect::<Vec<f64>>();
    let ma_60 = compute_single_ma(60, closes).await;
    // 确保两个Vec的长度相同  
    assert_eq!(stock_data.len(), ma_60.len());
    // 使用zip迭代两个Vec并更新ma60字段  
    for (model, ma60_value) in stock_data.iter_mut().zip(ma_60.iter()) {
        model.ma60 = *ma60_value;
    }
    // stock_data.reverse(); //不倒序的话数据库里的数据日期是由旧到新的
    let _ = StockDataCurd::insert_many(code, stock_data).await?;
    Ok(())
}
// async fn compute_live_ma(code:&str,mas:Vec<f64>) ->AppResult<()>{
// async fn compute_live_ma(code:&str,price:f64,history_close:&Vec<f64>) ->AppResult<()>{
async fn compute_live_ma(price:f64,history_close:&Vec<f64>) ->AppResult<Vec<f64>>{
    // let history_close = CACHE.get().unwrap().get(code).ok_or(anyhow!("缓存中没有{}数据",code))?;
    // let mut data = vec![price];
    // data.extend_from_slice(history_close);
    let live_ma_data = compute_mul_ma(vec![5, 10, 20, 60], price, history_close).await;
    // let mut stock_data = REQUEST.get().unwrap().get_live_stock_data(&vec![code.into()]).await?;
    // let data = stock_data.get(code).unwrap();
    // let open = data.open;
    // // let close = data.price;
    // // let vol = data.volume;
    // init_db_coon().await;
    // // let mut x = StockDataCurd::query_by_nums(code, 59).await?.iter().map(|item|item.close).collect::<Vec<f64>>();
    // let y = StockDataCurd::query_by_nums(code, 19).await?;
    // println!("{:?}", y);
    // let mut x = StockDataCurd::query_by_nums(code, 59).await?.iter().map(|item|item.close).collect::<Vec<f64>>();
    // // x.push(data.price);
    // x.insert(0,data.price);
    // println!("现价是{}",data.price);
    // println!("过去价格是{:?}",x);
    // // let vec1 = compute_single_ma(20, x).await;
    // // x.reverse();
    // let ma_60 = compute_mul_ma(vec![5,10,20,60], &x).await;
    // println!("{:?}",ma_60);
    Ok(live_ma_data)
}
pub async fn handle_new_stock(code:&str,name:&str) ->AppResult<()>{
    let _ = StockInfoCurd::insert(StockInfo::new(code.to_string(), name.to_string())).await?;
    // let _ = handle_and_save_stock_data(code).await?;
    let _ = handle_and_save_stock_data(true,code).await?;
    Ok(())
}
///删除股票
/// 要删除分组和股票关系，股票信息，日线数据表
pub async fn handle_delete_stock(code:&str) ->AppResult<()>{
    let  _ = GroupStockRelationCurd::delete_by_stock_code(code.into()).await?;
    let _ = StockInfoCurd::delete_by_code(code.into()).await?;
    let _ =StockDataCurd::delete_table_by_stock_code(code).await?;
    Ok(())
}
pub async fn handle_stock_livedata(codes:&Vec<String>,now_data:&mut HashMap<String,StockLiveData>,history_price:&HashMap<String,Arc<Vec<f64>>>) ->AppResult<()>{
    for code in codes {
        let live_data = now_data.get_mut(code).ok_or(anyhow!("handle_stock_livedata时没有{}实时数据",code))?;
        let history_data: &Arc<Vec<f64>> = history_price.get(code).ok_or(anyhow!("handle_stock_livedata时没有{}历史价格",code))?;
        let live_ma = compute_live_ma(live_data.price, history_data).await?;
        live_data.ma5 = Some(live_ma[0]);
        live_data.ma10 = Some(live_ma[1]);
        live_data.ma20 = Some(live_ma[2]);
        live_data.ma60 = Some(live_ma[3]);
    }
    Ok(())
}

#[tokio::test]
async fn test_handle_new_stock() {
    init_http().await;
    init_db_coon().await;
    handle_new_stock("512690","白酒ETF").await.unwrap();
}
#[tokio::test]
async fn test_handle() {
    init_http().await;
    init_db_coon().await;
    let code = "515700";
    let stock_data = REQUEST.get().unwrap().get_live_stock_data(&vec![code.into()]).await.unwrap();
    let data = stock_data.get(code).unwrap();
    println!("{:?}",data);
    // compute_live_ma(code,data.price).await.unwrap();
}

