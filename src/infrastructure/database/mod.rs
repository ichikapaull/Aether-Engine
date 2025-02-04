pub mod mongodb;
pub mod redis;

pub use mongodb::MongoDBConnection;
pub use redis::RedisConnection;

#[derive(Clone)]
pub struct DatabaseConnections {
    pub mongodb: MongoDBConnection,
    pub redis: RedisConnection,
}

impl DatabaseConnections {
    pub async fn new(config: &crate::config::DatabaseConfig) -> crate::Result<Self> {
        let mongodb = MongoDBConnection::new(&config.mongodb_uri, "universe_simulator").await?;
        let redis = RedisConnection::new(&config.redis_uri)?;

        Ok(Self { mongodb, redis })
    }
} 