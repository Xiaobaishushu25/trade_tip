use chrono::{Datelike, Utc};
// #[derive(Debug, serde::Deserialize)]
// pub struct TransactionRecord {
//     #[serde(alias = "成交日期")]
//     pub date: String,
//     #[serde(alias = "成交时间")]
//     pub time: String,
//     #[serde(alias = "证券代码")]
//     pub code: String,
//     #[serde(alias = "证券名称")]
//     pub name: String,
//     #[serde(alias = "委托方向")]
//     pub direction: String,
//     #[serde(alias = "成交数量")]
//     pub num: i32,
//     #[serde(alias = "成交均价")]
//     pub price: f32,
//     #[serde(alias = "成交金额")]
//     pub amount: f32,
//     #[serde(skip_serializing)] // 忽略该字段
//     pub remark:String
// }
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize,Deserialize)]
#[sea_orm(table_name = "transaction_record")]
pub struct Model {
    /// 主键，由日期、时间、证券代码组成
    /// 在当日成交数据中没有"成交日期"字段，默认使用当前日期
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(alias = "成交日期", default = "default_date")]
    pub date: String,
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(alias = "成交时间")]
    pub time: String,
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(alias = "证券代码")]
    pub code: String,
    #[serde(alias = "证券名称")]
    pub name: String,
    #[serde(alias = "委托方向")]
    pub direction: String,
    #[serde(alias = "成交数量")]
    pub num: i32,
    ///交割单和历史成交中字段是"成交均价"，当日成交中字段是"成交价格"
    #[serde(alias = "成交均价", alias = "成交价格")]
    pub price: f32,
    #[serde(alias = "成交金额")]
    pub amount: f32,
    // #[serde(skip_serializing,default)] // 忽略该字段
    // 忽略反序列化，但包含序列化
    // #[serde(skip_deserializing)] // 忽略该字段
    // pub remark:String
    //备注
    pub remark: Option<String>,
}
fn default_date() -> String {
    let today = Utc::now();
    format!("{}-{}-{}", today.year(), today.month(), today.day())
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}
// impl Related<super::group_stock_relation::Entity> for crate::entities::stock_info::Entity {
//     fn to() -> RelationDef {
//         crate::entities::stock_info::Relation::GroupStockRs.def()
//     }
// }
impl ActiveModelBehavior for ActiveModel {}