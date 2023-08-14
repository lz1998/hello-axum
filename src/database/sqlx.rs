use sqlx::MySqlPool;
use tokio::sync::OnceCell;

static POOL: OnceCell<MySqlPool> = OnceCell::const_new();

pub async fn init_pool() -> MySqlPool {
    MySqlPool::connect("mysql://root:123456@localhost:3306/test")
        .await
        .expect("failed to connect database")
}

pub async fn get_pool() -> &'static MySqlPool {
    POOL.get_or_init(init_pool).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Executor;

    #[tokio::test]
    async fn test_sqlx() {
        let pool = get_pool().await;
        let result = pool.fetch_one("select 123").await;
        let _ = dbg!(result);
    }
}
