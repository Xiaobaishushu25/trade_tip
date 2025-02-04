// use std::io::{BufRead, BufReader};
// use std::process::{Command, Stdio};
use ashares::init_ashares;
use crate::service::http::http_client::HttpRequest;
use log::{error, info};
use tokio::sync::OnceCell;
use tokio::process::Command;
use tokio::io::{BufReader};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use encoding_rs::GBK;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc::Receiver;
use crate::config::config::DataConfig;
use crate::app_errors::AppResult;

pub mod http_client;
pub mod futures;

pub static REQUEST: OnceCell<HttpRequest> = OnceCell::const_new();
pub async fn init_http() {
    REQUEST
        .get_or_init(|| async {
            info!("初始化http.");
            HttpRequest::new()
        })
        .await;
}

pub async fn start_data_server(data_config: &DataConfig, mut receiver: Receiver<()>) -> AppResult<()> {
    // init_ashares().await;
    if data_config.use_ak_share{
        let command = &data_config.data_server;
        info!("use command:{} to start data server...", command);
        //todo 无法关闭Python进程
        #[cfg(target_os = "windows")]
        let mut child = Command::new("cmd")
            .args(&["/K", command])  // Keep the window open
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .creation_flags(0x08000000)//使得黑框不可见
            .spawn()?;  // Start the process

        #[cfg(not(target_os = "windows"))]
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;  // Start the process

        info!("start data server...");
        let child_stdout = child.stdout.take().unwrap();
        tokio::spawn(async move {
            let mut stdout_reader = BufReader::new(child_stdout);
            let mut buffer = Vec::new();
            while stdout_reader.read_until(b'\n', &mut buffer).await.unwrap() > 0 {
                let (decoded, _, _) = GBK.decode(&buffer); // 将字节数据解码为字符串
                info!("akshare: {}", decoded);
                buffer.clear(); // 清空缓冲区，准备读取下一行
            }
        });
        let child_stderr = child.stderr.take().unwrap();
        tokio::spawn(async move {
            let mut stderr_reader = BufReader::new(child_stderr);
            let mut buffer = Vec::new();
            while stderr_reader.read_until(b'\n', &mut buffer).await.unwrap() > 0 {
                let (decoded, _, _) = GBK.decode(&buffer); // 将字节数据解码为字符串
                error!("akshare: {}", decoded);
                buffer.clear(); // 清空缓冲区，准备读取下一行
            }
        });
        tokio::select! {
            _ = receiver.recv() => {
                error!("Received shutdown signal, stopping data server...");
                child.kill().await.expect("kill failed");
                error!("Data server stopped.");
            },
            _ = child.wait() => {
                info!("Data server stopped.");
            }
        }
    }
    Ok(())
}

// pub async fn start_data_server(command: &str) ->AppResult<()>{
//     init_ashares().await;
//     info!("use command:{} to start data server...", command);
//     #[cfg(target_os = "windows")]
//     let mut child = Command::new("cmd")
//         .args(&[
//             "/K", // 保持窗口打开
//             command,
//         ])
//         .stdout(Stdio::piped()) // 重定向标准输出
//         .stderr(Stdio::piped()) // 重定向标准错误
//         .spawn()?; // 启动进程
//
//     #[cfg(not(target_os = "windows"))]
//     let mut child = Command::new("sh")
//         .arg("-c")
//         .arg(command)
//         .stdout(Stdio::piped()) // 重定向标准输出
//         .stderr(Stdio::piped()) // 重定向标准错误
//         .spawn()?; // 启动进程
//
//     info!("start data server...");
//     let child_stdout = child.stdout.take().unwrap();
//     let handle = tokio::spawn(async move {
//         let stdout_reader = BufReader::new(child_stdout);
//         for line in stdout_reader.lines() {
//             if let Ok(line) = line {
//                 info!("{}", line); // 记录标准输出到日志
//             }
//         }
//     });
//     handle.await.unwrap();
//     info!("你能执行到这吗");
//     let child_stderr = child.stderr.take().unwrap();
//     let handle = tokio::spawn(async move {
//         let stderr_reader = BufReader::new(child_stderr);
//         for line in stderr_reader.lines() {
//             if let Ok(line) = line {
//                 error!("{}", line); // 记录标准错误到日志
//             }
//         }
//     });
//     handle.await.unwrap();
//     // let stderr_reader = BufReader::new(child_stderr);
//     // for line in stderr_reader.lines() {
//     //     if let Ok(line) = line {
//     //         error!("{}", line); // 记录标准错误到日志
//     //     }
//     // }
//     // 等待命令执行完成
//     let status = child.wait()?;
//     info!("Command status: {:?}", status);
//     if status.success() {
//         info!("Command executed successfully");
//     } else {
//         error!("Command failed with exit code: {:?}", status.code());
//     }
//     Ok(())
// }