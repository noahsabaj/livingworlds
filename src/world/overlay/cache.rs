//! Overlay color caching system with zero-copy Arc architecture
//!
//! This module provides lazy-loaded overlay colors with Arc-based caching for
//! zero-copy performance. Instead of cloning massive vertex buffers (336MB on Large worlds),
//! we use Arc reference counting for instant overlay switching.

use super::types::MapMode;
use crate::math::VERTICES_PER_HEX;
use crate::nations::NationColorRegistry;
use crate::world::minerals::calculate_total_richness;
use crate::world::{Province, ProvinceStorage, WorldColors};
use bevy::log::{debug, info};
use bevy::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Zero-copy overlay color cache using Arc for instant switching
/// Eliminates the 1GB of memory operations that occurred with cloning
#[derive(Resource)]
pub struct CachedOverlayColors {
    /// Currently active overlay colors (Arc for zero-copy)
    pub current: Arc<Vec<[f32; 4]>>,
    /// Current overlay type for tracking
    pub current_type: MapMode,
    /// LRU cache with Arc for zero-copy retrieval
    pub cache: HashMap<MapMode, Arc<Vec<[f32; 4]>>>,
    /// Maximum cache entries (increased to 10 for smoother switching)
    pub max_cache_size: usize,
}

impl Default for CachedOverlayColors {
    fn default() -> Self {
        Self {
            current: Arc::new(Vec::new()),
            current_type: MapMode::Terrain,
            cache: HashMap::with_capacity(8), // Pre-allocate for common overlay types
            max_cache_size: 10,               // Increased to keep more overlays in memory
        }
    }
}

impl CachedOverlayColors {
    /// Get colors for an overlay with zero-copy Arc retrieval
    /// Returns Arc<Vec> for instant access without cloning
    pub fn get_or_calculate(
        &mut self,
        mode: MapMode,
        province_storage: &ProvinceStorage,
        world_seed: u32,
    ) -> Arc<Vec<[f32; 4]>> {
        self.get_or_calculate_with_nations(mode, province_storage, world_seed, None, None, None)
    }

    /// Get colors with optional nation color registry for political mode
    pub fn get_or_calculate_with_nations(
        &mut self,
        mode: MapMode,
        province_storage: &ProvinceStorage,
        world_seed: u32,
        nation_colors: Option<&NationColorRegistry>,
        climate_storage: Option<&crate::world::terrain::ClimateStorage>,
        infrastructure_storage: Option<&crate::world::InfrastructureStorage>,
    ) -> Arc<Vec<[f32; 4]>> {
        // If requesting current overlay, return Arc clone (just increments refcount)
        if mode == self.current_type {
            return Arc::clone(&self.current);
        }

        // Check cache - zero-copy retrieval with Arc
        // SPECIAL CASE: Don't use cached Political mode if nation colors are now available but weren't used in cache
        let use_cache = if mode == MapMode::Political && nation_colors.is_some() {
            // For Political mode with nation colors, only use cache if we're confident it was calculated with nation colors
            // Since we now exclude Political from pre-calculation, any cached Political should have nation colors
            if let Some(cached) = self.cache.get(&mode) {
                debug!("Using cached Political mode (calculated with nation colors)");
                true
            } else {
                false
            }
        } else {
            self.cache.contains_key(&mode)
        };

        if use_cache {
            if let Some(cached) = self.cache.get(&mode) {
                // Swap current into cache, cached becomes current (no cloning!)
                let old_current = std::mem::replace(&mut self.current, Arc::clone(cached));
                if self.current_type != mode {
                    self.cache.insert(self.current_type, old_current);
                }
                self.current_type = mode;
                return Arc::clone(&self.current);
            }
        }

        info!("Calculating overlay colors for: {}", mode.display_name());
        if mode == MapMode::Political {
            if nation_colors.is_some() {
                debug!("Political mode: Calculating with nation colors");
            } else {
                warn!("Political mode: No nation colors available - will show terrain-like colors");
            }
        }
        let start = std::time::Instant::now();

        // Calculate colors in parallel for better performance
        let colors = Arc::new(self.calculate_colors_parallel_with_nations(
            mode,
            province_storage,
            world_seed,
            nation_colors,
            climate_storage,
            infrastructure_storage,
        ));

        debug!(
            "Calculated {} overlay in {:.2}ms ({} vertices, {:.1}MB)",
            mode.display_name(),
            start.elapsed().as_secs_f32() * 1000.0,
            colors.len(),
            (colors.len() * std::mem::size_of::<[f32; 4]>()) as f32 / (1024.0 * 1024.0)
        );

        // Manage cache size
        if self.cache.len() >= self.max_cache_size {
            // Remove least recently used (not current)
            if let Some(key_to_remove) = self
                .cache
                .keys()
                .find(|&&k| k != self.current_type)
                .cloned()
            {
                self.cache.remove(&key_to_remove);
                debug!("Evicted {} from cache", key_to_remove.display_name());
            }
        }

        // Add previous current to cache if it has data
        if !self.current.is_empty() {
            self.cache
                .insert(self.current_type, Arc::clone(&self.current));
        }

        self.current = Arc::clone(&colors);
        self.current_type = mode;

        colors
    }

    /// Calculate colors in parallel for massive performance improvement
    fn calculate_colors_parallel(
        &self,
        mode: MapMode,
        province_storage: &ProvinceStorage,
        world_seed: u32,
    ) -> Vec<[f32; 4]> {
        self.calculate_colors_parallel_with_nations(mode, province_storage, world_seed, None, None, None)
    }

    /// Calculate colors with optional nation colors for political mode
    fn calculate_colors_parallel_with_nations(
        &self,
        mode: MapMode,
        province_storage: &ProvinceStorage,
        world_seed: u32,
        nation_colors: Option<&NationColorRegistry>,
        climate_storage: Option<&crate::world::terrain::ClimateStorage>,
        infrastructure_storage: Option<&crate::world::InfrastructureStorage>,
    ) -> Vec<[f32; 4]> {
        // Use actual world seed for deterministic color generation
        let world_colors = WorldColors::new(world_seed);

        // Pre-allocate exact size to avoid reallocations
        let total_vertices = province_storage.provinces.len() * VERTICES_PER_HEX;
        let mut colors = Vec::with_capacity(total_vertices);

        // Convert to LinearRgba and use its to_f32_array() method like Bevy examples
        use bevy::color::LinearRgba;

        // Pre-build color map for political mode and terrain mode to use in parallel chunks
        let nation_colors_map = if mode == MapMode::Political || mode == MapMode::Terrain {
            let mut map = HashMap::new();
            if let Some(registry) = nation_colors {
                for province in &province_storage.provinces {
                    if let Some(owner) = province.owner {
                        if let Some(&color) = registry.colors.get(&owner) {
                            map.insert(province.id.0, color);
                        }
                    }
                }
            }
            Some(map)
        } else {
            None
        };

        // Calculate optimal chunk size based on CPU count
        let num_threads = rayon::current_num_threads();
        let provinces_per_thread = (province_storage.provinces.len() / num_threads).max(1000);
        let chunk_size = provinces_per_thread.min(50000); // Cap at 50k for cache efficiency

        // Process provinces in parallel chunks with optimized sizing
        let chunk_colors: Vec<Vec<[f32; 4]>> = province_storage
            .provinces
            .par_chunks(chunk_size)
            .map(|chunk| {
                // Pre-allocate exact size for this chunk
                let mut chunk_colors = Vec::with_capacity(chunk.len() * VERTICES_PER_HEX);

                // Process each province in the chunk
                for province in chunk {
                    // Calculate color for each province
                    let color = if mode == MapMode::Political {
                        // Political mode uses pre-built color map for nations
                        nation_colors_map
                            .as_ref()
                            .and_then(|map| map.get(&province.id.0).copied())
                            .unwrap_or_else(|| {
                                // For unclaimed provinces, show natural terrain colors (especially ocean)
                                if province.terrain == crate::world::TerrainType::Ocean {
                                    // Use natural ocean color instead of gray
                                    world_colors.terrain(
                                        province.terrain,
                                        province.elevation.value(),
                                        province.position,
                                    )
                                } else {
                                    // Unclaimed land provinces use neutral gray
                                    Color::srgb(0.15, 0.15, 0.15)
                                }
                            })
                    } else if mode == MapMode::Terrain {
                        // Terrain mode with transparent nation overlays
                        let base_terrain_color = world_colors.terrain(
                            province.terrain,
                            province.elevation.value(),
                            province.position,
                        );

                        // If province has an owner, blend with transparent nation color
                        if let Some(nation_color) = nation_colors_map
                            .as_ref()
                            .and_then(|map| map.get(&province.id.0).copied())
                        {
                            // Create transparent nation color (15% opacity)
                            let nation_rgba = nation_color.to_linear().to_f32_array();
                            let terrain_rgba = base_terrain_color.to_linear().to_f32_array();

                            // Blend: terrain shows through with transparent nation overlay
                            let alpha = 0.15; // Low opacity so terrain is clearly visible
                            Color::srgba(
                                terrain_rgba[0] * (1.0 - alpha) + nation_rgba[0] * alpha,
                                terrain_rgba[1] * (1.0 - alpha) + nation_rgba[1] * alpha,
                                terrain_rgba[2] * (1.0 - alpha) + nation_rgba[2] * alpha,
                                1.0
                            )
                        } else {
                            // No owner - just show natural terrain
                            base_terrain_color
                        }
                    } else {
                        // All other modes use the standard calculation
                        self.calculate_province_color(mode, province, &world_colors, climate_storage, infrastructure_storage)
                    };
                    let color_array = LinearRgba::from(color).to_f32_array();

                    // Unroll loop for better performance - compiler optimization hint
                    #[allow(clippy::needless_range_loop)]
                    for _ in 0..VERTICES_PER_HEX {
                        chunk_colors.push(color_array);
                    }
                }
                chunk_colors
            })
            .collect();

        // Parallel extend for combining chunks efficiently
        // Optimize: calculate total size directly without intermediate collection
        let total_size: usize = chunk_colors.iter().map(|c| c.len()).sum();

        // Pre-allocate exact final size
        colors.reserve_exact(total_size);

        // Combine chunks
        for chunk in chunk_colors {
            colors.extend(chunk);
        }

        colors
    }

    /// Calculate color for a single province based on map mode
    /// NOTE: Political and Terrain modes are handled separately and should never call this function
    fn calculate_province_color(
        &self,
        mode: MapMode,
        province: &Province,
        world_colors: &WorldColors,
        climate_storage_opt: Option<&crate::world::terrain::ClimateStorage>,
        infrastructure_storage_opt: Option<&crate::world::InfrastructureStorage>,
    ) -> Color {
        debug_assert!(
            mode != MapMode::Political && mode != MapMode::Terrain,
            "Political and Terrain modes should use pre-built color maps"
        );

        match mode {

            // Climate zones based on actual climate data
            MapMode::Climate => {
                // Use stored climate data if available
                if let Some(climate_storage) = climate_storage_opt {
                    if let Some(climate) = climate_storage.get(province.id) {
                        // Generate color from actual climate data
                        crate::world::colors::composite_climate_color(
                            climate.temperature,
                            climate.rainfall,
                            climate.zone,
                            climate_storage.normalized_temperature(climate.temperature),
                        )
                    } else {
                        // Fallback for provinces without climate data (e.g., ocean)
                        world_colors.terrain(
                            province.terrain,
                            province.elevation.value(),
                            province.position,
                        )
                    }
                } else {
                    // Fallback if climate storage not available
                    world_colors.terrain(
                        province.terrain,
                        province.elevation.value(),
                        province.position,
                    )
                }
            }

            // Population density heat map
            MapMode::Population => {
                let pop_normalized = (province.population as f32 / 100000.0).min(1.0);
                Color::srgb(pop_normalized, pop_normalized * 0.5, 0.0)
            }

            // Agricultural productivity
            MapMode::Agriculture => {
                let agri_value = province.agriculture.value();
                let agri_normalized = agri_value.min(3.0) / 3.0;
                Color::srgb(0.0, agri_normalized, 0.0)
            }

            // Infrastructure development
            MapMode::Infrastructure => {
                // Use stored infrastructure data if available
                if let Some(infra_storage) = infrastructure_storage_opt {
                    if let Some(infra) = infra_storage.get(province.id) {
                        // Generate color from infrastructure metrics
                        crate::world::colors::composite_infrastructure_color(
                            infra.connectivity,
                            infra.road_density,
                            infra.trade_volume,
                            infra.is_hub,
                        )
                    } else {
                        // Fallback for provinces without infrastructure (wilderness)
                        crate::world::colors::infrastructure_gradient_color(0.0)
                    }
                } else {
                    // Fallback if infrastructure storage not available
                    world_colors.terrain(
                        province.terrain,
                        province.elevation.value(),
                        province.position,
                    )
                }
            }

            // Unified minerals mode (combining all mineral types)
            MapMode::Minerals => {
                let total_richness = calculate_total_richness(province);
                world_colors.richness(total_richness)
            }

            // Safety: Political and Terrain modes are filtered before this function is called
            _ => unreachable!(
                "Unexpected map mode in calculate_province_color: {:?}",
                mode
            ),
        }
    }

    /// Pre-calculate common overlays for instant switching
    pub fn pre_calculate_common_modes(
        &mut self,
        province_storage: &ProvinceStorage,
        world_seed: u32,
    ) {
        // Pre-calculate frequently used modes during loading (excluding Political which needs nation colors)
        // This uses more memory but eliminates lag when switching
        let modes = vec![
            // Note: Political mode excluded from pre-calculation since it requires nation colors
            // and will be calculated on-demand with proper nation data
            MapMode::Climate,
            MapMode::Population,
            MapMode::Agriculture,
            MapMode::Minerals,
        ];

        info!("Pre-calculating {} common map modes", modes.len());
        let start = std::time::Instant::now();

        // Calculate in parallel
        let pre_calculated: Vec<_> = modes
            .par_iter()
            .map(|&mode| {
                let colors = self.calculate_colors_parallel(mode, province_storage, world_seed);
                (mode, Arc::new(colors))
            })
            .collect();

        // Store in cache
        for (mode, colors) in pre_calculated {
            self.cache.insert(mode, colors);
        }

        info!(
            "Pre-calculated common overlays in {:.2}s",
            start.elapsed().as_secs_f32()
        );
    }

    /// Clear cache to free memory
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        debug!("Cleared overlay color cache");
    }

    /// Get memory usage in MB (now accounts for Arc reference counting)
    pub fn memory_usage_mb(&self) -> f32 {
        const BYTES_PER_MB: f32 = 1024.0 * 1024.0;

        // Only count unique allocations (Arc may share references)
        let current_size = if Arc::strong_count(&self.current) == 1 {
            self.current.len() * std::mem::size_of::<[f32; 4]>()
        } else {
            0 // Shared with cache, don't double count
        };

        let cache_size: usize = self
            .cache
            .values()
            .map(|v| {
                // Only count if this is the sole owner
                if Arc::strong_count(v) == 1 {
                    v.len() * std::mem::size_of::<[f32; 4]>()
                } else {
                    0
                }
            })
            .sum();

        (current_size + cache_size) as f32 / BYTES_PER_MB
    }

    /// Get diagnostics about Arc reference counts for debugging
    pub fn arc_diagnostics(&self) {
        debug!(
            "Arc diagnostics - Current: {} refs, Cache entries: {}",
            Arc::strong_count(&self.current),
            self.cache.len()
        );
        for (mode, arc) in &self.cache {
            debug!("  {}: {} refs", mode.display_name(), Arc::strong_count(arc));
        }
    }
}
