use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub id: Uuid,
    pub galaxy_id: Uuid,
    pub name: String,
    pub position: Position3D,
    pub mass: f64,        // güneş kütlesi cinsinden
    pub temperature: f64, // Kelvin cinsinden
    pub age: f64,         // milyar yıl cinsinden
    pub spectral_type: SpectralType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpectralType {
    O, B, A, F, G, K, M,
}

impl Star {
    pub fn new(galaxy_id: Uuid, name: String, position: Position3D) -> Self {
        Self {
            id: Uuid::new_v4(),
            galaxy_id,
            name,
            position,
            mass: 1.0,           // varsayılan değer: 1 güneş kütlesi
            temperature: 5778.0,  // varsayılan değer: güneş sıcaklığı
            age: 4.6,            // varsayılan değer: güneş yaşı
            spectral_type: SpectralType::G,
        }
    }
} 