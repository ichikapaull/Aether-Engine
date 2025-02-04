use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{Reply, Rejection};
use crate::{
    application::simulation::SimulationEngine,
    domain::{Star, Position3D, SpectralType},
    application::generation::StarGenerator,
};

#[derive(Debug, Deserialize)]
pub struct CreateStarRequest {
    pub galaxy_id: Uuid,
    pub position: Position3DRequest,
}

#[derive(Debug, Deserialize)]
pub struct Position3DRequest {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize)]
pub struct StarResponse {
    pub id: Uuid,
    pub name: String,
    pub spectral_type: SpectralType,
    pub mass: f64,
    pub temperature: f64,
    pub position: Position3D,
}

pub async fn create_star(
    req: CreateStarRequest,
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    let mut generator = StarGenerator::new();
    
    let position = Position3D {
        x: req.position.x,
        y: req.position.y,
        z: req.position.z,
    };
    
    let star = generator.generate(req.galaxy_id, position);
    let star_id = simulation.add_star(star.clone()).await;
    
    let response = StarResponse {
        id: star_id,
        name: star.name,
        spectral_type: star.spectral_type,
        mass: star.mass,
        temperature: star.temperature,
        position: star.position,
    };
    
    Ok(warp::reply::json(&response))
}

pub async fn get_star(
    id: Uuid,
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    // TODO: Implement star retrieval from simulation
    Err(warp::reject::not_found())
}

pub async fn list_stars(
    galaxy_id: Option<Uuid>,
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    // TODO: Implement star listing from simulation
    Ok(warp::reply::json(&Vec::<StarResponse>::new()))
} 