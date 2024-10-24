pub mod authorisation;
pub mod secret;
pub mod token;

use redis::aio::MultiplexedConnection;
use redis::RedisResult;

pub async fn get_redis_con(client: redis::Client) -> RedisResult<MultiplexedConnection> {
    client.get_multiplexed_async_connection().await
}
