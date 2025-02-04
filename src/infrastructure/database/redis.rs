use redis::{Client, Connection, RedisResult, AsyncCommands};
use uuid::Uuid;
use std::time::Duration;

pub struct RedisConnection {
    client: Client,
}

impl RedisConnection {
    pub fn new(connection_string: &str) -> RedisResult<Self> {
        let client = Client::open(connection_string)?;
        Ok(Self { client })
    }

    pub async fn cache_galaxy_data(&self, galaxy_id: Uuid, data: &[u8], ttl: Duration) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("galaxy:{}", galaxy_id);
        
        conn.set_ex(&key, data, ttl.as_secs() as usize).await?;
        Ok(())
    }

    pub async fn get_cached_galaxy_data(&self, galaxy_id: Uuid) -> RedisResult<Option<Vec<u8>>> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("galaxy:{}", galaxy_id);
        
        conn.get(&key).await
    }

    pub async fn cache_simulation_state(&self, state_key: &str, state_data: &[u8]) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        conn.set(state_key, state_data).await?;
        Ok(())
    }

    pub async fn get_simulation_state(&self, state_key: &str) -> RedisResult<Option<Vec<u8>>> {
        let mut conn = self.client.get_async_connection().await?;
        conn.get(state_key).await
    }

    pub async fn publish_update(&self, channel: &str, message: &str) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        conn.publish(channel, message).await?;
        Ok(())
    }
} 