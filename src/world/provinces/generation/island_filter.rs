//! Island filtering and cleanup
//!
//! Removes small islands to prevent "spaghetti island" formations and ensure
//! cleaner, more realistic landmasses.

use std::collections::HashMap;

use crate::world::provinces::{Province, Elevation};
use crate::world::terrain::TerrainType;

/// Minimum island size to keep (in provinces)
/// Increased to 50 to remove smallest fragments but keep archipelagos
const MIN_ISLAND_SIZE: usize = 50;

/// Filters out small islands from the province map
pub struct IslandFilter {
    min_size: usize,
}

impl IslandFilter {
    pub fn new() -> Self {
        Self {
            min_size: MIN_ISLAND_SIZE,
        }
    }

    pub fn with_min_size(mut self, size: usize) -> Self {
        self.min_size = size;
        self
    }

    /// Filter small islands from provinces using flood fill algorithm
    pub fn filter(&self, provinces: &mut Vec<Province>) -> usize {
        // Build a quick lookup map for province indices
        let province_map: HashMap<u32, usize> = provinces
            .iter()
            .enumerate()
            .map(|(idx, p)| (p.id.value(), idx))
            .collect();

        // Track which provinces have been visited
        let mut visited = vec![false; provinces.len()];
        let mut islands_to_remove = Vec::new();

        // Find all connected land components
        for (idx, province) in provinces.iter().enumerate() {
            // Skip if already visited or if it's ocean
            if visited[idx] || province.terrain == TerrainType::Ocean {
                continue;
            }

            // Flood fill to find connected component size
            let component = self.find_connected_component(
                idx,
                provinces,
                &province_map,
                &mut visited,
            );

            // If component is too small, mark for removal
            if component.len() < self.min_size {
                islands_to_remove.extend(component);
            }
        }

        // Convert small islands to ocean
        let removed_count = islands_to_remove.len();
        for idx in islands_to_remove {
            provinces[idx].terrain = TerrainType::Ocean;
            provinces[idx].elevation = Elevation::new(0.1); // Shallow ocean
        }

        removed_count
    }

    /// Find a connected component of land provinces using flood fill
    fn find_connected_component(
        &self,
        start_idx: usize,
        provinces: &[Province],
        province_map: &HashMap<u32, usize>,
        visited: &mut [bool],
    ) -> Vec<usize> {
        let mut component = Vec::new();
        let mut stack = vec![start_idx];

        while let Some(current_idx) = stack.pop() {
            if visited[current_idx] {
                continue;
            }

            visited[current_idx] = true;
            component.push(current_idx);

            // Check all neighbors
            let current_province = &provinces[current_idx];
            for neighbor_id_opt in &current_province.neighbors {
                if let Some(neighbor_id) = neighbor_id_opt {
                    if let Some(&neighbor_idx) = province_map.get(&neighbor_id.value()) {
                        // Only add land neighbors that haven't been visited
                        if !visited[neighbor_idx]
                            && provinces[neighbor_idx].terrain != TerrainType::Ocean
                        {
                            stack.push(neighbor_idx);
                        }
                    }
                }
            }
        }

        component
    }
}