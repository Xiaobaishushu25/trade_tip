use sea_orm::{ColumnTrait, EntityName, EntityTrait, IntoActiveModel, Iterable, QueryTrait};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Query;
use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::entities::prelude::StockData;
use crate::entities::stock_data::{Column, Entity, TableName};

struct StockDataCurd;
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
    pub async fn find_some_days(table_name: &str,days_num:i32) -> AppResult<Vec<StockData>> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let entity = Entity {
            table_name: TableName::from_str_truncate(table_name),
        };
        let mut select = Entity::find();
        *QueryTrait::query(&mut select) = Query::select()
            .exprs(Column::iter().map(|col| col.select_as(Expr::col(col))))
            .from(entity.table_ref())
            .limit(days_num as u64)
            .to_owned();
        let result = select.clone().all(db).await?;
        Ok(result)
    }
    pub async fn find_all(table_name: &str) -> AppResult<Vec<StockData>> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let entity = Entity {
            table_name: TableName::from_str_truncate(table_name),
        };
        let mut select = Entity::find();
        *QueryTrait::query(&mut select) = Query::select()
            .exprs(Column::iter().map(|col| col.select_as(Expr::col(col))))
            .from(entity.table_ref())
            .to_owned();
        // Execute the select statement
        let result = select.clone().all(db).await?;
        Ok(result)
    }
}
#[tokio::test]
async fn test_insert() {
    init_db_coon().await;
    let model = StockData {
        name: "sz_123456".to_string(),
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
    let result = StockDataCurd::find_all("sz_123556").await;
    match result {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}