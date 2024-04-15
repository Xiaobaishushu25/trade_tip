use log::info;
use tokio::sync::OnceCell;
use crate::service::http::http::{HttpRequest};

pub mod http;
pub static REQUEST:OnceCell<HttpRequest> = OnceCell::const_new();
pub async fn init_http() {
    REQUEST.get_or_init(||async {
        info!("初始化http.");
        HttpRequest::new()
    }).await;
}