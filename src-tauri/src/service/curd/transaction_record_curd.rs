use crate::app_errors::AppResult;
use csv::{ ReaderBuilder};
use encoding_rs::GBK;
use std::fs::File;
use std::io::{BufReader, Read};
use sea_orm::{EntityTrait, Order, PaginatorTrait, QueryOrder, QuerySelect};
use crate::entities::prelude::{TransactionRecord, TransactionRecords};
use crate::entities::transaction_record::{ActiveModel, Column};
// 读取 CSV 文件并解析为 TransactionRecord 结构体
pub struct TransactionRecordCurd;
impl TransactionRecordCurd {
    /// 读取 CSV 文件并解析为 TransactionRecord 结构体
    /// 注意：东方财富导出的CSV文件编码为GBK，需要使用 encoding_rs 库进行解码转为 UTF-8 字符串，并且有很多空格，需要使用 trim() 方法去除。
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
        Ok(csv_reader
            .deserialize()
            .map(|record| record.unwrap())
            .collect::<Vec<TransactionRecord>>())
    }
    ///查询指定数量的历史交易数据，按照日期降序排列
    pub async fn query_by_num(num: u64) -> AppResult<Vec<TransactionRecord>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let transaction_records = TransactionRecords::find().order_by_desc(Column::Date).limit(num).all(db).await?;
        Ok(transaction_records)
    }
    /// 插入历史交易记录
    pub async fn insert(transaction_records: Vec<TransactionRecord>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        TransactionRecords::insert_many(
            transaction_records
                .into_iter()
                .map(|model| ActiveModel::from(model))
                .collect::<Vec<_>>(),
        ).exec(db).await?;
        Ok(())
    }
    
}

