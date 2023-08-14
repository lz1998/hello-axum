use crate::error::HelloResult;
use crate::redis::RedisJsonData;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserInfo {
    pub login: String,
    pub followers: i32,
    pub following: i32,
    pub public_repos: i32,
}

pub async fn get_user_info(username: String) -> HelloResult<UserInfo> {
    let key = format!("user:{}", username);

    // 读缓存，如果存在直接返回
    // 模式匹配语法，和之前的 handler 参数中的 Json(req): Json<T> 一样
    if let Some(RedisJsonData(info)) = crate::redis::CLIENT
        .get_async_connection()
        .await?
        .get(&key) // .get::<&str, Option<RedisJsonData<UserInfo>>>(&key)
        .await?
    {
        return Ok(info);
    }

    // 请求 api
    let client = reqwest::Client::builder()
        .user_agent("hello-axum")
        .build()?;
    let url = format!("https://api.github.com/users/{}", username);
    let info = client.get(url).send().await?.json().await?;

    // 写缓存
    crate::redis::CLIENT
        .get_async_connection()
        .await?
        .set(&key, RedisJsonData(&info))
        .await?;
    Ok(info)
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RepoInfo {
    pub name: String,
    pub stargazers_count: i32,
    pub open_issues_count: i32,
    pub forks: i32,
}

pub async fn get_repo_info(repo: String) -> HelloResult<RepoInfo> {
    let key = format!("repo:{}", repo);
    if let Some(RedisJsonData(info)) = crate::redis::CLIENT
        .get_async_connection()
        .await?
        .get(&key)
        .await?
    {
        return Ok(info);
    }

    let client = reqwest::Client::builder()
        .user_agent("hello-axum")
        .build()?;
    let url = format!("https://api.github.com/repos/{}", repo);
    let info = client.get(url).send().await?.json().await?;

    crate::redis::CLIENT
        .get_async_connection()
        .await?
        .set(&key, RedisJsonData(&info))
        .await?;
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_info() {
        let result = get_user_info("lz1998".into()).await;
        let _ = dbg!(result);
    }

    #[tokio::test]
    async fn test_get_repo_info() {
        let result = get_repo_info("lz1998/hello-axum".into()).await;
        let _ = dbg!(result);
    }

    #[tokio::test]
    async fn test_get_user_info_json() {
        let client = reqwest::Client::builder()
            .user_agent("hello-axum")
            .build()
            .unwrap();
        let response: serde_json::Value = client
            .get("https://api.github.com/repos/lz1998/hello-axum")
            .send()
            .await
            .expect("failed to send response")
            .json()
            .await
            .expect("failed parse json");
        let owner_avatar_url = &response["owner"]["avatar_url"];
        let _ = dbg!(owner_avatar_url);
    }
}
