use crate::app_errors::AppResult;
use csv::{ ReaderBuilder};
use encoding_rs::GBK;
use std::fs::File;
use std::io::{BufReader, Read};
use log::info;
use sea_orm::{ActiveModelTrait, EntityTrait, Order, PaginatorTrait, QueryOrder, QuerySelect};
use sea_orm::sea_query::extension::postgres::PgBinOper::Regex;
use crate::entities::{init_db_coon, open_db_log};
use crate::entities::prelude::{TransactionRecord, TransactionRecords};
use crate::entities::transaction_record::{ActiveModel, Column};
// 读取 CSV 文件并解析为 TransactionRecord 结构体
pub struct TransactionRecordCurd;
impl TransactionRecordCurd {
    /// 读取 CSV 文件并解析为 TransactionRecord 结构体
    /// 细数东方财富导出的CSV文件的问题：
    /// 1. 导出的CSV文件编码为GBK，需要使用 encoding_rs 库进行解码转为 UTF-8 字符串。
    /// 2. 导出的CSV文件中有些字段的值有不明空格(但是在excel看没有)，需要去除引空白字符。
    /// 2. 导出的CSV文件中有些字段的值带有不明空格和引号(但是在excel看没有)，需要用extract_and_trim函数去除引号和空白字符。
    /// 2. 导出的CSV文件中日期格式经常变化，有时为2021-10-18，有时为2021/10/18，需要用函数统一为2021-10-18格式。
    /// 该函数支持读取东方财富导出的历史成交、当日成交CSV文件，并解析为 TransactionRecord 结构体的 Vec
    pub async fn read_csv_file(file_path: &str) -> AppResult<Vec<TransactionRecord>> {
        let file = File::open(file_path)?;
        // 打开文件
        let mut reader = BufReader::new(file);
        // 读取整个文件为字节
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        // 将 GBK 字节流转换为 UTF-8
        let (decoded_string, _, _) = GBK.decode(&buffer);
        // 使用 csv 库处理转换后的字符串
        let mut csv_reader = ReaderBuilder::new()
            .trim(csv::Trim::All) // 自动去除列头和字段中的空格
            .from_reader(decoded_string.as_bytes());
        csv_reader
            .deserialize()
            .map(|record| {
                let record = record.map_err(|e| anyhow::anyhow!("csv deserialize error: {}", e))?;
                let mut model: TransactionRecord = record;
                model.date = handle_csv_date(&model.date); // 不知道为甚么date字段格式不一样，需要用这个函数统一格式
                model.code = extract_and_trim(&model.code); // 不知道为甚么code字段会带有引号，需要用这个函数去除引号和空白字符
                Ok(model)
            })
            .collect::<_>()
        //将所有的结果收集到一个 Result<Vec<TransactionRecord>>，这样如果有任何错误，它会直接返回
        // Ok(csv_reader
        //     .deserialize()
        //     .map(|record| {
        //         let mut model:TransactionRecord = record.unwrap();
        //         model.date = handle_csv_date(&model.date);// 不知道为甚么date字段格式不一样，需要用这个函数统一格式
        //         model.code = extract_and_trim(&model.code);// 不知道为甚么code字段会带有引号，需要用这个函数去除引号和空白字符
        //         model
        //     })
        //     .collect::<Vec<TransactionRecord>>())
    }
    ///查询指定数量的历史交易数据，按照日期降序排列
    pub async fn query_by_num(num: u64) -> AppResult<Vec<TransactionRecord>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let transaction_records = TransactionRecords::find().order_by_desc(Column::Date).limit(num).all(db).await?;
        Ok(transaction_records)
    }
    ///查询数据库中最新一条交易记录
    pub async fn query_latest_record() -> AppResult<Option<TransactionRecord>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let transaction_record = TransactionRecords::find().order_by_desc(Column::Date).one(db).await?;
        Ok(transaction_record)
    }
    /// 插入历史交易记录
    pub async fn insert(transaction_records: Vec<TransactionRecord>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        TransactionRecords::insert_many(
            transaction_records
                .into_iter()
                // .map(|model| ActiveModel::from(model))https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure
                .map(ActiveModel::from)
                .collect::<Vec<_>>(),
        ).exec(db).await?;
        Ok(())
    }
    pub async fn update(transaction_record: TransactionRecord) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let mut active_model = ActiveModel::from(transaction_record);
        active_model.reset(Column::Remark);
        active_model.update(db).await?;
        // TransactionRecords::update(active_model).exec(db).await?;
        info!("更新交易记录成功");
        Ok(())
    }
}
///不知道为什么导出的历史交易记录中有些字段的值带有引号(但是在excel看没有)，需要用这个函数去除引号和空白字符
fn extract_and_trim(input: &str) -> String {
    // 查找引号的位置
    if let Some(start) = input.find('"') {
        if let Some(end) = input.rfind('"') {
            // 提取引号内的内容并去除空白字符
            return input[start + 1..end].trim().to_string();
        }
    }
    // 如果没有引号，返回原字符串并去除空白字符
    input.trim().to_string()
}
fn handle_csv_date(input: &str) -> String {
    input.replace("/", "-")
}
#[tokio::test]
async fn test_transaction_record_curd() {
    // let vec = TransactionRecordCurd::read_csv_file("C:\\Users\\Xbss\\Desktop\\Table_1510.csv").await.unwrap();
    // println!("{:?}", vec);
    // let vec = TransactionRecordCurd::read_csv_file("C:\\Users\\Xbss\\Desktop\\Table_973.csv").await.unwrap();
    // println!("{:?}", vec);
    // let vec = TransactionRecordCurd::read_csv_file("C:\\Users\\Xbss\\Desktop\\Table_4638.csv").await.unwrap();
    let vec = TransactionRecordCurd::read_csv_file("C:\\Users\\Xbss\\Desktop\\Table_7991.csv").await.unwrap();
    println!("{:?}", vec);
}
#[tokio::test]
async fn test_insert_transaction_record() {
    open_db_log().await;
    init_db_coon().await;
    let transaction_record = TransactionRecord {
        code: "000001.SZ".to_string(),
        name: "平安银行".to_string(),
        date: "2021-10-18".to_string(),
        time: "15:00:00".to_string(),
        price: 11.11,
        amount: 1111.11,
        direction: "买入".to_string(),
        remark: Some("测试".to_string()),
        num: 0,
    };
    TransactionRecordCurd::insert(vec![transaction_record.clone()]).await.unwrap();
}
#[tokio::test]
async fn test_update_transaction_record() {
    open_db_log().await;
    init_db_coon().await;
    let transaction_record = TransactionRecord {
        code: "000001.SZ".to_string(),
        name: "平安银行".to_string(),
        date: "2021-10-18".to_string(),
        time: "15:00:00".to_string(),
        price: 11.11,
        amount: 1111.11,
        direction: "买入".to_string(),
        remark: Some("测试".to_string()),
        num: 0,
    };
    TransactionRecordCurd::update(transaction_record.clone()).await.unwrap();
}
