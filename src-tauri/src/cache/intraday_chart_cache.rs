use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bytes::Bytes;
use log::info;
use tokio::time::sleep;
use crate::app_errors::AppResult;
use crate::service::http::REQUEST;

pub struct IntradayChartCache{
    cache:Arc<RwLock<HashMap<String,Option<Bytes>>>>
}
impl IntradayChartCache{
    pub fn new() -> Self{
        Self{
            cache:Arc::new(RwLock::new(HashMap::new()))
        }
    }
    pub async fn get_intraday_chart(&self,code:&str) -> AppResult<Bytes>{
        {
            if let Some(Some(data)) = self.cache.read().unwrap().get(code){
                return Ok(data.clone())
            }
            // if let Some(op) = self.cache.read().unwrap().get(code){
            //     if let Some(data) = op{
            //         return Ok(data.clone())
            //     }
            // }
        }
        let data = REQUEST.get().unwrap().get_intraday_chart_img(code).await?;
        {
            self.cache.write().unwrap().insert(code.to_string(),Some(data.clone()));
        }
        let cache = self.cache.clone();
        let code = code.to_string();
        tokio::spawn(async move{
            sleep(std::time::Duration::from_secs(90)).await;
            info!("缓存过期了，清除");
            cache.write().unwrap().insert(code,None)
        });
        Ok(data)
    }
}