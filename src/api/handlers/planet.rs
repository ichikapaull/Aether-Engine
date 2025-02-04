use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{Reply, Rejection};
use crate::{
    application::simulation::SimulationEngine,
    domain::{Planet, PlanetType, PhysicalProperties, Atmosphere},
    application::generation::PlanetGenerator,
};

#[derive(Debug, Deserialize)]
pub struct CreatePlanetRequest {
    pub star_id: Uuid,
    pub distance_from_star: f64,
}

#[derive(Debug, Serialize)]
pub struct PlanetResponse {
    pub id: Uuid,
    pub name: String,
    pub planet_type: PlanetType,
    pub physical_properties: PhysicalProperties,
    pub atmosphere: Option<Atmosphere>,
    pub distance_from_star: f64,
}

pub async fn create_planet(
    req: CreatePlanetRequest,
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    let mut generator = PlanetGenerator::new();
    
    let planet = generator.generate(
        req.star_id,
        req.distance_from_star,
    );
    
    let planet_id = simulation.add_planet(planet.clone()).await;
    
    let response = PlanetResponse {
        id: planet_id,
        name: planet.name,
        planet_type: planet.planet_type,
        physical_properties: planet.physical_properties,
        atmosphere: planet.atmosphere,
        distance_from_star: req.distance_from_star,
    };
    
    Ok(warp::reply::json(&response))
}

pub async fn get_planet(
    id: Uuid,
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    // TODO: Implement planet retrieval from simulation
    Err(warp::reject::not_found())
}

pub async fn list_planets(
    star_id: Option<Uuid>,
    simulation: Arc<SimulationEngine>,
) -> Result<impl Reply, Rejection> {
    // TODO: Implement planet listing from simulation
    Ok(warp::reply::json(&Vec::<PlanetResponse>::new()))
} 