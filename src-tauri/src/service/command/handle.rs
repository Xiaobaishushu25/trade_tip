use crate::app_errors::AppError::AnyHow;
use crate::app_errors::{AppResult};
use crate::dtos::stock_dto::StockLiveData;
use crate::entities::init_db_coon;
use crate::entities::prelude::{Position, Positions, StockInfo, TransactionRecord};
use crate::entities::table::create_table_with_dyn_name;
use crate::service::curd::graphic_curd::GraphicCurd;
use crate::service::curd::group_stock_relation_curd::GroupStockRelationCurd;
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::curd::transaction_record_curd::TransactionRecordCurd;
use crate::service::http::{init_http, REQUEST};
use crate::utils::stock_util::{calculate_ago_minutes, calculate_ago_days_with_num, compute_mul_ma, compute_single_ma, calculate_ago_days_with_str};
use anyhow::anyhow;
use log::{error, info, warn};
use std::collections::HashMap;
use std::ops::Sub;
use std::sync::Arc;
use chrono::{Local, NaiveDateTime, NaiveTime};
use crate::config::config::{Config, DataConfig};
use crate::service::curd::position_curd::PositionCurd;

///处理并保存股票数据
/// 需要先根据code创建表，然后处理日线数据（主要是计算ma60，因为直接请求的日线数据没有ma60）
pub(crate) async fn handle_and_save_stock_data(create_need: bool, code: &str) -> AppResult<()> {
    // async fn handle_and_save_stock_data(create_need:bool,code:&str,num:i32,date:Option<String>) ->AppResult<()>{
    if create_need {
        create_table_with_dyn_name(code).await?;
    }
    // let _ = create_table_with_dyn_name(code).await?;
    // let mut stock_data = REQUEST.get().unwrap().get_stock_day_data(&code, calculate_day_num()).await?;
    //不需要那么多的数据，再减三百，因为有大量不是交易日的日期。
    let mut stock_data = REQUEST
        .get()
        .unwrap()
        .get_stock_day_data(&code, calculate_ago_days_with_num(2020, 1, 1) - 300)
        .await?;
    // let mut stock_data = REQUEST.get().unwrap().get_stock_day_data(&code, num).await?;
    //stock_data中的ma60是None，手动计算一下。
    let closes = stock_data
        .iter()
        .map(|item| item.close)
        .collect::<Vec<f64>>();
    let ma_60 = compute_single_ma(60, closes).await;
    // 确保两个Vec的长度相同
    assert_eq!(stock_data.len(), ma_60.len());
    // 使用zip迭代两个Vec并更新ma60字段
    for (model, ma60_value) in stock_data.iter_mut().zip(ma_60.iter()) {
        model.ma60 = *ma60_value;
    }
    // stock_data.reverse(); //不倒序的话数据库里的数据日期是由旧到新的
    StockDataCurd::insert_many(code, stock_data).await?;
    Ok(())
}
// async fn compute_live_ma(code:&str,mas:Vec<f64>) ->AppResult<()>{
// async fn compute_live_ma(code:&str,price:f64,history_close:&Vec<f64>) ->AppResult<()>{
async fn compute_live_ma(price: f64, history_close: &Vec<f64>) -> AppResult<Vec<f64>> {
    let live_ma_data = compute_mul_ma(vec![5, 10, 20, 60], price, history_close).await;
    Ok(live_ma_data)
}
pub async fn handle_new_stock(code: &str, name: &str) -> AppResult<()> {
    let _ = StockInfoCurd::insert(StockInfo::new(code.to_string(), name.to_string())).await?;
    // let _ = handle_and_save_stock_data(code).await?;
    handle_and_save_stock_data(true, code).await?;
    Ok(())
}
///删除股票
/// 要删除分组和股票关系，股票信息，日线数据表, 图形数据
pub async fn handle_delete_stock(code: &str) -> AppResult<()> {
    GroupStockRelationCurd::delete_by_stock_code(code.into()).await?;
    StockInfoCurd::delete_by_code(code.into()).await?;
    StockDataCurd::delete_table_by_stock_code(code).await?;
    GraphicCurd::delete_by_code(code.into()).await?;
    Ok(())
}
pub async fn handle_stock_livedata(
    codes: &Vec<String>,
    now_data: &mut HashMap<String, StockLiveData>,
    history_price: &HashMap<String, Arc<Vec<f64>>>,
) -> AppResult<()> {
    for code in codes {
        let live_data = now_data
            .get_mut(code)
            .ok_or(anyhow!("handle_stock_livedata时没有{}实时数据", code))?;
        let history_data: &Arc<Vec<f64>> = history_price
            .get(code)
            .ok_or(anyhow!("handle_stock_livedata时没有{}历史价格", code))?;
        let live_ma = compute_live_ma(live_data.price, history_data).await?;
        live_data.ma5 = Some(live_ma[0]);
        live_data.ma10 = Some(live_ma[1]);
        live_data.ma20 = Some(live_ma[2]);
        live_data.ma60 = Some(live_ma[3]);
    }
    Ok(())
}

///以下区域是历史交易数据处理函数
/// 处理和保存交易记录
/// 读取csv文件，然后查询数据库中最新记录，然后只保存最新记录之后的数据并返回。
///
pub(crate) async fn handle_and_save_record(path: String) -> AppResult<Vec<TransactionRecord>> {
    //read_csv_file已经做了排序，所以不需要再排序
    let mut pending_data = TransactionRecordCurd::read_csv_file(&path).await?;
    //把pending_data中的数据按日期升序排序
    // pending_data.sort_by(|a, b| a.date.cmp(&b.date));
    // pending_data.reverse();
    // pending_data.sort_by(|a, b| b.date.cmp(&a.date));
    let truncated_data =
        if let Some(latest_record) = TransactionRecordCurd::query_latest_record().await? {
            let latest_key = (
                latest_record.date.clone(),
                latest_record.time.clone(),
                latest_record.code.clone(),
            );
            // 找到最新记录的索引
            let latest_index = pending_data.iter().position(|record| {
                let key = (
                    record.date.clone(),
                    record.time.clone(),
                    record.code.clone(),
                );
                key == latest_key
            });
            if let Some(index) = latest_index {
                // 只保留最新记录之后的数据
                info!("最新记录是：{:?}", latest_record);
                &pending_data[index + 1..]
            } else {
                info!("没有找到最新记录");
                // 如果没有找到最新记录，则保留所有数据
                &pending_data
            }
        } else {
            info!("目前没有交易记录");
            // 如果没有最新记录，则保留所有数据
            &pending_data
        };
    info!("处理待插入的交易记录：{:?}", truncated_data);
    //如果插入出错，则返回错误，不会走到Ok(truncated_data.to_vec())返回数据。
    match TransactionRecordCurd::insert(truncated_data.to_vec()).await {
        Ok(_) => {
            let mut data = truncated_data.to_vec();
            info!("插入交易记录成功：{:?}", data);
            data.reverse(); //需要再倒序，不然返给前端的是反的。
            Ok(data)
        }
        Err(e) => {
            error!("待处理的全部交易记录：{:?}", pending_data);
            error!("当前待插入的交易记录：{:?}", truncated_data);
            Err(AnyHow(anyhow::anyhow!("插入交易记录失败：{}", e)))
        }
    }
    // Ok(truncated_data.to_vec())
}
/// 处理股票，判断能不能做t，如果能做t，则返回up或者down，否则返回normal。
/// return: Vec<(code, status)> ，其中status为up或者down或者normal
/// 策略：取当天9：30（价格a）至10:00（价格b）之间的数据
///     若a<b，且大部分时间股价在均线（分时线也有一个均线）上，若10:00后突破其中最高价，则可以买入。
///     若a>b，且大部分时间股价在均线（分时线也有一个均线）下，若10:00后跌破其中最低价，则可以卖出。
/// 由于均线不好搞，我直接用一个函数let line = |x: f64| first + slope * x;来作为均线。
/// todo 用直线函数不准确，待优化
/// 判断标准是：70%的时间股价在均线上方，且b相较于a涨了0.1%以上，则视为up。
///          70%的时间股价在均线下方，且b相较于a跌了0.1%以上，则视为down。
pub async fn handle_can_t(codes: Vec<String>,data_config:&DataConfig) -> AppResult<Vec<(String, String)>> {
    // warn!("up trigger:{:?}, down trigger:{:?}",data_config.up_t_trigger,data_config.down_t_trigger);
    let mut can_t = Vec::with_capacity(codes.len());
    let start_date_time = Local::now()
        .date_naive()
        .and_time(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    //获得start_date_time的前一天
    // let start_date_time = start_date_time.sub(chrono::Duration::days(1));
    let end_date_time = Local::now()
        .date_naive()
        .and_time(NaiveTime::from_hms_opt(10, 0, 0).unwrap());
    // let end_date_time = end_date_time.sub(chrono::Duration::days(1));
    let count = calculate_ago_minutes("9:30") as u32;
    let frequency = 1;
    for code in codes {
        let stock_data = REQUEST
            .get()
            .unwrap()
            .get_price_min_tx(&code, count, frequency)
            .await?;
        let vec: Vec<(NaiveDateTime, f64)> = stock_data
            .iter()
            .map(|item| (item.time, item.close))
            .filter(|(time, _)| start_date_time <= *time && *time < end_date_time)
            .collect::<Vec<_>>();
        if vec.len() < 2 {
            can_t.push((code.clone(), "normal".into()));
            continue;
        }
        let first = vec.first().unwrap().1;
        let last = vec.last().unwrap().1;
        let num = vec.len();
        // let percentage_change = (((last - first) / first) * 100.0).abs();
        let percentage_change = ((last - first) / first) * 100.0;
        let slope = (last - first) / (num as f64);
        let line = |x: f64| first + slope * x;
        let res_count = vec.iter()
            .enumerate()
            .filter(|(i, &(_, close))| {
                let x = *i as f64;
                close > line(x)
            })
            .count();
        warn!("code:{:?},first: {:?}, last: {:?}, num: {:?},res_count: {:?}, percentage_change: {:?}%",code, first, last, num,res_count, percentage_change);
        // if (res_count as f64)/(num as f64) > 0.7&&percentage_change > 0.1 {
        if (res_count as f64)/(num as f64) > 0.7&&percentage_change > data_config.up_t_trigger {
            can_t.push((code.clone(), "up".into()));
        // }else if (res_count as f64)/(num as f64) < 0.3&&percentage_change < 0.1 {
        }else if (res_count as f64)/(num as f64) < 0.3&&percentage_change < data_config.down_t_trigger {
            can_t.push((code.clone(), "down".into()));
        }else {
            can_t.push((code.clone(), "normal".into()));
        }
    }
    Ok(can_t)
}
/// 处理更新持仓操作
/// return: (插入false/更新true)
///先尝试直接更新，如果更新失败，再尝试插入。
pub async fn handle_update_position(date: String, position_num: f64) -> AppResult<bool> {
    match PositionCurd::update_position_by_id(date.clone(), position_num).await{
        Ok(_) => Ok(true),
        Err(e) => {
            if e.to_string().contains("未找到") {
                let days = calculate_ago_days_with_str(date.as_str())+1;
                let mut position = Position::new(date.clone(), position_num);
                let mut index_map: HashMap<&str, &str> = HashMap::new();
                index_map.insert("sh", "sh000001");        // 上证指数
                index_map.insert("sz", "sz399001");        // 深证成指
                index_map.insert("cyb", "sz399006");       // 创业板指
                index_map.insert("sz50", "sh000016");      // 上证50
                index_map.insert("hs300", "sh000300");     // 沪深300
                index_map.insert("zz500", "sh000905");     // 中证500
                for (key, value) in &index_map {
                    let k_data = REQUEST.get().unwrap().get_stock_day_data_with_market(value, days).await?;
                    let data = k_data.iter().find(|item| item.date == date).ok_or(anyhow!("没有找到{}的数据",date))?;
                    position.set_field(key, data.close);
                }
                PositionCurd::insert_position(position).await?;
                Ok(false)
            }else {
                Err(AnyHow(anyhow::anyhow!("插入持仓记录失败：{}", e)))
            }
        }
    }
}
#[tokio::test]
async fn test_handle_new_stock() {
    init_http().await;
    init_db_coon().await;
    handle_new_stock("512690", "白酒ETF").await.unwrap();
}
#[tokio::test]
async fn test_handle() {
    init_http().await;
    init_db_coon().await;
    let code = "515700";
    let stock_data = REQUEST
        .get()
        .unwrap()
        .get_live_stock_data(&vec![code.into()])
        .await
        .unwrap();
    let data = stock_data.get(code).unwrap();
    println!("{:?}", data);
    // compute_live_ma(code,data.price).await.unwrap();
}
// #[tokio::test]
// async fn test_handle_insert_position() {
//     init_http().await;
//     handle_insert_position("2024-11-07".into(), 10000.0).await.unwrap();
// }