use redis::AsyncCommands;

pub async fn multiplexed_async_connection(
    redis_url: String,
) -> redis::RedisResult<redis::aio::MultiplexedConnection> {
    let client = redis::Client::open(redis_url)?;
    client.get_multiplexed_async_connection().await
}
pub async fn get_cached_data(key: &str, redis_url: &str) -> redis::RedisResult<Option<String>> {
    let mut conn = multiplexed_async_connection(redis_url.to_string()).await?;
    let cached_data: Option<String> = conn.get(key).await?;
    Ok(cached_data)
}

pub async fn delete_cached_data(key: &str, redis_url: &str) -> redis::RedisResult<Option<String>> {
    let mut conn = multiplexed_async_connection(redis_url.to_string()).await?;
    let cached_data: Option<String> = conn.del(key).await?;
    Ok(cached_data)
}

pub async fn set_cached_data(key: &str, data: &str, redis_url: &str) -> redis::RedisResult<()> {
    let mut conn = multiplexed_async_connection(redis_url.to_string()).await?;
    let _: () = conn.set_ex(key, data, 60).await?;
    Ok(())
}
pub fn get_connection(connection_addr: &str) -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open(connection_addr)?;
    client.get_connection()
}
