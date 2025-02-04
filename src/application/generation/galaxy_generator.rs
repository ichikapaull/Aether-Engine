use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use crate::domain::Galaxy;

pub struct GalaxyGenerator {
    noise: Perlin,
    rng: ThreadRng,
}

impl GalaxyGenerator {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
            rng: thread_rng(),
        }
    }

    pub fn generate(&mut self, name: String, seed: u64) -> Galaxy {
        let mut galaxy = Galaxy::new(name, seed);
        
        // Galaksi özelliklerini seed'e göre belirle
        let mut seeded_rng = StdRng::seed_from_u64(seed);
        
        // Galaksi boyutunu rastgele belirle (gerçekçi aralıklar içinde)
        galaxy.size.radius = seeded_rng.gen_range(25_000.0..100_000.0);
        galaxy.size.thickness = galaxy.size.radius * 0.01; // Disk kalınlığı genelde çapın %1'i kadardır
        
        // Galaksi yaşını belirle (5-14 milyar yıl arası)
        galaxy.age = seeded_rng.gen_range(5.0..14.0);
        
        // Yıldız sayısını galaksi büyüklüğüne göre hesapla
        let size_factor = galaxy.size.radius / 50_000.0;
        galaxy.star_count = (size_factor * 100_000_000.0) as u64;
        
        galaxy
    }

    pub fn generate_spiral_arms(&self, galaxy: &Galaxy) -> Vec<SpiralArm> {
        let arm_count = 4; // Tipik bir sarmal galaksi için
        let mut arms = Vec::with_capacity(arm_count);
        
        for i in 0..arm_count {
            let rotation = 2.0 * std::f64::consts::PI * (i as f64 / arm_count as f64);
            arms.push(SpiralArm {
                initial_angle: rotation,
                pitch_angle: 0.2, // Yaklaşık 11.5 derece
                tightness: 0.8,
            });
        }
        
        arms
    }
}

pub struct SpiralArm {
    initial_angle: f64,
    pitch_angle: f64,
    tightness: f64,
}

impl Default for GalaxyGenerator {
    fn default() -> Self {
        Self::new()
    }
} 