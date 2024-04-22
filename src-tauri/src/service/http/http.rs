use log::info;
use reqwest::Client;
use reqwest::header::HeaderMap;
use crate::app_errors::AppResult;
use crate::entities::prelude::StockData;
use crate::service::http::{init_http, REQUEST};

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
    pub async fn get_stock_day_data(&self)->AppResult<()> {
        let url = "https://money.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_MarketData.getKLineData?symbol=sh600519&scale=240&ma=10,20,30,60&datalen=1023";
        // let result = self.client.get(url).headers(self.header_map.clone()).send().await?;
        let result = self.client.get(url).headers(self.header_map.clone()).send().await?;
        let model = StockData {
            name: "".to_string(),
            date: "".to_string(),
            open: 0.0,
            close: 0.0,
            high: 0.0,
            low: 0.0,
            vol: 0.0,
            ma5: None,
            ma10: None,
            ma20: None,
            ma30: None,
            ma60: None,
        };
        let vec1 = vec![model];
        let string1 = serde_json::to_string(&vec1).unwrap();
        println!("{}", string1);
        // let string = result.json::<Vec<StockData>>().await.unwrap();
        let string = result.text().await.unwrap();
        println!("{}", string);
        let deserialized: Vec<StockData> = serde_json::from_str(&string).unwrap();
        println!("{:?}",deserialized);
        // let deserialized: Vec<StockData> = serde_json::from_str(&string1).unwrap();
        Ok(())
    }
}
#[tokio::test]
async fn test_get_stock_day_data() {
    init_http().await;
    REQUEST.get().unwrap().get_stock_day_data().await.unwrap();
}