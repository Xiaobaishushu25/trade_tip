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