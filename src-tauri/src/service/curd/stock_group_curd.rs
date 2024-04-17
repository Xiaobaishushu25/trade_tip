use arraystring::typenum::op;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, NotSet, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use crate::app_errors::AppError::AnyHow;
use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::entities::prelude::{ActiveStockGroup, StockGroup, StockGroups, StockInfos};
use crate::entities::stock_group::Column;
use crate::service::curd::stock_group_curd;


struct StockGroupCurd;
impl StockGroupCurd {
    pub async fn insert(mut stock_group: StockGroup) -> AppResult<StockGroup>{
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        //查询index列的最大值
        // let max_index = StockGroups::find().select_only().expr(Column::Index.max()).into_tuple::<i32>().one(db).await?.unwrap_or(1);
        //注意，这里泛型是into_tuple::<Option<i32>>，有可能没有index值
        let max_index = StockGroups::find().select_only().expr(Column::Index.max()).into_tuple::<Option<i32>>().one(db).await?.unwrap().unwrap_or(0);
        println!("max_index:{:?}",max_index);
        // let max_index = StockGroups::find().select_only().expr(Column::Index.max()).into_tuple::<Option<i32>>().one(db).await?.unwrap_or(Some(1)).unwrap();
        // println!("max_index:{:?}",max_index);
        // let max_index = StockInfos::find().select_only().column(Column::Index).order_by_asc(Column::Index).one(db).await?.unwrap();
        stock_group.index = max_index+1;
        // let mut model: ActiveStockGroup = stock_group.into();
        let model = ActiveStockGroup{
            index: Set(max_index+1),
            name: Set(stock_group.name),
            ..Default::default()
        };
        // model.id = NotSet;
        //这里实际会有两句，插入后再查询一遍返回刚插的数据，如果不需要可以用let _忽略，这样就不会在查一次了
        let result = model.insert(db).await?;
        // let _ = model.insert(db).await?;
        Ok(result)
        // Err(AnyHow(anyhow::anyhow!("插入失败")))
    }
    pub async fn delete_by_id(id: i32) -> AppResult<i32> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = StockGroups::find_by_id(id).one(db).await?.ok_or(anyhow::anyhow!("删除失败:未找到id为{}的记录",id))?;
        let result = model.delete(db).await?;
        Ok(result.rows_affected as i32)
    }
}
#[tokio::test]
async fn test_insert() {
    init_db_coon().await;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let result = StockGroupCurd::insert(StockGroup::new("测试3".to_string())).await;
    println!("{:?}",result);
}
#[test]
fn test_option() {
    let index = None;
    println!("{:?}",index);
    let option = index.unwrap_or(Some(1));
    println!("{:?}",option);
}