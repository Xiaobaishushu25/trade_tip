use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Config {
    pub display_config: DisplayConfig,
    pub data_config: DataConfig,
}
impl Config {
    pub fn init_default() -> Self {
        Config {
            display_config: DisplayConfig::init(),
            data_config:DataConfig::init(),
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct DisplayConfig {
    a_extend: bool,
}
impl DisplayConfig {
    pub fn init() -> Self {
        DisplayConfig {
            a_extend: false,
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct DataConfig {
    update_freq:i32,
    box_num:i32,
}
impl DataConfig {
    pub fn init() -> Self {
        DataConfig {
            update_freq: 30,
            box_num: 40,
        }
    }
}