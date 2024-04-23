use chrono::{DateTime, Local, NaiveDate};
use crate::app_errors::AppError::AnyHow;
use crate::app_errors::AppResult;

///根据给定的num，和收盘价列表计算Ma num日均线，数据不足的地方填None；保留三位小数。
/// 保证返回的均线数据的长度和close_price一样长。否则返回err。
pub async fn compute_ma(day_count: usize, close_price: Vec<f64>) -> Vec<Option<f64>> {
    let mut sum:f64; 
    let mut result = vec![];
    let mut window_start = 0; // 滑动窗口的起始位置
    sum = close_price.iter().take(day_count-1).sum::<f64>();
    for _ in 0..(day_count-1){
        result.push(None); //一共放置day_count-1个None，比如计算60日均线，则放置59个None,因为下面计算第一步就是再加一次
    }
    for i in (day_count-1)..close_price.len(){
        sum += close_price[i];
        let ma = sum / day_count as f64;
        let x = format!("{:.3}", ma).parse::<f64>().unwrap();
        result.push(Some(x));
        sum -= close_price[window_start];
        window_start += 1;
    }
    result
}
///根据输入的股票代码判断是沪市（sh）还是深市（sz）的
pub fn get_market_by_code(code:&str)->AppResult<String>{
    return if code.starts_with("15")||code.starts_with("000")||code.starts_with("002")||code.starts_with("300")||code.starts_with("200"){
        Ok("sz".into()) //深圳
    }
    else if code.starts_with("51")||code.starts_with("60")||code.starts_with("588")||code.starts_with("688")||code.starts_with("900"){
        Ok("sh".into())//上海
    }else {
        Err(AnyHow(anyhow::anyhow!("无法判断代码:{code}的市场")))
    }
}
pub fn calculate_day_num() -> i32{
    // 获取当前系统时间
    let now = Local::now().naive_local().date();
    let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    (now.signed_duration_since(date).num_days() as i32) - 300
}
#[tokio::test]
async fn test_compute_ma() {
    let data = (0..).take(70).map(|x|x as f64).collect::<Vec<f64>>();
    compute_ma(60,data).await;
}
#[tokio::test]
async fn test_get_market_by_code() {
    get_market_by_code("000001").unwrap();
}
#[tokio::test]
async fn test_calculate_day_num() {
    println!("{}",calculate_day_num());
}