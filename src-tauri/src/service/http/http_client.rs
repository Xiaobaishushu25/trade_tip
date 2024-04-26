use std::collections::HashMap;
use anyhow::{anyhow, Context};
use log::{error, info};
use reqwest::Client;
use reqwest::header::HeaderMap;
use crate::app_errors::AppResult;
use crate::dtos::stock::StockLiveData;
use crate::entities::prelude::StockData;
use crate::service::http::{init_http, REQUEST};
use crate::utils::stock_util::{get_market_by_code};

// pub static REQUEST:OnceLock<HttpRequest> = OnceLock::new();
pub struct HttpRequest{
    header_map: HeaderMap,
    client: Client,
}
impl HttpRequest {
    pub(crate) fn new() ->Self {
        let client = reqwest::Client::new();
        let mut header_map = HeaderMap::new();
        header_map.insert("User-Agent", "Apifox/1.0.0 (https://apifox.com)".parse().unwrap());
        header_map.insert("Accept", "*/*".parse().unwrap());
        HttpRequest{
            header_map,
            client,
        }
    }
    pub async fn get(&self,url:&str)->AppResult<reqwest::Response> {
        info!("发起get请求:{}",url);
        let result = self.client.get(url).headers(self.header_map.clone()).send().await?;
        Ok(result)
    }
    pub async fn post(&self,url:&str,json:&serde_json::Value)->AppResult<reqwest::Response> {
        info!("发起post请求:{}",url);
        let result = self.client.post(url).headers(self.header_map.clone()).json(json).send().await?;
        Ok(result)
    }
    ///注意，这个接口返回的数据只有5、10、20、30日均线的数据。
    /// num是一共需要获取的数据条数
    pub async fn get_stock_day_data(&self,code:&str,num:i32)->AppResult<Vec<StockData>> {
        let url = format!("https://money.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_MarketData.getKLineData?symbol={}{code}&scale=240&ma=5,10,20,30&datalen={num}",get_market_by_code(code)?);
        println!("{:?}", url);
        // let url = format!("https://money.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_MarketData.getKLineData?symbol={}&scale=240&ma=5,10,20,30&datalen={num}",get_market_by_code(code)?);
        // let result = self.client.get(url).headers(self.header_map.clone()).send().await?;
        // let result = self.client.get(url.clone()).send().await.with_context(||format!("请求url:{}",url))?;
        let result = self.client.get(url.clone()).send().await?;
        let stock_data = result.json::<Vec<StockData>>().await.with_context(||format!("发生错误了:{}",url))?;
        // let stock_data = result.json::<Vec<StockData>>().await.with_context(||{error!("发生错误了:{}",url);format!("请求url:{}",url)})?;
        // let stock_data = result.json::<Vec<StockData>>().await?;
        // let closes = stock_data.iter().map(|item| item.close).collect::<Vec<f64>>();
        // let vec = compute_ma(5, closes).await;
        // println!("{:?}", vec);
        // println!("{:?}", stock_data);
        Ok(stock_data)
    }
    // pub async fn get_live_stock_data(&self,codes:Vec<&str>)->AppResult<HashMap<String,StockLiveData>> {
    pub async fn get_live_stock_data(&self,codes:&Vec<String>)->AppResult<HashMap<String,StockLiveData>> {
        let codes = codes.iter().map(|item| format!("{}{item}",get_market_by_code(item).unwrap())).collect::<Vec<String>>();
        let codes = codes.join(",");
        let url = format!("https://qt.gtimg.cn/q={}",codes);
        let result = self.client.get(url.clone()).send().await.with_context(||format!("请求url:{}",url))?;
        let content = result.text().await.unwrap();
        let split = content.split("v_").filter(|item| !item.is_empty()).collect::<Vec<&str>>();
        let mut live_data = HashMap::with_capacity(codes.len());
        for item in split {
            let split = item.split("~").collect::<Vec<&str>>();
            if split.len() >= 33 {
                // println!("name{:?} 价格{:?} 涨跌百分比{:?} code{:?} 开盘价{:?} 最低价{:?} 最高价{:?} 成交量{:?}", 
                //          split[1],split[3],split[32],split[2],split[5],split[34],split[33],split[6]);
                live_data.insert(split[2].to_string(),StockLiveData{
                    code: split[2].to_string(),
                    price: split[3].parse::<f64>()?,
                    change:0f64,
                    percent: split[32].parse::<f64>()?,
                    open: split[5].parse::<f64>()?,
                    low: split[34].parse::<f64>()?,
                    high: split[33].parse::<f64>()?,
                    volume: split[6].parse::<f64>()?,
                    ma5:None,
                    ma10:None,
                    ma20:None,
                    ma30:None ,
                    ma60:None ,
                });
                // let code = split[0];
                // let content = split[1];
                // let split = content.split("~");
                // let split = split.collect::<Vec<&str>>();
                // println!("{:?}", split);
            }
        }
        // let stock_data = result.json::<Vec<StockData>>().await.with_context(||format!("发生错误了:{}",url))?;
        // let stock_data = result.json::<Vec<StockData>>().await.with_context(||{error!("发生错误了:{}",url);format!("请求url:{}",url)})?;
        // let stock_data = result.json::<Vec<StockData>>().await?;
        // let closes = stock_data.iter().map(|item| item.close).collect::<Vec<f64>>();
        // let vec = compute_ma(5, closes).await;
        // println!("{:?}", vec);
        // info!("{:?}", live_data);
        Ok(live_data)
    }
}
#[tokio::test]
async fn test_get_stock_day_data() {
    init_http().await;
    REQUEST.get().unwrap().get_stock_day_data("600519",1023).await.unwrap();
}
#[tokio::test]
async fn test_get_live_data() {
    init_http().await;
    let vec1 = vec!["601288".to_string(), "159967".into(), "512720".into(), "516160".into()];
    REQUEST.get().unwrap().get_live_stock_data(&vec1).await.unwrap();
}