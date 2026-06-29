use redis::{Client, Connection};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RedisManager {
    pub client: Client,
}

impl RedisManager {
    pub fn new(address: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(address)?;
        Ok(Self { client })
    }

    pub async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, redis::RedisError> {
        self.client.get_multiplexed_async_connection().await
    }
}
