use crate::cache::intraday_chart_cache::IntradayChartCache;
use crate::config::config::{Config, DataConfig};
use crate::dtos::graphic_dto::GraphicDTO;
use crate::dtos::stock_dto::{StockInfoG, StockLiveData};
use crate::entities::prelude::{Graphic, Position, StockData, StockGroup, StockInfo, TransactionRecord};
use crate::service::command::handle::{handle_and_save_record, handle_can_t, handle_delete_stock, handle_new_stock, handle_stock_livedata, handle_update_position};
use crate::service::curd::graphic_curd::GraphicCurd;
use crate::service::curd::group_stock_relation_curd::GroupStockRelationCurd;
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_group_curd::StockGroupCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::curd::transaction_record_curd::TransactionRecordCurd;
use crate::service::http::{init_http, REQUEST};
use crate::{get_close_prices, MyState, IS_MARKET_OPEN, UPDATEING};
use log::{error, info};
use std::collections::HashMap;
use std::process::exit;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::mpsc::Sender;
use tokio::time::{sleep,Duration};
use crate::app_errors::AppError::Tip;
use crate::service::curd::position_curd::PositionCurd;

#[tauri::command]
pub async fn update_live_state<'r>(
    state: State<'r, MyState>,
    app_handle: AppHandle,
    config: State<'r, Mutex<Config>>,
    group_name: String,
    live_state: bool,
) -> Result<(), String> {
    // info!("更新状态:{}",live_state);
    UPDATEING.store(live_state, Ordering::Relaxed);
    if live_state {
        let _ = query_live_stocks_data_by_group_name(state,config, group_name, app_handle).await;
    }
    Ok(())
}
// #[tauri::command]
// pub async fn update_all_stock_day_k() -> Result<String, String> {
//     // state.update_live_state(live_state);
//     match update_all_day_k().await {
//         Ok(_) => Ok("更新成功".to_string()),
//         Err(e) => {
//             handle_error("更新日线数据失败", e.to_string())
//             // Err(format!("更新日线数据失败:{}",e.to_string()))
//         }
//     }
// }
#[tauri::command]
pub async fn get_response(url: String) -> Result<String, String> {
    match REQUEST.get().unwrap().get_response(&url).await {
        Ok(response) => {
            // info!("ok");
            Ok(response.text().await.unwrap())
        }
        Err(e) => {
            handle_error(&format!("http请求错误,地址为{}", url), e.to_string())
            // error!("http请求错误:{}",e);
            // Err(format!("http请求错误:{}",e.to_string()))
        }
    }
}
#[tauri::command]
pub async fn add_stock_info(code: String, name: String) -> Result<StockInfo, String> {
    let stock_info = StockInfo::new(code, name);
    match StockInfoCurd::insert(stock_info.clone()).await {
        Ok(stock_info) => {
            // info!("插入成功:{:?}",stock_info);
            Ok(stock_info)
        }
        Err(e) => {
            handle_error(&format!("插入失败:{:?}", stock_info), e.to_string())
            // error!("插入失败:{}",e);
            // Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn query_stock_info() -> Result<Vec<StockInfo>, String> {
    match StockInfoCurd::query_all().await {
        Ok(stock_infos) => {
            // info!("查询所有信息成功:{:?}",stock_infos);
            Ok(stock_infos)
        }
        Err(e) => {
            handle_error("查询所有股票信息失败", e.to_string())
            // error!("查询失败:{}",e);
            // Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn query_all_groups() -> Result<Vec<StockGroup>, String> {
    info!("查询所有的分组");
    match StockGroupCurd::query_all().await {
        Ok(stock_groups) => {
            // info!("查询所有分组成功:{:?}",stock_groups);
            Ok(stock_groups)
        }
        Err(e) => {
            handle_error("查询所有分组失败", e.to_string())
            // error!("查询失败:{}",e);
            // Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn query_stocks_by_group_name(name: String) -> Result<Vec<StockInfoG>, String> {
    // error!("根据分组名查询分组中的股票:{}",name);
    if name == "持有" {
        return match StockInfoCurd::query_all_hold_info().await {
            Ok(more_infos) => {
                // error!("查询持有分组成功:{:?}",more_infos);
                Ok(more_infos)
            }
            Err(e) => {
                handle_error("查询持有分组失败", e.to_string())
            }
        };
    }
    match GroupStockRelationCurd::query_stocks_by_group_name(name.clone()).await {
        Ok(more_infos) => {
            // info!("根据分组名查询成功:{:?}",more_infos);
            Ok(more_infos)
        }
        Err(e) => {
            handle_error(
                &format!("根据分组名{}查询其中股票失败", name),
                e.to_string(),
            )
            // error!("查询失败:{}",e);
            // Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn query_groups_by_code(code: String) -> Result<Vec<String>, String> {
    match GroupStockRelationCurd::query_groups_by_stock_code(code.clone()).await {
        Ok(stock_groups) => {
            // info!("根据股票代码查询分组成功:{:?}",stock_groups);
            Ok(stock_groups)
        }
        Err(e) => {
            handle_error(&format!("根据股票代码{}查询分组失败", code), e.to_string())
            // error!("查询失败:{}",e);
            // Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn create_group(name: String) -> Result<i32, String> {
    match StockGroupCurd::insert(StockGroup::new(name)).await {
        Ok(stock_group) => {
            // info!("插入分组成功:{:?}",stock_group);
            Ok(stock_group.index)
        }
        Err(e) => {
            handle_error("插入分组失败", e.to_string())
            // error!("插入分组失败:{}",e);
            // Err(e.to_string())
        }
    }
}
/// 更新分组索引
#[tauri::command]
pub async fn update_groups(groups: Vec<StockGroup>) -> Result<(), String> {
    match StockGroupCurd::update_all_index(groups).await {
        Ok(_) => {
            // info!("删除分组成功:{:?}",count);
            Ok(())
        }
        Err(e) => {
            handle_error("更新分组索引失败", e.to_string())
            // error!("更新分组索引失败:{}",e);
            // Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn delete_group(name: String) -> Result<i32, String> {
    match StockGroupCurd::delete_by_name(name.clone()).await {
        Ok(count) => {
            // info!("删除分组成功:{:?}",count);
            Ok(count)
        }
        Err(e) => {
            handle_error(&format!("删除分组{}失败", name.as_str()), e.to_string())
            // error!("删除分组失败:{}",e);
            // Err(e.to_string())
        }
    }
}
/// 更新股票的分组信息（前端需保证is_new==true和group_names长度为0不同时出现）
/// 如果是一个新的股票，则先插入股票信息表，在更新分组表
/// 如果不是一个新的股票，但是分组名为空，则先删除分组关系表，再删除股票信息表相关信息。若分组名不空，直接更新分组表
#[tauri::command]
pub async fn update_stock_groups<'r>(
    state: State<'r, MyState>,
    config: State<'r, Mutex<Config>>,
    is_new: bool,
    code: String,
    name: String,
    group_names: Vec<String>,
    app_handle: AppHandle
) -> Result<(), String> {
    info!(
        "is_new:{},code:{},name:{},group_names:{:?}",
        is_new, code, name, group_names
    );
    if is_new {
        let use_ak_share = config.lock().unwrap().data_config.use_ak_share;
        match handle_new_stock(&code, &name, use_ak_share).await {
            Ok(_) => {
                match get_close_prices(Some(&code)).await {
                    Ok(prices) => state.update_history_close_price(
                        code.clone(),
                        prices.get(&code).unwrap().clone(),
                    ),
                    Err(e) => {
                        return handle_error(&format!("更新缓存失败:{}", code), e.to_string());
                        // error!("更新缓存失败:{}",e);
                        // return Err(format!("更新缓存失败:{}",e.to_string()));
                    }
                }
                info!("创建成功:{}", code);
            }
            Err(e) => {
                //https://docs.rs/anyhow/1.0.95/anyhow/struct.Error.html#display-representations
                //To print causes as well using anyhow’s default formatting of causes, use the alternate selector “{:#}”.
                // 不知道为啥么用{:#}无法打印出Caused by的信息
                // println!("发生错误: {:#}", e);
                //用{:?}能够打印出Caused by的信息，但是backtrace不可用，推测是thiserror的问题，单测了anyhow是可以的。
                // println!("发生错误: {:?}", e);
                //如果新建的过程发生了错误，直接删除这个股票（因为有可能已经添加了一些该股票的表和信息），起到一个事务的效果。
                let _ = handle_delete_stock(&code).await;
                let error = format!("错误：{:?}", e);
                // error!("{}", error);
                return handle_error(&format!("创建{}失败", code), error);
            }
        }
    } else if group_names.is_empty() {//全部没选上
        return delete_stock(code.clone(),app_handle).await;
    };
    match GroupStockRelationCurd::update_groups_by_code(code.clone(), group_names).await {
        Ok(_) => {
            // info!("更新分组成功。");
            Ok(())
        }
        Err(e) => {
            handle_error(&format!("更新{}分组失败", code), e.to_string())
            // error!("更新分组失败:{}",e);
            // Err(e.to_string())
        }
    }
}
/// 从指定分组中移除股票。
///
/// 该函数根据传入的股票代码 (`code`) 和分组名称 (`group_name`) 从分组中移除对应的股票。
/// 如果分组名称为 "全部"，则会调用 `handle_delete_stock` 函数删除该股票。
/// 否则，会调用 `GroupStockRelationCurd::delete_by_code_and_group_name` 方法从指定分组中移除股票。
///
/// # 参数
/// - `code`: 股票代码，类型为 `String`。
/// - `group_name`: 分组名称，类型为 `String`。
///
/// # 返回值
/// - 成功时返回 `Ok(())`。
/// - 失败时返回 `Err(String)`，其中包含错误信息。
///
/// # 错误处理
/// - 如果删除操作失败，会调用 `handle_error` 函数记录错误信息，并返回包含错误描述的 `Err(String)`。
///
/// # 注意
/// - 当 `group_name` 为 "全部" 时，会删除该股票的所有记录。
/// - 其他情况下，仅从指定分组中移除该股票。
#[tauri::command]
pub async fn remove_stock_from_group(app_handle: AppHandle,code: String, group_name: String) -> Result<(), String> {
    if group_name == "全部" {
        return delete_stock(code.clone(),app_handle).await;
    }
    match GroupStockRelationCurd::delete_by_code_and_group_name(code.clone(), group_name.clone())
        .await
    {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            handle_error(
                &format!("从{}分组删除股票{}失败", group_name, code),
                e.to_string(),
            )
            // error!("删除失败:{}",e);
            // Err(e.to_string())
        }
    }
}
/// 删除指定股票。
///
/// 该函数根据传入的股票代码 (`code`) 删除对应的股票记录。
/// 删除成功后，会通过 `app_handle` 发送一个 `delete_stock` 事件，通知前端的StockTable.vue。
///
/// # 参数
/// - `code`: 股票代码，类型为 `String`。
/// - `app_handle`: Tauri 应用的 `AppHandle` 实例，用于发送事件。
///
/// # 返回值
/// - 成功时返回 `Ok(())`。
/// - 失败时返回 `Err(String)`，其中包含错误信息。
/// # 错误处理
/// - 如果删除操作失败，会调用 `handle_error` 函数记录错误信息，并返回包含错误描述的 `Err(String)`。
///
/// # 事件通知
/// - 删除成功后，会通过 `app_handle.emit` 发送一个 `delete_stock` 事件，事件数据为被删除的股票代码。
///
/// # 注意
/// - 该函数会永久删除股票记录，请谨慎调用。
//todo :彻底删除股票并没有提醒用户确认（可以通过分组管理不勾选任何分组/从全部分组页面选择移除/右键菜单删除可以彻底删除股票）。
#[tauri::command]
pub async fn delete_stock(code: String, app_handle: AppHandle)-> Result<(), String>{
    match handle_delete_stock(&code).await {
        Ok(_) => {
            info!("删除{}成功",code);
            app_handle.emit("delete_stock", code).unwrap();
            Ok(())
        }
        Err(e) => {
            handle_error(&format!("删除{}失败", code), e.to_string())
        }
    }
}
#[tauri::command]
pub async fn update_stock_hold(code: String, hold: bool) -> Result<(), String> {
    match StockInfoCurd::update_hold_by_code(code.clone(), hold).await {
        Ok(_) => {
            // info!("更新成功。");
            Ok(())
        }
        Err(e) => {
            handle_error(&format!("更新{}持仓状态失败", code), e.to_string())
            // error!("更新失败:{}",e);
            // Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn query_stocks_day_k_limit(code: String) -> Result<Vec<StockData>, String> {
    match StockDataCurd::query_by_nums(&code, 1000).await {
        Ok(stock_data_list) => {
            // info!("查询成功:{:?}",stock_datas);
            Ok(stock_data_list)
        }
        Err(e) => {
            handle_error(&format!("查询{}日线数据失败", code), e.to_string())
            // error!("查询日线数据失败:{}",e);
            // Err(format!("查询日线数据失败:{}",e))
        }
    }
}
#[tauri::command]
pub async fn query_graphic_by_code(code: String) -> Result<Vec<GraphicDTO>, String> {
    match GraphicCurd::query_by_code(code.clone()).await {
        Ok(data_list) => {
            info!("查询成功:{:?}", data_list);
            let data = data_list
                .into_iter()
                .map(|item| From::<Graphic>::from(item))
                .collect::<Vec<_>>();
            // info!("查询成功:{:?}",data);
            Ok(data)
        }
        Err(e) => {
            handle_error(&format!("查询{}图形元素失败", code), e.to_string())
            // error!("查询失败:{}",e);
            // Err(e.to_string())
        }
    }
}
/// 保存图形元素
/// code:因为graphic有可能是空数组，所以必须传入。
#[tauri::command]
pub async fn save_graphic(code: String, graphic: Vec<GraphicDTO>) -> Result<(), String> {
    // info!("保存图形元素:{:?}", graphic);
    if graphic.is_empty() {
        return Ok(());
    }
    match GraphicCurd::update_all(
        code,
        graphic
            .into_iter()
            .map(|item| item.into())
            .collect::<Vec<_>>(),
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            handle_error("保存图形元素失败", e.to_string())
            // error!("查询失败:{}",e);
            // Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn delete_graphic_by_id(id: String) -> Result<(), String> {
    match GraphicCurd::delete_by_id(id.clone()).await {
        Ok(_) => {
            info!("删除图形{:?}成功", id);
            Ok(())
        }
        Err(e) => {
            handle_error(&format!("删除分组{:?}内的图形失败", id), e.to_string())
            // error!("删除图形{:?}失败",id);
            // Err(format!("删除图形{:?}失败:{}",id,e.to_string()))
        }
    }
}
#[tauri::command]
pub async fn delete_graphic_by_group_id(group_id: String) -> Result<(), String> {
    match GraphicCurd::delete_by_group_id(group_id.clone()).await {
        Ok(_) => {
            info!("删除图形{:?}成功", group_id);
            Ok(())
        }
        Err(e) => {
            handle_error(&format!("删除图形{:?}失败", group_id), e.to_string())
            // error!("删除图形{:?}失败",group_id);
            // Err(format!("删除图形{:?}失败:{}",group_id,e.to_string()))
        }
    }
}
#[tauri::command]
pub async fn query_box() -> Result<HashMap<String, Vec<f64>>, String> {
    match GraphicCurd::query_only_horizontal_all().await {
        Ok(data) => {
            // info!("查到了箱体数据{:?}",data);
            Ok(data)
        }
        Err(e) => {
            handle_error("查询箱体数据失败", e.to_string())
            // error!("查询失败:{}",e);
            // Err(e.to_string())
        }
    }
}
///查询单个股票的实时数据，用于点击k线图时获取最新数据，无需等待
#[tauri::command]
// pub async fn query_live_stock_data_by_code(code:String,app_handle: tauri::AppHandle,) -> Result<HashMap<String,StockLiveData>,String> {
pub async fn query_live_stock_data_by_code<'r>(
    code: String,
    state: State<'r, MyState>,
    app_handle: AppHandle,
) -> Result<StockLiveData, String> {
    // info!("查询实时数据:{}",group_name);
    let data: &Arc<Vec<f64>>;
    let mut history_close_price = HashMap::new();
    {
        let guard = state.history_close_price.lock().unwrap();
        data = guard.get(&code).unwrap();
        history_close_price.insert(code.clone(), data.clone());
    }
    let codes = vec![code.clone()];
    match REQUEST.get().unwrap().get_live_data(codes.clone(),&app_handle).await {
    // match REQUEST.get().unwrap().get_live_stock_data(&codes).await {
        Ok(mut stock_data_list) => {
            match handle_stock_livedata(&codes, &mut stock_data_list, &history_close_price).await {
                Ok(_) => {
                    let option = stock_data_list.get(&code);
                    if option.is_none() {
                        let err_msg = format!("未找到{}数据", codes.join(","));
                        error!("{}数据", err_msg);
                        return Err(err_msg);
                    }
                    Ok(option.unwrap().clone())
                    // info!("查询成功:{:?}",stock_data_list);
                }
                Err(e) => {
                    handle_error("处理股票实时信息失败", e.to_string())
                    // error!("处理股票实时信息失败:{}",e);
                    // Err(e.to_string())
                }
            }
        }
        Err(e) => {
            handle_error("根据股票代码查询股票实时信息失败", e.to_string())
            // error!("查询股票实时信息失败失败:{}",e);
            // Err(e.to_string())
        }
    }
}
/// 查询分组内多个股票的实时数据
/// 先查询分组内的所有股票代码
/// todo：目前是仅根据UPDATEING的值判断是否需要请求实时数据的，本来想加上是否开市(IS_MARKET_OPEN)的判断,
/// 但是由于是否开市是提前初始化的，如果加上的话会导致第一次请求就被取消，无法第一次更新股票表格的价格导致空白。
/// 如果要加上判断开市还需要一个是否初始化过的变量。
#[tauri::command]
pub async fn query_live_stocks_data_by_group_name<'r>(
    state: State<'r, MyState>,
    config: State<'r,Mutex<Config>>,
    group_name: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    if group_name.is_empty(){//应用刚启动会有一个空的分组名进来，不知道哪里传来
        return Ok(());
    }
    // error!("查询{}分组内的实时数据", group_name);
    // if !IS_MARKET_OPEN.load(Ordering::Relaxed) {
    //     info!("市场未开市,不进行查询操作！");
    //     return Ok(());
    // }
    //查询分组内的所有股票代码
    let result = if group_name == "持有" {
        StockInfoCurd::query_all_hold_only_code().await
    } else {
        GroupStockRelationCurd::query_only_code_by_group_name(group_name.clone()).await
    };
    let update_freq = config.lock().unwrap().data_config.update_freq;
    match result {
        Ok(codes) => {
            if codes.is_empty() {
                return Ok(());
            }
            let mut history_close_price = HashMap::new();
            for code in &codes {
                let guard = state.history_close_price.lock().unwrap();
                let data = guard.get(code).unwrap();
                history_close_price.insert(code.clone(), data.clone());
            }
            let handle = tokio::spawn(async move {
                let history_close_price = history_close_price;
                loop {
                    let need_update = UPDATEING.load(Ordering::Relaxed);
                    if need_update {
                        let codess = codes.clone();
                        match REQUEST.get().unwrap().get_live_data(codess,&app_handle).await {
                        // match REQUEST.get().unwrap().get_live_stock_data(&codes).await {
                            Ok(mut stock_data_list) => {
                                match handle_stock_livedata(
                                    &codes,
                                    &mut stock_data_list,
                                    &history_close_price,
                                )
                                .await
                                {
                                    Ok(_) => {
                                        app_handle
                                            .emit("live_stock_data", stock_data_list)
                                            .unwrap();
                                    }
                                    Err(e) => {
                                        error!("处理股票实时信息失败:{}", e);
                                        app_handle.emit("get_live_data_error", e.to_string()).unwrap();
                                    }
                                }
                            }
                            Err(e) => {
                                error!("查询股票实时信息失败:{}", e);
                                app_handle.emit("get_live_data_error", e.to_string()).unwrap();
                            }
                        }
                    }
                    sleep(Duration::from_secs(update_freq as u64)).await;
                }
            });
            state.set_task(handle);
            Ok(())
        }
        Err(e) => {
            handle_error("根据分组查询分组内股票实时信息失败", e.to_string())
            // error!("查询股票实时信息失败失败:{}",e);
            // Err(e.to_string())
        }
    }
}
#[tauri::command]
pub async fn query_intraday_chart_img<'r>(
    state: State<'r, IntradayChartCache>,
    code: String,
) -> Result<tauri::ipc::Response, String> {
    match state.get_intraday_chart(&code).await {
        Ok(data) => {
            let response = tauri::ipc::Response::new(data.to_vec());
            Ok(response)
        }
        Err(e) => handle_error("查询股票分时图失败", e.to_string()),
    }
}
///历史交易数据查询。默认查询100条记录。
#[tauri::command]
pub async fn query_transaction_records() -> Result<Vec<TransactionRecord>, String> {
    info!("查询交易记录");
    match TransactionRecordCurd::query_by_num(100).await {
        Ok(data) => Ok(data),
        Err(e) => handle_error("读取交易记录文件失败", e.to_string()),
    }
}
///查询指定股票的交易记录。
#[tauri::command]
pub async fn query_transaction_records_by_code(
    code: String,
) -> Result<Vec<TransactionRecord>, String> {
    match TransactionRecordCurd::query_by_code(code.clone()).await {
        Ok(data) => Ok(data),
        Err(e) => handle_error("查询交易记录失败", e.to_string()),
    }
}
///读取新增的交易记录文件，并保存到数据库中，成功保存后返回新增的交易记录数据。
#[tauri::command]
pub async fn read_save_transaction_records(path: String) -> Result<Vec<TransactionRecord>, String> {
    match handle_and_save_record(path).await {
        Ok(data) => Ok(data),
        Err(e) => handle_error("读取、保存交易记录文件失败", e.to_string()),
    }
}
#[tauri::command]
pub async fn update_transaction_record(record: TransactionRecord) -> Result<(), String> {
    info!("更新交易记录:{:?}", record);
    match TransactionRecordCurd::update(record).await {
        Ok(_) => Ok(()),
        Err(e) => handle_error("更新交易记录失败", e.to_string()),
    }
}
#[tauri::command]
pub async fn delete_transaction_record_by_primary(
    date: String,
    time: String,
    code: String,
) -> Result<(), String> {
    match TransactionRecordCurd::delete_by_primary_key(date, time, code).await {
        Ok(_) => Ok(()),
        Err(e) => handle_error("删除交易记录失败", e.to_string()),
    }
}
#[tauri::command]
pub async fn delete_all_transaction_records() -> Result<(), String> {
    match TransactionRecordCurd::delete_all().await {
        Ok(_) => Ok(()),
        Err(e) => handle_error("删除交易记录失败", e.to_string()),
    }
}
#[tauri::command]
pub async fn save_transaction_records(path: String) -> Result<(), String> {
    match TransactionRecordCurd::save_to_file(path).await {
        Ok(_) => Ok(()),
        Err(e) => handle_error("导出交易记录失败", e.to_string()),
    }
}

#[tauri::command]
pub async fn get_config(state: State<'_, Mutex<Config>>) -> Result<Config, String> {
    let mutex_guard = state.lock().unwrap();
    Ok((*mutex_guard).clone())
}
#[tauri::command]
pub async fn update_data_config(data_config:DataConfig,state: State<'_, Mutex<Config>>) -> Result<(), String> {
    info!("更新数据配置:{:?}", data_config);
    let mut mutex_guard = state.lock().unwrap();
    mutex_guard.data_config = data_config;
    Ok(())
}
#[tauri::command]
pub async fn save_config(config: Config) -> Result<(), String> {
    info!("保存配置：{:?}",config);
    match config.save_to_file().await{
        Ok(_) => Ok(()),
        Err(e) => handle_error("保存配置文件失败", e.to_string()),
    }
}
///判断是否可以做T
///codes:股票代码列表
/// 返回值：Vec<(股票代码,status)>，status为up或者down或者normal
#[tauri::command]
pub async fn judge_can_t(codes: Vec<String>,config: State<'_, Mutex<Config>>) -> Result<Vec<(String, String)>, String> {
    let data_config = {
        let guard = config.lock().unwrap();
        guard.data_config.clone()
    };
    info!("判断是否可以做t:{:?}",data_config);
    match handle_can_t(codes,&data_config).await{
        Ok(data) => Ok(data),
        Err(e) => handle_error("判断是否可以做t失败", e.to_string()),
    }
}
#[tauri::command]
pub async fn query_all_positions() -> Result<Vec<Position>, String> {
    match PositionCurd::query_all().await{
        Ok(data) => {
            Ok(data)
        },
        Err(e) => handle_error("查询持仓失败", e.to_string()),
    }
}
///更新指定日期的持仓数据
/// date:日期（“YYYY-MM-DD”）
/// position_num:持仓百分比
/// 如果更新成功，返回Ok(())，同时emit一个position change事件，payload为(String,position_num)
#[tauri::command]
pub async fn update_position(app_handle: tauri::AppHandle,date: String, position_num: f64) -> Result<(), String> {
    match handle_update_position(date.clone(), position_num).await{
        Ok(flag) => {
            if flag {
                info!("更新持仓数据成功");
                app_handle.emit("position_update", (date, position_num)).unwrap();
            }else {
                //如果flag为false，则表示已经插入了一个数据，此时查询最新值必定有值，可以直接unwrap()
                let data = PositionCurd::query_latest_position().await.unwrap().unwrap();
                info!("插入持仓数据成功:{:?}", data);
                app_handle.emit("position_insert", data).unwrap();
            }
            Ok(())
        },
        Err(e) => handle_error("更新持仓失败", e.to_string()),
    }
}
///获取当前是否是交易时间
#[tauri::command]
pub async fn get_is_market_open() -> bool {
    IS_MARKET_OPEN.load(Ordering::Relaxed)
}

#[tauri::command]
pub async fn exit_app(send:State<'_, Sender<()>>,config: State<'_, Mutex<Config>>)->Result<(),()> {
    info!("退出程序");
    if config.lock().unwrap().data_config.use_ak_share{
        if let Err(e) =send.send(()).await{
            error!("发送退出信号失败:{}",e);
        }
    }
    // sleep(Duration::from_secs(1)).await;
    exit(0)
}
///todo 这样不行啊，所有的记录错误都在这一行
///处理错误，先记录到日志中，再返回错误
/// 使用泛型 T，这样它可以返回任何类型的 Result
fn handle_error<T>(tip: &str, e: String) -> Result<T, String> {
    error!("{}:{}", tip, e); //查询股票分时图失败:Network Error: net::ERR_CONNECTION_RESET
    Err(format!("{}:{}", tip, e))
}