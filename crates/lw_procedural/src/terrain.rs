//! Terrain generation using Simplex noise

use noise::{NoiseFn, Perlin};
use lw_core::Fixed32;

pub struct TerrainGenerator {
    seed: u32,
    noise: Perlin,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        let noise = Perlin::new(seed);
        Self { seed, noise }
    }
    
    pub fn generate(&self, width: usize, height: usize) -> HeightMap {
        let mut heights = vec![Fixed32::ZERO; width * height];
        
        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 / width as f64;
                let ny = y as f64 / height as f64;
                
                // Multi-octave noise for terrain
                let value = self.octave_noise(nx * 4.0, ny * 4.0, 6);
                heights[y * width + x] = Fixed32::from_float(value as f32);
            }
        }
        
        HeightMap { width, height, heights }
    }
    
    fn octave_noise(&self, x: f64, y: f64, octaves: i32) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;
        
        for _ in 0..octaves {
            value += self.noise.get([x * frequency, y * frequency]) * amplitude;
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }
        
        value / max_value
    }
}

pub struct HeightMap {
    pub width: usize,
    pub height: usize,
    pub heights: Vec<Fixed32>,
}

impl HeightMap {
    pub fn get(&self, x: usize, y: usize) -> Fixed32 {
        self.heights[y * self.width + x]
    }
    
    pub fn normalize(&mut self) {
        let min = *self.heights.iter().min().unwrap();
        let max = *self.heights.iter().max().unwrap();
        let range = max - min;
        
        if range > Fixed32::EPSILON {
            for h in &mut self.heights {
                *h = (*h - min) / range;
            }
        }
    }
}