//! Territory assignment and building
//!
//! Handles assigning provinces to nations using growth algorithms
//! and building Territory entities from contiguous province regions.

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::world::{Province, TerrainType};
use super::super::types::*;

/// Assign territory to nations using parallel growth algorithm with atomic operations
///
/// Uses a Dijkstra-based expansion from capitals with terrain costs for organic borders.
pub fn assign_territory_to_nations(
    nations: &mut Vec<(NationId, Nation)>,
    provinces: &mut [Province],
    density: crate::nations::NationDensity,
) -> HashMap<NationId, Vec<u32>> {
    if nations.is_empty() {
        return HashMap::new();
    }

    // Determine growth limit based on density
    let avg_provinces_per_nation = (provinces.len() / nations.len()).max(1);
    let growth_limit = match density {
        crate::nations::NationDensity::Sparse => avg_provinces_per_nation * 2,
        crate::nations::NationDensity::Balanced => avg_provinces_per_nation,
        crate::nations::NationDensity::Fragmented => avg_provinces_per_nation / 2,
    };

    // Use atomic operations for thread-safe ownership tracking
    let atomic_owners: Vec<AtomicU32> = (0..provinces.len())
        .map(|_| AtomicU32::new(0)) // 0 = unclaimed
        .collect();

    // Initialize capitals with atomic ownership
    for (nation_idx, (_, nation)) in nations.iter().enumerate() {
        let capital_idx = nation.capital_province as usize;
        atomic_owners[capital_idx].store((nation_idx as u32) + 1, Ordering::SeqCst);
    }

    // Initialize Perlin noise for organic borders
    let perlin = Perlin::new(12345); // Fixed seed for consistent noise field

    // Parallel territory expansion for all nations using Dijkstra's algorithm
    let nation_claims: Vec<Vec<u32>> = nations
        .par_iter()
        .enumerate()
        .map(|(nation_idx, (_, nation))| {
            let mut claimed_provinces = vec![nation.capital_province];

            // Use BinaryHeap for Dijkstra-based expansion (min-heap via Reverse)
            // Tuple: (Reverse(accumulated_cost), province_id)
            let mut frontier = std::collections::BinaryHeap::new();
            frontier.push((std::cmp::Reverse(0u32), nation.capital_province));

            let nation_id_atomic = (nation_idx as u32) + 1; // +1 because 0 means unclaimed

            while claimed_provinces.len() < growth_limit {
                let (std::cmp::Reverse(current_cost), current_province_id) = match frontier.pop() {
                    Some(val) => val,
                    None => break,
                };

                let neighbors = provinces[current_province_id as usize].neighbors;

                for neighbor_opt in neighbors.iter() {
                    if let Some(neighbor_id) = neighbor_opt {
                        let neighbor_idx = neighbor_id.0 as usize;

                        // Try to claim this neighbor if it's unclaimed
                        // We use compare_exchange to ensure atomic claiming
                        if neighbor_idx < provinces.len()
                            && atomic_owners[neighbor_idx]
                                .compare_exchange(
                                    0,
                                    nation_id_atomic,
                                    Ordering::SeqCst,
                                    Ordering::SeqCst,
                                )
                                .is_ok()
                        {
                            claimed_provinces.push(neighbor_id.0);

                            // Calculate movement cost based on terrain to create organic borders
                            let neighbor_province = &provinces[neighbor_idx];
                            let mut terrain_cost = 10; // Base cost

                            // Elevation costs
                            if neighbor_province.elevation.is_mountain() {
                                terrain_cost += 30;
                            } else if neighbor_province.elevation.value() > 0.5 {
                                terrain_cost += 15; // Hills
                            }

                            // Biome costs
                            match neighbor_province.terrain {
                                TerrainType::Wetlands | TerrainType::Mangrove => terrain_cost += 20,
                                TerrainType::SubtropicalDesert | TerrainType::ColdDesert | TerrainType::TropicalDesert | TerrainType::PolarDesert => terrain_cost += 12,
                                TerrainType::TropicalRainforest | TerrainType::TemperateRainforest => terrain_cost += 15,
                                TerrainType::Taiga | TerrainType::BorealForest | TerrainType::TemperateDeciduousForest | TerrainType::TropicalSeasonalForest | TerrainType::MediterraneanForest => terrain_cost += 12,
                                _ => {}, // Plains/Grasslands/Savanna are base cost
                            };

                            // River crossing penalty
                            if neighbor_province.terrain == TerrainType::River {
                                terrain_cost += 50;
                            }

                            // Use Perlin noise for organic borders
                            let scale = 0.05;
                            let noise_val = perlin.get([
                                neighbor_province.position.x as f64 * scale,
                                neighbor_province.position.y as f64 * scale
                            ]);

                            // Map noise [-1, 1] to cost modifier [0, 20]
                            let noise_cost = ((noise_val + 1.0) * 10.0) as u32;
                            let new_cost = current_cost + terrain_cost + noise_cost;

                            frontier.push((std::cmp::Reverse(new_cost), neighbor_id.0));
                        }
                    }
                }
            }

            claimed_provinces
        })
        .collect();

    // Build return mapping: NationId -> Vec<province_id>
    let mut ownership_map = HashMap::new();
    for (idx, (nation_id, _)) in nations.iter().enumerate() {
        ownership_map.insert(*nation_id, nation_claims[idx].clone());
    }

    // Log statistics
    let owned_provinces: usize = nation_claims.iter().map(|v| v.len()).sum();

    info!(
        "Parallel territory assignment complete. {} provinces claimed by {} nations (avg: {})",
        owned_provinces,
        nations.len(),
        owned_provinces / nations.len().max(1)
    );

    debug!(
        "Territory distribution: min={}, max={}, median={}",
        nation_claims.iter().map(|v| v.len()).min().unwrap_or(0),
        nation_claims.iter().map(|v| v.len()).max().unwrap_or(0),
        {
            let nation_count = nation_claims.len();
            let mut sizes: Vec<_> = Vec::with_capacity(nation_count);
            sizes.extend(nation_claims.iter().map(|v| v.len()));
            sizes.sort();
            sizes.get(sizes.len() / 2).copied().unwrap_or(0)
        }
    );

    ownership_map
}

/// Build Territory entities from contiguous province regions (parallel version)
///
/// Returns a map of nation_entity -> vec of territories for that nation.
/// Uses BFS to find contiguous regions and O(1) HashMap lookups to avoid
/// quadratic complexity.
pub fn build_territories_from_provinces(
    provinces: &[Province],
) -> HashMap<Entity, Vec<Territory>> {
    // Parallel grouping: Group provinces by owner nation first
    let provinces_by_nation: HashMap<Entity, Vec<usize>> = provinces
        .par_iter()
        .enumerate()
        .filter_map(|(idx, province)| province.owner_entity.map(|owner| (owner, idx)))
        .fold(HashMap::new, |mut acc, (nation_entity, province_idx)| {
            acc.entry(nation_entity)
                .or_insert_with(Vec::new)
                .push(province_idx);
            acc
        })
        .reduce(HashMap::new, |mut acc1, acc2| {
            for (nation_id, mut indices) in acc2 {
                acc1.entry(nation_id)
                    .or_insert_with(Vec::new)
                    .append(&mut indices);
            }
            acc1
        });

    // Build HashMap for O(1) province lookups
    let province_lookup: HashMap<u32, &Province> = provinces
        .iter()
        .map(|p| (p.id.value(), p))
        .collect();
    let province_lookup = Arc::new(province_lookup);

    // Parallel territory building: Process each nation's provinces in parallel
    let nation_territories: HashMap<Entity, Vec<Territory>> = provinces_by_nation
        .par_iter()
        .map(|(&nation_entity, province_indices)| {
            let province_map = Arc::clone(&province_lookup);
            let mut territories = Vec::new();
            let mut visited = HashSet::with_capacity(province_indices.len());

            // For this nation, find all contiguous territories
            for &province_idx in province_indices {
                let province = &provinces[province_idx];

                if visited.contains(&province.id.value()) {
                    continue;
                }

                // Start a new territory from this province
                let mut territory_provinces = HashSet::new();
                let mut frontier = VecDeque::new();
                frontier.push_back(province.id.value());

                let mut sum_x = 0.0;
                let mut sum_y = 0.0;
                let mut count = 0;

                // BFS to find all contiguous provinces owned by same nation
                while let Some(current_id) = frontier.pop_front() {
                    if visited.contains(&current_id) {
                        continue;
                    }

                    // O(1) HashMap lookup
                    if let Some(&current_province) = province_map.get(&current_id) {
                        if current_province.owner_entity == Some(nation_entity) {
                            visited.insert(current_id);
                            territory_provinces.insert(current_id);

                            sum_x += current_province.position.x;
                            sum_y += current_province.position.y;
                            count += 1;

                            // Add unvisited neighbors
                            for neighbor_opt in current_province.neighbors {
                                if let Some(neighbor_id) = neighbor_opt {
                                    if !visited.contains(&neighbor_id.value()) {
                                        frontier.push_back(neighbor_id.value());
                                    }
                                }
                            }
                        }
                    }
                }

                // Create territory if we found provinces
                if !territory_provinces.is_empty() {
                    let center = Vec2::new(sum_x / count as f32, sum_y / count as f32);

                    let territory = Territory {
                        provinces: territory_provinces,
                        center,
                        is_core: true, // All territories are core at world generation
                    };

                    territories.push(territory);
                }
            }

            (nation_entity, territories)
        })
        .collect();

    info!("Built territories for {} nations", nation_territories.len());

    for (nation_entity, territories) in &nation_territories {
        let total_provinces: usize = territories.iter().map(|t| t.provinces.len()).sum();
        info!(
            "Nation entity {:?} has {} territories with {} total provinces",
            nation_entity,
            territories.len(),
            total_provinces
        );
    }

    nation_territories
}
