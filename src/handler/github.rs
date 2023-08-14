use crate::api::github::{get_repo_info, get_user_info, RepoInfo, UserInfo};
use crate::error::HelloResult;
use axum::Json;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GithubInfoRequest {
    users: Vec<String>,
    repos: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GithubInfoResponse {
    user_infos: Vec<Option<UserInfo>>,
    repo_infos: Vec<Option<RepoInfo>>,
}

pub async fn github_info(
    Json(req): Json<GithubInfoRequest>,
) -> HelloResult<Json<GithubInfoResponse>> {
    // 同时执行多个相同类型的任务
    // 1. 把 req.users 转换为 stream
    // 2. 利用 .map，把 String 转为 Future
    // 3. buffered 收集 Future 结果
    // 4. collect 变成 Vector
    // 可以直接 .await 得到 Vec<HelloResult<UserInfo>>，这里先不 .await ，下面用 join! 和另一个 future 一起执行
    let user_infos_future = futures::stream::iter(req.users)
        .map(get_user_info) // 等价于 .map(|s| get_user_info(s))
        .buffered(10) // 最多同时10并发
        .collect();

    let repo_infos_future = futures::stream::iter(req.repos)
        .map(get_repo_info) // 等价于 .map(|s| get_repo_info(s))
        .buffered(10) // 最多同时10并发
        .collect();

    // 同时执行两个不同的 future
    let (user_infos, repo_infos): (Vec<HelloResult<UserInfo>>, Vec<HelloResult<RepoInfo>>) =
        tokio::join!(user_infos_future, repo_infos_future);

    // 把 Error 变成 None，也就是 null
    let user_infos = user_infos.into_iter().map(|info| info.ok()).collect();
    let repo_infos = repo_infos.into_iter().map(|info| info.ok()).collect();

    Ok(Json(GithubInfoResponse {
        user_infos,
        repo_infos,
    }))
}
