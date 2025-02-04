use serde::{Deserialize, Serialize};
use std::fs;
use crate::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub simulation: SimulationConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationConfig {
    pub tick_rate: u32,
    pub initial_time_scale: f64,
    pub max_entities: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub mongodb_uri: String,
    pub redis_uri: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // Önce config.json dosyasını dene
        if let Ok(content) = fs::read_to_string("config.json") {
            return Ok(serde_json::from_str(&content)?);
        }

        // Varsayılan konfigürasyonu döndür
        Ok(Self::default())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            simulation: SimulationConfig {
                tick_rate: 60,
                initial_time_scale: 1.0,
                max_entities: 1_000_000,
            },
            database: DatabaseConfig {
                mongodb_uri: "mongodb://localhost:27017".to_string(),
                redis_uri: "redis://localhost:6379".to_string(),
            },
        }
    }
} 