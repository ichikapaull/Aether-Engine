use warp::{Filter, Reply};
use std::sync::Arc;
use crate::application::simulation::SimulationEngine;
use super::handlers::{galaxy, star, planet};

pub fn setup_routes(
    simulation: Arc<SimulationEngine>,
) -> impl Filter<Extract = impl Reply> + Clone {
    let api = warp::path("api");
    let api_v1 = api.and(warp::path("v1"));

    // Galaksi rotaları
    let galaxy_routes = api_v1
        .and(warp::path("galaxies"))
        .and(
            get_galaxy_routes(simulation.clone())
                .or(create_galaxy_route(simulation.clone()))
                .or(list_galaxies_route(simulation.clone()))
        );

    // Yıldız rotaları
    let star_routes = api_v1
        .and(warp::path("stars"))
        .and(
            get_star_routes(simulation.clone())
                .or(create_star_route(simulation.clone()))
                .or(list_stars_route(simulation.clone()))
        );

    // Gezegen rotaları
    let planet_routes = api_v1
        .and(warp::path("planets"))
        .and(
            get_planet_routes(simulation.clone())
                .or(create_planet_route(simulation.clone()))
                .or(list_planets_route(simulation.clone()))
        );

    // WebSocket rotası
    let ws_route = api_v1
        .and(warp::path("ws"))
        .and(warp::ws())
        .and(with_simulation(simulation.clone()))
        .and_then(handlers::ws_handler);

    // Tüm rotaları birleştir
    galaxy_routes
        .or(star_routes)
        .or(planet_routes)
        .or(ws_route)
        .with(warp::cors().allow_any_origin())
        .recover(handle_rejection)
}

// Yardımcı fonksiyonlar
fn with_simulation(
    simulation: Arc<SimulationEngine>,
) -> impl Filter<Extract = (Arc<SimulationEngine>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || simulation.clone())
}

fn get_galaxy_routes(
    simulation: Arc<SimulationEngine>,
) -> impl Filter<Extract = impl Reply> + Clone {
    warp::get()
        .and(warp::path::param())
        .and(with_simulation(simulation))
        .and_then(galaxy::get_galaxy)
}

fn create_galaxy_route(
    simulation: Arc<SimulationEngine>,
) -> impl Filter<Extract = impl Reply> + Clone {
    warp::post()
        .and(warp::body::json())
        .and(with_simulation(simulation))
        .and_then(galaxy::create_galaxy)
}

// Benzer şekilde diğer rota tanımlamaları...

async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = "Payload too large";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    Ok(warp::reply::with_status(message.to_string(), code))
} 