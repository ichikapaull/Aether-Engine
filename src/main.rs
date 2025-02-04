use std::sync::Arc;
use universe_simulator::{AppState, Result};
use universe_simulator::config::AppConfig;
use universe_simulator::api::setup_routes;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // Loglama sistemini başlat
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting Universe Simulator...");

    // Konfigürasyonu yükle
    let config = AppConfig::load()?;
    info!("Configuration loaded successfully");

    // Uygulama durumunu oluştur
    let app_state = AppState::new(config).await?;
    info!("Application state initialized");

    // Simülasyonu başlat
    app_state.start().await?;
    info!("Simulation engine started");

    // API rotalarını oluştur
    let routes = setup_routes(app_state.simulation_engine.clone());
    info!("API routes configured");

    // Sunucuyu başlat
    let addr = ([127, 0, 0, 1], 3000).into();
    info!("Starting server on http://{}", addr);
    
    warp::serve(routes)
        .run(addr)
        .await;

    Ok(())
} 