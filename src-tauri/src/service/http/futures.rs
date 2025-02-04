use std::collections::HashMap;
use anyhow::Context;
use itertools::Itertools;
use log::{error, info};
use serde_json::Value;
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::dtos::stock_dto::StockLiveData;
use crate::entities::init_db_coon;
use crate::entities::prelude::StockData;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::http_client::HttpRequest;
use crate::service::http::{init_http, start_data_server, REQUEST};

// 定义常量，用于存储API的URL。
const SERVER_URL: &str = "http://127.0.0.1:8080/api/public";
impl HttpRequest{
    ///https://akshare.akfamily.xyz/data/futures/futures.html#id57
    pub async fn get_futures_daily_history(&self,symbol: &str,
                                           start_date: &str,
                                           end_date: &str,) -> AppResult<Vec<StockData>> {
        let params = HashMap::from([
            ("symbol", symbol),
            ("period", "daily"),
            ("start_date", start_date),
            ("end_date", end_date),
        ]);
        let endpoint = "futures_hist_em";
        let full_url = format!("{}/{}", SERVER_URL, endpoint);
        info!("{}", full_url);
        let response = self.client.get(&full_url).query(&params).send().await?;
        let text = response.status();
        let item_list:Vec<Value> = response.json().await.with_context(|| {
            format!("解析response.json()失败,错误码{}",text)
        })?;
        // let item_list:Vec<Value> = response.json().await.context("解析futures_daily_history失败")?;
        let mut stock_data_list = Vec::new();
        for item in item_list {
            //2025-01-20T00:00:00.000
            let date = item["时间"].as_str().unwrap().chars().take(10).collect::<String>();
            let open = item["开盘"].as_f64().unwrap();
            let close = item["收盘"].as_f64().unwrap();
            let high = item["最高"].as_f64().unwrap();
            let low = item["最低"].as_f64().unwrap();
            let volume = item["成交量"].as_f64().unwrap();
            let stock_data = StockData {
                date,
                open,
                close,
                high,
                low,
                vol: volume,
                ma5: None,
                ma10: None,
                ma20: None,
                ma30: None,
                ma60: None,
            };
            stock_data_list.push(stock_data);
        }
        Ok(stock_data_list)
    }
    /// 获取期货实时数据
    ///
    /// 该函数根据传入的期货代码列表 (`codes`)，查询每个代码对应的实时市场数据，并返回一个包含实时数据的 `HashMap`。
    /// 每个期货代码的实时数据包括价格、涨跌幅、开盘价、最高价、最低价、成交量等信息。
    ///
    /// # 参数
    /// - `codes`: 一个包含期货代码的 `Vec<String>`。如果列表为空，函数将直接返回一个空的 `HashMap`。
    ///
    /// # 返回值
    /// - 返回一个 `AppResult<HashMap<String, StockLiveData>>`，其中：
    ///   - `HashMap` 的键是期货代码（`String`）。
    ///   - `HashMap` 的值是 `StockLiveData` 结构体，包含期货的实时市场数据。
    ///
    /// # 错误
    /// - 如果某个期货代码的信息未找到，将返回一个 `Tip` 错误，提示 `"{code}信息未找到"`。
    /// - 如果数据获取或处理过程中发生其他错误，将返回相应的 `AppResult::Err`。
    ///
    /// # 实现细节
    /// 1. 如果 `codes` 为空，直接返回一个空的 `HashMap`。
    /// 2. 遍历 `codes`，跳过已经处理过的代码。
    /// 3. 对于每个代码，通过 `StockInfoCurd::query_info_by_code` 查询期货的详细信息。
    /// 4. 从期货名称中过滤掉数字部分，得到用于查询的符号（`symbol`）。
    /// 5. 调用 `get_futures_live_data_by_symbol` 获取该符号的实时数据。
    /// 6. 将实时数据解析为 `StockLiveData` 结构体，并插入到 `HashMap` 中。
    /// 7. 返回包含所有期货实时数据的 `HashMap`。
    pub async fn get_futures_live_datas(&self,codes: Vec<String>)->AppResult<HashMap<String, StockLiveData>>{
        if codes.is_empty() {
            return Ok(HashMap::new());
        }
        // let codes = codes.iter().map(|code| code.to_lowercase()).collect::<Vec<_>>();
        let mut live_data = HashMap::with_capacity(codes.len());
        let mut is_handled = Vec::new();
        for code in &codes {
            if is_handled.contains(code) {
                info!("{}已处理,", code);
                continue;
            }
            let info = StockInfoCurd::query_info_by_code(code.clone()).await?.ok_or(Tip(format!("{}信息未找到", code)))?;
            let full_name = info.name;
            let name: String = full_name.chars()
                .filter(|c| !c.is_digit(10)) // Filter out digits
                .collect::<String>();
            let item_list = self.get_futures_live_data_by_symbol(&name).await?;
            for item in item_list {
                let mut symbol = item["symbol"].as_str().unwrap().to_lowercase();//ma505
                //特殊处理，因为甲醇在东方财富是MA505，但是通过那个接口获取的是MA2505
                //又因为目前只有甲醇的代码是大写的，其他都是小写的，所以需要特殊处理一下
                if symbol.contains("ma2"){
                    symbol = symbol.replace("ma2", "MA");
                };
                if codes.contains(&symbol){
                    let percent = item["changepercent"].as_f64().unwrap() * 100.0;
                    let percent_rounded: f64 = format!("{:.2}", percent).parse().unwrap();
                    live_data.insert(
                        symbol.clone(),
                        StockLiveData {
                            code: symbol.clone(),
                            price: item["trade"].as_f64().unwrap(),
                            change: 0f64,
                            percent: percent_rounded,
                            open: item["open"].as_f64().unwrap(),
                            low: item["low"].as_f64().unwrap(),
                            high: item["high"].as_f64().unwrap(),
                            volume: item["volume"].as_f64().unwrap(),
                            ma5: None,
                            ma10: None,
                            ma20: None,
                            ma30: None,
                            ma60: None,
                        },
                    );
                    is_handled.push(symbol.clone());
                }
            }
        }
        Ok(live_data)
    }
    ///https://akshare.akfamily.xyz/data/futures/futures.html#id55
    /// http://127.0.0.1:8080/api/public/futures_symbol_mark 获取所有的期货品种
    /// name：eg 花生
    async fn get_futures_live_data_by_symbol(&self, name: &str) ->AppResult<Vec<Value>>{
        let name = match name{
            "热卷" => "热轧卷板",
            "淀粉" => "玉米淀粉",
            "甲醇" => "郑醇",
            _ => name,
        };
        let params = HashMap::from([
            ("symbol", name),
        ]);
        let endpoint = "futures_zh_realtime";
        let full_url = format!("{}/{}", SERVER_URL, endpoint);
        info!("查询{}期货实时数据:{}", name, full_url);
        let response = self.client.get(&full_url).query(&params).send().await?;
        let item_list:Vec<Value> = response.json().await.with_context(|| {format!("解析{}期货实时数据失败.",name)})?;
        // println!("{}", item_list);
        Ok(item_list)
    }
    ///https://akshare.akfamily.xyz/data/futures/futures.html#id54
    /// 这个不好用，目前没用到
    pub async fn get_futures_live_data2(&self,symbol: &str)->AppResult<()>{
        let params = HashMap::from([
            ("symbol", symbol),
            ("market", "CF"),
            ("adjust", "0"),
        ]);
        let endpoint = "futures_zh_spot";
        //http://127.0.0.1:8080/api/public/futures_zh_spot?market=CF&adjust=0&symbol=SS2503
        //试了PK2505（花生）怎么不行啊
        let full_url = format!("{}/{}", SERVER_URL, endpoint);
        println!("{}", full_url);
        let response = self.client.get(&full_url).query(&params).send().await?;
        println!("{:?}", response);
        let item_list:Value = response.json().await?;
        println!("{}", item_list);
        Ok(())
    }
}
#[tokio::test]
async fn test_get_futures_daily_history() {
    init_http().await;
    // tokio::spawn(async {
    //     start_data_server("E:\\ANACONDA\\Main\\Scripts\\activate.bat && conda activate akenv &&python -m aktools").await.unwrap();
    // });
    REQUEST.get().unwrap().get_futures_daily_history("rb2505", "20241225", "20250127").await.unwrap();
}
#[tokio::test]
async fn test_get_futures_live_data() {
    init_http().await;
    // REQUEST.get().unwrap().get_futures_live_data("豆粕").await.unwrap();
    REQUEST.get().unwrap().get_futures_live_data2("HC2505").await.unwrap();
}
#[tokio::test]
async fn test_get_futures_live_data_by_codes() {
    init_http().await;
    init_db_coon().await;
    let codes = vec![
        "rb2505".to_string(),
        "hc2503".to_string(),
        "i2505".to_string(),
    ];
    REQUEST.get().unwrap().get_futures_live_datas(codes).await.unwrap();
}