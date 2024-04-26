use std::vec;
use sea_orm::{ColumnTrait, EntityName, EntityTrait, IntoActiveModel, Iterable, Order, QueryTrait};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Query;
use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::entities::prelude::StockData;
use crate::entities::stock_data::{Column, Entity, TableName};
use crate::entities::table::drop_table_with_dyn_name;
///默认所有查询返回的股票数据是按照日期降序排列的，即日期新的在前面
/// 插入数据按照日期升序排列
pub struct StockDataCurd;
impl StockDataCurd {
    pub async fn insert(table_name: &str, stock_data: StockData) -> AppResult<String> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let entity = Entity {
            table_name: TableName::from_str_truncate(table_name),
        };
        // Prepare insert statement
        // let mut insert = Entity::insert(model.clone().into_active_model());
        let mut insert = Entity::insert(stock_data.into_active_model());
        // Reset the table name of insert statement
        insert.query().into_table(entity.table_ref());
        let result = insert.exec(db).await?;
        // Execute the insert statement
        Ok(result.last_insert_id)
    }
    ///批量插入，最好按照升序排列（先插日期旧的），这样看起来比较直观
    pub async fn insert_many(table_name: &str, stock_data: Vec<StockData>) -> AppResult<()> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let entity = Entity {
            table_name: TableName::from_str_truncate(table_name),
        };
        // Prepare insert statement
        let mut insert = Entity::insert_many(stock_data.into_iter().map(|model| model.into_active_model()));
        insert.query().into_table(entity.table_ref());
        let _ = insert.exec(db).await?;
        Ok(())
    }
    ///查询指定数量的数据(仅收盘价)，按照日期降序排列
    pub async fn query_only_close_price_by_nums(table_name: &str, days_num:i32) -> AppResult<Vec<f64>> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let entity = Entity {
            table_name: TableName::from_str_truncate(table_name),
        };
        let mut select = Entity::find();
        *QueryTrait::query(&mut select) = Query::select()
            // .exprs(Column::iter().map(|col| col.select_as(Expr::col(col))))
            .exprs([Expr::col(Column::Close)])
            .from(entity.table_ref())
            .order_by(Column::Date,Order::Desc)
            .limit(days_num as u64)
            .to_owned();
        // let result = select.clone().all(db).await?;
        let result = select.into_tuple::<f64>().all(db).await?;
        Ok(result)
    }
    ///查询指定数量的数据，按照日期降序排列
    pub async fn query_by_nums(table_name: &str, days_num:i32) -> AppResult<Vec<StockData>> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let entity = Entity {
            table_name: TableName::from_str_truncate(table_name),
        };
        let mut select = Entity::find();
        *QueryTrait::query(&mut select) = Query::select()
            .exprs(Column::iter().map(|col| col.select_as(Expr::col(col))))
            .from(entity.table_ref())
            .order_by(Column::Date,Order::Desc)
            .limit(days_num as u64)
            .to_owned();
        let result = select.clone().all(db).await?;
        Ok(result)
    }
    pub async fn query_latest(table_name: &str) -> AppResult<Option<StockData>> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let entity = Entity {
            table_name: TableName::from_str_truncate(table_name),
        };
        let mut select = Entity::find();
        *QueryTrait::query(&mut select) = Query::select()
            .exprs(Column::iter().map(|col| col.select_as(Expr::col(col))))
            .order_by(Column::Date,Order::Desc)
            .limit(1)
            .from(entity.table_ref())
            .to_owned();
        // Execute the select statement
        // let result = select.clone().all(db).await?;
        let result = select.clone().one(db).await?;
        Ok(result)
    }
    pub async fn query_all(table_name: &str) -> AppResult<Vec<StockData>> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let entity = Entity {
            table_name: TableName::from_str_truncate(table_name),
        };
        let mut select = Entity::find();
        *QueryTrait::query(&mut select) = Query::select()
            .exprs(Column::iter().map(|col| col.select_as(Expr::col(col))))
            .order_by(Column::Date,Order::Desc)
            .from(entity.table_ref())
            .to_owned();
        // Execute the select statement
        let result = select.clone().all(db).await?;
        Ok(result)
    }
    pub async fn delete_table_by_stock_code(code: &str) -> AppResult<()> {
        // let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        // let entity = Entity {
        //     table_name: TableName::from_str_truncate(code),
        // };
        let _  = drop_table_with_dyn_name(code).await?;
        Ok(())
    }
}
#[tokio::test]
async fn test_insert() {
    init_db_coon().await;
    let model = StockData {
        date: "2023-01-01".to_string(),
        open: 1.0,
        close: 2.0,
        high: 3.0,
        low: 4.0,
        vol: 5.0,
        ma5: Some(6.0),
        ma10: Some(7.0),
        ma20: Some(8.0),
        ma30: Some(9.0),
        ma60: Some(10.0),
    };
    let result = StockDataCurd::insert("sz_123556", model).await;
    match result {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
#[tokio::test]
async fn test_find_all() {
    init_db_coon().await;
    let result = StockDataCurd::query_all("159869").await;
    match result {
        Ok(result) => {
            let result = result.into_iter().map(|item| item.date).collect::<Vec<_>>();
            println!("{:?}", result);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}