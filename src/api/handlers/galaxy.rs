use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{Reply, Rejection};
use crate::{
    application::simulation::SimulationEngine,
    domain::Galaxy,
    application::generation::GalaxyGenerator,
};

#[derive(Debug, Deserialize)]
pub struct CreateGalaxyRequest {
    pub name: String,
    pub seed: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct GalaxyResponse {
    pub id: Uuid,
    pub name: String,
    pub star_count: u64,
    pub age: f64,
}

pub async fn create_galaxy(
    req: CreateGalaxyRequest,
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    let mut generator = GalaxyGenerator::new();
    let seed = req.seed.unwrap_or_else(|| rand::random());
    
    let galaxy = generator.generate(req.name, seed);
    let galaxy_id = simulation.add_galaxy(galaxy.clone()).await;
    
    let response = GalaxyResponse {
        id: galaxy_id,
        name: galaxy.name,
        star_count: galaxy.star_count,
        age: galaxy.age,
    };
    
    Ok(warp::reply::json(&response))
}

pub async fn get_galaxy(
    id: Uuid,
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    // TODO: Implement galaxy retrieval from simulation
    // For now, return a 404
    Err(warp::reject::not_found())
}

pub async fn list_galaxies(
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    // TODO: Implement galaxy listing from simulation
    Ok(warp::reply::json(&Vec::<GalaxyResponse>::new()))
} 