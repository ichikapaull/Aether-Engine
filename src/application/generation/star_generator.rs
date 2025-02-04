use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use uuid::Uuid;
use crate::domain::{Star, Position3D, SpectralType};

pub struct StarGenerator {
    noise: Perlin,
    rng: ThreadRng,
}

impl StarGenerator {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
            rng: thread_rng(),
        }
    }

    pub fn generate(&mut self, galaxy_id: Uuid, position: Position3D) -> Star {
        let mut star = Star::new(
            galaxy_id,
            self.generate_star_name(),
            position,
        );

        // Yıldız kütlesini belirle (0.08 - 150 güneş kütlesi arası)
        star.mass = self.generate_stellar_mass();
        
        // Spektral sınıfı kütle bazlı belirle
        star.spectral_type = self.determine_spectral_type(star.mass);
        
        // Sıcaklığı spektral sınıfa göre belirle
        star.temperature = self.calculate_temperature(&star.spectral_type);
        
        // Yaşı kütle ve spektral sınıfa göre belirle
        star.age = self.calculate_age(star.mass);
        
        star
    }

    fn generate_star_name(&mut self) -> String {
        // Gerçekçi yıldız isimleri için basit bir sistem
        let prefix = ["HD", "HIP", "GJ", "Kepler"];
        let number = self.rng.gen_range(100000..999999);
        format!("{}-{}", prefix[self.rng.gen_range(0..prefix.len())], number)
    }

    fn generate_stellar_mass(&mut self) -> f64 {
        // Kroupa IMF dağılımını kullan
        let x: f64 = self.rng.gen();
        if x < 0.7 {
            // M-tipi cüceler (en yaygın)
            self.rng.gen_range(0.08..0.45)
        } else if x < 0.9 {
            // K ve G tipi yıldızlar
            self.rng.gen_range(0.45..1.4)
        } else {
            // Daha masif yıldızlar
            self.rng.gen_range(1.4..150.0)
        }
    }

    fn determine_spectral_type(&self, mass: f64) -> SpectralType {
        match mass {
            m if m >= 16.0 => SpectralType::O,
            m if m >= 2.1 => SpectralType::B,
            m if m >= 1.4 => SpectralType::A,
            m if m >= 1.04 => SpectralType::F,
            m if m >= 0.8 => SpectralType::G,
            m if m >= 0.45 => SpectralType::K,
            _ => SpectralType::M,
        }
    }

    fn calculate_temperature(&self, spectral_type: &SpectralType) -> f64 {
        match spectral_type {
            SpectralType::O => self.rng.gen_range(30000.0..50000.0),
            SpectralType::B => self.rng.gen_range(10000.0..30000.0),
            SpectralType::A => self.rng.gen_range(7500.0..10000.0),
            SpectralType::F => self.rng.gen_range(6000.0..7500.0),
            SpectralType::G => self.rng.gen_range(5200.0..6000.0),
            SpectralType::K => self.rng.gen_range(3700.0..5200.0),
            SpectralType::M => self.rng.gen_range(2400.0..3700.0),
        }
    }

    fn calculate_age(&mut self, mass: f64) -> f64 {
        // Kütleye bağlı olarak maksimum yaşı hesapla
        let max_age = 10.0 / mass.powf(2.5);
        self.rng.gen_range(0.0..max_age)
    }
}

impl Default for StarGenerator {
    fn default() -> Self {
        Self::new()
    }
} 