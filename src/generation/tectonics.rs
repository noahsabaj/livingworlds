//! Tectonic plate generation for continent placement

use rand::{Rng, rngs::StdRng};
use crate::constants::*;
use super::types::MapBounds;

#[derive(Debug, Clone)]
pub struct TectonicSystem {
    pub plate_centers: Vec<(f32, f32)>,
    pub continent_centers: Vec<(f32, f32)>,
}

pub fn generate(rng: &mut StdRng, bounds: MapBounds, seed: u32) -> TectonicSystem {
    let num_plates = TECTONIC_PLATES_BASE + (seed % TECTONIC_PLATES_VARIATION) as usize;
    let mut plate_centers = Vec::new();
    let mut continent_centers = Vec::new();
    
    // Place tectonic plates randomly across the map
    for _ in 0..num_plates {
        let px = rng.gen_range(bounds.x_min * 0.95..bounds.x_max * 0.95);
        let py = rng.gen_range(bounds.y_min * 0.95..bounds.y_max * 0.95);
        plate_centers.push((px, py));
        
        // 80% chance this plate has a major continent
        if rng.gen_range(0.0..1.0) < 0.8 {
            // Continent offset from plate center for variety
            let offset_x = rng.gen_range(-800.0..800.0);
            let offset_y = rng.gen_range(-600.0..600.0);
            continent_centers.push((px + offset_x, py + offset_y));
        }
    }
    
    println!("Generated {} tectonic plates with {} landmasses", 
             plate_centers.len(), continent_centers.len());
    
    TectonicSystem {
        plate_centers,
        continent_centers,
    }
}