use crate::database::user;
use crate::database::user::{get_user, insert_user};
use crate::error::HelloResult;
use axum::Json;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterResponse {
    pub id: i32,
}

pub async fn register(Json(req): Json<RegisterRequest>) -> HelloResult<Json<RegisterResponse>> {
    let user = insert_user(user::ActiveModel {
        username: ActiveValue::Set(req.username),
        password: ActiveValue::Set(req.password),
        ..Default::default()
    })
    .await?;
    Ok(Json(RegisterResponse { id: user.id }))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginResponse {
    pub success: bool,
    pub token: String,
}

pub async fn login(Json(req): Json<LoginRequest>) -> HelloResult<Json<LoginResponse>> {
    let user = get_user(&req.username).await?;
    let user = match user {
        Some(user) => user,
        None => {
            return Ok(Json(LoginResponse {
                success: false,
                ..Default::default()
            }))
        }
    };
    if req.password != user.password {
        return Ok(Json(LoginResponse {
            success: false,
            ..Default::default()
        }));
    }
    // 这里偷个懒，token 就是 username + password 的 json
    Ok(Json(LoginResponse {
        success: true,
        token: serde_json::to_string(&req).unwrap(),
    }))
}
