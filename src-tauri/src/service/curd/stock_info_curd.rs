use log::info;
use sea_orm::{ActiveModelTrait, ColumnTrait, DeleteResult, EntityOrSelect, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use crate::app_errors::AppResult;
use crate::entities::{init_db_coon, stock_info};

use crate::entities::prelude::{StockInfos,StockInfo,ActiveStockInfo};
use crate::entities::stock_info::{Column, Entity};

pub struct StockInfoCurd;
impl StockInfoCurd {
    pub async fn insert(mut stock_info: StockInfo) -> AppResult<StockInfo> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        //考虑到有删除的情况，所以需要重新计算index
        let num = StockInfos::find().count(db).await?;
        //查询index列的最大值
        // let max_index = StockInfos::find().order_by_desc(Column::Index).limit(1).one(db).await?.unwrap().index;
        // let max_index = StockInfos::find().select_only().column(Column::Index).filter(Column::Index.max()).one(db).await?.unwrap().index;
        let max_index = StockInfos::find().select_only().column(Column::Index).order_by_asc(Column::Index).one(db).await?.unwrap();
        // stock_info.index = max_index + 1;
        //https://www.sea-ql.org/SeaORM/docs/advanced-query/custom-select/#:~:text=By%20default%2C%20SeaORM%20will%20select%20all%20columns%20defined,all%20columns%20assert_eq%21%28cake%3A%3AEntity%3A%3Afind%28%29.build%28DbBackend%3A%3APostgres%29.to_string%28%29%2C%20r%23%22SELECT%20%22cake%22.%22id%22%2C%20%22cake%22.%22name%22%20FROM%20%22cake%22%22%23
        println!("max_index:{:?}",max_index);
        stock_info.index = 1;
        //查询一共有多少条记录
        
        // let model: ActiveStockInfo = StockInfo {
        //     code: stock_info.code,
        //     index: stock_info.index,
        //     name: stock_info.name,
        //     r#box: None,
        //     hold: stock_info.hold,
        // }.into();
        let model: ActiveStockInfo = stock_info.into();
        let result = model.insert(db).await?;
        info!("{:?}",result);
        // let result = ActiveModel::insert(model).exec(&db).await?;
        // let result = StockInfo::insert(model).exec(&db).await?;//InsertResult { last_insert_id: "512764" }
        Ok(result)
    }
    ///根据code删除，返回影响行数
    pub async fn delete_by_code(code: String) -> AppResult<i32> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = StockInfos::find_by_id(code.clone()).one(db).await?.ok_or(anyhow::anyhow!("删除失败:未找到code为{}的记录",code))?;
        let result = model.delete(db).await?;
        // result.rows_affected
        Ok(result.rows_affected as i32)
    }
    ///根据code更新持有情况
    pub async fn update_hold(code:String,hold:bool) -> AppResult<StockInfo> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = StockInfos::find_by_id(code.clone()).one(db).await?.ok_or(anyhow::anyhow!("更新失败:未找到code为{}的记录",code))?;
        let mut active_model: ActiveStockInfo = model.into();
        active_model.hold = Set(hold);
        let new_model = active_model.update(db).await?;
        Ok(new_model)
    }
    ///根据code更新索引
    pub async fn update_index(code:String,index:i32) -> AppResult<StockInfo> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = StockInfos::find_by_id(code.clone()).one(db).await?.ok_or(anyhow::anyhow!("更新失败:未找到code为{}的记录",code))?;
        let mut active_model: ActiveStockInfo = model.into();
        active_model.index = Set(index);
        let new_model = active_model.update(db).await?;
        Ok(new_model)
    }
    pub async fn find_all() -> AppResult<Vec<StockInfo>> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let result = StockInfos::find().order_by_asc(stock_info::Column::Index).all(db).await?;
        Ok(result)
    }
}
#[tokio::test]
async fn test_insert() {
    init_db_coon().await;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let result = StockInfoCurd::insert(StockInfo {
        code: "123459".to_string(),
        index: 0,
        name: "测试".to_string(),
        r#box: None,
        hold: false,
    }).await;
    println!("{:?}",result);
}
#[tokio::test]
async fn test_delete_by_code() {
    init_db_coon().await;
    let result = StockInfoCurd::delete_by_code("123457".to_string()).await;
    println!("{:?}",result);
}
#[tokio::test]
async fn test_update_hold() {
    init_db_coon().await;
    let result = StockInfoCurd::update_hold("124357".to_string(),false).await;
    println!("{:?}",result);
}
#[tokio::test]
async fn test_update_index() {
    init_db_coon().await;
    let result = StockInfoCurd::update_index("123457".to_string(),3).await;
    println!("{:?}",result);
}
#[tokio::test]
async fn test_find_all() {
    init_db_coon().await;
    let result = StockInfoCurd::find_all().await;
    println!("{:?}",result);
}