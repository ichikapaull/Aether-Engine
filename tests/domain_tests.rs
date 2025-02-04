mod common;

use universe_simulator::domain::{Galaxy, Star, Planet, PlanetType};
use uuid::Uuid;

#[test]
fn test_galaxy_creation() {
    let name = "Test Galaxy".to_string();
    let seed = 12345;
    let galaxy = Galaxy::new(name.clone(), seed);
    
    assert_eq!(galaxy.name, name);
    assert_eq!(galaxy.seed, seed);
    assert!(galaxy.size.radius > 0.0);
    assert!(galaxy.size.thickness > 0.0);
    assert!(galaxy.star_count > 0);
}

#[test]
fn test_star_creation() {
    let galaxy_id = Uuid::new_v4();
    let position = Star::Position3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    
    let star = Star::new(galaxy_id, "Test Star".to_string(), position);
    
    assert_eq!(star.galaxy_id, galaxy_id);
    assert!(star.mass > 0.0);
    assert!(star.temperature > 0.0);
}

#[test]
fn test_planet_creation() {
    let star_id = Uuid::new_v4();
    let planet = Planet::new(
        star_id,
        "Test Planet".to_string(),
        PlanetType::Terrestrial,
    );
    
    assert_eq!(planet.star_id, star_id);
    assert!(planet.physical_properties.mass > 0.0);
    assert!(planet.physical_properties.radius > 0.0);
} 