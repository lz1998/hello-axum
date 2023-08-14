use axum::extract::Query;
use std::collections::HashMap;

pub async fn hello(query: Query<HashMap<String, String>>) -> &'static str {
    tracing::info!("hello, query: {query:?}"); // 日志输出参数，比如 /hello?a=1&b=2
    "hello"
}