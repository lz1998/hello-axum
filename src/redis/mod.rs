use lazy_static::lazy_static;
use redis::{FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs, Value};
use serde::de::DeserializeOwned;
use serde::Serialize;

lazy_static! {
    pub static ref CLIENT: redis::Client =
        redis::Client::open("redis://:123456@localhost/0").unwrap();
}

// 因为 impl Trait for Struct 的时候，Trait/Struct 至少要有一个定义在当前 mod
// 所以定义 RedisJsonData，对其他 Struct 进行包装
pub struct RedisJsonData<T>(pub T);

impl<T: DeserializeOwned> FromRedisValue for RedisJsonData<T> {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match *v {
            Value::Data(ref bytes) => serde_json::from_slice(bytes)
                .map(RedisJsonData)
                .map_err(|_| RedisError::from((redis::ErrorKind::ResponseError, ""))),
            _ => Err(RedisError::from((
                redis::ErrorKind::TypeError,
                "Response type not demo_data compatible",
            ))),
        }
    }
}

impl<T: Serialize> ToRedisArgs for RedisJsonData<T> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(serde_json::to_string(&self.0).unwrap().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::AsyncCommands;

    #[tokio::test]
    async fn test_client() {
        let mut conn = CLIENT.get_async_connection().await.unwrap();
        let result: Option<String> = conn.get("a").await.unwrap();
        dbg!(result);
        let _: () = conn.set("a", "1").await.unwrap();
        let result: Option<String> = conn.get("a").await.unwrap();
        dbg!(result);
    }
}
