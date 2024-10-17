#[derive(Debug, serde::Deserialize)]
pub struct TransactionRecord {
    #[serde(rename = "成交日期")]
    pub date: String,
    #[serde(rename = "成交时间")]
    pub time: String,
    #[serde(rename = "证券代码")]
    pub code: String,
    #[serde(rename = "证券名称")]
    pub name: String,
    #[serde(rename = "委托方向")]
    pub direction: String,
    #[serde(rename = "成交数量")]
    pub num: i32,
    #[serde(rename = "成交均价")]
    pub price: f32,
    #[serde(rename = "成交金额")]
    pub amount: f32,
}
