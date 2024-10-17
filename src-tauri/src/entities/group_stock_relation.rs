//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "group_stock_relation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub group_name: String,
    #[sea_orm(primary_key)]
    pub stock_code: String,
    pub index: i32,
}

impl Model {
    pub fn new(group_name: String, stock_code: String) -> Self {
        Self {
            group_name,
            stock_code,
            index: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::stock_group::Entity",
        from = "Column::GroupName",
        to = "super::stock_group::Column::Name"
    )]
    StockGroups,
    #[sea_orm(
        belongs_to = "super::stock_info::Entity",
        from = "Column::StockCode",
        to = "super::stock_info::Column::Code"
    )]
    StockInfos,
}
impl Related<super::stock_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StockGroups.def()
    }
}
impl Related<super::stock_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StockInfos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
