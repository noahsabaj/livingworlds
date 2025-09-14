//! Overlay color caching system
//!
//! This module provides lazy-loaded overlay colors with LRU cache for memory efficiency.
//! Instead of pre-calculating all overlays (~1.2GB), we load on-demand (~135MB each).

use super::types::ResourceOverlay;
use crate::components::MineralType;
use crate::math::VERTICES_PER_HEX;
use crate::world::minerals::calculate_total_richness;
use crate::world::{Province, ProvinceStorage, StoneAbundance, TerrainType, WorldColors};
use bevy::prelude::*;
use std::collections::HashMap;

/// Lazy-loaded overlay colors with LRU cache for memory efficiency
/// Instead of pre-calculating all 9 overlays (~1.2GB), we load on-demand (~135MB each)
#[derive(Resource)]
pub struct CachedOverlayColors {
    /// Currently active overlay colors (always loaded)
    pub current: Vec<[f32; 4]>,
    /// Current overlay type for tracking
    pub current_type: ResourceOverlay,
    /// LRU cache: stores recently used overlays (max 2 entries)
    /// This allows fast switching between recent overlays
    pub cache: HashMap<ResourceOverlay, Vec<[f32; 4]>>,
    /// Maximum cache entries (default: 2 = current + 1 previous)
    pub max_cache_size: usize,
}

impl Default for CachedOverlayColors {
    fn default() -> Self {
        Self {
            current: Vec::new(),
            current_type: ResourceOverlay::None,
            cache: HashMap::new(),
            max_cache_size: 2, // Keep current + 1 previous overlay
        }
    }
}

impl CachedOverlayColors {
    /// Get colors for an overlay, calculating if not cached
    pub fn get_or_calculate(
        &mut self,
        overlay: ResourceOverlay,
        province_storage: &ProvinceStorage,
    ) -> &Vec<[f32; 4]> {
        // If requesting current overlay, return it
        if overlay == self.current_type {
            return &self.current;
        }

        if let Some(cached) = self.cache.get(&overlay) {
            // Move to current
            self.current = cached.clone();
            self.current_type = overlay;
            return &self.current;
        }

        info!("Calculating overlay colors for: {}", overlay.display_name());
        let start = std::time::Instant::now();

        // Generate colors for all vertices (7 colors per province)
        let colors: Vec<[f32; 4]> = province_storage
            .provinces
            .iter() // Use sequential iteration since order matters
            .flat_map(|province| {
                // Create WorldColors instance for color calculations
                let world_colors = WorldColors::new(0); // seed doesn't matter for these calculations

                let province_color = match overlay {
                    ResourceOverlay::None => world_colors.terrain(
                        province.terrain,
                        province.elevation.value(),
                        Vec2::ZERO,
                    ),
                    ResourceOverlay::Mineral(mineral_type) => {
                        let color =
                            if let Some(abundance) = province.get_mineral_abundance(mineral_type) {
                                if mineral_type == MineralType::Stone {
                                    world_colors.stone_abundance(StoneAbundance::new(abundance))
                                } else {
                                    world_colors.mineral_abundance(abundance)
                                }
                            } else {
                                world_colors.terrain(
                                    province.terrain,
                                    province.elevation.value(),
                                    Vec2::ZERO,
                                )
                            };
                        // Helper for ocean default
                        if province.terrain == TerrainType::Ocean {
                            world_colors.terrain(
                                province.terrain,
                                province.elevation.value(),
                                Vec2::ZERO,
                            )
                        } else {
                            color
                        }
                    }
                    ResourceOverlay::AllMinerals => {
                        let color = {
                            let total_richness = calculate_total_richness(province);
                            world_colors.richness(total_richness)
                        };
                        if province.terrain == TerrainType::Ocean {
                            world_colors.terrain(
                                province.terrain,
                                province.elevation.value(),
                                Vec2::ZERO,
                            )
                        } else {
                            color
                        }
                    }
                };

                let linear = province_color.to_linear();
                let color_array = [linear.red, linear.green, linear.blue, linear.alpha];
                vec![color_array; VERTICES_PER_HEX]
            })
            .collect();

        debug!("Calculated {} overlay in {:.2}ms (buffer size: {} colors for {} provinces = {} vertices)",
               overlay.display_name(),
               start.elapsed().as_secs_f32() * 1000.0,
               colors.len(),
               province_storage.provinces.len(),
               province_storage.provinces.len() * VERTICES_PER_HEX);

        // Store in cache and manage size
        if self.cache.len() >= self.max_cache_size {
            // Remove least recently used (not current)
            if let Some(key_to_remove) = self
                .cache
                .keys()
                .find(|&&k| k != self.current_type)
                .cloned()
            {
                self.cache.remove(&key_to_remove);
            }
        }

        // Add previous current to cache if it's not None
        if self.current_type != ResourceOverlay::None && !self.current.is_empty() {
            self.cache.insert(self.current_type, self.current.clone());
        }

        self.current = colors;
        self.current_type = overlay;

        &self.current
    }

    /// Clear cache to free memory
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        debug!("Cleared overlay color cache");
    }

    /// Get memory usage in MB
    pub fn memory_usage_mb(&self) -> f32 {
        const BYTES_PER_MB: f32 = 1024.0 * 1024.0;
        let current_size = self.current.len() * std::mem::size_of::<[f32; 4]>();
        let cache_size: usize = self
            .cache
            .values()
            .map(|v| v.len() * std::mem::size_of::<[f32; 4]>())
            .sum();
        (current_size + cache_size) as f32 / BYTES_PER_MB
    }
}
