use crate::app_errors::AppResult;
use crate::CURRENT_DIR;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    display_config: DisplayConfig,
    data_config: DataConfig,
}
impl Config {
    /**
     * 加载配置文件
     */
    pub async fn load() -> Self {
        info!("load config");
        // let current_dir = &env::current_dir().unwrap();
        // let current_dir = current_dir.to_string_lossy();
        let path = format!("{}/data/config", CURRENT_DIR.clone());
        match check_config_file(&path, &CURRENT_DIR.clone()) {
            Ok(config) => {
                info!("load config success{:?}", config);
                config
            }
            Err(e) => {
                error!("创建或解析配置文件{}失败:{}", path, e.to_string());
                panic!("创建或解析配置文件{}失败:{}", path, e)
            }
        }
    }
    pub async fn update(&mut self, config: Config) -> AppResult<()> {
        match save_config(&config).await {
            Ok(_) => {
                info!("save config success");
                *self = config;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
// impl Default for Config {
//     fn default() -> Self {
//         Config {
//             display_config: Default::default(),
//             data_config: Default::default(),
//         }
//     }
// }

// pub(crate) async fn load_config() -> Config {
//     let current_dir = &env::current_dir().unwrap();
//     let current_dir = current_dir.to_string_lossy();
//     let path = format!("{}/data/config", current_dir);
//     match check_config_file(&path,&current_dir){
//         Ok(config)=>{
//             config
//         },
//         Err(e)=>{
//             error!("创建或解析配置文件{}失败:{}",path,e.to_string());
//             panic!("创建或解析配置文件{}失败:{}",path,e.to_string())
//         }
//     }
// }
/**
 * 保存配置文件
 */
async fn save_config(config: &Config) -> AppResult<()> {
    let path = format!("{}/data/config", CURRENT_DIR.clone());
    let mut config_file = OpenOptions::new()
        .write(true) // 以写入模式打开文件
        .truncate(true) // 清空文件内容
        .open(path)?;
    config_file.write_all(serde_json::to_string(config)?.as_bytes())?;
    Ok(())
}
fn check_config_file(path: &str, current_dir: &str) -> AppResult<Config> {
    let mut config_file: File = if PathBuf::from(path).exists() {
        info!("配置存在");
        if let Ok(config) = serde_json::from_str::<Config>(&fs::read_to_string(path)?) {
            return Ok(config); //如果正确解析配置文件，直接返回
        } else {
            error!("配置文件格式错误，将重新创建配置文件。");
            //清除配置文件内容
            // 打开文件并清空内容
            OpenOptions::new()
                .write(true) // 以写入模式打开文件
                .truncate(true) // 清空文件内容
                .open(path)?
        }
    } else {
        info!("配置不存在,创建配置。");
        fs::create_dir_all(format!("{}/data", current_dir))?;
        File::create(path)?
    };
    //如果上面正确读取配置文件就已经返回了，到这里说明配置文件没有内容，需要初始化默认配置
    // let config = Config::init_default();
    let config = Config::default();
    let config_string = serde_json::to_string(&config)?;
    config_file.write_all(config_string.as_bytes())?;
    Ok(config)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    a_extend: bool,
    bs_size: i32,      //BS点的大小
    k_show_begin: i32, //K线显示百分比
}
impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            a_extend: false,
            bs_size: 14,
            k_show_begin: 85,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataConfig {
    update_freq: i32,
    box_num: i32,
}
impl Default for DataConfig {
    fn default() -> Self {
        DataConfig {
            update_freq: 30,
            box_num: 5,
        }
    }
}
