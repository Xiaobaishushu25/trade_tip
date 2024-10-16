use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use log::{error, info};
use crate::app_errors::AppResult;
use crate::config::config::{Config};

pub mod config;
pub async fn load_config() -> Config{
    let current_dir = &env::current_dir().unwrap();
    let current_dir = current_dir.to_string_lossy();
    let path = format!("{}/data/config", current_dir);
    match check_config_file(&path,&current_dir){
        Ok(flag)=>{
            flag
        },
        Err(e)=>{
            error!("创建或解析配置文件{}失败:{}",path,e.to_string());
            panic!("创建或解析配置文件{}失败:{}",path,e.to_string())
        }
    }
}
pub async fn save_config(config:Config)->AppResult<()>{
    
}
fn check_config_file(path:&str,current_dir:&str)->AppResult<Config>{
    let mut config_file:File = if PathBuf::from(path).exists() {
        info!("配置存在");
        if let Ok(config) = serde_json::from_str::<Config>(&fs::read_to_string(path)?){
            return Ok(config) //如果正确解析配置文件，直接返回
        }else { 
            error!("配置文件格式错误，将重新创建配置文件。");
            //清除配置文件内容
            // 打开文件并清空内容
            OpenOptions::new()
                .write(true)  // 以写入模式打开文件
                .truncate(true)  // 清空文件内容
                .open(path)?
        }
    } else {
        info!("配置不存在,创建配置。");
        let _ = fs::create_dir_all(format!("{}/data", current_dir))?;
        let config_file = File::create(path)?;
        config_file
    };
    //如果上面正确读取配置文件就已经返回了，到这里说明配置文件没有内容，需要初始化默认配置
    let config = Config::init_default();
    let config_string = serde_json::to_string(&config)?;
    config_file.write_all(config_string.as_bytes())?;
    Ok(config)
}
