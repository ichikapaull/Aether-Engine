mod common;

use universe_simulator::application::simulation::{SimulationEngine, PhysicsEngine};
use tokio::time::Duration;

#[tokio::test]
async fn test_simulation_engine() {
    let engine = SimulationEngine::new(60);
    assert_eq!(engine.is_running(), false);
    
    // Simülasyonu başlat
    let mut engine = engine;
    tokio::spawn(async move {
        engine.start().await;
    });
    
    // Simülasyonun başladığını kontrol et
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(engine.is_running(), true);
}

#[tokio::test]
async fn test_physics_calculations() {
    let physics = PhysicsEngine::new();
    
    // Test cismi ekle
    let body_id = Uuid::new_v4();
    physics.register_celestial_body(
        body_id,
        1.0,  // kütle
        0.0,  // x
        0.0,  // y
        0.0,  // z
    );
    
    // Fizik hesaplamalarını test et
    physics.update(1.0);  // 1 saniyelik güncelleme
    
    // Sonuçları kontrol et
    let body = physics.get_body(body_id).unwrap();
    assert_eq!(body.position.x, 0.0);  // Başlangıç konumunda olmalı
} 