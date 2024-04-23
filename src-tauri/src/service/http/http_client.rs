use anyhow::Context;
use log::info;
use reqwest::Client;
use reqwest::header::HeaderMap;
use crate::app_errors::AppResult;
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
        // let result = self.client.get(url).headers(self.header_map.clone()).send().await?;
        let result = self.client.get(url.clone()).send().await.with_context(||format!("请求url:{}",url))?;
        let stock_data = result.json::<Vec<StockData>>().await?;
        // let closes = stock_data.iter().map(|item| item.close).collect::<Vec<f64>>();
        // let vec = compute_ma(5, closes).await;
        // println!("{:?}", vec);
        // println!("{:?}", stock_data);
        Ok(stock_data)
    }
}
#[tokio::test]
async fn test_get_stock_day_data() {
    init_http().await;
    REQUEST.get().unwrap().get_stock_day_data("600519",1023).await.unwrap();
}