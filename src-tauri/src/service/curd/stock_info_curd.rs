use crate::app_errors::AppResult;
use crate::entities::{init_db_coon, stock_info};
use log::info;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::entities::prelude::{ActiveStockInfo, StockInfo, StockInfos};
use crate::entities::stock_info::{Column, Entity};
use crate::service::curd::group_stock_relation_curd::MoreStockInfo;

pub struct StockInfoCurd;
impl StockInfoCurd {
    ///按照给定的stock_info插入数据
    /// 注意，除了index字段（index字段会查询当前index列最大值并+1插入），其他字段原样插入
    pub async fn insert(mut stock_info: StockInfo) -> AppResult<StockInfo> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        //考虑到有删除的情况，所以需要重新计算index
        // let num = StockInfos::find().count(db).await?;
        //查询index列的最大值
        // let max_index = StockInfos::find().order_by_desc(Column::Index).limit(1).one(db).await?.unwrap().index;
        // let max_index = StockInfos::find().select_only().column(Column::Index).filter(Column::Index.max()).one(db).await?.unwrap().index;
        // let max_index = StockInfos::find().select_only().column(Column::Index).order_by_asc(Column::Index).one(db).await?.unwrap();
        // let max_index = StockInfos::find().select_only().expr(Column::Index.max()).into_tuple::<Option<i32>>().one(db).await?.unwrap().unwrap_or(0);
        // stock_info.index = max_index+1;
        let model: ActiveStockInfo = stock_info.into();
        let result = model.insert(db).await?;
        // info!("{:?}",result);
        // let result = ActiveModel::insert(model).exec(&db).await?;
        // let result = StockInfo::insert(model).exec(&db).await?;//InsertResult { last_insert_id: "512764" }
        Ok(result)
    }
    ///根据code删除，返回影响行数
    pub async fn delete_by_code(code: String) -> AppResult<i32> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = StockInfos::find_by_id(code.clone())
            .one(db)
            .await?
            .ok_or(anyhow::anyhow!("删除失败:未找到code为{}的记录", code))?;
        let result = model.delete(db).await?;
        // result.rows_affected
        Ok(result.rows_affected as i32)
    }
    ///根据code更新持有情况
    pub async fn update_hold(code: String, hold: bool) -> AppResult<StockInfo> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = StockInfos::find_by_id(code.clone())
            .one(db)
            .await?
            .ok_or(anyhow::anyhow!("更新失败:未找到code为{}的记录", code))?;
        let mut active_model: ActiveStockInfo = model.into();
        active_model.hold = Set(hold);
        let new_model = active_model.update(db).await?;
        Ok(new_model)
    }
    ///根据code更新索引
    // pub async fn update_index(code:String,index:i32) -> AppResult<StockInfo> {
    //     let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
    //     let model = StockInfos::find_by_id(code.clone()).one(db).await?.ok_or(anyhow::anyhow!("更新失败:未找到code为{}的记录",code))?;
    //     let mut active_model: ActiveStockInfo = model.into();
    //     active_model.index = Set(index);
    //     let new_model = active_model.update(db).await?;
    //     Ok(new_model)
    // }
    ///查询所有(按照索引排序)
    pub async fn find_all() -> AppResult<Vec<StockInfo>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        // let result = StockInfos::find().order_by_asc(stock_info::Column::Index).all(db).await?;
        let result = StockInfos::find().all(db).await?;
        Ok(result)
    }
    pub async fn find_all_hold() -> AppResult<Vec<MoreStockInfo>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let infos = StockInfos::find()
            .filter(Column::Hold.eq(true))
            .all(db)
            .await?
            .into_iter()
            .map(|model|{
                MoreStockInfo{
                    group_name:"持有".into(),
                    index:0,
                    stock_code:model.code,
                    stock_name:model.name,
                    r#box: model.r#box,
                }
            })
            .collect::<Vec<_>>();
        Ok(infos)
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
        name: "测试".to_string(),
        r#box: None,
        hold: false,
    })
    .await;
    println!("{:?}", result);
}
#[tokio::test]
async fn test_delete_by_code() {
    init_db_coon().await;
    let result = StockInfoCurd::delete_by_code("123457".to_string()).await;
    println!("{:?}", result);
}
#[tokio::test]
async fn test_update_hold() {
    init_db_coon().await;
    let result = StockInfoCurd::update_hold("124357".to_string(), false).await;
    println!("{:?}", result);
}
#[tokio::test]
async fn test_update_index() {
    init_db_coon().await;
    // let result = StockInfoCurd::update_index("123457".to_string(),3).await;
    // println!("{:?}",result);
}
#[tokio::test]
async fn test_find_all() {
    init_db_coon().await;
    let result = StockInfoCurd::find_all().await;
    println!("{:?}", result);
}
