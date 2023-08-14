use crate::handler::auth::User;

pub async fn must_login(
    user: User,
    // 除了 User，可以添加多个其他 FromRequest/FromRequestParts 参数
    // 最多有一个实现了 FromRequest 的参数（消耗 body 所有权），比如 Json<T>
    // 可以有多个实现了 FromRequestParts 的参数（不消耗 body 所有权），比如 Query<T>、HeaderMap
) -> String {
    format!("must_login, username: {}", user.username)
}

pub async fn optional_login(user: Option<User>) -> String {
    match user {
        None => String::from("optional_login, not login"),
        Some(user) => format!("optional_login, username: {}", user.username),
    }
}

pub async fn no_login() -> String {
    String::from("no login")
}
