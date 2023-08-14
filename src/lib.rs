pub mod database;
pub mod error;

use axum::extract::Query;
use error::{HelloError, HelloResult};
use std::collections::HashMap;

pub async fn hello(query: Query<HashMap<String, String>>) -> &'static str {
    tracing::info!("hello, query: {query:?}"); // 日志输出参数，比如 /hello?a=1&b=2
    "hello"
}

pub async fn not_found_handler() -> HelloResult<()> {
    Err(HelloError::NotFound)
}

pub async fn not_implemented_handler() -> HelloResult<()> {
    Err(HelloError::NotImplemented)
}

pub async fn read_file_handler() -> HelloResult<String> {
    // 写法 1
    // await 后面的问号会自动返回错误，类型是 std::io::Error
    // 因为定义错误时 std::io::Error 加了 #[from]，问号返回会自动转换错误类型
    // 如果没有 impl From<std::io::Error> for HelloError，只能用 写法2
    let content = tokio::fs::read_to_string("hello.txt").await?; // 这个文件不存在，会报错
    Ok(content)

    // 写法 2
    // 用 map_err 转换错误类型
    // tokio::fs::read_to_string("hello.txt")
    //     .await
    //     .map_err(HelloError::IO) // 和这样写效果相同 .map_err(|err| HelloError::IO(err))
}
