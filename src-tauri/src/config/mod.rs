use serde::{Deserialize, Serialize};
use std::io::Write;

pub mod config;

// pub struct ConfigState {
//     pub config:Arc<RwLock<Config>>, // 使用Arc共享RwLock，保证多线程安全，提供内部可变性
// }
// impl ConfigState {
//     pub fn new() -> Self {
//         let config = Config::load();
//         ConfigState {
//             config: Arc::new(RwLock::new(config)),
//         }
//     }
// }
