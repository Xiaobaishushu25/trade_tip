//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "stock_group")]
pub struct Model {
    pub index: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
}

impl Model {
    pub fn new(name: String) -> Self {
        Self { index: 0, name }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::group_stock_relation::Entity")]
    GroupStockRs,
}
impl Related<super::group_stock_relation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupStockRs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
