//! Mineral resource system for Living Worlds
//!
//! This module handles generation, extraction, and processing of mineral resources.
//! Resources drive technological advancement, trade, and conflict between nations.
//!
//! # Performance Characteristics
//! - Vein generation: O(n) where n = number of veins
//! - Resource calculation: O(1) per province using spatial indexing
//! - Parallel processing: Scales linearly with CPU cores
//! - Memory usage: ~7 bytes per province for mineral storage

use bevy::prelude::*;
use rand::prelude::*;
use rand::rngs::StdRng;
use rayon::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

use crate::components::MineralType;
use crate::constants::*;
use crate::math::{euclidean_vec2, gaussian_falloff};
use crate::world::{Abundance, Province, TerrainType};

#[derive(Debug, Error)]
pub enum MineralGenerationError {
    #[error("No suitable provinces found for {mineral:?} generation")]
    NoSuitableProvinces { mineral: MineralType },

    #[error("Invalid position: {0}")]
    InvalidPosition(String),

    #[error("Invalid richness value {0} (must be between {1} and {2})")]
    InvalidRichness(f32, f32, f32),
}

/// Number of ore veins to generate for each mineral type
pub const IRON_VEIN_COUNT: usize = 40;
pub const COPPER_VEIN_COUNT: usize = 30;
pub const TIN_VEIN_COUNT: usize = 12; // Rare - creates bronze bottleneck
pub const GOLD_VEIN_COUNT: usize = 10;
pub const COAL_DEPOSIT_COUNT: usize = 25;
pub const GEM_VEIN_COUNT: usize = 5;

/// Vein generation parameters
pub const VEIN_FALLOFF_DISTANCE: f32 = 500.0;
pub const MIN_RICHNESS: f32 = 0.5;
pub const MAX_RICHNESS: f32 = 2.0;
pub const VEINS_PER_BOUNDARY_SEGMENT: usize = 2;
pub const COPPER_VEIN_CHANCE: f64 = 0.5;
pub const VEIN_POSITION_JITTER: f32 = 50.0;
pub const GOLD_HOTSPOT_CHANCE: f64 = 0.3;

/// Stone abundance by terrain
pub const STONE_ABUNDANCE_MOUNTAINS: u8 = 80;
pub const STONE_ABUNDANCE_HILLS: u8 = 60;
pub const STONE_ABUNDANCE_TUNDRA: u8 = 50;
pub const STONE_ABUNDANCE_DESERT: u8 = 40;
pub const STONE_ABUNDANCE_BEACH: u8 = 30;
pub const STONE_ABUNDANCE_DEFAULT: u8 = 20;

/// Mineral value weights for richness calculation
pub const IRON_VALUE_WEIGHT: f32 = 1.0;
pub const COPPER_VALUE_WEIGHT: f32 = 1.5;
pub const TIN_VALUE_WEIGHT: f32 = 3.0;
pub const GOLD_VALUE_WEIGHT: f32 = 10.0;
pub const COAL_VALUE_WEIGHT: f32 = 0.8;
pub const STONE_VALUE_WEIGHT: f32 = 0.2;
pub const GEM_VALUE_WEIGHT: f32 = 20.0;

/// Abundance calculation parameters
pub const ABUNDANCE_BASE_MULTIPLIER: f32 = 100.0;
pub const MAX_ABUNDANCE: f32 = 100.0;

/// Spatial index cell size (should be ~2x VEIN_FALLOFF_DISTANCE for optimal performance)
pub const VEIN_SPATIAL_CELL_SIZE: f32 = 1000.0;

/// Base extraction rates per day (with level 1 mine)
pub const EXTRACTION_RATES: [(MineralType, f32); 7] = [
    (MineralType::Stone, 10.0), // Fast extraction
    (MineralType::Coal, 8.0),   // Bulk extraction
    (MineralType::Iron, 5.0),   // Moderate
    (MineralType::Copper, 4.0), // Moderate
    (MineralType::Tin, 2.0),    // Slow (rare)
    (MineralType::Gold, 0.5),   // Very slow
    (MineralType::Gems, 0.1),   // Extremely slow
];

/// Depletion rate per unit extracted
pub const DEPLETION_RATES: [(MineralType, f32); 7] = [
    (MineralType::Stone, 0.001),  // Nearly infinite
    (MineralType::Coal, 0.01),    // ~100 years
    (MineralType::Iron, 0.005),   // ~200 years
    (MineralType::Copper, 0.005), // ~200 years
    (MineralType::Tin, 0.008),    // ~125 years
    (MineralType::Gold, 0.02),    // ~50 years
    (MineralType::Gems, 0.03),    // ~33 years
];

pub struct MineralPlugin;

impl Plugin for MineralPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VeinSpatialIndex>();
        // Note: Overlay update system would be added here when overlay system is integrated
    }
}

// System to update mineral overlays would go here when integrated

/// Represents a concentrated deposit of a mineral
#[derive(Debug, Clone)]
pub struct OreVein {
    position: Vec2,
    mineral_type: MineralType,
    richness: f32, // MIN_RICHNESS to MAX_RICHNESS multiplier
}

impl OreVein {
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn mineral_type(&self) -> MineralType {
        self.mineral_type
    }

    pub fn richness(&self) -> f32 {
        self.richness
    }

    /// Validate that position is finite
    fn validate_position(position: Vec2) -> Result<(), MineralGenerationError> {
        if !position.x.is_finite() || !position.y.is_finite() {
            return Err(MineralGenerationError::InvalidPosition(format!(
                "Position contains non-finite values: {:?}",
                position
            )));
        }
        Ok(())
    }
}

/// Builder for creating ore veins with validation
pub struct OreVeinBuilder {
    position: Option<Vec2>,
    mineral_type: Option<MineralType>,
    richness: f32,
}

impl OreVeinBuilder {
    pub fn new() -> Self {
        Self {
            position: None,
            mineral_type: None,
            richness: 1.0,
        }
    }

    pub fn position(mut self, pos: Vec2) -> Self {
        self.position = Some(pos);
        self
    }

    pub fn mineral_type(mut self, mineral: MineralType) -> Self {
        self.mineral_type = Some(mineral);
        self
    }

    pub fn richness(mut self, richness: f32) -> Self {
        self.richness = richness.clamp(MIN_RICHNESS, MAX_RICHNESS);
        self
    }

    pub fn build(self) -> Result<OreVein, MineralGenerationError> {
        let position = self
            .position
            .ok_or_else(|| MineralGenerationError::InvalidPosition("Position not set".into()))?;

        OreVein::validate_position(position)?;

        let mineral_type = self.mineral_type.ok_or_else(|| {
            MineralGenerationError::InvalidPosition("Mineral type not set".into())
        })?;

        if self.richness < MIN_RICHNESS || self.richness > MAX_RICHNESS {
            return Err(MineralGenerationError::InvalidRichness(
                self.richness,
                MIN_RICHNESS,
                MAX_RICHNESS,
            ));
        }

        Ok(OreVein {
            position,
            mineral_type,
            richness: self.richness,
        })
    }
}

/// Generate ore vein centers using clustering algorithm
pub fn generate_ore_veins(
    mineral_type: MineralType,
    count: usize,
    seed: u32,
    provinces: &[Province],
    terrain_bias: fn(&TerrainType, f32) -> f32,
) -> Result<Vec<OreVein>, MineralGenerationError> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut veins = Vec::with_capacity(count);

    // Filter provinces by terrain suitability
    let suitable_provinces: Vec<&Province> = provinces
        .iter()
        .filter(|p| terrain_bias(&p.terrain, p.elevation.value()) > 0.0)
        .collect();

    if suitable_provinces.is_empty() {
        warn!(
            "No suitable provinces found for {:?} generation",
            mineral_type
        );
        return Ok(veins); // Return empty vec instead of error for flexibility
    }

    // Generate vein centers biased toward suitable terrain
    for _ in 0..count {
        let province = suitable_provinces.choose(&mut rng).ok_or_else(|| {
            MineralGenerationError::NoSuitableProvinces {
                mineral: mineral_type,
            }
        })?;

        // Add some random offset within the province
        let offset_x = rng.gen_range(-HEX_SIZE_PIXELS..HEX_SIZE_PIXELS);
        let offset_y = rng.gen_range(-HEX_SIZE_PIXELS..HEX_SIZE_PIXELS);

        let vein = OreVeinBuilder::new()
            .position(Vec2::new(
                province.position.x + offset_x,
                province.position.y + offset_y,
            ))
            .mineral_type(mineral_type)
            .richness(rng.gen_range(MIN_RICHNESS..MAX_RICHNESS))
            .build()?;

        veins.push(vein);
    }

    Ok(veins)
}

// SPATIAL INDEXING FOR O(1) VEIN LOOKUPS

/// Spatial index for fast vein proximity queries
#[derive(Resource)]
pub struct VeinSpatialIndex {
    /// Grid of vein indices organized by spatial cells
    grid: HashMap<(i32, i32), Vec<usize>>,
    /// All veins stored flat for index access
    veins: Vec<OreVein>,
    /// Cell size for spatial hashing
    cell_size: f32,
}

impl Default for VeinSpatialIndex {
    fn default() -> Self {
        Self::new(VEIN_SPATIAL_CELL_SIZE)
    }
}

impl VeinSpatialIndex {
    pub fn new(cell_size: f32) -> Self {
        Self {
            grid: HashMap::new(),
            veins: Vec::new(),
            cell_size,
        }
    }

    /// Build spatial index from ore veins
    pub fn build_from_veins(&mut self, ore_veins: HashMap<MineralType, Vec<OreVein>>) {
        self.grid.clear();
        self.veins.clear();

        // Flatten all veins and build spatial index
        for (_mineral_type, veins) in ore_veins {
            for vein in veins {
                let idx = self.veins.len();
                let cell_x = (vein.position.x / self.cell_size).floor() as i32;
                let cell_y = (vein.position.y / self.cell_size).floor() as i32;

                self.grid
                    .entry((cell_x, cell_y))
                    .or_insert_with(Vec::new)
                    .push(idx);

                self.veins.push(vein);
            }
        }

        info!(
            "Built spatial index with {} veins in {} cells",
            self.veins.len(),
            self.grid.len()
        );
    }

    /// Query veins near a position (returns indices and distances)
    pub fn query_near(
        &self,
        position: Vec2,
        mineral_type: MineralType,
        radius: f32,
    ) -> Vec<(usize, f32)> {
        let mut results = Vec::new();

        let min_x = ((position.x - radius) / self.cell_size).floor() as i32;
        let max_x = ((position.x + radius) / self.cell_size).floor() as i32;
        let min_y = ((position.y - radius) / self.cell_size).floor() as i32;
        let max_y = ((position.y + radius) / self.cell_size).floor() as i32;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(indices) = self.grid.get(&(x, y)) {
                    for &idx in indices {
                        if let Some(vein) = self.veins.get(idx) {
                            if vein.mineral_type == mineral_type {
                                let distance = euclidean_vec2(position, vein.position);
                                if distance <= radius {
                                    results.push((idx, distance));
                                }
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Get vein by index
    pub fn get_vein(&self, idx: usize) -> Option<&OreVein> {
        self.veins.get(idx)
    }
}

/// Terrain bias configuration for data-driven approach
#[derive(Debug, Clone)]
pub struct TerrainBiasConfig {
    pub terrain: TerrainType,
    pub min_elevation: Option<f32>,
    pub max_elevation: Option<f32>,
    pub bias: f32,
}

/// Get terrain bias for a mineral type using data-driven configuration
pub fn get_terrain_bias(mineral: MineralType, terrain: &TerrainType, elevation: f32) -> f32 {
    use MineralType::*;
    use TerrainType::*;

    let configs = match mineral {
        Iron => vec![
            TerrainBiasConfig {
                terrain: Alpine,
                min_elevation: None,
                max_elevation: None,
                bias: 3.0,
            },
            TerrainBiasConfig {
                terrain: Chaparral,
                min_elevation: None,
                max_elevation: None,
                bias: 2.0,
            },
            TerrainBiasConfig {
                terrain: Tundra,
                min_elevation: None,
                max_elevation: None,
                bias: 2.5,
            },
            TerrainBiasConfig {
                terrain: SubtropicalDesert,
                min_elevation: Some(0.5),
                max_elevation: None,
                bias: 1.5,
            },
        ],
        Copper => vec![
            TerrainBiasConfig {
                terrain: Alpine,
                min_elevation: None,
                max_elevation: None,
                bias: 2.0,
            },
            TerrainBiasConfig {
                terrain: Chaparral,
                min_elevation: None,
                max_elevation: None,
                bias: 1.5,
            },
            TerrainBiasConfig {
                terrain: Tundra,
                min_elevation: None,
                max_elevation: None,
                bias: 1.8,
            },
            TerrainBiasConfig {
                terrain: TropicalDesert,
                min_elevation: Some(0.4),
                max_elevation: None,
                bias: 1.0,
            },
        ],
        Tin => vec![
            TerrainBiasConfig {
                terrain: Alpine,
                min_elevation: Some(0.8),
                max_elevation: None,
                bias: 4.0,
            },
            TerrainBiasConfig {
                terrain: Chaparral,
                min_elevation: Some(0.65),
                max_elevation: None,
                bias: 1.0,
            },
            TerrainBiasConfig {
                terrain: Tundra,
                min_elevation: Some(0.6),
                max_elevation: None,
                bias: 0.8,
            },
        ],
        Gold => vec![
            TerrainBiasConfig {
                terrain: Alpine,
                min_elevation: Some(0.85),
                max_elevation: None,
                bias: 5.0,
            },
            TerrainBiasConfig {
                terrain: River,
                min_elevation: Some(0.4),
                max_elevation: None,
                bias: 2.0,
            },
            TerrainBiasConfig {
                terrain: Tundra,
                min_elevation: Some(0.65),
                max_elevation: None,
                bias: 1.5,
            },
        ],
        Coal => vec![
            TerrainBiasConfig {
                terrain: TemperateDeciduousForest,
                min_elevation: None,
                max_elevation: Some(0.4),
                bias: 3.0,
            },
            TerrainBiasConfig {
                terrain: TemperateGrassland,
                min_elevation: None,
                max_elevation: Some(0.35),
                bias: 2.0,
            },
            TerrainBiasConfig {
                terrain: Tundra,
                min_elevation: None,
                max_elevation: Some(0.5),
                bias: 2.0,
            },
            TerrainBiasConfig {
                terrain: TropicalRainforest,
                min_elevation: None,
                max_elevation: Some(0.3),
                bias: 1.5,
            },
        ],
        Gems => vec![
            TerrainBiasConfig {
                terrain: Alpine,
                min_elevation: Some(0.9),
                max_elevation: None,
                bias: 10.0,
            },
            TerrainBiasConfig {
                terrain: Tundra,
                min_elevation: Some(0.7),
                max_elevation: None,
                bias: 3.0,
            },
        ],
        _ => vec![],
    };

    for config in configs {
        if config.terrain == *terrain {
            let elevation_ok = config.min_elevation.map_or(true, |min| elevation >= min)
                && config.max_elevation.map_or(true, |max| elevation <= max);
            if elevation_ok {
                return config.bias;
            }
        }
    }

    0.0
}

/// Calculate mineral abundance for a province based on distance to ore veins
pub fn calculate_province_resources(province: &mut Province, spatial_index: &VeinSpatialIndex) {
    // Mark province as modified
    province.mark_dirty();

    // Stone is everywhere but concentrated in rocky areas
    province.stone = Abundance::new(get_stone_abundance(&province.terrain));

    // Use generic function to eliminate DRY violations
    set_mineral_abundance(province, MineralType::Iron, spatial_index);
    set_mineral_abundance(province, MineralType::Copper, spatial_index);
    set_mineral_abundance(province, MineralType::Tin, spatial_index);
    set_mineral_abundance(province, MineralType::Gold, spatial_index);
    set_mineral_abundance(province, MineralType::Coal, spatial_index);
    set_mineral_abundance(province, MineralType::Gems, spatial_index);
}

/// Generic function to set mineral abundance (eliminates DRY)
fn set_mineral_abundance(
    province: &mut Province,
    mineral_type: MineralType,
    spatial_index: &VeinSpatialIndex,
) {
    let abundance = calculate_abundance_with_index(
        province.position,
        mineral_type,
        &province.terrain,
        spatial_index,
    );

    match mineral_type {
        MineralType::Iron => province.iron = Abundance::new(abundance),
        MineralType::Copper => province.copper = Abundance::new(abundance),
        MineralType::Tin => province.tin = Abundance::new(abundance),
        MineralType::Gold => province.gold = Abundance::new(abundance),
        MineralType::Coal => province.coal = Abundance::new(abundance),
        MineralType::Gems => province.gems = Abundance::new(abundance),
        MineralType::Stone => province.stone = Abundance::new(abundance),
        _ => {}
    }
}

/// Calculate abundance using spatial index for O(1) lookups
fn calculate_abundance_with_index(
    position: Vec2,
    mineral_type: MineralType,
    terrain: &TerrainType,
    spatial_index: &VeinSpatialIndex,
) -> u8 {
    // Query nearby veins using spatial index (O(1) average case)
    let nearby_veins = spatial_index.query_near(position, mineral_type, VEIN_FALLOFF_DISTANCE);

    if nearby_veins.is_empty() {
        return 0;
    }

    let (closest_idx, min_distance) = nearby_veins
        .iter()
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .copied()
        .unwrap_or((0, f32::MAX));

    if min_distance > VEIN_FALLOFF_DISTANCE {
        return 0;
    }

    let closest_richness = spatial_index
        .get_vein(closest_idx)
        .map(|v| v.richness())
        .unwrap_or(1.0);

    // Apply Gaussian falloff using centralized function
    let base_abundance =
        ABUNDANCE_BASE_MULTIPLIER * gaussian_falloff(min_distance, VEIN_FALLOFF_DISTANCE);
    let terrain_multiplier = get_terrain_extraction_bonus(terrain);

    (base_abundance * closest_richness * terrain_multiplier).min(MAX_ABUNDANCE) as u8
}

/// Get stone abundance based on terrain type - uses centralized properties
fn get_stone_abundance(terrain: &TerrainType) -> u8 {
    terrain.properties().stone_abundance
}

/// Calculate resources for all provinces in parallel
pub fn calculate_all_province_resources(
    provinces: &mut [Province],
    spatial_index: &VeinSpatialIndex,
) {
    // Use rayon for parallel processing
    provinces.par_iter_mut().for_each(|province| {
        // Mark province as modified
        province.mark_dirty();

        // Stone abundance
        province.stone = Abundance::new(get_stone_abundance(&province.terrain));

        province.iron = Abundance::new(calculate_abundance_with_index(
            province.position,
            MineralType::Iron,
            &province.terrain,
            spatial_index,
        ));
        province.copper = Abundance::new(calculate_abundance_with_index(
            province.position,
            MineralType::Copper,
            &province.terrain,
            spatial_index,
        ));
        province.tin = Abundance::new(calculate_abundance_with_index(
            province.position,
            MineralType::Tin,
            &province.terrain,
            spatial_index,
        ));
        province.gold = Abundance::new(calculate_abundance_with_index(
            province.position,
            MineralType::Gold,
            &province.terrain,
            spatial_index,
        ));
        province.coal = Abundance::new(calculate_abundance_with_index(
            province.position,
            MineralType::Coal,
            &province.terrain,
            spatial_index,
        ));
        province.gems = Abundance::new(calculate_abundance_with_index(
            province.position,
            MineralType::Gems,
            &province.terrain,
            spatial_index,
        ));
    });
}

/// Terrain affects how easy resources are to extract - uses centralized properties
fn get_terrain_extraction_bonus(terrain: &TerrainType) -> f32 {
    terrain.properties().extraction_difficulty
}

/// Generate mineral resources for the entire world using terrain and elevation
pub fn generate_world_minerals(seed: u32, provinces: &mut [Province]) {
    info!(
        "Generating mineral resources for {} provinces",
        provinces.len()
    );
    let start_time = std::time::Instant::now();

    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut ore_veins: HashMap<MineralType, Vec<OreVein>> = HashMap::new();

    // Iron in mountains and hills (high elevation areas)
    let mut iron_veins = Vec::new();
    let mountain_provinces: Vec<&Province> = provinces
        .iter()
        .filter(|p| p.terrain == TerrainType::Alpine || p.terrain == TerrainType::Chaparral)
        .collect();
    for _ in 0..(mountain_provinces.len() / 20).max(10) {
        if let Some(province) = mountain_provinces.choose(&mut rng) {
            if let Ok(vein) = OreVeinBuilder::new()
                .position(province.position)
                .mineral_type(MineralType::Iron)
                .richness(rng.gen_range(1.0..MAX_RICHNESS))
                .build()
            {
                iron_veins.push(vein);
            }
        }
    }

    // Copper in hills and volcanic areas
    let mut copper_veins = Vec::new();
    let hill_provinces: Vec<&Province> = provinces
        .iter()
        .filter(|p| {
            p.terrain == TerrainType::Chaparral
                || (p.elevation.value() > 0.4 && p.elevation.value() < 0.7)
        })
        .collect();
    for _ in 0..(hill_provinces.len() / 30).max(8) {
        if let Some(province) = hill_provinces.choose(&mut rng) {
            if let Ok(vein) = OreVeinBuilder::new()
                .position(province.position)
                .mineral_type(MineralType::Copper)
                .richness(rng.gen_range(0.8..1.5))
                .build()
            {
                copper_veins.push(vein);
            }
        }
    }

    // Tin in moderate elevations
    let mut tin_veins = Vec::new();
    let tin_provinces: Vec<&Province> = provinces
        .iter()
        .filter(|p| p.elevation.value() > 0.3 && p.elevation.value() < 0.6)
        .collect();
    for _ in 0..(tin_provinces.len() / 40).max(6) {
        if let Some(province) = tin_provinces.choose(&mut rng) {
            if let Ok(vein) = OreVeinBuilder::new()
                .position(province.position)
                .mineral_type(MineralType::Tin)
                .richness(rng.gen_range(0.7..1.3))
                .build()
            {
                tin_veins.push(vein);
            }
        }
    }

    // Gold in specific terrain types
    let mut gold_veins = Vec::new();
    let gold_provinces: Vec<&Province> = provinces
        .iter()
        .filter(|p| {
            p.terrain == TerrainType::SubtropicalDesert
                || p.terrain == TerrainType::Chaparral
                || (p.terrain == TerrainType::Alpine && p.elevation.value() > 0.8)
        })
        .collect();
    for _ in 0..(gold_provinces.len() / 100).max(3) {
        if let Some(province) = gold_provinces.choose(&mut rng) {
            if let Ok(vein) = OreVeinBuilder::new()
                .position(province.position)
                .mineral_type(MineralType::Gold)
                .richness(rng.gen_range(1.5..2.5))
                .build()
            {
                gold_veins.push(vein);
            }
        }
    }

    // Coal in forests and swamps
    let mut coal_veins = Vec::new();
    let coal_provinces: Vec<&Province> = provinces
        .iter()
        .filter(|p| {
            p.terrain == TerrainType::TemperateDeciduousForest
                || p.terrain == TerrainType::BorealForest
                || p.terrain == TerrainType::TropicalRainforest
        })
        .collect();
    for _ in 0..(coal_provinces.len() / 25).max(8) {
        if let Some(province) = coal_provinces.choose(&mut rng) {
            if let Ok(vein) = OreVeinBuilder::new()
                .position(province.position)
                .mineral_type(MineralType::Coal)
                .richness(rng.gen_range(0.8..1.5))
                .build()
            {
                coal_veins.push(vein);
            }
        }
    }

    // Gems at the highest peaks
    let mut gem_veins = Vec::new();
    let peak_provinces: Vec<&Province> = provinces
        .iter()
        .filter(|p| p.terrain == TerrainType::Alpine && p.elevation.value() > 0.85)
        .collect();
    for _ in 0..(peak_provinces.len() / 200).max(2) {
        if let Some(province) = peak_provinces.choose(&mut rng) {
            if let Ok(vein) = OreVeinBuilder::new()
                .position(province.position)
                .mineral_type(MineralType::Gems)
                .richness(rng.gen_range(2.0..3.0))
                .build()
            {
                gem_veins.push(vein);
            }
        }
    }

    // Store veins in HashMap
    ore_veins.insert(MineralType::Iron, iron_veins);
    ore_veins.insert(MineralType::Copper, copper_veins);
    ore_veins.insert(MineralType::Tin, tin_veins);
    ore_veins.insert(MineralType::Gold, gold_veins);
    ore_veins.insert(MineralType::Coal, coal_veins);
    ore_veins.insert(MineralType::Gems, gem_veins);

    let mut spatial_index = VeinSpatialIndex::new(VEIN_SPATIAL_CELL_SIZE);
    spatial_index.build_from_veins(ore_veins);

    calculate_all_province_resources(provinces, &spatial_index);

    let elapsed = start_time.elapsed();
    info!(
        "Mineral generation completed in {:.2}ms",
        elapsed.as_secs_f32() * 1000.0
    );
}

/// Get mineral abundance for a specific province and mineral type
pub fn get_mineral_abundance(province: &Province, mineral_type: MineralType) -> u8 {
    match mineral_type {
        MineralType::Iron => province.iron.value(),
        MineralType::Copper => province.copper.value(),
        MineralType::Tin => province.tin.value(),
        MineralType::Gold => province.gold.value(),
        MineralType::Coal => province.coal.value(),
        MineralType::Stone => province.stone.value(),
        MineralType::Gems => province.gems.value(),
        _ => 0,
    }
}

/// Calculate total mineral richness for a province
pub fn calculate_total_richness(province: &Province) -> f32 {
    // Weighted sum of all minerals
    let iron_value = province.iron.value() as f32 * IRON_VALUE_WEIGHT;
    let copper_value = province.copper.value() as f32 * COPPER_VALUE_WEIGHT;
    let tin_value = province.tin.value() as f32 * TIN_VALUE_WEIGHT;
    let gold_value = province.gold.value() as f32 * GOLD_VALUE_WEIGHT;
    let coal_value = province.coal.value() as f32 * COAL_VALUE_WEIGHT;
    let stone_value = province.stone.value() as f32 * STONE_VALUE_WEIGHT;
    let gem_value = province.gems.value() as f32 * GEM_VALUE_WEIGHT;

    (iron_value + copper_value + tin_value + gold_value + coal_value + stone_value + gem_value)
        / MAX_ABUNDANCE
}
