//! Erosion simulation for realistic terrain generation
//!
//! This module implements hydraulic and thermal erosion algorithms
//! that transform tectonic heightmaps into realistic terrain with
//! river valleys, sediment deposits, and natural drainage patterns.

use bevy::prelude::*;
use rand::{Rng, rngs::StdRng, SeedableRng};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::math::{euclidean_vec2, linear_falloff};

// CONSTANTS - Erosion parameters tuned for realism

/// Number of water droplets to simulate for hydraulic erosion
const HYDRAULIC_ITERATIONS: usize = 500_000;

/// Initial water volume for each droplet
const INITIAL_WATER_VOLUME: f32 = 1.0;

/// Water evaporation rate per step
const EVAPORATION_RATE: f32 = 0.01;

/// Sediment capacity multiplier
const SEDIMENT_CAPACITY: f32 = 4.0;

/// Minimum slope for sediment pickup
const MIN_SLOPE: f32 = 0.01;

/// Erosion rate - how much material is picked up
const EROSION_RATE: f32 = 0.3;

/// Deposition rate - how much sediment is dropped
const DEPOSITION_RATE: f32 = 0.3;

/// Gravity cScreenshot from 2025-09-13 15-00-22.pngonstant for water flow
const GRAVITY: f32 = 4.0;

/// Maximum lifetime of a water droplet (steps)
const MAX_DROPLET_LIFETIME: usize = 100;

/// Thermal erosion threshold angle (radians)
const THERMAL_ANGLE_THRESHOLD: f32 = 0.6;  // ~34 degrees

/// Thermal erosion rate
const THERMAL_EROSION_RATE: f32 = 0.1;

/// Number of thermal erosion iterations (reduced for performance)
const THERMAL_ITERATIONS: usize = 10;

/// Blur radius for smoothing after erosion
const SMOOTHING_RADIUS: usize = 2;

/// Inertia - how much previous direction affects new direction
const INERTIA: f32 = 0.3;


/// A water droplet for hydraulic erosion simulation
#[derive(Debug, Clone)]
struct WaterDroplet {
    position: Vec2,
    velocity: Vec2,
    water: f32,
    sediment: f32,
    lifetime: usize,
}

impl WaterDroplet {
    fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            water: INITIAL_WATER_VOLUME,
            sediment: 0.0,
            lifetime: 0,
        }
    }
}

/// Heightmap for erosion simulation
pub struct HeightMap {
    /// Width in grid cells
    pub width: usize,
    /// Height in grid cells
    pub height: usize,
    /// Elevation data (0.0 to 1.0)
    pub data: Vec<f32>,
    /// Cell size in world units
    pub cell_size: f32,
}

impl HeightMap {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
            cell_size,
        }
    }

    /// Get elevation at grid coordinates
    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            0.0
        }
    }

    /// Set elevation at grid coordinates
    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value.clamp(0.0, 1.0);
        }
    }

    /// Get interpolated elevation at floating point position
    pub fn get_interpolated(&self, pos: Vec2) -> f32 {
        // Clamp position to valid range
        let x = (pos.x / self.cell_size).clamp(0.0, (self.width - 1) as f32);
        let y = (pos.y / self.cell_size).clamp(0.0, (self.height - 1) as f32);

        let x0 = (x.floor() as usize).min(self.width - 1);
        let y0 = (y.floor() as usize).min(self.height - 1);
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        let h00 = self.get(x0, y0);
        let h10 = self.get(x1, y0);
        let h01 = self.get(x0, y1);
        let h11 = self.get(x1, y1);

        // Bilinear interpolation
        let h0 = h00 * (1.0 - fx) + h10 * fx;
        let h1 = h01 * (1.0 - fx) + h11 * fx;
        h0 * (1.0 - fy) + h1 * fy
    }

    /// Calculate gradient at position
    pub fn get_gradient(&self, pos: Vec2) -> Vec2 {
        let epsilon = self.cell_size * 0.5;

        let h_left = self.get_interpolated(pos - Vec2::new(epsilon, 0.0));
        let h_right = self.get_interpolated(pos + Vec2::new(epsilon, 0.0));
        let h_down = self.get_interpolated(pos - Vec2::new(0.0, epsilon));
        let h_up = self.get_interpolated(pos + Vec2::new(0.0, epsilon));

        Vec2::new(
            (h_left - h_right) / (2.0 * epsilon),
            (h_down - h_up) / (2.0 * epsilon),
        )
    }

    /// Deposit or erode at position
    pub fn modify_at(&mut self, pos: Vec2, amount: f32) {
        let x = (pos.x / self.cell_size) as usize;
        let y = (pos.y / self.cell_size) as usize;

        // Distribute change to neighboring cells based on proximity
        for dx in 0..=1 {
            for dy in 0..=1 {
                let nx = (x + dx).min(self.width - 1);
                let ny = (y + dy).min(self.height - 1);

                let cell_pos = Vec2::new(
                    nx as f32 * self.cell_size,
                    ny as f32 * self.cell_size,
                );

                let distance = euclidean_vec2(pos, cell_pos);
                let weight = linear_falloff(distance, self.cell_size);

                let current = self.get(nx, ny);
                self.set(nx, ny, current + amount * weight);
            }
        }
    }
}


/// Main erosion system that combines all erosion types
pub struct ErosionSystem {
    heightmap: HeightMap,
    rng: StdRng,
}

impl ErosionSystem {
    pub fn new(heightmap: HeightMap, rng: StdRng) -> Self {
        Self { heightmap, rng }
    }

    /// Run full erosion simulation
    pub fn erode(&mut self, iterations: usize) {
        println!("  Starting erosion simulation ({} iterations)...", iterations);
        println!("  Heightmap dimensions: {}x{} ({} total cells)",
            self.heightmap.width,
            self.heightmap.height,
            self.heightmap.width * self.heightmap.height
        );

        // Reduced passes for better performance (was 3)
        let num_passes = if self.heightmap.width * self.heightmap.height > 100_000 { 1 } else { 2 };

        for pass in 0..num_passes {
            println!("    Erosion pass {}/{}", pass + 1, num_passes);

            // Hydraulic erosion - water carving channels
            self.hydraulic_erosion(iterations / num_passes);

            // Thermal erosion - material sliding down slopes
            println!("      Starting thermal erosion ({} iterations)...", THERMAL_ITERATIONS);
            self.thermal_erosion(THERMAL_ITERATIONS);
            println!("      Thermal erosion complete");

            // Smooth to remove artifacts
            println!("      Smoothing terrain...");
            self.smooth(1);
            println!("      Smoothing complete");
        }

        println!("  Erosion simulation complete");
    }

    /// Hydraulic erosion - water flowing and carving terrain (PARALLELIZED)
    fn hydraulic_erosion(&mut self, droplets: usize) {
        let batch_size = 500;  // Process 500 droplets at a time
        let num_batches = (droplets + batch_size - 1) / batch_size;

        let base_seed = self.rng.gen::<u64>();

        for batch in 0..num_batches {
            let batch_start = batch * batch_size;
            let batch_end = (batch_start + batch_size).min(droplets);
            let batch_droplets = batch_end - batch_start;

            // Progress reporting
            let progress = (batch_start as f32 / droplets as f32 * 100.0) as u32;
            print!("\r      Hydraulic erosion: {}%", progress);

            // Shared heightmap data for read-only access
            let heightmap_width = self.heightmap.width;
            let heightmap_height = self.heightmap.height;
            let heightmap_cell_size = self.heightmap.cell_size;
            let heightmap_data = &self.heightmap.data;

            let erosion_changes: Vec<(Vec2, f32)> = (0..batch_droplets)
                .into_par_iter()
                .flat_map(|i| {
                    // Each thread gets its own RNG with unique seed
                    let mut thread_rng = StdRng::seed_from_u64(base_seed + (batch_start + i) as u64);

                    // Random starting position
                    let start_x = thread_rng.gen_range(0.0..heightmap_width as f32) * heightmap_cell_size;
                    let start_y = thread_rng.gen_range(0.0..heightmap_height as f32) * heightmap_cell_size;
                    let mut droplet = WaterDroplet::new(Vec2::new(start_x, start_y));

                    // Track changes this droplet wants to make
                    let mut droplet_changes = Vec::new();

                    // Simulate droplet flowing downhill
                    while droplet.lifetime < MAX_DROPLET_LIFETIME && droplet.water > 0.001 {
                        let gradient = Self::calculate_gradient_static(
                            droplet.position,
                            heightmap_data,
                            heightmap_width,
                            heightmap_height,
                            heightmap_cell_size
                        );

                        let flow_dir = -gradient.normalize_or_zero();

                        droplet.velocity = droplet.velocity * INERTIA + flow_dir * (1.0 - INERTIA);
                        droplet.velocity = droplet.velocity.normalize_or_zero() * droplet.velocity.length().min(1.0);

                        // Move droplet
                        let old_pos = droplet.position;
                        droplet.position += droplet.velocity * heightmap_cell_size;

                        // Keep within bounds
                        droplet.position.x = droplet.position.x.clamp(0.0,
                            (heightmap_width - 1) as f32 * heightmap_cell_size);
                        droplet.position.y = droplet.position.y.clamp(0.0,
                            (heightmap_height - 1) as f32 * heightmap_cell_size);

                        let old_height = Self::get_interpolated_static(
                            old_pos,
                            heightmap_data,
                            heightmap_width,
                            heightmap_height,
                            heightmap_cell_size
                        );
                        let new_height = Self::get_interpolated_static(
                            droplet.position,
                            heightmap_data,
                            heightmap_width,
                            heightmap_height,
                            heightmap_cell_size
                        );
                        let height_diff = new_height - old_height;

                        let slope = gradient.length().max(MIN_SLOPE);
                        let capacity = slope * droplet.water * droplet.velocity.length() * SEDIMENT_CAPACITY;

                        if height_diff > 0.0 || droplet.sediment > capacity {
                            // Deposit sediment
                            let amount_to_deposit = if height_diff > 0.0 {
                                height_diff.min(droplet.sediment)
                            } else {
                                (droplet.sediment - capacity) * DEPOSITION_RATE
                            };

                            droplet.sediment -= amount_to_deposit;
                            droplet_changes.push((old_pos, amount_to_deposit));
                        } else {
                            // Erode terrain
                            let amount_to_erode = ((capacity - droplet.sediment) * EROSION_RATE)
                                .min(-height_diff);

                            droplet.sediment += amount_to_erode;
                            droplet_changes.push((old_pos, -amount_to_erode));
                        }

                        // Evaporate water
                        droplet.water *= 1.0 - EVAPORATION_RATE;
                        droplet.lifetime += 1;
                    }

                    droplet_changes
                })
                .collect();

            // Apply all erosion changes from this batch
            for (pos, amount) in erosion_changes {
                self.heightmap.modify_at(pos, amount);
            }
        }

        println!("\r      Hydraulic erosion: 100%");
    }

    /// Thread-safe static version of get_interpolated
    fn get_interpolated_static(
        pos: Vec2,
        data: &[f32],
        width: usize,
        height: usize,
        cell_size: f32,
    ) -> f32 {
        // Clamp position to valid range first
        let x = (pos.x / cell_size).clamp(0.0, (width - 1) as f32);
        let y = (pos.y / cell_size).clamp(0.0, (height - 1) as f32);

        let x0 = (x.floor() as usize).min(width - 1);
        let y0 = (y.floor() as usize).min(height - 1);
        let x1 = (x0 + 1).min(width - 1);
        let y1 = (y0 + 1).min(height - 1);

        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        // Ensure indices are valid before access
        let idx00 = y0 * width + x0;
        let idx10 = y1 * width + x0;
        let idx01 = y0 * width + x1;
        let idx11 = y1 * width + x1;

        // Extra safety check
        let max_idx = width * height - 1;
        if idx00 > max_idx || idx10 > max_idx || idx01 > max_idx || idx11 > max_idx {
            // Fallback to nearest valid cell
            return data[y0.min(height - 1) * width + x0.min(width - 1)];
        }

        let h00 = data[idx00];
        let h10 = data[idx10];
        let h01 = data[idx01];
        let h11 = data[idx11];

        // Bilinear interpolation
        let h0 = h00 * (1.0 - fx) + h01 * fx;
        let h1 = h10 * (1.0 - fx) + h11 * fx;
        h0 * (1.0 - fy) + h1 * fy
    }

    /// Thread-safe static version of calculate_gradient
    fn calculate_gradient_static(
        pos: Vec2,
        data: &[f32],
        width: usize,
        height: usize,
        cell_size: f32,
    ) -> Vec2 {
        let epsilon = cell_size * 0.5;

        let h_left = Self::get_interpolated_static(
            pos - Vec2::new(epsilon, 0.0),
            data,
            width,
            height,
            cell_size
        );
        let h_right = Self::get_interpolated_static(
            pos + Vec2::new(epsilon, 0.0),
            data,
            width,
            height,
            cell_size
        );
        let h_down = Self::get_interpolated_static(
            pos - Vec2::new(0.0, epsilon),
            data,
            width,
            height,
            cell_size
        );
        let h_up = Self::get_interpolated_static(
            pos + Vec2::new(0.0, epsilon),
            data,
            width,
            height,
            cell_size
        );

        Vec2::new(
            (h_left - h_right) / (2.0 * epsilon),
            (h_down - h_up) / (2.0 * epsilon),
        )
    }

    /// Thermal erosion - material sliding down steep slopes (PARALLELIZED)
    fn thermal_erosion(&mut self, iterations: usize) {
        for iter in 0..iterations {
            let width = self.heightmap.width;
            let height = self.heightmap.height;
            let cell_size = self.heightmap.cell_size;
            let heightmap_data = &self.heightmap.data;

            // Collect erosion changes in parallel
            let changes: Vec<(usize, usize, f32)> = (1..height - 1)
                .into_par_iter()
                .flat_map(|y| {
                    let mut row_changes = Vec::new();

                    for x in 1..width - 1 {
                        let center_idx = y * width + x;
                        let center_height = heightmap_data[center_idx];

                        let mut max_diff = 0.0;
                        let mut lowest_neighbor = (x, y);

                        for dy in -1i32..=1 {
                            for dx in -1i32..=1 {
                                if dx == 0 && dy == 0 { continue; }

                                let nx = (x as i32 + dx) as usize;
                                let ny = (y as i32 + dy) as usize;

                                if nx > 0 && nx < width - 1 && ny > 0 && ny < height - 1 {
                                    let neighbor_idx = ny * width + nx;
                                    let neighbor_height = heightmap_data[neighbor_idx];
                                    let height_diff = center_height - neighbor_height;

                                    // Account for diagonal distance (Manhattan distance of 2 = diagonal)
                                    let distance = if dx.abs() + dy.abs() == 2 {
                                        std::f32::consts::SQRT_2  // Diagonal distance
                                    } else {
                                        1.0  // Cardinal direction distance
                                    };

                                    let slope = height_diff / (distance * cell_size);

                                    if slope > THERMAL_ANGLE_THRESHOLD && height_diff > max_diff {
                                        max_diff = height_diff;
                                        lowest_neighbor = (nx, ny);
                                    }
                                }
                            }
                        }

                        // If slope exceeds threshold, schedule material transfer
                        if max_diff > 0.0 {
                            let amount = max_diff * THERMAL_EROSION_RATE;
                            row_changes.push((x, y, -amount));
                            row_changes.push((lowest_neighbor.0, lowest_neighbor.1, amount));
                        }
                    }

                    row_changes
                })
                .collect();

            // Apply all changes sequentially to avoid race conditions
            for (x, y, amount) in changes {
                let current = self.heightmap.get(x, y);
                self.heightmap.set(x, y, current + amount);
            }

            // Progress reporting for longer runs
            if iterations > 5 && (iter + 1) % 5 == 0 {
                print!("\r        Thermal erosion: {}%", ((iter + 1) * 100 / iterations));
            }
        }

        if iterations > 5 {
            println!();
        }
    }

    /// Smooth the heightmap to remove artifacts (PARALLELIZED)
    fn smooth(&mut self, radius: usize) {
        let width = self.heightmap.width;
        let height = self.heightmap.height;
        let old_data = &self.heightmap.data;

        let new_data: Vec<f32> = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                let mut row = Vec::with_capacity(width);

                for x in 0..width {
                    let mut sum = 0.0;
                    let mut count = 0;

                    for dy in -(radius as i32)..=(radius as i32) {
                        for dx in -(radius as i32)..=(radius as i32) {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            if nx >= 0 && nx < width as i32 &&
                               ny >= 0 && ny < height as i32 {
                                let idx = ny as usize * width + nx as usize;
                                sum += old_data[idx];
                                count += 1;
                            }
                        }
                    }

                    row.push(if count > 0 { sum / count as f32 } else { 0.0 });
                }

                row
            })
            .collect();

        self.heightmap.data = new_data;
    }

    pub fn get_heightmap(self) -> HeightMap {
        self.heightmap
    }
}


/// Apply erosion to province elevations
pub fn apply_erosion_to_provinces(
    provinces: &mut [crate::components::Province],
    dimensions: crate::resources::MapDimensions,
    rng: StdRng,
    iterations: usize,
) {
    println!("  Applying erosion simulation to terrain...");

    // Create heightmap from provinces - use much coarser grid for performance
    // Instead of dividing by 20, use a larger cell size based on world size
    let cell_size = match provinces.len() {
        n if n < 400_000 => 100.0,  // Small worlds: 100 pixel cells
        n if n < 700_000 => 150.0,  // Medium worlds: 150 pixel cells
        _ => 200.0,                 // Large worlds: 200 pixel cells
    };

    let grid_width = (dimensions.width_pixels / cell_size).max(50.0) as usize;
    let grid_height = (dimensions.height_pixels / cell_size).max(50.0) as usize;

    println!("  Creating erosion heightmap: {}x{} grid (cell size: {})",
        grid_width, grid_height, cell_size);

    let mut heightmap = HeightMap::new(grid_width, grid_height, cell_size);

    // Fill heightmap with province elevations
    for province in provinces.iter() {
        let x = ((province.position.x - dimensions.bounds.x_min) / cell_size) as usize;
        let y = ((province.position.y - dimensions.bounds.y_min) / cell_size) as usize;

        if x < grid_width && y < grid_height {
            heightmap.set(x, y, province.elevation.value());
        }
    }

    let mut erosion = ErosionSystem::new(heightmap, rng);
    erosion.erode(iterations);
    let eroded = erosion.get_heightmap();

    // Apply eroded elevations back to provinces
    for province in provinces.iter_mut() {
        let pos = Vec2::new(
            province.position.x - dimensions.bounds.x_min,
            province.position.y - dimensions.bounds.y_min,
        );
        let new_elevation = eroded.get_interpolated(pos);
        province.elevation = crate::components::Elevation::new(new_elevation);
    }

    println!("  Erosion complete - terrain now has realistic valleys and drainage");
}