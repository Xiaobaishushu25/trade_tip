use crate::app_errors::AppError::AnyHow;
use crate::app_errors::{AppError, AppResult};
use crate::dtos::stock_dto::StockLiveData;
use crate::entities::prelude::StockData;
use crate::entities::stock_data_time::StockDataTime;
use crate::service::http::{init_http, REQUEST};
use crate::utils::stock_util::{calculate_ago_minutes, get_market_by_code};
use anyhow::{anyhow, Context};
use bytes::Bytes;
use chrono::{Local, NaiveDateTime, NaiveTime};
use log::info;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::{Value};
use std::collections::HashMap;

// pub static REQUEST:OnceLock<HttpRequest> = OnceLock::new();
pub struct HttpRequest {
    header_map: HeaderMap,
    client: Client,
}
impl HttpRequest {
    pub(crate) fn new() -> Self {
        let client = reqwest::Client::new();
        let mut header_map = HeaderMap::new();
        header_map.insert(
            "User-Agent",
            "Apifox/1.0.0 (https://apifox.com)".parse().unwrap(),
        );
        header_map.insert("Accept", "*/*".parse().unwrap());
        HttpRequest { header_map, client }
    }
    pub async fn get(&self, url: &str) -> AppResult<reqwest::Response> {
        info!("发起get请求:{}", url);
        let result = self
            .client
            .get(url)
            .headers(self.header_map.clone())
            .send()
            .await?;
        Ok(result)
    }
    /// 获取股票历史日线数据1
    ///注意，这个接口返回的均线数据只有5、10、20、30日均线的数据。
    /// 注意：获取的数据是按照日期从旧到新排列的
    /// code是股票代码（eg：000001）,注意，不要带市场标识，自动判断
    /// num是一共需要获取的数据条数
    /// 返回值是一个Vec<StockData>，里面包含了股票的日K线数据。
    pub async fn get_stock_day_data(&self, code: &str, num: i32) -> AppResult<Vec<StockData>> {
        let url = format!("https://money.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_MarketData.getKLineData?symbol={}{code}&scale=240&ma=5,10,20,30&datalen={num}",get_market_by_code(code)?);
        println!("{:?}", url);
        // let url = format!("https://money.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_MarketData.getKLineData?symbol={}&scale=240&ma=5,10,20,30&datalen={num}",get_market_by_code(code)?);
        // let result = self.client.get(url).headers(self.header_map.clone()).send().await?;
        // let result = self.client.get(url.clone()).send().await.with_context(||format!("请求url:{}",url))?;
        let result = self.client.get(url.clone()).send().await?;
        let stock_data = result
            .json::<Vec<StockData>>()
            .await
            .with_context(|| format!("发生错误了:{}", url))?;
        Ok(stock_data)
    }
    /// 获取股票历史日线数据2
    /// 注意：获取的数据是按照日期从旧到新排列的，这个接口不返回均线数据。
    /// code是股票代码（eg：sh000001）,注意，需要带市场标识！！
    /// num是一共需要获取的数据条数
    /// 返回值是一个Vec<StockData>，里面包含了股票的日线数据。
    pub async fn get_stock_day_data_with_market(&self, code: &str, num: i32) -> AppResult<Vec<StockData>> {
        let url = format!("https://money.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_MarketData.getKLineData?symbol={code}&scale=240&datalen={num}");
        println!("{:?}", url);
        let result = self.client.get(url.clone()).send().await?;
        let stock_data = result
            .json::<Vec<StockData>>()
            .await
            .with_context(|| format!("发生错误了:{}", url))?;
        Ok(stock_data)
    }
    // pub async fn get_live_stock_data(&self,codes:Vec<&str>)->AppResult<HashMap<String,StockLiveData>> {
    pub async fn get_live_stock_data(
        &self,
        codes: &Vec<String>,
    ) -> AppResult<HashMap<String, StockLiveData>> {
        let codes = codes
            .iter()
            .map(|item| format!("{}{item}", get_market_by_code(item).unwrap()))
            .collect::<Vec<String>>();
        let codes = codes.join(",");
        let url = format!("https://qt.gtimg.cn/q={}", codes);
        info!("发起get请求:{}", url);
        let result = self
            .client
            .get(url.clone())
            .send()
            .await
            .with_context(|| format!("请求url:{}", url))?;
        let content = result.text().await.unwrap();
        let split = content
            .split("v_")
            .filter(|item| !item.is_empty())
            .collect::<Vec<&str>>();
        let mut live_data = HashMap::with_capacity(codes.len());
        for item in split {
            let split = item.split("~").collect::<Vec<&str>>();
            if split.len() >= 33 {
                // println!("name{:?} 价格{:?} 涨跌百分比{:?} code{:?} 开盘价{:?} 最低价{:?} 最高价{:?} 成交量{:?}",
                //          split[1],split[3],split[32],split[2],split[5],split[34],split[33],split[6]);
                live_data.insert(
                    split[2].to_string(),
                    StockLiveData {
                        code: split[2].to_string(),
                        price: split[3].parse::<f64>()?,
                        change: 0f64,
                        percent: split[32].parse::<f64>()?,
                        open: split[5].parse::<f64>()?,
                        low: split[34].parse::<f64>()?,
                        high: split[33].parse::<f64>()?,
                        volume: split[6].parse::<f64>()?,
                        ma5: None,
                        ma10: None,
                        ma20: None,
                        ma30: None,
                        ma60: None,
                    },
                );
            }
        }
        Ok(live_data)
    }
    ///使用腾讯API获取股票分时数据
    /// code: 股票代码 eg. 159967
    /// count: 获取数据条数
    // frequency: 分时线，可选值：'1m','5m','15m','30m','60m'
    /// frequency: 分时线，可选值：'1','5','15','30','60' ,单位是min
    pub(crate) async fn get_price_min_tx(
        &self,
        code: &str,
        count: u32,
        frequency: u32,
    ) -> AppResult<Vec<StockDataTime>> {
        // let ts: u32 = frequency.trim_end_matches('d').parse().unwrap_or(1);
        let code_with_market = format!("{}{}", get_market_by_code(code)?, code);
        let url = format!("http://ifzq.gtimg.cn/appstock/app/kline/mkline?param={code_with_market},m{frequency},,{count}");
        info!("Fetching min stock data from {}", url);
        let resp = self.get(&url).await?.text().await?;
        let json: Value = serde_json::from_str(&resp)?;
        // 把json数据解析，放https://www.json.cn/这里面结构一看一目了然
        let data = json["data"][code_with_market]["m1"]
            .as_array()
            .ok_or(AnyHow(anyhow!("Invalid data format")))?;
        let mut stock_data_time: Vec<StockDataTime> = Vec::new();
        for entry in data {
            let entry_array = entry
                .as_array()
                .ok_or(AnyHow(anyhow!("Invalid entry format")))?;
            if entry_array.len() < 6 {
                continue;
            }
            // Parse each field and convert to appropriate types
            let time_str = entry_array[0]
                .as_str()
                .ok_or(AnyHow(anyhow!("Time parsing error{}", entry_array[0])))?;
            // println!("Time: {}", time_str);//Time: 202410251411
            let time = NaiveDateTime::parse_from_str(time_str, "%Y%m%d%H%M")?;
            let open = entry_array[1].as_str().unwrap().parse::<f64>().unwrap();
            //注意，这里不能直接用to_string()，因为println!("string: {:?}", string); string: "\"0.497\""
            // let open = entry_array[1].to_string().parse::<f64>()?;
            let close = entry_array[2].as_str().unwrap().parse::<f64>()?;
            let high = entry_array[3].as_str().unwrap().parse::<f64>()?;
            let low = entry_array[4].as_str().unwrap().parse::<f64>()?;
            let volume = entry_array[5].as_str().unwrap().parse::<f64>()?;
            stock_data_time.push(StockDataTime {
                time,
                open,
                close,
                high,
                low,
                volume,
            });
        }
        // Update last close price if needed
        if let Some(last_close) = json["data"][code]["qt"][code][3].as_f64() {
            if let Some(last) = stock_data_time.last_mut() {
                last.close = last_close;
            }
        }
        Ok(stock_data_time)
    }
    ///获取股票的日内走势图,其实是一个图片，读取为Bytes
    pub async fn get_intraday_chart_img(&self, code: &str) -> AppResult<Bytes> {
        match get_market_by_code(code)?.as_str() {
            "sh" => {
                let url = format!("https://webquotepic.eastmoney.com/GetPic.aspx?nid=1.{}&imageType=GNR&token=4f1862fc3b5e77c150a2b985b12db0fd",code);
                let result = self
                    .client
                    .get(url.clone())
                    .send()
                    .await
                    .with_context(|| format!("请求url:{}", url))?;
                let content = result.bytes().await.unwrap();
                Ok(content)
            }
            "sz" => {
                let url = format!("https://webquotepic.eastmoney.com/GetPic.aspx?nid=0.{}&imageType=GNR&token=4f1862fc3b5e77c150a2b985b12db0fd",code);
                let result = self
                    .client
                    .get(url.clone())
                    .send()
                    .await
                    .with_context(|| format!("请求url:{}", url))?;
                let content = result.bytes().await.unwrap();
                Ok(content)
            }
            _ => Err(AppError::from(anyhow!("无法判断股票的市场！"))),
        }
    }
}
#[tokio::test]
async fn test_get_stock_day_data() {
    init_http().await;
    REQUEST
        .get()
        .unwrap()
        .get_stock_day_data("600519", 1023)
        .await
        .unwrap();
}
#[tokio::test]
async fn test_get_live_data() {
    init_http().await;
    let vec1 = vec![
        "601288".to_string(),
        "159967".into(),
        "512720".into(),
        "516160".into(),
    ];
    REQUEST
        .get()
        .unwrap()
        .get_live_stock_data(&vec1)
        .await
        .unwrap();
}
#[tokio::test]
async fn test_get_price_min_tx() {
    init_http().await;
    // let code = "159967";
    // let code = "159869";
    let code = "159992";
    let count = calculate_ago_minutes("9:30") as u32;
    let frequency = 1;
    let stock_data = REQUEST
        .get()
        .unwrap()
        .get_price_min_tx(code, count, frequency)
        .await
        .unwrap();
    let start_date_time = Local::now()
        .date_naive()
        .and_time(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    let end_date_time = Local::now()
        .date_naive()
        .and_time(NaiveTime::from_hms_opt(10, 0, 0).unwrap());
    let vec: Vec<(NaiveDateTime, f64)> = stock_data
        .iter()
        .map(|item| (item.time, item.close))
        .filter(|(time, _)| start_date_time <= *time && *time < end_date_time)
        .collect::<Vec<_>>();
    let first = vec.first().unwrap().1;
    let last = vec.last().unwrap().1;
    let num = vec.len();
    let percentage_change = ((last - first) / first) * 100.0;
    println!("first: {:?}, last: {:?}, num: {:?}, percentage_change: {:?}%", first, last, num, percentage_change);
    let slope = (last - first) / (num as f64);
    let line = |x: f64| first + slope * x;
    let res_count = vec.iter()
        .enumerate()
        .filter(|(i, &(_, close))| {
            let x = *i as f64;
            close > line(x)
        })
        .count();
    println!("res_count: {}", res_count);
    // let res = vec.iter()
    //     .enumerate()
    //     .filter_map(|(i, &(time, close))| {
    //         let x = i as f64;
    //         if close > line(x) {
    //             Some((time, close))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect::<Vec<_>>();
    // println!("{:?}", res.len());
    // let filtered_data: Vec<(NaiveDateTime, f64)> = vec
    //     .into_iter()
    //     .filter(|(time, _)| start_date_time <= *time && *time < end_date_time)
    //     .collect();
    //
    // // 打印筛选后的数据
    // for (time, close) in &filtered_data {
    //     println!("Time: {}, Close: {}", time, close);
    // }
}
