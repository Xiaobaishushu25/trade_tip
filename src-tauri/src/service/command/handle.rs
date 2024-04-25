use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::entities::prelude::StockInfo;
use crate::entities::table::{create_table_with_dyn_name};
use crate::service::curd::group_stock_relation_curd::GroupStockRelationCurd;
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::{init_http, REQUEST};
use crate::utils::stock_util::{calculate_day_num, compute_ma};
///处理并保存股票数据
/// 需要先根据code创建表，然后处理日线数据（主要是计算ma60）
async fn handle_and_save_stock_data(code:&str) ->AppResult<()>{
    let _ = create_table_with_dyn_name(code).await?;
    let mut stock_data = REQUEST.get().unwrap().get_stock_day_data(&code, calculate_day_num()).await?;
    //stock_data中的ma60是None，手动计算一下。
    let closes = stock_data.iter().map(|item| item.close).collect::<Vec<f64>>();
    let ma_60 = compute_ma(60, closes).await;
    // println!("{:?}", vec);
    // println!("{:?}", stock_data);
    // 确保两个Vec的长度相同  
    assert_eq!(stock_data.len(), ma_60.len());
    // 使用zip迭代两个Vec并更新ma60字段  
    for (model, ma60_value) in stock_data.iter_mut().zip(ma_60.iter()) {
        model.ma60 = *ma60_value;
    }
    stock_data.reverse();
    let _ = StockDataCurd::insert_many(code, stock_data).await?;
    Ok(())
}
pub async fn handle_new_stock(code:&str,name:&str) ->AppResult<()>{
    let _ = StockInfoCurd::insert(StockInfo::new(code.to_string(), name.to_string())).await?;
    let _ = handle_and_save_stock_data(code).await?;
    Ok(())
}
pub async fn handle_delete_stock(code:&str) ->AppResult<()>{
    let  _ = GroupStockRelationCurd::delete_by_stock_code(code.into()).await?;
    let _ = StockInfoCurd::delete_by_code(code.into()).await?;
    let _ =StockDataCurd::delete_table_by_stock_code(code).await?;
    Ok(())
}

#[tokio::test]
async fn test_handle_new_stock() {
    init_http().await;
    init_db_coon().await;
    handle_new_stock("512690","白酒ETF").await.unwrap();
}