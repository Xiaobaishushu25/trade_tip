use crate::app_errors::AppError::AnyHow;
use crate::app_errors::AppResult;
use chrono::{Local, NaiveDate, NaiveTime};
use std::str::FromStr;
use log::info;

///根据给定的num，和收盘价列表计算Ma num日均线，数据不足的地方填None；保留三位小数。
/// 保证返回的均线数据的长度和close_price一样长。
///  close_price要是按照日期从旧到新排列
pub async fn compute_single_ma(day_count: usize, close_price: &Vec<f64>) -> Vec<Option<f64>> {
    let mut sum: f64;
    let mut result = vec![];
    let mut window_start = 0; // 滑动窗口的起始位置
    sum = close_price.iter().take(day_count - 1).sum::<f64>();
    for _ in 0..(day_count - 1) {
        result.push(None); //一共放置day_count-1个None，比如计算60日均线，则放置59个None,因为下面计算第一步就是再加一次
    }
    for i in (day_count - 1)..close_price.len() {
        sum += close_price[i];
        let ma = sum / day_count as f64;
        let x = format!("{:.3}", ma).parse::<f64>().unwrap();
        result.push(Some(x));
        sum -= close_price[window_start];
        window_start += 1;
    }
    result
}
///同时计算多条ma，用于实时更新当日最新的ma
/// mas：需要计算的ma，要从小到大的顺序排列，比如[5,10,20,60]
/// close_prices：最近n日的的收盘价列表，顺序是按照日期从新到旧排列,因为是从头求和
/// 返回：返回一个长度为mas.len()的Vec，其中每个元素代表计算出的ma值
///一开始的函数签名： pub async fn compute_mul_ma(mas:Vec<usize>,close_prices:&Vec<f64>)->Vec<f64>{}
/// 为了不大量克隆close_prices，选择把price单独传进来
pub async fn compute_mul_ma(mas: Vec<usize>, now_price: f64, close_prices: &Vec<f64>) -> Vec<f64> {
    // close_prices.reverse();
    //从后往前遍历close_prices
    let mut sum: f64 = now_price;
    let mut result = vec![];
    for (index, value) in close_prices.iter().enumerate() {
        sum += value;
        if mas.contains(&(index + 2)) {
            let value = sum / ((index + 2) as f64);
            let rounded = (value * 1000.0).round() / 1000.0; // 四舍五入到最接近的0.001，gpt说这样比parse效率高
            result.push(rounded);
            // result.push(sum / (index+1) as f64);
        }
    }
    result
}
///根据输入的股票代码判断是沪市（sh）还是深市（sz）的,还是哪个交易所的
/// code:股票代码，如000001
/// return:(市场代码，如sh,是否为主连（仅适用于期货）)
pub fn get_market_by_code(code: &str) -> AppResult<(String,bool)> {
    info!("get_market_by_code:{}",code);
    if code.starts_with("15")
        || code.starts_with("000")
        || code.starts_with("002")
        || code.starts_with("300")
        || code.starts_with("200")
    {
        Ok(("sz".into(),false)) //深圳
    } else if code.starts_with("51")
        || code.starts_with("60")
        || code.starts_with("588")
        || code.starts_with("688")
        || code.starts_with("900")
    {
        Ok(("sh".into(),false)) //上海
    } else {
        let prefix: String = code.chars()
            .filter(|c| !c.is_digit(10)) // Filter out digits
            .collect::<String>()
            .to_lowercase();
        let prefix = prefix.as_str();
        let first_match = match prefix {
            "rb" | "au" | "ag" | "cu" | "al" | "zn" | "pb" | "ni" | "sn" | "ss" | "fu" | "bu" | "ru" | "wr" | "hc" | "sp"| "ao"| "br" => Ok(("shfe".into(),false)), // 上海期货交易所
            "m" | "y" | "a" | "b" | "p" | "c" | "cs" | "jd" | "bb" | "fb" | "l" | "v" | "pp" | "j" | "jm" | "i" | "eg" | "rr" | "lh"| "pg"| "eb" => Ok(("dce".into(),false)), // 大连商品交易所
            "rs"|"pf"|"pk"|"cj"|"ap"|"rm"|"oi"|"cy"|"cf"|"sr"|"sm"|"sf"|"sh"|"sa"|"ur"|"ma"|"px"|"pr"|"fg" | "wh" | "pm" | "ri" | "lr" | "jr" | "zc" | "ta" | "sc" => Ok(("czce".into(),false)), // 郑州商品交易所
            "if" | "ih" | "ic" | "t" | "tf" | "ts" => Ok(("cffex".to_string(),false)), // 中国金融期货交易所
            _ => { Err(AnyHow(anyhow::anyhow!("无法判断代码:{code}的市场")))}
        };
        //进行二次判断是为了支持处理主连代码（都是以m结尾的期货代码）
        if let Err(_) = first_match{
            if code.len()==0 { return Err(AnyHow(anyhow::anyhow!("无法判断代码:{code}的市场"))) }
            //直接去掉最后一个字符再次进行判断（实际应该加一步判断最后一个字符是不是m的，这里偷懒了），如果可以匹配则成功并将主连标志设为true
            let new_code = &code[..code.len() - 1];
            return if let Ok(result) = get_market_by_code(new_code) {
                Ok((result.0, true))
            } else { Err(AnyHow(anyhow::anyhow!("无法判断代码:{code}的市场"))) }
        }else {
            first_match
        }
    }
}
///判断当前是否是交易时间
/// todo: 目前没有实现 http://ifzq.gtimg.cn/appstock/app/kline/mkline?param=sz159967,m1,,673
pub fn judge_market_is_open() -> AppResult<bool> {
    Ok(false)
}
///计算当前时间和给定的日期之间的天数差1
/// 注意：如果输入日期是当天，则返回0
/// year: 年份
/// month: 月份
/// day: 日
/// return: 天数差
pub fn calculate_ago_days_with_num(year: i32, month: u32, day: u32) -> i32 {
    // 获取当前系统时间
    let now = Local::now().naive_local().date();
    // let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    now.signed_duration_since(date).num_days() as i32
}
///计算给定日期字符串距离当前日期的天数差2，并返回该差值（以天数为单位)
/// 注意：如果输入日期是当天，则返回0
/// str: 日期字符串，如"2024-04-26"
pub fn calculate_ago_days_with_str(str: &str) -> i32 {
    // 获取当前系统时间
    let now = Local::now().naive_local().date();
    let date = NaiveDate::from_str(str).unwrap();
    now.signed_duration_since(date).num_days() as i32
}
///计算当前时间和给定的时间之间的分钟差
/// target_time: 时间字符串，如"15:30"
pub fn calculate_ago_minutes(target_time:&str) -> i64 {
    let target_time = NaiveTime::parse_from_str(target_time, "%H:%M").unwrap();
    // 获取当前的日期和时间
    let now = Local::now().time();
    let delta = now - target_time;
    delta.num_minutes()
}
#[tokio::test]
async fn test_compute_ma() {
    let data = (0..).take(70).map(|x| x as f64).collect::<Vec<f64>>();
    compute_single_ma(60, &data).await;
}
#[tokio::test]
async fn test_get_market_by_code() {
    // let string = get_market_by_code("000001").unwrap();
    let string = get_market_by_code("hcm").unwrap();
    // let string = get_market_by_code("pk505").unwrap();
    println!("{:?}", string);
}
#[tokio::test]
async fn test_calculate_day_num() {
    println!("{}", calculate_ago_days_with_num(2020, 1, 1));
    println!("{}", calculate_ago_days_with_str("2024-11-08".into()));
}
#[test]
fn test_parse_date() {
    println!("{:?}", NaiveDate::from_str("2024-04-26"));
    println!("{:?}", calculate_ago_days_with_str("2024-04-24"));
}
#[test]
fn test_calculate_minutes() {
    let i = calculate_ago_minutes("15:30");
    println!("{}", i);
}