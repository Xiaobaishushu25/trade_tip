use log::info;
use reqwest::Client;
use reqwest::header::HeaderMap;
use crate::app_errors::AppResult;

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
}
// pub fn init_http() {
//     REQUEST.get_or_init(||{
//         info!("初始化http.");
//         HttpRequest::new()
//     });
// }