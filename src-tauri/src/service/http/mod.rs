use crate::service::http::http_client::HttpRequest;
use log::info;
use tokio::sync::OnceCell;

pub mod http_client;
pub static REQUEST: OnceCell<HttpRequest> = OnceCell::const_new();
pub async fn init_http() {
    REQUEST
        .get_or_init(|| async {
            info!("初始化http.");
            HttpRequest::new()
        })
        .await;
}
