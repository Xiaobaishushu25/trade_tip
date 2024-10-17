use crate::app_errors::AppResult;
use crate::config::config::Config;
use std::sync::{Arc, RwLock};

pub struct ConfigState {
    pub config: Arc<RwLock<Config>>, // 使用Arc共享RwLock，保证多线程安全，提供内部可变性
}
impl ConfigState {
    pub async fn new() -> Self {
        let config = Config::load().await;
        ConfigState {
            config: Arc::new(RwLock::new(config)),
        }
    }
    pub async fn update_config(&self, config: Config) -> AppResult<()> {
        self.config.write().unwrap().update(config).await?;
        Ok(())
    }
}
