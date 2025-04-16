use std::collections::HashMap;
use anyhow::Context;
use itertools::Itertools;
use log::{error, info};
use serde_json::Value;
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::dtos::stock_dto::StockLiveData;
use crate::entities::init_db_coon;
use crate::entities::prelude::StockData;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::http::http_client::HttpRequest;
use crate::service::http::{init_http, start_data_server, REQUEST};
use crate::utils::stock_util::get_market_by_code;

// 定义常量，用于存储API的URL。
const SERVER_URL: &str = "http://127.0.0.1:8080/api/public";
impl HttpRequest{
    ///https://akshare.akfamily.xyz/data/futures/futures.html#id57
    ///eg. http://127.0.0.1:8080/api/public/futures_hist_em?start_date=2023-02-04&symbol=cs2503&period=daily&end_date=2025-02-03
    /// 主连合约不要用这函数！！
    pub async fn get_futures_daily_history(&self,symbol: &str,
                                           start_date: &str,
                                           end_date: &str,) -> AppResult<Vec<StockData>> {
        let params = HashMap::from([
            ("symbol", symbol),
            ("period", "daily"),
            ("start_date", start_date),
            ("end_date", end_date),
        ]);
        let endpoint = "futures_hist_em";
        let full_url = format!("{}/{}", SERVER_URL, endpoint);
        info!("{}", full_url);
        let response = self.client.get(&full_url).query(&params).send().await?;
        let text = response.status();
        let item_list:Vec<Value> = response.json().await.with_context(|| {
            format!("解析{}日线数据的response.json()失败,错误码{}",symbol,text)
        })?;
        // let item_list:Vec<Value> = response.json().await.context("解析futures_daily_history失败")?;
        let mut stock_data_list = Vec::new();
        for item in item_list {
            //2025-01-20T00:00:00.000
            let date = item["时间"].as_str().unwrap().chars().take(10).collect::<String>();
            let open = item["开盘"].as_f64().unwrap();
            let close = item["收盘"].as_f64().unwrap();
            let high = item["最高"].as_f64().unwrap();
            let low = item["最低"].as_f64().unwrap();
            let volume = item["成交量"].as_f64().unwrap();
            let stock_data = StockData {
                date,
                open,
                close,
                high,
                low,
                vol: volume,
                ma5: None,
                ma10: None,
                ma20: None,
                ma30: None,
                ma60: None,
            };
            stock_data_list.push(stock_data);
        }
        Ok(stock_data_list)
    }
    ///新浪财经-期货-主力连续合约历史数据 https://akshare.akfamily.xyz/data/futures/futures.html#id66
    /// 这个函数大体和[HttpRequest::get_futures_daily_history()]一样，但是好几处小差别（接口不同，也没有period，提取关键词也不同），还是单独写一个函数吧。
    ///eg. http://127.0.0.1:8080/api/public/futures_main_sina?start_date=2023-02-04&symbol=M0&end_date=2025-02-03
    pub async fn get_futures_main(&self,code:&str,start_date: &str,
                              end_date: &str,)->AppResult<Vec<StockData>>{
        //东方财富传来的主连代码都是以m结尾，而新浪财经-期货-主力连续合约api的接口要求是以0表示主连，所以需要去掉最后的m加上0
        // code.trim_end_matches('m')
        let mut code = code[..code.len() - 1].to_uppercase();
        code.push('0');
        let params = HashMap::from([
            ("symbol", code.as_str()),
            ("start_date", start_date),
            ("end_date", end_date),
        ]);
        let endpoint = "futures_main_sina";
        let full_url = format!("{}/{}", SERVER_URL, endpoint);
        info!("{}", full_url);
        let response = self.client.get(&full_url).query(&params).send().await?;
        let text = response.status();
        let item_list:Vec<Value> = response.json().await.with_context(|| {
            format!("解析{}主连合约的response.json()失败,错误码{}",code,text)
        })?;
        let mut stock_data_list = Vec::new();
        for item in item_list {
            //2025-01-20T00:00:00.000
            let date = item["日期"].as_str().unwrap().chars().take(10).collect::<String>();
            let open = item["开盘价"].as_f64().unwrap();
            let close = item["收盘价"].as_f64().unwrap();
            let high = item["最高价"].as_f64().unwrap();
            let low = item["最低价"].as_f64().unwrap();
            let volume = item["持仓量"].as_f64().unwrap();//这里应该是成交量的，但是接口返回的是持仓量，没有成交量
            let stock_data = StockData {
                date,
                open,
                close,
                high,
                low,
                vol: volume,
                ma5: None,
                ma10: None,
                ma20: None,
                ma30: None,
                ma60: None,
            };
            stock_data_list.push(stock_data);
        }
        Ok(stock_data_list)
        // let name = name.replace("主连", "连续");
        // let symbol = match name{
        //     "PVC连续" => "V0",
        //     "棕榈油连续" => "P0",
        //     "豆二连续" => "B0",
        //     "豆粕连续" => "M0",
        //     "铁矿石连续" => "I0",
        //     "鸡蛋连续" => "JD0",
        //     "PVC连续" => "V0",
        //     "PVC连续" => "V0",
        //     "PVC连续" => "V0",
        // }
    }
    /// 获取期货实时数据
    ///
    /// 该函数根据传入的期货代码列表 (`codes`)，查询每个代码对应的实时市场数据，并返回一个包含实时数据的 `HashMap`。
    /// 每个期货代码的实时数据包括价格、涨跌幅、开盘价、最高价、最低价、成交量等信息。
    ///
    /// # 参数
    /// - `codes`: 一个包含期货代码的 `Vec<String>`。如果列表为空，函数将直接返回一个空的 `HashMap`。
    ///
    /// # 返回值
    /// - 返回一个 `AppResult<HashMap<String, StockLiveData>>`，其中：
    ///   - `HashMap` 的键是期货代码（`String`）。
    ///   - `HashMap` 的值是 `StockLiveData` 结构体，包含期货的实时市场数据。
    ///
    /// # 错误
    /// - 如果某个期货代码的信息未找到，将返回一个 `Tip` 错误，提示 `"{code}信息未找到"`。
    /// - 如果数据获取或处理过程中发生其他错误，将返回相应的 `AppResult::Err`。
    ///
    /// # 实现细节
    /// 1. 如果 `codes` 为空，直接返回一个空的 `HashMap`。
    /// 2. 遍历 `codes`，跳过已经处理过的代码。
    /// 3. 对于每个代码，通过 `StockInfoCurd::query_info_by_code` 查询期货的详细信息。
    /// 4. 从期货名称中过滤掉数字部分，得到用于查询的符号（`symbol`）。
    /// 5. 调用 `get_futures_live_data_by_symbol` 获取该符号的实时数据。
    /// 6. 将实时数据解析为 `StockLiveData` 结构体，并插入到 `HashMap` 中。
    /// 7. 返回包含所有期货实时数据的 `HashMap`。
    pub async fn get_futures_live_datas(&self,codes: Vec<String>)->AppResult<HashMap<String, StockLiveData>>{
        if codes.is_empty() {
            return Ok(HashMap::new());
        }
        // let codes = codes.iter().map(|code| code.to_lowercase()).collect::<Vec<_>>();
        let mut live_data = HashMap::with_capacity(codes.len());
        let mut is_handled = Vec::new();
        for code in &codes {
            if is_handled.contains(code) {
                info!("{}已处理,", code);
                continue;
            }
            let is_main = get_market_by_code(&code)?.1;
            let info = StockInfoCurd::query_info_by_code(code.clone()).await?.ok_or(Tip(format!("{}信息未找到", code)))?;
            let mut full_name = info.name;
            //代码中有主连（如豆粕主连），需要去掉"主连"，非主连（如MA505）需要去掉数字。
            //并且 主连的代码和接口返回的代码不一样，所以需要特殊处理一下。
            if is_main {
                full_name = full_name.replace("主连","");
            }
            let name: String = full_name.chars()
                .filter(|c| !c.is_digit(10)) // Filter out digits
                .collect::<String>();

            let item_list = self.get_futures_live_data_by_symbol(&name).await?;
            info!("{}获取到{:?}数据",name,item_list);
            for item in item_list {
                //新浪接口返回的symbol是一般形如MA2505，但是东方财富大部分期货代码是小写的，如ma505，所以需要特殊转小写。
                let mut symbol = item["symbol"].as_str().unwrap().to_lowercase();//ma505
                //但是东方财富又有一些大写的代码，如SA505，所以需要特殊处理一下。
                //特殊处理，因为甲醇在东方财富是MA505，但是通过新浪接口获取的是MA2505，经过to_lowercase()后是ma2505
                //凡是东方财富内代码为XX50X之类的（如MA505），都需要将上面获得的symbol去掉2，然后转为大写
                if symbol.contains("ma2"){
                    symbol = symbol.replace("ma2", "MA");
                }else if symbol.contains("sa2") { 
                    symbol = symbol.replace("sa2", "SA");
                }else if symbol.contains("pf2") {
                    symbol = symbol.replace("pf2", "PF");
                }else if symbol.contains("fg2") {
                    symbol = symbol.replace("fg2", "FG");
                }
                //特殊处理：如果是主连，则需要将接口返回的symbol去掉末尾的0，加上m，与传入的code判断是否一致（需要忽略大小写）
                //如果一致，就匹配成功，把code赋值给symbol
                if is_main{
                    // 去掉 symbol 末尾的一个 '0'
                    let trimmed_symbol = if symbol.ends_with('0') {
                        &symbol[..symbol.len() - 1]
                    } else {
                        &symbol
                    };
                    // 加上'm'
                    let new_symbol = format!("{}m", trimmed_symbol);
                    // 忽略大小写比较
                    symbol = if new_symbol.eq_ignore_ascii_case(code) {
                        // 匹配成功，返回新的 symbol
                        code.clone()
                    } else {
                        symbol
                    }
                }
                if codes.contains(&symbol){
                    let percent = item["changepercent"].as_f64().unwrap() * 100.0;
                    let percent_rounded: f64 = format!("{:.2}", percent).parse().unwrap();
                    live_data.insert(
                        symbol.clone(),
                        StockLiveData {
                            code: symbol.clone(),
                            price: item["trade"].as_f64().unwrap(),
                            change: 0f64,
                            percent: percent_rounded,
                            open: item["open"].as_f64().unwrap(),
                            low: item["low"].as_f64().unwrap(),
                            high: item["high"].as_f64().unwrap(),
                            volume: item["volume"].as_f64().unwrap(),
                            ma5: None,
                            ma10: None,
                            ma20: None,
                            ma30: None,
                            ma60: None,
                        },
                    );
                    is_handled.push(symbol.clone());
                }
            }
        }
        Ok(live_data)
    }
    ///https://akshare.akfamily.xyz/data/futures/futures.html#id55
    /// http://127.0.0.1:8080/api/public/futures_symbol_mark 获取所有的期货品种
    /// name：eg 花生
    /// eg http://127.0.0.1:8080/api/public/futures_zh_realtime?symbol=%E9%83%91%E9%86%87
    async fn get_futures_live_data_by_symbol(&self, name: &str) ->AppResult<Vec<Value>>{
        let name = match name{
            "热卷" => "热轧卷板",
            "淀粉" => "玉米淀粉",
            "甲醇" => "郑醇",
            _ => name,
        };
        let params = HashMap::from([
            ("symbol", name),
        ]);
        let endpoint = "futures_zh_realtime";
        let full_url = format!("{}/{}", SERVER_URL, endpoint);
        info!("查询{}期货实时数据:{}", name, full_url);
        let response = self.client.get(&full_url).query(&params).send().await?;
        info!("{}期货实时数据:{:?}", name, response);
        let result = response.json().await;
        info!("{}期货实时数据:{:?}", name, result);
        let item_list:Vec<Value> = result.with_context(|| {format!("解析{}期货实时数据失败.", name)})?;
        // println!("{}", item_list);
        Ok(item_list)
    }
    ///https://akshare.akfamily.xyz/data/futures/futures.html#id54
    /// 这个不好用，目前没用到
    pub async fn get_futures_live_data2(&self,symbol: &str)->AppResult<()>{
        let params = HashMap::from([
            ("symbol", symbol),
            ("market", "CF"),
            ("adjust", "0"),
        ]);
        let endpoint = "futures_zh_spot";
        //http://127.0.0.1:8080/api/public/futures_zh_spot?market=CF&adjust=0&symbol=SS2503
        //试了PK2505（花生）怎么不行啊
        let full_url = format!("{}/{}", SERVER_URL, endpoint);
        println!("{}", full_url);
        let response = self.client.get(&full_url).query(&params).send().await?;
        println!("{:?}", response);
        let item_list:Value = response.json().await?;
        println!("{}", item_list);
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use reqwest::header::{HeaderMap, HeaderValue};
    use serde_json::Value;
    use crate::entities::init_db_coon;
    use crate::service::http::{init_http, REQUEST};

    #[tokio::test]
    async fn test_get_futures_daily_history() {
        init_http().await;
        // tokio::spawn(async {
        //     start_data_server("E:\\ANACONDA\\Main\\Scripts\\activate.bat && conda activate akenv &&python -m aktools").await.unwrap();
        // });
        REQUEST.get().unwrap().get_futures_daily_history("rb2505", "20241225", "20250127").await.unwrap();
    }
    #[tokio::test]
    async fn test_get_futures_live_data() {
        init_http().await;
        // REQUEST.get().unwrap().get_futures_live_data("豆粕").await.unwrap();
        REQUEST.get().unwrap().get_futures_live_data2("HC2505").await.unwrap();
    }
    #[tokio::test]
    async fn test_get_futures_live_data_by_codes() {
        init_http().await;
        init_db_coon().await;
        let codes = vec![
            "rb2505".to_string(),
            "hc2503".to_string(),
            "i2505".to_string(),
        ];
        REQUEST.get().unwrap().get_futures_live_datas(codes).await.unwrap();
    }
    #[tokio::test]
    async fn test_request() {
        init_http().await;
        // 创建一个 HeaderMap 来存储请求头
        let mut headers = HeaderMap::new();

        // 添加浏览器的请求头
        headers.insert(
            "User-Agent",
            HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"),
        );
        headers.insert(
            "Accept",
            HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"),
        );
        headers.insert(
            "Accept-Encoding",
            HeaderValue::from_static("gzip, deflate, br, zstd"),
        );
        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("en,zh-CN;q=0.9,zh;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
        );
        headers.insert(
            "Cache-Control",
            HeaderValue::from_static("no-cache"),
        );
        headers.insert(
            "Connection",
            HeaderValue::from_static("keep-alive"),
        );
        headers.insert(
            "Host",
            HeaderValue::from_static("127.0.0.1:8080"),
        );
        headers.insert(
            "Pragma",
            HeaderValue::from_static("no-cache"),
        );
        headers.insert(
            "Sec-Fetch-Dest",
            HeaderValue::from_static("document"),
        );
        headers.insert(
            "Sec-Fetch-Mode",
            HeaderValue::from_static("navigate"),
        );
        headers.insert(
            "Sec-Fetch-Site",
            HeaderValue::from_static("none"),
        );
        headers.insert(
            "Sec-Fetch-User",
            HeaderValue::from_static("?1"),
        );
        headers.insert(
            "Upgrade-Insecure-Requests",
            HeaderValue::from_static("1"),
        );
        headers.insert(
            "sec-ch-ua",
            HeaderValue::from_static("\"Microsoft Edge\";v=\"137\", \"Chromium\";v=\"137\", \"Not/A)Brand\";v=\"24\""),
        );
        headers.insert(
            "sec-ch-ua-mobile",
            HeaderValue::from_static("?0"),
        );
        headers.insert(
            "sec-ch-ua-platform",
            HeaderValue::from_static("\"Windows\""),
        );
        let client = reqwest::Client::new();
        let response = client.get("http://127.0.0.1:8080/api/public/futures_zh_realtime?symbol=%E9%83%91%E9%86%87")
            .headers(headers)
            .send().await.unwrap();
        // let response = client.get("http://127.0.0.1:8080/api/public/futures_zh_realtime?symbol=%E9%83%91%E9%86%87")
        //     .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        //     .header("Accept-Encoding", "br, deflate, gzip, x-gzip")
        //     .header("Connection", "keep-alive")
        //     .header("Accept", "*/*")
        //     .header("Cookie","CSRF-Token-ZEOCTHF=DEyuY3Trn5Sq5gjmXrcyMwNiJyJanxgxoZAHViUV94S4eStFabm6gaW4QgURAAaf")
        //     .send().await.unwrap();
        // let response = REQUEST.get().unwrap().get_response("http://127.0.0.1:8080/api/public/futures_zh_realtime?symbol=%E9%83%91%E9%86%87").await.unwrap();
        println!("{:?}", response.headers());
    }
    #[tokio::test]
    async fn test_http() {
        let client = reqwest::Client::new();
        let client1 = reqwest::ClientBuilder::new().no_proxy().build().unwrap();
        let response = client1.get("http://127.0.0.1:8080/api/public/futures_zh_realtime")
            .query(&[("symbol", "郑醇")]).send().await.unwrap();//502 Bad Gateway
        println!("{:?}", response.json::<Value>().await.unwrap());
        let response = client.get("http://127.0.0.1:8080/api/public/futures_zh_realtime")
            .query(&[("symbol", "郑醇")]).send().await.unwrap();//502 Bad Gateway
        println!("{:?}", response);
        // let body= ureq::get("http://127.0.0.1:8080/api/public/futures_zh_realtime?symbol=%E9%83%91%E9%86%87")
        //     .call().unwrap()
        //     .body_mut().read_to_string();//正常
        // println!("{:?}", body);
    }
}