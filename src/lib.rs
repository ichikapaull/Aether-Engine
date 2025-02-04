pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod config;
pub mod utils;

use std::sync::Arc;
use tokio::sync::RwLock;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub simulation_engine: Arc<application::simulation::SimulationEngine>,
    pub config: Arc<config::AppConfig>,
}

impl AppState {
    pub async fn new(config: config::AppConfig) -> Result<Self> {
        let simulation_engine = Arc::new(
            application::simulation::SimulationEngine::new(config.simulation.tick_rate)
        );

        Ok(Self {
            simulation_engine,
            config: Arc::new(config),
        })
    }

    pub async fn start(&self) -> Result<()> {
        // Simülasyon motorunu başlat
        let mut engine = self.simulation_engine.clone();
        tokio::spawn(async move {
            engine.start().await;
        });

        Ok(())
    }
} 