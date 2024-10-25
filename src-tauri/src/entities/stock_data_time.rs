use chrono::NaiveDateTime;
use serde::Deserialize;
///这个结构体用来存储带分时线的股票数据
#[derive(Debug, Deserialize)]
pub struct StockDataTime {
    pub time: NaiveDateTime,//2024-10-25T14:59:00
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}