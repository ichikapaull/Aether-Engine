use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub planet_id: Uuid,
    pub resource_type: ResourceType,
    pub abundance: f64,        // 0-1 arası
    pub accessibility: f64,    // 0-1 arası
    pub distribution: ResourceDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Water,
    Iron,
    Copper,
    Gold,
    Uranium,
    RareEarthElements,
    Helium3,
    Deuterium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDistribution {
    pub concentration: f64,    // 0-1 arası
    pub depth: f64,           // km cinsinden
    pub pattern: DistributionPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionPattern {
    Uniform,
    Clustered,
    Scattered,
    Layered,
}

impl Resource {
    pub fn new(planet_id: Uuid, resource_type: ResourceType) -> Self {
        Self {
            id: Uuid::new_v4(),
            planet_id,
            resource_type,
            abundance: 0.5,
            accessibility: 0.5,
            distribution: ResourceDistribution {
                concentration: 0.5,
                depth: 1.0,
                pattern: DistributionPattern::Uniform,
            },
        }
    }
} 