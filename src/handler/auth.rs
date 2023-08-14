use crate::error::HelloError;
use axum::extract::FromRequestParts;
use axum::headers::Cookie;
use axum::http::request::Parts;
use axum::{RequestPartsExt, TypedHeader};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct User {
    pub username: String,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for User {
    type Rejection = HelloError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let cookie = parts
            .extract::<TypedHeader<Cookie>>()
            .await
            .map_err(|_| HelloError::Forbidden)?;
        let token = cookie.get("token").ok_or(HelloError::Forbidden)?;
        let user = serde_json::from_str(token).map_err(|_| HelloError::Forbidden)?;
        Ok(user)
    }
}
