use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use uuid::Uuid;
use crate::domain::{
    Planet, PlanetType, PhysicalProperties, 
    Atmosphere, AtmosphericGas, OrbitalProperties
};

pub struct PlanetGenerator {
    noise: Perlin,
    rng: ThreadRng,
}

impl PlanetGenerator {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
            rng: thread_rng(),
        }
    }

    pub fn generate(&mut self, star_id: Uuid, distance_from_star: f64) -> Planet {
        let planet_type = self.determine_planet_type(distance_from_star);
        let name = self.generate_planet_name();
        
        let mut planet = Planet::new(star_id, name, planet_type);
        
        // Fiziksel özellikleri belirle
        planet.physical_properties = self.generate_physical_properties(&planet_type);
        
        // Atmosferi belirle (bazı gezegen türleri için None olabilir)
        planet.atmosphere = self.generate_atmosphere(&planet_type, distance_from_star);
        
        // Yörünge özelliklerini belirle
        planet.orbital_properties = self.generate_orbital_properties(distance_from_star);
        
        planet
    }

    fn determine_planet_type(&mut self, distance: f64) -> PlanetType {
        // Yıldıza olan uzaklığa göre gezegen tipini belirle
        let random = self.rng.gen::<f64>();
        match distance {
            d if d < 0.7 => {
                // İç sistem
                if random < 0.8 { PlanetType::Terrestrial }
                else { PlanetType::DwarfPlanet }
            },
            d if d < 2.0 => {
                // Asteroid kuşağı bölgesi
                if random < 0.6 { PlanetType::Terrestrial }
                else { PlanetType::DwarfPlanet }
            },
            d if d < 15.0 => {
                // Dış sistem
                if random < 0.7 { PlanetType::GasGiant }
                else { PlanetType::IceGiant }
            },
            _ => {
                // Uzak dış sistem
                if random < 0.8 { PlanetType::IceGiant }
                else { PlanetType::DwarfPlanet }
            }
        }
    }

    fn generate_physical_properties(&mut self, planet_type: &PlanetType) -> PhysicalProperties {
        match planet_type {
            PlanetType::Terrestrial => PhysicalProperties {
                mass: self.rng.gen_range(0.055..10.0),        // Merkür'den biraz küçük - Süper Dünya
                radius: self.rng.gen_range(0.38..2.0),
                gravity: 0.0,  // Kütle ve yarıçapa göre hesaplanacak
                temperature: self.rng.gen_range(50.0..700.0),
            },
            PlanetType::GasGiant => PhysicalProperties {
                mass: self.rng.gen_range(15.0..5000.0),       // Neptün'den Süper Jüpiter'e
                radius: self.rng.gen_range(3.0..22.0),
                gravity: 0.0,
                temperature: self.rng.gen_range(30.0..165.0),
            },
            PlanetType::IceGiant => PhysicalProperties {
                mass: self.rng.gen_range(10.0..50.0),
                radius: self.rng.gen_range(2.5..6.0),
                gravity: 0.0,
                temperature: self.rng.gen_range(20.0..100.0),
            },
            PlanetType::DwarfPlanet => PhysicalProperties {
                mass: self.rng.gen_range(0.0001..0.05),
                radius: self.rng.gen_range(0.04..0.35),
                gravity: 0.0,
                temperature: self.rng.gen_range(30.0..200.0),
            },
        }
    }

    fn generate_atmosphere(&mut self, planet_type: &PlanetType, distance: f64) -> Option<Atmosphere> {
        match planet_type {
            PlanetType::Terrestrial => {
                if distance < 1.5 && self.rng.gen::<f64>() < 0.7 {
                    Some(self.generate_terrestrial_atmosphere())
                } else {
                    None
                }
            },
            PlanetType::GasGiant | PlanetType::IceGiant => {
                Some(self.generate_giant_atmosphere(planet_type))
            },
            PlanetType::DwarfPlanet => None,
        }
    }

    fn generate_terrestrial_atmosphere(&mut self) -> Atmosphere {
        Atmosphere {
            pressure: self.rng.gen_range(0.001..5.0),
            composition: vec![
                (AtmosphericGas::Nitrogen, self.rng.gen_range(65.0..80.0)),
                (AtmosphericGas::Oxygen, self.rng.gen_range(10.0..30.0)),
                (AtmosphericGas::CarbonDioxide, self.rng.gen_range(0.0..5.0)),
            ],
        }
    }

    fn generate_giant_atmosphere(&mut self, planet_type: &PlanetType) -> Atmosphere {
        match planet_type {
            PlanetType::GasGiant => Atmosphere {
                pressure: self.rng.gen_range(100.0..1000.0),
                composition: vec![
                    (AtmosphericGas::Hydrogen, self.rng.gen_range(75.0..90.0)),
                    (AtmosphericGas::Helium, self.rng.gen_range(10.0..25.0)),
                ],
            },
            PlanetType::IceGiant => Atmosphere {
                pressure: self.rng.gen_range(50.0..500.0),
                composition: vec![
                    (AtmosphericGas::Hydrogen, self.rng.gen_range(60.0..80.0)),
                    (AtmosphericGas::Helium, self.rng.gen_range(15.0..30.0)),
                    (AtmosphericGas::Methane, self.rng.gen_range(1.0..5.0)),
                ],
            },
            _ => unreachable!(),
        }
    }

    fn generate_orbital_properties(&mut self, distance: f64) -> OrbitalProperties {
        OrbitalProperties {
            semi_major_axis: distance,
            eccentricity: self.rng.gen_range(0.0..0.2),
            orbital_period: distance.powf(1.5), // Kepler'in 3. yasası
            rotation_period: self.rng.gen_range(0.1..100.0),
        }
    }

    fn generate_planet_name(&mut self) -> String {
        // Basit bir gezegen isimlendirme sistemi
        let prefix = ["Alpha", "Beta", "Gamma", "Delta"];
        let suffix = self.rng.gen_range(1..999);
        format!("{}-{}", prefix[self.rng.gen_range(0..prefix.len())], suffix)
    }
}

impl Default for PlanetGenerator {
    fn default() -> Self {
        Self::new()
    }
} 