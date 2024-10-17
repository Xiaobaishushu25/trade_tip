use crate::app_errors::AppResult;
use crate::entities::transaction_record::TransactionRecord;
use csv::{ByteRecord, ReaderBuilder};
use encoding_rs::GBK;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

// 读取 CSV 文件并解析为 TransactionRecord 结构体
pub struct TransactionRecordCurd;
impl TransactionRecordCurd {
    /// 读取 CSV 文件并解析为 TransactionRecord 结构体
    /// 注意：东方财富导出的CSV文件编码为GBK，需要使用 encoding_rs 库进行解码转为 UTF-8 字符串，并且有很多空格，需要使用 trim() 方法去除。
    pub fn read_csv_file(file_path: &str) -> AppResult<Vec<TransactionRecord>> {
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
}
#[test]
fn test_transaction_record_curd() {
    let file = File::open("C:\\Users\\Xbss\\Desktop\\Table_1510.csv").unwrap();
    println!(
        "{:?}",
        TransactionRecordCurd::read_csv_file("C:\\Users\\Xbss\\Desktop\\Table_1510.csv").unwrap()
    );
    // // 打开文件
    // let mut reader = BufReader::new(file);
    //
    // // 读取整个文件为字节
    // let mut buffer = Vec::new();
    // reader.read_to_end(&mut buffer).unwrap();
    //
    // // 将 GBK 字节流转换为 UTF-8
    // let (decoded_string, _, _) = GBK.decode(&buffer);
    //
    // // 使用 csv 库处理转换后的字符串
    // let mut csv_reader = ReaderBuilder::new()
    //     .trim(csv::Trim::All)  // 自动去除列头和字段中的空格
    //     .from_reader(decoded_string.as_bytes());
    //
    // // 读取和打印每一条记录
    // // for result in csv_reader.records() {
    // for result in csv_reader.deserialize() {
    //     let record: TransactionRecord = result.unwrap();
    //     println!("{:?}", record);
    // };
}
