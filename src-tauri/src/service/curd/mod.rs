use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::entities::position::Model;
use crate::entities::prelude::Position;
use crate::service::command::handle::handle_and_save_stock_data;
use crate::service::curd::position_curd::PositionCurd;
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::{init_http, REQUEST};
use crate::utils::stock_util::{
    calculate_ago_days_with_str, compute_single_ma, get_market_by_code,
};
use anyhow::anyhow;
use chrono::{Duration, Utc};
use itertools::izip;
use log::{error, info};
use std::collections::HashMap;
use async_recursion::async_recursion;
use tokio::time::sleep;

pub mod graphic_curd;
pub mod group_stock_relation_curd;
pub mod position_curd;
pub mod stock_data_curd;
pub mod stock_group_curd;
pub mod stock_info_curd;
pub mod transaction_record_curd;

/// 更新所有股票的日k数据
/// v0.5.2更新：由于0.5.0版本支持了期货，但是期货数据很容易异常，如果发生error就抛回会连带后面大概率正常的股票日线也不更新，
/// 所以对期货进行了特殊处理：如果期货数据异常，仅记录日志但是不返回，继续尝试更新下一个。
#[async_recursion]
pub async fn update_all_day_k(can_handle_futures:bool,second:bool) -> AppResult<()> {
    let mut failure_flag = false;//是否有错误发生，这里用于指示期货数据是否有异常
    //是否需要更新股票的日线数据，因为股票的日线数据一般是统一的，如果一个不需要更新的话其他的也就不更新了。
    //如果不加这个指标，那么要么每个股票都请求一遍（怕频率过高被封ip），要么股票和期货都不请求
    //一般第二次调用是为了再次更新期货数据，股票数据就不需要更新了。
    //0.6.4删除，因为遇到了股票数据只有部分是最新的，导致了剩余的股票不更新日线的情况
    // let mut stock_need_update = !second;
    let codes = StockInfoCurd::query_all_only_code().await?;
    for code in codes {
        //只有大于1天的才要更新，正常情况下今天的ma数据是么有的，所以最新的就是前一天的，ago应该是1，大于1的说明需要更新
        match StockDataCurd::query_latest(&code).await? {
            Some(latest_data) => {
                let num = calculate_ago_days_with_str(&latest_data.date);
                info!(
                    "{}最新的日期是{:?},距今天的天数是{:?}",
                    code, latest_data.date, num
                );
                //0.6.4修改，因为要挨个请求股票日线，一直失败报错：ailed:HttpError(reqwest::Error { kind: Request, url: "https://money.finance.sina.com.cn/quotes_
                // service/api/json_v2.php/CN_MarketData.getKLineData?symbol=sz159866&scale=240&ma=5,10,20,30&datalen=2", source: hyper_util::client::legacy::Error(SendRequest, hyper::Error(Io, Os { code: 10054, kind: ConnectionReset, message: "远程主机强迫关闭了一个现有的连接。" })
                //貌似请求频率太高引起的？但是我在test_get_stock_day_data()中测试一次性请求20次都没问题
                // 但是改成num > 1后过滤掉一些不需要请求的数据（在只有部分不是最新的情况下），可以正常更新了。
                //而且改为大于1是不是可以避免在当天白天更新期货数据，期货数据是有夜盘和白天的两部分的。
                // if num >= 1 {
                if num > 1 {
                    //这边的data很有可能多了,因为有很多非交易日，所以需要过滤
                    //这里要多请求一天。以数据库中最新数据是15号为例，若在17号晚上更新，计算出的num是两天，然后请求出来的是16、17号的数据，
                    //导致在find(|&(_, x)| x.date == latest_data.date)时找不到匹配的元素，后面都无法更新。
                    // let data = REQUEST
                    //     .get()
                    //     .unwrap()
                    //     .get_stock_day_data(&code, num + 1)
                    //     .await?;
                    let (market,is_main_future) = get_market_by_code(&code)?;
                    let is_stock = market == "sh" || market == "sz";
                    let data = if is_stock {
                        // if !stock_need_update{
                        //     continue;
                        // }
                        REQUEST
                            .get()
                            .unwrap()
                            .get_stock_day_data(&code, num + 1)
                            .await?
                    } else {
                        if (!can_handle_futures)||failure_flag{//如果不能处理期货，或者已经有异常就跳过
                            continue;
                        }
                        let now = Utc::now().date_naive();
                        let formatted_now = now.format("%Y-%m-%d").to_string();
                        let futures_result = if is_main_future{
                            REQUEST.get().unwrap().get_futures_main(&code, &latest_data.date, &formatted_now).await
                        }else {
                            REQUEST.get().unwrap().get_futures_daily_history(
                                &code,
                                &latest_data.date,
                                &formatted_now,
                            ).await
                        };
                        // let futures_result = REQUEST
                        //     .get()
                        //     .unwrap()
                        //     .get_futures_daily_history(&code, &latest_data.date, &formatted_now)
                        //     .await;
                        if futures_result.is_err() {
                            error!("更新期货{}的历史日线失败：{:?}",code,futures_result.err().unwrap());
                            failure_flag = true;
                            continue;
                        };
                        let mut data = futures_result.unwrap();
                        // 删除前一天历史数据，当天白天插入的话就会，之后就不会更新夜盘数据，所以直接删掉后加
                        // if let Err(e) = StockDataCurd::delete_with_num(&code, 1).await{
                        //     error!("删除前一天的日线数据失败：{:?}",e);
                        // }else {
                        //     data.pop();
                        // }
                        data //这里的futures_result是Ok(vec)
                    };
                    info!("{:?}更新数据{:?}，最新日期是{:?}",code,data,latest_data.date);
                    let index = data
                        .iter()
                        .enumerate() // 将迭代器和索引配对
                        .find(|&(_, x)| x.date == latest_data.date) // 查找匹配的元素
                        .map(|(i, _)| i) // 获取索引
                        .unwrap();
                    // println!("最新的日期是{:?},索引是{:?}",latest_data.date,index);
                    // let option = data.iter().find(|x| x.date == latest_data.date).unwrap();
                    let data_after_index = &mut data[index + 1..].to_vec(); //这个是由旧日期到新日期的顺序
                    if data_after_index.is_empty() {
                        // if is_stock{
                        //     stock_need_update = false;
                        // }//其实这里也可以判断下给期货_need_update = false，但是期货数据问题很多，还是一个个处理，所以这里就不统一设置不更新了。
                        // return Ok(());//return Ok(())的话如果在有某个股票最新日期不一致时会出错，所以这里要continue
                        continue;//这里的data_after_index是空，说明不需要更新，直接continue到下一个继续
                    }
                    //过去的历史收盘价
                    let mut history =
                        StockDataCurd::query_only_close_price_by_nums(&code, 59).await?;
                    history.reverse(); //查出的数据是从新到旧，所以要reverse
                    history.extend(data_after_index.iter().map(|item| item.close));
                    if is_stock {
                        let ma_60 = compute_single_ma(60, &history).await;
                        let ma_60 = ma_60
                            .into_iter()
                            .filter(|x| x.is_some())
                            .collect::<Vec<_>>();
                        // 确保两个Vec的长度相同
                        assert_eq!(data_after_index.len(), ma_60.len());
                        // 使用zip迭代两个Vec并更新ma60字段
                        for (model, ma60_value) in data_after_index.iter_mut().zip(ma_60.iter()) {
                            model.ma60 = *ma60_value;
                        }
                    } else {
                        let ma_5 = compute_single_ma(5, &history).await;
                        // error!("计算的期货{}的ma5为{:?}",code,ma_5);
                        let ma_10 = compute_single_ma(10, &history).await;
                        let ma_20 = compute_single_ma(20, &history).await;
                        let ma_30 = compute_single_ma(30, &history).await;
                        let ma_60 = compute_single_ma(60, &history).await;
                        // 假设 data_after_index 的长度为 N，ma_5, ma_10 等的长度为 M
                        // 我们需要确保 M >= N，否则会越界

                        // 获取 data_after_index 的长度
                        let data_len = data_after_index.len();

                        // 确保移动平均线的长度足够
                        assert!(ma_5.len() >= data_len, "ma_5 的长度不足");
                        assert!(ma_10.len() >= data_len, "ma_10 的长度不足");
                        assert!(ma_20.len() >= data_len, "ma_20 的长度不足");
                        assert!(ma_30.len() >= data_len, "ma_30 的长度不足");
                        assert!(ma_60.len() >= data_len, "ma_60 的长度不足");

                        // 倒序遍历 data_after_index
                        for i in (0..data_len).rev() {
                            // 获取当前索引对应的移动平均线值
                            let ma5_value = ma_5[ma_5.len() - data_len + i];
                            let ma10_value = ma_10[ma_10.len() - data_len + i];
                            let ma20_value = ma_20[ma_20.len() - data_len + i];
                            let ma30_value = ma_30[ma_30.len() - data_len + i];
                            let ma60_value = ma_60[ma_60.len() - data_len + i];

                            // 赋值给 data_after_index 中的模型
                            let model = &mut data_after_index[i];
                            model.ma5 = ma5_value;
                            model.ma10 = ma10_value;
                            model.ma20 = ma20_value;
                            model.ma30 = ma30_value;
                            model.ma60 = ma60_value;
                        }
                    }
                    // error!("待插入数据{:?}",data_after_index);
                    StockDataCurd::insert_many(&code, data_after_index.to_vec()).await?;
                }
            }
            None => {
                crate::service::command::handle::handle_and_save_stock_data(false, &code, can_handle_futures)
                    .await?;
            }
        };
    }
    if can_handle_futures && failure_flag && !second{//如果有可以处理期货，并且之前失败了，并且是第一次更新，就再更新一次
        error!("first update daily info failed, 20s later will retry again.");
        sleep(std::time::Duration::from_secs(20)).await;
        let _ = update_all_day_k(true,true).await;
        info!("second update daily info ended");
    }
    Ok(())
}
///更新历史持仓数据
/// 首先从持仓表中查询是否有数据
/// 有：读取最后一个仓位占比，作为待插入的默认仓位占比，计算当前日期与已保存数据的距离天数，然后插入所有缺失数据
/// 无：还未开启持仓记录功能，不做任何处理
pub async fn update_all_position() -> AppResult<()> {
    let option_position = PositionCurd::query_latest_position().await?;
    if let Some(position) = option_position {
        let default_position = position.position;
        let latest_date = position.date;
        let days = calculate_ago_days_with_str(latest_date.as_str()) + 1;
        // let mut position = Position::new(date.clone(), position);
        let mut index_map: HashMap<&str, &str> = HashMap::new();
        index_map.insert("sh", "sh000001"); // 上证指数
        index_map.insert("sz", "sz399001"); // 深证成指
        index_map.insert("cyb", "sz399006"); // 创业板指
        index_map.insert("sz50", "sh000016"); // 上证50
        index_map.insert("hs300", "sh000300"); // 沪深300
        index_map.insert("zz500", "sh000905"); // 中证500
        let mut need_insert_data: HashMap<String, Model> = HashMap::with_capacity(days as usize);
        //主要思路挨个请求指数数据，然后根据日期过滤出日期在 latest_date 之后的数据，然后插入到need_insert_data中
        for (key, value) in &index_map {
            let price_data = REQUEST
                .get()
                .unwrap()
                .get_stock_day_data_with_market(value, days)
                .await?;
            let data_after_latest_date = price_data
                .iter()
                .filter(|item| item.date > latest_date) // 过滤出日期在 latest_date 之后的数据
                .collect::<Vec<_>>();
            if data_after_latest_date.is_empty() {
                info!("{}数据为空，不更新持仓仓位变化数据", key);
                return Ok(());
            }
            //将数据插入到need_insert_data中，如果目前还不存在，就新建，否则就直接设置字段值
            for item in data_after_latest_date {
                if let Some(data) = need_insert_data.get_mut(&item.date) {
                    data.set_field(key, item.close);
                } else {
                    let mut new_position = Position::new(item.date.clone(), default_position);
                    new_position.set_field(key, item.close);
                    need_insert_data.insert(item.date.clone(), new_position);
                }
            }
        }
        //注意是.cloned()而不是.clone()，这里.cloned()可以获得所有权
        PositionCurd::insert_many_positions(need_insert_data.values().cloned().collect()).await?;
    }
    Ok(())
}
#[tokio::test]
async fn test_update_all_day_k() {
    init_db_coon().await;
    init_http().await;
    update_all_day_k(true,false).await.unwrap();
}
