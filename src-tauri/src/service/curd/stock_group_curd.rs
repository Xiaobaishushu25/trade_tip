use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, ModelTrait, NotSet, QueryOrder, QuerySelect, Related};
use sea_orm::ActiveValue::Set;
use crate::app_errors::AppError::AnyHow;
use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::entities::prelude::{ActiveStockGroup, GroupStockRs, StockGroup, StockGroups, StockInfos};
use crate::entities::stock_group::Column;


pub struct StockGroupCurd;
impl StockGroupCurd {
    pub async fn insert(mut stock_group: StockGroup) -> AppResult<StockGroup>{
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        //注意，这里泛型是into_tuple::<Option<i32>>，有可能没有index值
        let max_index = StockGroups::find().select_only().expr(Column::Index.max()).into_tuple::<Option<i32>>().one(db).await?.unwrap().unwrap_or(0);
        println!("max_index:{:?}",max_index);
        stock_group.index = max_index+1;
        let model = ActiveStockGroup{
            index: Set(max_index+1),
            name: Set(stock_group.name),
            // ..Default::default()
        };
        // model.id = NotSet;
        //这里实际会有两句，插入后再查询一遍返回刚插的数据，如果不需要可以用let _忽略，这样就不会在查一次了
        let result = model.insert(db).await?;
        // let _ = model.insert(db).await?;
        Ok(result)
    }
    pub async fn delete_by_name(name: String) -> AppResult<i32> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = StockGroups::find_by_id(name.clone()).one(db).await?.ok_or(anyhow::anyhow!("删除失败:未找到name为{}的分组",name))?;
        let result = model.delete(db).await?;
        Ok(result.rows_affected as i32)
    }
    ///查询所有的分组信息，并排好序
    pub async fn find_all() -> AppResult<Vec<StockGroup>> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let vec = StockGroups::find().order_by_asc(Column::Index).all(db).await?;
        Ok(vec)
    }
    // pub async fn find_by_id(id: i32) -> AppResult<Vec<String>> {
    // pub async fn find_by_name(id: i32) -> AppResult<()> {
    //     let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
    //     let model = StockGroups::find_by_id(id).one(db).await?.unwrap();
    //     let vec = model.find_related(GroupStockRs).all(db).await?;
    //     // let vec = StockGroups::find_related().all(db).await?;
    //     println!("{:?}",vec);
    //     Ok(())
    // }
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
// #[tokio::test]
// async fn test_find_by_id() {
//     init_db_coon().await;
//     let result = StockGroupCurd::find_by_id(1).await;
//     println!("{:?}",result);
// }