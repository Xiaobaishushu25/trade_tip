use crate::cache::intraday_chart_cache::IntradayChartCache;
use crate::config::config::{Config, DataConfig};
use crate::dtos::graphic_dto::GraphicDTO;
use crate::dtos::stock_dto::{StockInfoG, StockLiveData};
use crate::entities::prelude::{Graphic, Position, StockData, StockGroup, StockInfo, TransactionRecord};
use crate::service::command::handle::{handle_and_save_record, handle_can_t, handle_delete_stock, handle_new_stock, handle_stock_livedata};
use crate::service::curd::graphic_curd::GraphicCurd;
use crate::service::curd::group_stock_relation_curd::GroupStockRelationCurd;
use crate::service::curd::stock_data_curd::StockDataCurd;
use crate::service::curd::stock_group_curd::StockGroupCurd;
use crate::service::curd::stock_info_curd::StockInfoCurd;
use crate::service::curd::transaction_record_curd::TransactionRecordCurd;
use crate::service::curd::update_all_day_k;
use crate::service::http::{init_http, REQUEST};
use crate::{get_close_prices, MyState, IS_MARKET_OPEN, UPDATEING};
use log::{error, info};
use std::collections::HashMap;
use std::process::exit;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Emitter, State};
use tokio::time::sleep;
use crate::service::curd::position_curd::PositionCurd;

#[tauri::command]
pub async fn update_live_state<'r>(
    state: State<'r, MyState>,
    app_handle: tauri::AppHandle,
    group_name: String,
    live_state: bool,
) -> Result<(), String> {
    // info!("更新状态:{}",live_state);
    UPDATEING.store(live_state, Ordering::Relaxed);
    if live_state {
        let _ = query_live_stocks_data_by_group_name(state, group_name, app_handle).await;
    }
    Ok(())
}
#[tauri::command]
pub async fn update_all_stock_day_k() -> Result<String, String> {
    // state.update_live_state(live_state);
    match update_all_day_k().await {
        Ok(_) => Ok("更新成功".to_string()),
        Err(e) => {
            handle_error("更新日线数据失败", e.to_string())
            // Err(format!("更新日线数据失败:{}",e.to_string()))
        }
    }
}
#[tauri::command]
pub async fn get_response(url: String) -> Result<String, String> {
    match REQUEST.get().unwrap().get(&url).await {
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
    info!("查询所有分组");
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
    if name == "持有" {
        return match StockInfoCurd::query_all_hold_info().await {
            Ok(more_infos) => {
                Ok(more_infos)
            }
            Err(e) => {
                handle_error("查询持有分组失败", e.to_string())
            }
        };
    }
    match GroupStockRelationCurd::query_stocks_by_group_name(name.clone()).await {
        Ok(more_infos) => {
            // // info!("根据分组名查询成功:{:?}",more_infos);
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
    is_new: bool,
    code: String,
    name: String,
    group_names: Vec<String>,
) -> Result<(), String> {
    info!(
        "is_new:{},code:{},name:{},group_names:{:?}",
        is_new, code, name, group_names
    );
    if is_new {
        match handle_new_stock(&code, &name).await {
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
                info!("创建成功:{:?}", code);
            }
            Err(e) => {
                return handle_error(&format!("创建{}失败", code), e.to_string());
                // error!("创建失败:{}",e);
                // return Err(e.to_string());
            }
        }
    } else if group_names.is_empty() {
        //全部没选上
        return match handle_delete_stock(&code).await {
            Ok(_) => {
                // info!("删除成功:{:?}",code);
                Ok(())
            }
            Err(e) => {
                handle_error(&format!("删除{}失败", code), e.to_string())
                // error!("删除失败:{}",e);
                // Err(e.to_string())
            }
        };
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
#[tauri::command]
pub async fn remove_stock_from_group(code: String, group_name: String) -> Result<(), String> {
    match GroupStockRelationCurd::delete_by_code_and_group_name(code.clone(), group_name.clone())
        .await
    {
        Ok(_) => {
            // info!("删除成功。");
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
    match REQUEST.get().unwrap().get_live_stock_data(&codes).await {
        Ok(mut stock_data_list) => {
            match handle_stock_livedata(&codes, &mut stock_data_list, &history_close_price).await {
                Ok(_) => {
                    Ok(stock_data_list.get(&code).unwrap().clone())
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
    group_name: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    info!("查询实时数据:{}", group_name);
    // if !IS_MARKET_OPEN.load(Ordering::Relaxed) {
    //     info!("市场未开市,不进行查询操作！");
    //     return Ok(());
    // }
    //查询分组内的所有股票代码
    let result = if group_name == "持有" {
        StockInfoCurd::query_all_hold_only_code().await
    } else {
        GroupStockRelationCurd::query_only_code_by_group_name(group_name).await
    };
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
                        match REQUEST.get().unwrap().get_live_stock_data(&codes).await {
                            Ok(mut stock_data_list) => {
                                match handle_stock_livedata(
                                    &codes,
                                    &mut stock_data_list,
                                    &history_close_price,
                                )
                                .await
                                {
                                    Ok(_) => {
                                        // info!("查询成功:{:?}",stock_data_list);
                                        app_handle
                                            .emit("live_stock_data", stock_data_list)
                                            .unwrap();
                                    }
                                    Err(e) => {
                                        error!("处理股票实时信息失败:{}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("查询股票实时信息失败失败:{}", e);
                            }
                        }
                    }
                    sleep(Duration::from_secs(10)).await;
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
// #[tauri::command]
// pub async fn save_config<'r>(
//     config_state: State<'r, ConfigState>,
//     config: Config,
// ) -> Result<(), String> {
//     match config_state.update_config(config).await {
//         Ok(_) => Ok(()),
//         Err(e) => handle_error("更新配置文件失败", e.to_string()),
//     }
// }
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
/// 如果更新成功，返回Ok(())，同时emit一个position_change事件，payload为(String,position_num)
#[tauri::command]
pub async fn update_position(app_handle: tauri::AppHandle,date: String, position_num: f64) -> Result<(), String> {
    match PositionCurd::update_position_by_id(date.clone(), position_num).await{
        Ok(_) => {
            app_handle.emit("position_change", (date, position_num)).unwrap();
            Ok(())
        },
        Err(e) => handle_error("更新持仓失败", e.to_string()),
    }
}
///插入持仓数据
/// date:日期（“YYYY-MM-DD”）
/// position_num:持仓百分比
/// 如果插入成功，返回Ok(())，同时emit一个position_change事件，payload为(bool,Position)，布尔表示插入(true)/更新(false)，Position表示持仓数据
// #[tauri::command]
// pub async fn insert_position(app_handle: tauri::AppHandle, date: String, position_num: f64) -> Result<(), String> {
//     match handle_insert_position(date, position_num).await{
//         Ok(data) => {
//             app_handle.emit("position_change", data).unwrap();
//             Ok(())
//         },
//         Err(e) => handle_error("插入持仓失败", e.to_string()),
//     }
// }
///获取当前是否是交易时间
#[tauri::command]
pub async fn get_is_market_open() -> bool {
    IS_MARKET_OPEN.load(Ordering::Relaxed)
}

#[tauri::command]
pub async fn exit_app() {
    info!("退出程序");
    exit(0)
}
///处理错误，先记录到日志中，再返回错误
/// 使用泛型 T，这样它可以返回任何类型的 Result
fn handle_error<T>(tip: &str, e: String) -> Result<T, String> {
    error!("{}:{}", tip, e); //查询股票分时图失败:Network Error: net::ERR_CONNECTION_RESET
    Err(format!("{}:{}", tip, e))
}
#[tokio::test]
async fn test_market_is_open() {
    init_http().await;
    let response1 = REQUEST.get().unwrap().get("https://qt.gtimg.cn/q=sz159992").await.unwrap();
    let string1 = response1.text().await.unwrap();
    println!("{:?}", string1);
    sleep(Duration::from_secs(1)).await;
    let response2 = REQUEST.get().unwrap().get("https://qt.gtimg.cn/q=sz159992").await.unwrap();
    let string2 = response2.text().await.unwrap();
    println!("{:?}", string2);
    if string1==string2 {
        println!("market is closed");
    }else {
        println!("market is open");
    }
}