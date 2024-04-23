use std::backtrace::Backtrace;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error:`{0}`")]
    AnyHow(#[from] anyhow::Error),
    #[error("io::Error:`{0}`")]
    IoError(#[from] io::Error),
    #[error("serde_json::Error:`{0}`")]
    SerdeError(#[from] serde_json::Error),
    #[error("reqwest::Error:`{0}`")]
    // HttpError(#[from] #[backtrace] reqwest::Error),
    HttpError(#[from] reqwest::Error),
    //数据库错误
    #[error("sea_orm::DbErr:`{0}`")]
    SqlxError(#[from] sea_orm::DbErr),
    // #[error("http::ParseError:`{0}`")]
    // ParseError(#[from] ParseError),
    // #[error("sea_orm::DbErr:Error:`{0}`")]
    // DbErr(#[from] sea_orm::DbErr),
}
pub type AppResult<T> = Result<T, AppError>;
