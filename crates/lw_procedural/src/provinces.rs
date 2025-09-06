//! Voronoi-based province generation

use lw_core::{Fixed32, Vec2fx, DeterministicRNG};

pub struct ProvinceGenerator {
    rng: DeterministicRNG,
}

impl ProvinceGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: DeterministicRNG::new(seed),
        }
    }
    
    pub fn generate(&mut self, width: usize, height: usize, count: usize) -> Vec<ProvinceData> {
        let points = self.generate_points(width, height, count);
        self.lloyd_relaxation(points, width, height, 3)
    }
    
    fn generate_points(&mut self, width: usize, height: usize, count: usize) -> Vec<Vec2fx> {
        let mut points = Vec::with_capacity(count);
        
        for _ in 0..count {
            let x = self.rng.range_fixed(Fixed32::ZERO, Fixed32::from_num(width as i32));
            let y = self.rng.range_fixed(Fixed32::ZERO, Fixed32::from_num(height as i32));
            points.push(Vec2fx::new(x, y));
        }
        
        points
    }
    
    fn lloyd_relaxation(&mut self, mut points: Vec<Vec2fx>, width: usize, height: usize, iterations: usize) -> Vec<ProvinceData> {
        // Simplified Lloyd relaxation
        for _ in 0..iterations {
            // In a full implementation, compute Voronoi cells and move points to centroids
            for point in &mut points {
                // Simple jitter for now
                let dx = self.rng.range_fixed(Fixed32::from_num(-5), Fixed32::from_num(5));
                let dy = self.rng.range_fixed(Fixed32::from_num(-5), Fixed32::from_num(5));
                point.x = (point.x + dx).clamp(Fixed32::ZERO, Fixed32::from_num(width as i32));
                point.y = (point.y + dy).clamp(Fixed32::ZERO, Fixed32::from_num(height as i32));
            }
        }
        
        // Create province data
        points.into_iter().enumerate().map(|(id, center)| {
            ProvinceData {
                id: id as u32,
                center,
                area: Fixed32::from_num(100), // Placeholder area
                neighbors: Vec::new(),
            }
        }).collect()
    }
}

pub struct ProvinceData {
    pub id: u32,
    pub center: Vec2fx,
    pub area: Fixed32,
    pub neighbors: Vec<u32>,
}