//! Overlay color caching system with zero-copy Arc architecture
//!
//! This module provides lazy-loaded overlay colors with Arc-based caching for
//! zero-copy performance. Uses ECS queries for province data and ownership.

use super::types::MapMode;
use crate::math::VERTICES_PER_HEX;
use crate::nations::Nation;
use crate::relationships::Controls;
use crate::world::{ProvinceData, ProvinceEntityOrder, WorldColors};
use bevy::log::{debug, info, warn};
use bevy::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Zero-copy overlay color cache using Arc for instant switching
#[derive(Resource)]
pub struct CachedOverlayColors {
    /// Currently active overlay colors (Arc for zero-copy)
    pub current: Arc<Vec<[f32; 4]>>,
    /// Current overlay type for tracking
    pub current_type: MapMode,
    /// LRU cache with Arc for zero-copy retrieval
    pub cache: HashMap<MapMode, Arc<Vec<[f32; 4]>>>,
    /// Maximum cache entries
    pub max_cache_size: usize,
}

impl Default for CachedOverlayColors {
    fn default() -> Self {
        Self {
            current: Arc::new(Vec::new()),
            current_type: MapMode::Terrain,
            cache: HashMap::with_capacity(8),
            max_cache_size: 10,
        }
    }
}

/// Province data extracted for parallel processing (Send + Sync safe)
#[derive(Clone)]
struct ProvinceRenderData {
    index: usize,
    terrain: crate::world::TerrainType,
    elevation: f32,
    position: Vec2,
    population: u32,
    agriculture: f32,
    // Mineral abundances
    iron: u8,
    copper: u8,
    tin: u8,
    gold: u8,
    coal: u8,
    stone: u8,
    gems: u8,
}

impl CachedOverlayColors {
    /// Get colors with ECS queries for nation ownership
    pub fn get_or_calculate_ecs(
        &mut self,
        mode: MapMode,
        province_entity_order: &ProvinceEntityOrder,
        province_data_query: &Query<&ProvinceData>,
        world_seed: u32,
        nations_query: &Query<(Entity, &Nation)>,
        controls_query: &Query<&Controls>,
        climate_storage: Option<&crate::world::terrain::ClimateStorage>,
        infrastructure_storage: Option<&crate::world::InfrastructureStorage>,
    ) -> Arc<Vec<[f32; 4]>> {
        // If requesting current overlay, return Arc clone (just increments refcount)
        if mode == self.current_type {
            return Arc::clone(&self.current);
        }

        // Check cache
        let use_cache = match mode {
            MapMode::Political => self.cache.contains_key(&mode),
            MapMode::Infrastructure if infrastructure_storage.is_some() => false,
            _ => self.cache.contains_key(&mode),
        };

        if use_cache {
            if let Some(cached) = self.cache.get(&mode) {
                let old_current = std::mem::replace(&mut self.current, Arc::clone(cached));
                if self.current_type != mode {
                    self.cache.insert(self.current_type, old_current);
                }
                self.current_type = mode;
                return Arc::clone(&self.current);
            }
        }

        info!("Calculating overlay colors for: {}", mode.display_name());
        let start = std::time::Instant::now();

        // Calculate colors
        let colors = Arc::new(self.calculate_colors_ecs(
            mode,
            province_entity_order,
            province_data_query,
            world_seed,
            nations_query,
            controls_query,
            climate_storage,
            infrastructure_storage,
        ));

        debug!(
            "Calculated {} overlay in {:.2}ms ({} vertices)",
            mode.display_name(),
            start.elapsed().as_secs_f32() * 1000.0,
            colors.len(),
        );

        // Manage cache size
        if self.cache.len() >= self.max_cache_size {
            if let Some(key_to_remove) = self
                .cache
                .keys()
                .find(|&&k| k != self.current_type)
                .cloned()
            {
                self.cache.remove(&key_to_remove);
            }
        }

        if !self.current.is_empty() {
            self.cache.insert(self.current_type, Arc::clone(&self.current));
        }

        self.current = Arc::clone(&colors);
        self.current_type = mode;

        colors
    }

    /// Calculate colors using ECS data
    fn calculate_colors_ecs(
        &self,
        mode: MapMode,
        province_entity_order: &ProvinceEntityOrder,
        province_data_query: &Query<&ProvinceData>,
        world_seed: u32,
        nations_query: &Query<(Entity, &Nation)>,
        controls_query: &Query<&Controls>,
        climate_storage: Option<&crate::world::terrain::ClimateStorage>,
        infrastructure_storage: Option<&crate::world::InfrastructureStorage>,
    ) -> Vec<[f32; 4]> {
        let world_colors = WorldColors::new(world_seed);
        let province_count = province_entity_order.len();

        // Build nation colors map for political/terrain modes
        // Uses Controls relationship - O(nations) instead of O(provinces)
        let nation_colors_map: HashMap<usize, Color> = if mode == MapMode::Political || mode == MapMode::Terrain {
            let mut map = HashMap::new();
            for (nation_entity, nation) in nations_query.iter() {
                if let Ok(controls) = controls_query.get(nation_entity) {
                    for &province_entity in controls.provinces() {
                        // Find the index of this province entity
                        if let Some(idx) = province_entity_order.index_of(province_entity) {
                            map.insert(idx, nation.color);
                        }
                    }
                }
            }
            map
        } else {
            HashMap::new()
        };

        // Extract province data for parallel processing
        let province_render_data: Vec<ProvinceRenderData> = province_entity_order
            .entities
            .iter()
            .enumerate()
            .filter_map(|(idx, &entity)| {
                province_data_query.get(entity).ok().map(|data| ProvinceRenderData {
                    index: idx,
                    terrain: data.terrain,
                    elevation: data.elevation.value(),
                    position: data.position,
                    population: data.population,
                    agriculture: data.agriculture.value(),
                    iron: data.iron.value(),
                    copper: data.copper.value(),
                    tin: data.tin.value(),
                    gold: data.gold.value(),
                    coal: data.coal.value(),
                    stone: data.stone.value(),
                    gems: data.gems.value(),
                })
            })
            .collect();

        if province_render_data.len() != province_count {
            warn!(
                "Province count mismatch: expected {}, got {}",
                province_count,
                province_render_data.len()
            );
        }

        // Calculate optimal chunk size
        let num_threads = rayon::current_num_threads();
        let chunk_size = (province_count / num_threads).max(1000).min(50000);

        // Process in parallel
        let chunk_colors: Vec<Vec<[f32; 4]>> = province_render_data
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut chunk_colors = Vec::with_capacity(chunk.len() * VERTICES_PER_HEX);

                for data in chunk {
                    let color = match mode {
                        MapMode::Political => {
                            nation_colors_map
                                .get(&data.index)
                                .copied()
                                .unwrap_or_else(|| {
                                    if data.terrain == crate::world::TerrainType::Ocean {
                                        world_colors.terrain(data.terrain, data.elevation, data.position)
                                    } else {
                                        Color::srgb(0.15, 0.15, 0.15)
                                    }
                                })
                        }
                        MapMode::Terrain => {
                            let base = world_colors.terrain(data.terrain, data.elevation, data.position);
                            if let Some(&nation_color) = nation_colors_map.get(&data.index) {
                                let nation_rgba = nation_color.to_linear().to_f32_array();
                                let terrain_rgba = base.to_linear().to_f32_array();
                                let alpha = 0.15;
                                Color::srgba(
                                    terrain_rgba[0] * (1.0 - alpha) + nation_rgba[0] * alpha,
                                    terrain_rgba[1] * (1.0 - alpha) + nation_rgba[1] * alpha,
                                    terrain_rgba[2] * (1.0 - alpha) + nation_rgba[2] * alpha,
                                    1.0,
                                )
                            } else {
                                base
                            }
                        }
                        MapMode::Climate => {
                            world_colors.terrain(data.terrain, data.elevation, data.position)
                        }
                        MapMode::Population => {
                            let pop_normalized = (data.population as f32 / 100000.0).min(1.0);
                            Color::srgb(pop_normalized, pop_normalized * 0.5, 0.0)
                        }
                        MapMode::Agriculture => {
                            let agri_normalized = data.agriculture.min(3.0) / 3.0;
                            Color::srgb(0.0, agri_normalized, 0.0)
                        }
                        MapMode::Infrastructure => {
                            world_colors.terrain(data.terrain, data.elevation, data.position)
                        }
                        MapMode::Minerals => {
                            let total = data.iron as u32 + data.copper as u32 + data.tin as u32
                                + data.gold as u32 + data.coal as u32 + data.stone as u32 + data.gems as u32;
                            world_colors.richness(total as f32 / 100.0)
                        }
                    };

                    let color_array = color.to_linear().to_f32_array();
                    for _ in 0..VERTICES_PER_HEX {
                        chunk_colors.push(color_array);
                    }
                }
                chunk_colors
            })
            .collect();

        // Combine chunks
        let total_size: usize = chunk_colors.iter().map(|c| c.len()).sum();
        let mut colors = Vec::with_capacity(total_size);
        for chunk in chunk_colors {
            colors.extend(chunk);
        }

        colors
    }

    /// Clear cache to free memory
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        debug!("Cleared overlay color cache");
    }

    /// Get memory usage in MB
    pub fn memory_usage_mb(&self) -> f32 {
        const BYTES_PER_MB: f32 = 1024.0 * 1024.0;

        let current_size = if Arc::strong_count(&self.current) == 1 {
            self.current.len() * std::mem::size_of::<[f32; 4]>()
        } else {
            0
        };

        let cache_size: usize = self
            .cache
            .values()
            .map(|v| {
                if Arc::strong_count(v) == 1 {
                    v.len() * std::mem::size_of::<[f32; 4]>()
                } else {
                    0
                }
            })
            .sum();

        (current_size + cache_size) as f32 / BYTES_PER_MB
    }

    /// Get diagnostics about Arc reference counts
    pub fn arc_diagnostics(&self) {
        debug!(
            "Arc diagnostics - Current: {} refs, Cache entries: {}",
            Arc::strong_count(&self.current),
            self.cache.len()
        );
    }
}
