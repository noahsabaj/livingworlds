//! Capital province selection
//!
//! Handles selecting suitable provinces to be nation capitals using parallel evaluation
//! and spatial distribution algorithms.

use bevy::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng as StdRng;
use rayon::prelude::*;

use crate::world::{Province, TerrainType};

/// Select suitable provinces to be nation capitals using parallel evaluation
///
/// Uses a spatial distribution algorithm to ensure capitals are well-spaced across the map.
pub fn select_capital_provinces(
    provinces: &[Province],
    nation_count: u32,
    rng: &mut StdRng,
) -> Vec<usize> {
    // Parallel filter to find all land provinces that could be capitals
    let suitable_provinces: Vec<usize> = provinces
        .par_iter()
        .enumerate()
        .filter_map(|(idx, p)| {
            if !matches!(
                p.terrain,
                TerrainType::Ocean | TerrainType::River | TerrainType::Alpine
            ) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    if suitable_provinces.len() <= nation_count as usize {
        return suitable_provinces;
    }

    // Calculate minimum distance for good spacing
    let min_distance_squared = calculate_min_capital_distance(provinces.len(), nation_count);

    // Use spatial partitioning with parallel evaluation for capital selection
    let mut selected_capitals = Vec::new();
    let mut remaining_candidates = suitable_provinces.clone();

    for _ in 0..nation_count {
        if remaining_candidates.is_empty() {
            break;
        }

        // Parallel evaluate all remaining candidates for distance constraints
        let scores: Vec<(usize, f32)> = remaining_candidates
            .par_iter()
            .map(|&candidate_idx| {
                let position = provinces[candidate_idx].position;

                // Calculate minimum distance to any existing capital
                let min_dist = if selected_capitals.is_empty() {
                    f32::MAX
                } else {
                    selected_capitals
                        .iter()
                        .map(|&other_idx: &usize| {
                            position.distance_squared(provinces[other_idx].position)
                        })
                        .min_by(|a: &f32, b: &f32| a.total_cmp(b))
                        .unwrap_or(f32::MAX)
                };

                (candidate_idx, min_dist)
            })
            .collect();

        // Find candidates that meet distance requirements
        let valid_candidates: Vec<usize> = scores
            .into_iter()
            .filter(|&(_, dist)| dist >= min_distance_squared || selected_capitals.is_empty())
            .map(|(idx, _)| idx)
            .collect();

        // Select a random valid candidate or fallback to any candidate
        let selected = if !valid_candidates.is_empty() {
            let idx = rng.gen_range(0..valid_candidates.len());
            valid_candidates[idx]
        } else if !remaining_candidates.is_empty() {
            // Fallback: pick the candidate with maximum minimum distance
            let best = remaining_candidates
                .par_iter()
                .map(|&idx| {
                    let pos = provinces[idx].position;
                    let min_dist = selected_capitals
                        .iter()
                        .map(|&other| provinces[other].position.distance_squared(pos))
                        .min_by(|a, b| a.total_cmp(b))
                        .unwrap_or(0.0);
                    (idx, min_dist)
                })
                .max_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(idx, _)| idx)
                .unwrap_or(remaining_candidates[rng.gen_range(0..remaining_candidates.len())]);
            best
        } else {
            break;
        };

        selected_capitals.push(selected);
        remaining_candidates.retain(|&x| x != selected);
    }

    selected_capitals
}

/// Calculate minimum distance between capitals based on world size
///
/// Returns the squared distance threshold for capital spacing.
pub fn calculate_min_capital_distance(province_count: usize, nation_count: u32) -> f32 {
    let world_area = province_count as f32;
    let area_per_nation = world_area / nation_count as f32;
    let radius = (area_per_nation / std::f32::consts::PI).sqrt();
    radius * radius * 0.5 // Squared distance, with some overlap allowed
}
