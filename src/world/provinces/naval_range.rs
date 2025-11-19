//! Naval range calculation for cross-water threat projection
//!
//! This module calculates which provinces are within naval strike range
//! by traversing water hexes via BFS (breadth-first search).

use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};
use crate::world::{ProvinceStorage, ProvinceId};
use super::CoastalProvinceCache;

/// Maximum naval range in hexes (provinces that can project naval power)
pub const NAVAL_RANGE_HEXES: u32 = 3;

/// Naval range neighbor detection
pub struct NavalRangeCalculator;

impl NavalRangeCalculator {
    /// Get all provinces within naval range of a nation's coastal provinces
    ///
    /// Returns set of province IDs reachable via water within NAVAL_RANGE_HEXES.
    /// This enables nations with coastal access to threaten provinces across
    /// narrow seas (e.g., across straits, island chains).
    ///
    /// # Algorithm
    /// 1. Start from all coastal provinces of the nation
    /// 2. BFS through water provinces up to NAVAL_RANGE_HEXES distance
    /// 3. Mark all land provinces encountered as reachable
    /// 4. Return set of reachable provinces
    pub fn get_naval_range_provinces(
        nation_provinces: &HashSet<u32>,
        coastal_cache: &CoastalProvinceCache,
        province_storage: &ProvinceStorage,
    ) -> HashSet<ProvinceId> {
        let mut reachable = HashSet::new();

        // Start from nation's coastal provinces
        let coastal_start: Vec<ProvinceId> = nation_provinces
            .iter()
            .map(|&id| ProvinceId::new(id))
            .filter(|&id| coastal_cache.is_coastal(id))
            .collect();

        if coastal_start.is_empty() {
            return reachable; // Landlocked nation - no naval range
        }

        // BFS from coastal provinces, traversing water hexes
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        for coastal_id in coastal_start {
            queue.push_back((coastal_id, 0_u32)); // (province_id, distance)
            visited.insert(coastal_id);
        }

        while let Some((current_id, distance)) = queue.pop_front() {
            if distance > NAVAL_RANGE_HEXES {
                continue;
            }

            let current_idx = current_id.value() as usize;
            let Some(current_province) = province_storage.provinces.get(current_idx) else {
                continue;
            };

            // Add all neighbors within range
            for neighbor_opt in &current_province.neighbors {
                if let Some(&neighbor_id) = neighbor_opt.as_ref() {
                    let neighbor_idx = neighbor_id.value() as usize;
                    let Some(neighbor_province) = province_storage.provinces.get(neighbor_idx) else {
                        continue;
                    };

                    if visited.contains(&neighbor_id) {
                        continue;
                    }

                    visited.insert(neighbor_id);

                    // If water, continue BFS
                    if neighbor_province.terrain.properties().is_water {
                        queue.push_back((neighbor_id, distance + 1));
                    } else {
                        // Land province within range - mark as reachable
                        reachable.insert(neighbor_id);
                    }
                }
            }
        }

        reachable
    }
}
