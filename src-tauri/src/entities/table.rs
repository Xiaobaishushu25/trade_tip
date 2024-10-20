use crate::app_errors::AppResult;
use crate::entities::group_stock_relation::Relation::StockInfos;
use crate::entities::prelude::{Graphics, GroupStockRs, StockGroups, TransactionRecords};
use crate::entities::stock_data::{Column, Entity, TableName};
use crate::entities::{init_db_coon, open_db_log, stock_group, stock_info, DB};
use crate::service::curd::stock_group_curd::StockGroupCurd;
use log::{error, info};
use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{
    sea_query, ConnectionTrait, DatabaseConnection, EntityName, EntityTrait, ExecResult, Schema,
    Statement,
};

// use std::env;
// use sea_query::{ColumnDef, Iden, SqliteQueryBuilder, Table, Value};
// #[cfg(feature = "derive")]
// use sea_query::Iden;
//
// enum StockData {
//     TableName(String),
//     Id,
//     FontSize,
//     FontId,
// }
// // Mapping between Enum variant and its corresponding string value
// impl Iden for StockData {
//     fn unquoted(&self, s: &mut dyn std::fmt::Write) {
//         write!(
//             s,
//             "{}",
//             match self {
//                 Self::TableName(name) => name,
//                 Self::Id => "id",
//                 Self::FontId => "font_id",
//                 Self::FontSize => "font_size",
//             }
//         ).unwrap();
//     }
// }
// #[derive(Iden)]
// pub(crate) enum StockInfo {
//     Table,
//     Code,
//     Index,
//     Name,
//     Box,
//     Hold
// }
//
// async fn create_table(){
//     let mut url = "sqlite:".to_string();
//     url.push_str(&*env::current_dir().unwrap().to_string_lossy());
//     url.push_str("/data.db?mode=rwc");
//     // let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
//     let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
//     let table = Table::create()
//         .table(StockData::TableName("sz_123456".into()))
//         .if_not_exists()
//         .col(ColumnDef::new(StockData::Id).integer().not_null().auto_increment().primary_key())
//         .col(ColumnDef::new(StockData::FontSize).integer().not_null())
//         // .col(ColumnDef::new(StockData::CharDataacter).string().not_null())
//         // .col(ColumnDef::new(StockData::SizeW).integer().not_null())
//         // .col(ColumnDef::new(StockData::SizeH).integer().not_null())
//         .col(ColumnDef::new(StockData::FontId).integer().default(Value::Int(None)))
//         .build(SqliteQueryBuilder);
//     let info_table = Table::create()
//         .table(StockInfo::Table)
//         .if_not_exists()
//         .col(ColumnDef::new(StockInfo::Code).string().not_null().primary_key())
//         .col(ColumnDef::new(StockInfo::Index).integer().not_null())
//         .col(ColumnDef::new(StockInfo::Name).string().not_null())
//         .col(ColumnDef::new(StockInfo::Box).string().null())
//         .col(ColumnDef::new(StockInfo::Hold).boolean().not_null().default(Value::Bool(Some(false))))
//         .build(SqliteQueryBuilder);
//     // let result = sqlx::query(&table).execute(&pool).await;
//     let result = sqlx::query(&info_table).execute(&pool).await;
//     println!("Create table character: {result:?}\n");
// }
// #[tokio::test]
// async fn test_create_table() {
//     create_table().await;
// }
///需要使用TableCreateStatement建表，一般用在动态名称建表中。
pub async fn create_dyn_table(
    db: &DatabaseConnection,
    stmt: &TableCreateStatement,
) -> AppResult<ExecResult> {
    let builder = db.get_database_backend();
    Ok(db.execute(builder.build(stmt)).await?)
}
pub async fn create_table_with_dyn_name(name: &str) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
    let entity: Entity = Entity {
        table_name: TableName::from_str_truncate(name),
    };
    let create_table_stmt = sea_query::Table::create()
        .table(entity.table_ref())
        // .col(ColumnDef::new(Column::Name).string().not_null())
        .col(
            ColumnDef::new(Column::Date)
                .string()
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(Column::Open).double().not_null())
        .col(ColumnDef::new(Column::Close).double().not_null())
        .col(ColumnDef::new(Column::High).double().not_null())
        .col(ColumnDef::new(Column::Low).double().not_null())
        .col(ColumnDef::new(Column::Vol).double().not_null())
        .col(ColumnDef::new(Column::Ma5).double())
        .col(ColumnDef::new(Column::Ma10).double())
        .col(ColumnDef::new(Column::Ma20).double())
        .col(ColumnDef::new(Column::Ma30).double())
        .col(ColumnDef::new(Column::Ma60).double())
        .to_owned();
    create_dyn_table(&db, &create_table_stmt).await?;
    Ok(())
}
pub async fn drop_table_with_dyn_name(table_name: &str) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
    let entity: Entity = Entity {
        table_name: TableName::from_str_truncate(table_name),
    };
    let _ = drop_table(db, entity).await;
    Ok(())
}
// use sea_orm::{Schema, ConnectionTrait, EntityTrait};

async fn create_table<E>(db_connection: &sea_orm::DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    let backend = db_connection.get_database_backend();
    let schema = Schema::new(backend);
    let execution = db_connection.execute(backend.build(&schema.create_table_from_entity(entity)));
    match execution.await {
        Ok(_) => info!("Created {}", entity.table_name()),
        Err(e) => error!("Error: {}", e),
    }
}
///https://github.com/SeaQL/sea-orm/issues/1399
///注意，表不存在时Drop也会返回Ok
async fn drop_table<E>(db_connection: &sea_orm::DatabaseConnection, entity: E) -> AppResult<()>
where
    E: EntityTrait,
{
    // println!(
    //     "{:?}",
    //     Statement::from_sql_and_values(
    //         db_connection.get_database_backend(),
    //         &format!(r#"DROP TABLE "{}""#, entity.table_name()),
    //         vec![],
    //     )
    // );
    info!(
        "{:?}",
        Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            &format!(r#"DROP TABLE "{}""#, entity.table_name()),
            vec![],
        )
    );
    let _ = db_connection
        .execute(Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            &format!(r#"DROP TABLE {}"#, entity.table_name()),
            vec![],
        ))
        .await?;
    Ok(())
}
pub async fn create_all_need_table(db: &DatabaseConnection) {
    let _ = create_table(db, stock_info::Entity).await;
    let _ = create_table(db, Graphics).await;
    let _ = create_table(db, StockGroups).await;
    let _ = create_table(db, GroupStockRs).await;
    let _ = create_table(db, TransactionRecords).await;
    StockGroupCurd::insert_init(db).await.unwrap();
}
#[tokio::test]
async fn test_create_table_with_dyn_name() {
    init_db_coon().await;
    let result = create_table_with_dyn_name("sz_123456").await;
    println!("{result:?}")
}
#[tokio::test]
async fn test_drop_table_with_dyn_name() {
    init_db_coon().await;
    let result = drop_table_with_dyn_name("sz_123456").await;
    println!("{result:?}")
}

#[tokio::test]
async fn test_create_stock_info() {
    init_db_coon().await;
    let result = create_table(&DB.get().unwrap(), stock_info::Entity).await;
    println!("{result:?}")
}
#[tokio::test]
async fn test_create_stock_group() {
    init_db_coon().await;
    let result = create_table(&DB.get().unwrap(), stock_group::Entity).await;
    println!("{result:?}")
}

#[tokio::test]
async fn test_create_group_stock_r() {
    init_db_coon().await;
    let result = create_table(&DB.get().unwrap(), GroupStockRs).await;
    println!("{result:?}")
}
#[tokio::test]
async fn test_create_graphic() {
    init_db_coon().await;
    let result = create_table(&DB.get().unwrap(), Graphics).await;
    println!("{result:?}")
}
#[tokio::test]
async fn test_create_transaction_records() {
    open_db_log().await;
    init_db_coon().await;
    let result = create_table(&DB.get().unwrap(), TransactionRecords).await;
    println!("{result:?}")
}

