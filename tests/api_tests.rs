mod common;

use warp::test::request;
use universe_simulator::api::setup_routes;
use serde_json::json;

#[tokio::test]
async fn test_create_galaxy() {
    let app = common::setup_test_app().await;
    let api = setup_routes(app.simulation_engine);
    
    let response = request()
        .method("POST")
        .path("/api/v1/galaxies")
        .json(&json!({
            "name": "Test Galaxy",
            "seed": 12345
        }))
        .reply(&api)
        .await;
    
    assert_eq!(response.status(), 200);
    
    let galaxy = serde_json::from_slice::<serde_json::Value>(response.body())
        .expect("Invalid JSON response");
    
    assert_eq!(galaxy["name"], "Test Galaxy");
    assert!(galaxy["id"].is_string());
}

#[tokio::test]
async fn test_get_galaxy() {
    let app = common::setup_test_app().await;
    let api = setup_routes(app.simulation_engine);
    
    // Önce bir galaksi oluştur
    let create_response = request()
        .method("POST")
        .path("/api/v1/galaxies")
        .json(&json!({
            "name": "Test Galaxy",
            "seed": 12345
        }))
        .reply(&api)
        .await;
    
    let galaxy = serde_json::from_slice::<serde_json::Value>(create_response.body())
        .expect("Invalid JSON response");
    let galaxy_id = galaxy["id"].as_str().unwrap();
    
    // Oluşturulan galaksiyi getir
    let get_response = request()
        .method("GET")
        .path(&format!("/api/v1/galaxies/{}", galaxy_id))
        .reply(&api)
        .await;
    
    assert_eq!(get_response.status(), 200);
} 