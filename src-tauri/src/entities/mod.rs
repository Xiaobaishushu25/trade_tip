use std::env;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

pub mod table;
pub mod stock_info;
pub(crate) mod prelude;

// pub static DB:OnceLock<DatabaseConnection> = OnceLock::new();
pub static DB:OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn init_db_coon(){
    DB.get_or_init(|| async {
        let mut url = "sqlite:".to_string();
        url.push_str(&*env::current_dir().unwrap().to_string_lossy());
        url.push_str("/data.db?mode=rwc");
        let mut opt = ConnectOptions::new(url.to_owned());
        opt.max_connections(1000)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);
        Database::connect(opt).await.expect("数据库打开失败")
    }).await;
}
//注意，不能与log4rs同时使用
pub async fn open_db_log(){
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}
#[tokio::test]
async fn test_init_db_coon() {
    init_db_coon().await;
    println!("{:?}", DB.get());
}