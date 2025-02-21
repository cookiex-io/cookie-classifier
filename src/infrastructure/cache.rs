use redis::aio::MultiplexedConnection;
use redis::RedisResult;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref REDIS_HOST_NAME:String = std::env::var("REDIS_HOST_NAME").expect("Must set REDIS_HOST_NAME");
    pub static ref REDIS_PRIMARY_PASSWORD:String = std::env::var("REDIS_PRIMARY_PASSWORD").expect("Must set REDIS_PRIMARY_PASSWORD");
}

#[derive(Debug, Clone)]
pub(crate) struct Redis {
    connection: MultiplexedConnection,
    key: String,
}

impl Redis {
    pub(crate) fn new(connection: MultiplexedConnection, key: String) -> Self {
        Self { connection, key }
    }

    pub(crate) async fn query(
        &mut self,
        previous_window: u64,
        current_window: u64,
    ) -> RedisResult<(Option<u32>, Option<u32>)> {
        redis::pipe()
            .cmd("HGET")
            .arg(&self.key)
            .arg(previous_window)
            .cmd("HGET")
            .arg(&self.key)
            .arg(current_window)
            .query_async(&mut self.connection)
            .await
    }

    pub(crate) async fn increment(
        &mut self,
        current_window: u64,
        window_size: u64,
    ) -> RedisResult<()> {
        redis::pipe()
            .cmd("HINCRBY")
            .arg(&self.key)
            .arg(current_window)
            .arg(1)
            .cmd("HEXPIREAT")
            .arg(&self.key)
            .arg(current_window + 2 * window_size)
            .arg("NX")
            .arg(current_window)
            .exec_async(&mut self.connection)
            .await
    }
}