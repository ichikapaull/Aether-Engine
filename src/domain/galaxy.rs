use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Galaxy {
    pub id: Uuid,
    pub name: String,
    pub seed: u64,
    pub size: GalaxySize,
    pub age: f64, // milyar yıl cinsinden
    pub star_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxySize {
    pub radius: f64,      // ışık yılı cinsinden
    pub thickness: f64,   // ışık yılı cinsinden
}

impl Galaxy {
    pub fn new(name: String, seed: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            seed,
            size: GalaxySize {
                radius: 50_000.0,    // varsayılan değer: 50,000 ışık yılı
                thickness: 1_000.0,   // varsayılan değer: 1,000 ışık yılı
            },
            age: 13.6,               // varsayılan değer: 13.6 milyar yıl
            star_count: 100_000_000, // varsayılan değer: 100 milyon yıldız
        }
    }
} 