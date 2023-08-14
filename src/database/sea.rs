use sea_orm::entity::prelude::*;
use sea_orm::SqlxMySqlConnector;
use tokio::sync::OnceCell;

use super::sqlx::init_pool;

pub async fn init_conn() -> DatabaseConnection {
    let pool = init_pool().await;
    SqlxMySqlConnector::from_sqlx_mysql_pool(pool)
}

static CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_conn() -> &'static DatabaseConnection {
    CONN.get_or_init(init_conn).await
}
