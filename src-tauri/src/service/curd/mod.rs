use log::info;
use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::{init_http, REQUEST};
use crate::utils::stock_util::{calculate_ago_with_num, calculate_ago_with_str, compute_single_ma};

pub mod stock_info_curd;
pub mod stock_data_curd;
pub mod stock_group_curd;
pub mod group_stock_relation_curd;
/// 更新所有股票的日k数据
pub async fn update_all_day_k() ->AppResult<()> {
    let codes = StockInfoCurd::query_all_only_code().await?;
    for code in codes {
        //只有大于1天的才要更新，正常情况下今天的ma数据是么有的，所以最新的就是前一天的，ago应该是1，大于1的说明需要更新
        match StockDataCurd::query_latest(&code).await?{
            Some(latest_data) => {
                let num = calculate_ago_with_str(&latest_data.date);
                println!("{}最新的日期是{:?},距今天的天数是{:?}",code, latest_data.date,num);
                if num>1{
                    //这边的data很有可能多了
                    let data = REQUEST.get().unwrap().get_stock_day_data(&code, num).await?;
                    let index = data.iter()
                        .enumerate() // 将迭代器和索引配对  
                        .find(|&(i, x)| x.date == latest_data.date) // 查找匹配的元素  
                        .map(|(i, _)| i) // 获取索引  
                        .unwrap(); // 假设我们确信可以找到匹配项
                    // let option = data.iter().find(|x| x.date == latest_data.date).unwrap();
                    let data_after_index = &mut data[index + 1..].to_vec(); //这个是由旧日期到新日期的顺序
                    //过去的历史收盘价
                    // let mut history = StockDataCurd::query_by_nums(&code, (60 - data_after_index.len()+1) as i32).await?.iter().map(|item| item.close).collect::<Vec<_>>();
                    let mut history = StockDataCurd::query_by_nums(&code, 59).await?.iter().map(|item| item.close).collect::<Vec<_>>();
                    history.reverse(); //查出的数据是从新到旧，所以要reverse
                    history.extend(data_after_index.iter().map(|item| item.close));
                    println!("{:?}",history.len());
                    let ma_60 = compute_single_ma(60, history).await;
                    let ma_60 = ma_60.into_iter().filter(|x| x.is_some()).collect::<Vec<_>>();
                    // println!("{:?}",history);
                    // 确保两个Vec的长度相同  
                    assert_eq!(data_after_index.len(), ma_60.len());
                    // 使用zip迭代两个Vec并更新ma60字段  
                    for (model, ma60_value) in data_after_index.iter_mut().zip(ma_60.iter()) {
                        model.ma60 = *ma60_value;
                    }
                    println!("待插入数据{:?}",data_after_index);
                    StockDataCurd::insert_many(&code, data_after_index.to_vec()).await.unwrap();
                }
            }
            None => {
                crate::service::command::handle::handle_and_save_stock_data(false,&code).await?;
            }
        };
    }
    Ok(())
}
#[tokio::test]
async fn test_update_all_day_k() {
    init_db_coon().await;
    init_http().await;
    update_all_day_k().await.unwrap();
}