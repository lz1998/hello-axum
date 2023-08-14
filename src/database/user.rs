use super::sea::get_conn;
use crate::error::HelloResult;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DeleteResult, UpdateResult};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn insert_user(user: ActiveModel) -> HelloResult<Model> {
    let conn = get_conn().await;
    user.insert(conn).await.map_err(Into::into)
}

pub async fn get_user(username: &str) -> HelloResult<Option<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::Username.eq(username))
        .one(conn)
        .await
        .map_err(Into::into)
}

pub async fn delete_user(username: &str) -> HelloResult<DeleteResult> {
    let conn = get_conn().await;
    Entity::delete_many()
        .filter(Column::Username.eq(username))
        .exec(conn)
        .await
        .map_err(Into::into)
}

pub async fn update_user(username: &str, password: &str) -> HelloResult<UpdateResult> {
    let conn = get_conn().await;
    Entity::update_many()
        .filter(Column::Username.eq(username))
        .set(ActiveModel {
            password: ActiveValue::Set(password.into()),
            // 如果字段过多，可以使用 ..Default::default 表示不需要填写的字段
            ..Default::default()
        })
        .exec(conn)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::ActiveValue;

    #[tokio::test]
    async fn test_insert() {
        let result = insert_user(ActiveModel {
            id: ActiveValue::NotSet, // 有 auto increment，不需要设置
            username: ActiveValue::Set("123".into()),
            password: ActiveValue::Set("123".into()),
        })
        .await;
        println!("{result:?}");
    }

    #[tokio::test]
    async fn test_get() {
        let result = get_user("123").await;
        println!("{result:?}");
    }

    #[tokio::test]
    async fn test_delete() {
        let result = delete_user("123").await;
        println!("{result:?}");
    }

    #[tokio::test]
    async fn test_update() {
        let result = update_user("123", "1234").await;
        println!("{result:?}");
    }
}
