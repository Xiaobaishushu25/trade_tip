use crate::app_errors::AppResult;
use crate::entities::prelude::{TransactionRecord, TransactionRecords};
use crate::entities::transaction_record::{ActiveModel, Column};
use crate::entities::{init_db_coon, open_db_log};
use csv::{ReaderBuilder, WriterBuilder};
use encoding_rs::{GBK};
use log::{info};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, QuerySelect};
use std::fs::File;
use std::io::{BufReader, Read};
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
    /// 目前只支持GBK编码和UTF-8编码，其他编码待支持（可以使用charset-normalizer-rs库进行检测编码方式）。
    pub async fn read_csv_file(file_path: &str) -> AppResult<Vec<TransactionRecord>> {
        let file = File::open(file_path)?;
        // 打开文件
        let mut reader = BufReader::new(file);
        // 读取整个文件为字节
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        //从字节流中获取编码
        // let result = from_bytes(&buffer, None);
        // let ecoding = result.get_best().unwrap().encoding();//gbk、utf-8
        // let decoded_string = if ecoding== "gbk" {
        //     // 将 GBK 字节流转换为 UTF-8
        //     let (decoded_string, _, _) = GBK.decode(&buffer);
        //     decoded_string.to_string()
        // }else {
        //     //由buffer构造字符串;
        //     String::from_utf8(buffer).unwrap()
        // };
        let decoded_string = if let Ok(utf8_string) = String::from_utf8(buffer.clone()) {
            utf8_string
        } else {
            // 尝试使用GBK解码
            let (decoded, _, _) = GBK.decode(&buffer);
            decoded.to_string()
        };
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
    pub async fn save_to_file(path: String) -> AppResult<()> {
        let file = File::create(path)?;
        // let mut wtr = Writer::from_writer(file);
        let mut wtr = WriterBuilder::new()
            .has_headers(false) // 禁用默认表头
            .from_writer(file);
        // 写入表头
        wtr.write_record([
            "成交日期",
            "成交时间",
            "证券代码",
            "证券名称",
            "委托方向",
            "成交数量",
            "成交均价",
            "成交金额",
            "备注",
        ])
        .map_err(|e| anyhow::anyhow!("csv write header error: {}", e))?;
        let transaction_records = TransactionRecordCurd::query_all().await?;
        for model in transaction_records {
            wtr.serialize(model)
                .map_err(|e| anyhow::anyhow!("csv serialize error: {}", e))?;
        }
        wtr.flush()?;
        Ok(())
    }
    ///查询指定数量的历史交易数据，按照日期降序排列
    pub async fn query_by_num(num: u64) -> AppResult<Vec<TransactionRecord>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let transaction_records = TransactionRecords::find()
            .order_by_desc(Column::Date)
            .limit(num)
            .all(db)
            .await?;
        Ok(transaction_records)
    }
    ///查询所有历史交易数据，按照日期降序排列
    pub async fn query_all() -> AppResult<Vec<TransactionRecord>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let transaction_records = TransactionRecords::find()
            .order_by_desc(Column::Date)
            .all(db)
            .await?;
        Ok(transaction_records)
    }
    ///查询指定代码的历史交易数据，按照日期降序排列
    pub async fn query_by_code(code: String) -> AppResult<Vec<TransactionRecord>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let transaction_records = TransactionRecords::find()
            .filter(Column::Code.eq(code))
            .order_by_desc(Column::Date)
            .all(db)
            .await?;
        Ok(transaction_records)
    }
    ///查询数据库中最新一条交易记录
    pub async fn query_latest_record() -> AppResult<Option<TransactionRecord>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let transaction_record = TransactionRecords::find()
            .order_by_desc(Column::Date)
            .one(db)
            .await?;
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
        )
        .exec(db)
        .await?;
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
    pub async fn delete_by_primary_key(date: String, time: String, code: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        TransactionRecords::delete_by_id((date, time, code))
            .exec(db)
            .await?;
        Ok(())
    }
    /// 删除所有交易记录
    pub async fn delete_all() -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        TransactionRecords::delete_many().exec(db).await?;
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
    let vec = TransactionRecordCurd::read_csv_file("C:\\Users\\Xbss\\Desktop\\Table_7991.csv")
        .await
        .unwrap();
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
    TransactionRecordCurd::insert(vec![transaction_record.clone()])
        .await
        .unwrap();
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
    TransactionRecordCurd::update(transaction_record.clone())
        .await
        .unwrap();
}
#[tokio::test]
async fn test_read_file() {
    init_db_coon().await;
    // let path = "G:\\待处理文件\\TradeTip_10192314.csv";
    // let path = "C:\\Users\\Xbss\\Desktop\\18历史成交.csv";
    let path = "C:\\Users\\Xbss\\Desktop\\奇怪16.csv";
    // let result = from_path(&PathBuf::from(path), None).unwrap();
    // let best_guess = result.get_best();
    // println!("{:?}", best_guess.unwrap().encoding());
    let file = File::open(path).unwrap();
    // 打开文件
    let mut reader = BufReader::new(file);
    // 读取整个文件为字节
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    // 尝试使用UTF-8解码
    if let Ok(utf8_string) = String::from_utf8(buffer.clone()) {
        println!("读取为UTF-8: {}", utf8_string);
    } else {
        // 尝试使用GBK解码
        let (decoded, _, _) = GBK.decode(&buffer);
        println!("读取为GBK: {}", decoded);
    }
    // let x = Encoding::for_label(buffer.as_slice()).unwrap();
    let vec = TransactionRecordCurd::read_csv_file(path).await.unwrap();
    println!("{:?}", vec);
}
#[tokio::test]
async fn test_save_to_file() {
    init_db_coon().await;
    let path = "G:\\APP\\金融助手\\log\\transaction_record.csv";
    TransactionRecordCurd::save_to_file(path.to_string())
        .await
        .unwrap();
}
