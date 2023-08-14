use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

// 为了使用方便，可以用 HelloResult<T> 代替 Result<T, HelloError>
pub type HelloResult<T> = Result<T, HelloError>;

#[derive(Error, Debug)]
pub enum HelloError {
    #[error("not implemented")]
    NotImplemented,
    #[error("not found")]
    NotFound,
    // #[from] 表示自动 impl From<std::io::Error> for HelloError
    // 如果错误类型是 srd::io::Error，通过 ? 返回错误时可以自动转换成 HelloError
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
}

impl IntoResponse for HelloError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = self.to_string();
        (code, body).into_response()
    }
}
