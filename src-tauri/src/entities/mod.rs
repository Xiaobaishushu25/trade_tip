use std::{env, fs};
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use log::{error, info};
use sea_orm::{ColIdx, ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;
use crate::app_errors::AppResult;
use crate::entities::table::create_all_need_table;

pub mod table;
pub mod stock_info;
pub(crate) mod prelude;
pub mod stock_data;
pub mod stock_group;
pub mod group_stock_relation;
pub mod graphic;

// pub static DB:OnceLock<DatabaseConnection> = OnceLock::new();
pub static DB:OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn init_db_coon(){
    let current_dir = &env::current_dir().unwrap();
    let current_dir = current_dir.to_string_lossy();
    let path = format!("{}/data/data.db", current_dir);
    let exist = match check_db_file(&path,&current_dir){
        Ok(flag)=>{
            flag
        },
        Err(e)=>{
            error!("数据库文件不存在，创建数据库文件{}失败:{}",path,e.to_string());
            panic!("数据库文件不存在，创建数据库文件{}失败:{}",path,e.to_string())
        }
    };
    DB.get_or_init(|| async {
        // let mut url = "sqlite:".to_string();
        // url.push_str(&*env::current_dir().unwrap().to_string_lossy());
        // url.push_str(&*env::current_dir().unwrap().as_ref());
        // url.push_str("/data/data.db?mode=rwc");
        let url = format!("sqlite:{}?mode=rwc",path);
        // url.push_str("?mode=rwc");
        // url.push_str("?mode=rwc");
        let mut opt = ConnectOptions::new(url);
        opt.max_connections(1000)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);
        let db = Database::connect(opt).await.expect("数据库打开失败");
        if !exist {
            let _ = create_all_need_table(&db).await;
        };
        db
    }).await;
}
//注意，不能与log4rs同时使用
pub async fn open_db_log(){
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}
pub fn check_db_file(path:&str,current_dir:&str)->AppResult<bool>{
    if PathBuf::from(path).exists(){
        info!("数据库存在");
        return Ok(true)
    }else {
        info!("数据库不存在,创建数据库。");
        let _ = fs::create_dir_all(format!("{}/data", current_dir))?;
        let _ = File::create(path)?;
        return Ok(false);
    }
}
#[tokio::test]
async fn test_init_db_coon() {
    init_db_coon().await;
    println!("{:?}", DB.get());
}