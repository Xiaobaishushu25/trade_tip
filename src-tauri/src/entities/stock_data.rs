use arraystring::{typenum::U32, ArrayString};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

pub type TableName = ArrayString<U32>;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity {
    pub table_name: TableName,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        self.table_name.as_str()
    }
}
#[serde_as]
#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Deserialize, Serialize)]
pub struct Model {
    // #[serde(default = "path")]
    // pub name: String,
    #[serde(rename(deserialize = "day"))]
    // #[serde(rename(deserialize = "volume"))]
    pub date: String,
    #[serde_as(as = "DisplayFromStr")]
    pub open: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub close: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub high: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub low: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename(deserialize = "volume"))]
    pub vol: f64,
    #[serde(rename(deserialize = "ma_price5"))]
    pub ma5: Option<f64>,
    #[serde(rename(deserialize = "ma_price10"))]
    pub ma10: Option<f64>,
    #[serde(rename(deserialize = "ma_price20"))]
    pub ma20: Option<f64>,
    #[serde(rename(deserialize = "ma_price30"))]
    pub ma30: Option<f64>,
    #[serde(rename(deserialize = "ma_price60"))]
    // #[serde_as(as = "Option<DisplayFromStr>")]
    pub ma60: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    // Name,
    Date,
    Open,
    Close,
    High,
    Low,
    Vol,
    Ma5,
    Ma10,
    Ma20,
    Ma30,
    Ma60,
}

///必须要有一个主键：https://github.com/SeaQL/sea-orm/issues/485
#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Date,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            // Self::Id => ColumnType::Integer.def(),
            // Self::Name => ColumnType::String(None).def(),
            Column::Date => ColumnType::String(None).def(),
            Column::Open => ColumnType::Double.def(),
            Column::Close => ColumnType::Double.def(),
            Column::High => ColumnType::Double.def(),
            Column::Low => ColumnType::Double.def(),
            Column::Vol => ColumnType::Double.def(),
            Column::Ma5 => ColumnType::Double.def(),
            Column::Ma10 => ColumnType::Double.def(),
            Column::Ma20 => ColumnType::Double.def(),
            Column::Ma30 => ColumnType::Double.def(),
            Column::Ma60 => ColumnType::Double.def(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
