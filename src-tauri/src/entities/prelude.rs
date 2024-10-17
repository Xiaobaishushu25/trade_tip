//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

pub use super::stock_info::ActiveModel as ActiveStockInfo;
pub use super::stock_info::Entity as StockInfos; //表
pub use super::stock_info::Model as StockInfo; //表中的每一行记录结构

pub use super::stock_data::Model as StockData;

pub use super::stock_group::ActiveModel as ActiveStockGroup;
pub use super::stock_group::Entity as StockGroups;
pub use super::stock_group::Model as StockGroup;

pub use super::group_stock_relation::ActiveModel as ActiveGroupStockR;
pub use super::group_stock_relation::Entity as GroupStockRs;
pub use super::group_stock_relation::Model as GroupStockR;

pub use super::graphic::ActiveModel as ActiveGraphic;
pub use super::graphic::Entity as Graphics;
pub use super::graphic::Model as Graphic;
