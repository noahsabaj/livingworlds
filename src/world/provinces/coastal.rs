//! Coastal province detection and caching
//!
//! This module identifies provinces adjacent to water bodies (ocean, rivers, etc.)
//! and maintains a cache for efficient coastal lookups.

use bevy::prelude::*;
use std::collections::HashSet;
use crate::world::{ProvinceStorage, ProvinceId};

/// Resource tracking coastal provinces
#[derive(Resource, Default)]
pub struct CoastalProvinceCache {
    /// Set of province IDs that are coastal (adjacent to Ocean/water)
    pub coastal_provinces: HashSet<ProvinceId>,
    /// Whether cache has been built
    pub initialized: bool,
}

impl CoastalProvinceCache {
    /// Build coastal province cache from province storage
    pub fn build(&mut self, province_storage: &ProvinceStorage) {
        self.coastal_provinces.clear();

        for (idx, province) in province_storage.provinces.iter().enumerate() {
            // Skip if already water
            if province.terrain.properties().is_water {
                continue;
            }

            // Check if any neighbor is water
            for neighbor_opt in &province.neighbors {
                if let Some(neighbor_id) = neighbor_opt {
                    if let Some(neighbor) = province_storage.provinces.get(neighbor_id.value() as usize) {
                        if neighbor.terrain.properties().is_water {
                            self.coastal_provinces.insert(ProvinceId::new(idx as u32));
                            break;
                        }
                    }
                }
            }
        }

        self.initialized = true;
        info!("Built coastal cache: {} coastal provinces", self.coastal_provinces.len());
    }

    /// Check if a province is coastal
    pub fn is_coastal(&self, province_id: ProvinceId) -> bool {
        self.coastal_provinces.contains(&province_id)
    }
}

/// Initialize coastal cache on world generation completion
pub fn initialize_coastal_cache(
    mut coastal_cache: ResMut<CoastalProvinceCache>,
    province_storage: Res<ProvinceStorage>,
) {
    if !coastal_cache.initialized && !province_storage.provinces.is_empty() {
        coastal_cache.build(&province_storage);
    }
}
