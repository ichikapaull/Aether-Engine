use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub id: Uuid,
    pub star_id: Uuid,
    pub name: String,
    pub planet_type: PlanetType,
    pub physical_properties: PhysicalProperties,
    pub atmosphere: Option<Atmosphere>,
    pub orbital_properties: OrbitalProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanetType {
    Terrestrial,
    GasGiant,
    IceGiant,
    DwarfPlanet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalProperties {
    pub mass: f64,         // Dünya kütlesi cinsinden
    pub radius: f64,       // Dünya yarıçapı cinsinden
    pub gravity: f64,      // m/s² cinsinden
    pub temperature: f64,  // Kelvin cinsinden
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atmosphere {
    pub pressure: f64,     // bar cinsinden
    pub composition: Vec<(AtmosphericGas, f64)>, // (gaz türü, yüzde)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AtmosphericGas {
    Hydrogen,
    Helium,
    Nitrogen,
    Oxygen,
    CarbonDioxide,
    Methane,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalProperties {
    pub semi_major_axis: f64,  // AU cinsinden
    pub eccentricity: f64,     // 0-1 arası
    pub orbital_period: f64,   // Dünya yılı cinsinden
    pub rotation_period: f64,  // Dünya günü cinsinden
}

impl Planet {
    pub fn new(star_id: Uuid, name: String, planet_type: PlanetType) -> Self {
        Self {
            id: Uuid::new_v4(),
            star_id,
            name,
            planet_type,
            physical_properties: PhysicalProperties {
                mass: 1.0,
                radius: 1.0,
                gravity: 9.81,
                temperature: 288.0,
            },
            atmosphere: Some(Atmosphere {
                pressure: 1.0,
                composition: vec![
                    (AtmosphericGas::Nitrogen, 78.0),
                    (AtmosphericGas::Oxygen, 21.0),
                ],
            }),
            orbital_properties: OrbitalProperties {
                semi_major_axis: 1.0,
                eccentricity: 0.0167,
                orbital_period: 1.0,
                rotation_period: 1.0,
            },
        }
    }
} 