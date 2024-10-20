pub mod authorisation;
pub mod token;

use redis::{AsyncCommands, FromRedisValue, RedisResult};
use redis::aio::{ MultiplexedConnection};

pub async fn get_redis_con(client: redis::Client) -> RedisResult<MultiplexedConnection> {
    client
        .get_multiplexed_async_connection()
        .await
}
