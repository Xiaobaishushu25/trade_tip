use sea_orm::FromQueryResult;
use serde::Serialize;

/// 分组下的股票信息(额外带了一个分组名称（group_name），所以加了个G)
#[derive(FromQueryResult, Debug, Serialize)]
pub struct StockInfoG {
    pub(crate) group_name: String,
    pub index: i32,
    pub code: String,
    pub name: String,
    pub r#box: Option<String>,
    pub hold: bool,
}
///股票实时数据
#[derive(Serialize,Clone,Debug)]
pub struct StockLiveData {
    pub code: String,
    // pub name: String,
    pub price: f64,
    pub change: f64,
    pub percent: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub ma5: Option<f64>,
    pub ma10: Option<f64>,
    pub ma20: Option<f64>,
    pub ma30: Option<f64>,
    pub ma60: Option<f64>,
}