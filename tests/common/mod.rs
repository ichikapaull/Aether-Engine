use std::sync::Arc;
use universe_simulator::{AppState, config::AppConfig};

pub async fn setup_test_app() -> AppState {
    let config = AppConfig::default();
    AppState::new(config).await.expect("Test app kurulumu başarısız")
}

pub async fn setup_test_database() -> mongodb::Database {
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Test veritabanı bağlantısı başarısız");
    
    client.database("universe_simulator_test")
}

pub async fn cleanup_test_database(db: &mongodb::Database) {
    db.drop(None).await.expect("Test veritabanı temizleme başarısız");
} 