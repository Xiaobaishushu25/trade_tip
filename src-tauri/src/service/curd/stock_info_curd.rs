use log::info;
use sea_orm::{ActiveModelTrait, DeleteResult, EntityTrait, ModelTrait, PaginatorTrait};
use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
// use crate::dtos::stock_info::StockInfo;

// use crate::entities::stock_info::{ActiveModel, Model};
use crate::entities::prelude::{StockInfos,StockInfo,ActiveStockInfo};
pub struct StockInfoCurd;
impl StockInfoCurd {
    pub async fn insert(mut stock_info: StockInfo) -> AppResult<StockInfo> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let num = StockInfos::find().count(db).await?;
        stock_info.index = (num + 1) as i32;
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
    ///根据code删除
    pub async fn delete_by_code(code: String) -> AppResult<i32> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = StockInfos::find_by_id(code.clone()).one(db).await?.ok_or(anyhow::anyhow!("删除失败:未找到code为{}的记录",code))?;
        let result = model.delete(db).await?;
        // result.rows_affected
        Ok(result.rows_affected as i32)
    }
}
#[tokio::test]
async fn test_insert() {
    init_db_coon().await;
    let result = StockInfoCurd::insert(StockInfo {
        code: "123457".to_string(),
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