//! Mineral resource system for Living Worlds
//! 
//! This module handles generation, extraction, and processing of mineral resources.
//! Resources drive technological advancement, trade, and conflict between nations.

use bevy::prelude::*;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::collections::HashMap;

use crate::components::{
    Province, ProvinceResources,
    MineralType
};
use crate::terrain::TerrainType;
use crate::constants::*;
use crate::generation::tectonics::{TectonicSystem, BoundaryType};

// ============================================================================
// MINERAL GENERATION CONSTANTS
// ============================================================================

/// Number of ore veins to generate for each mineral type
pub const IRON_VEIN_COUNT: usize = 40;
pub const COPPER_VEIN_COUNT: usize = 30;
pub const TIN_VEIN_COUNT: usize = 12;  // Rare - creates bronze bottleneck
pub const GOLD_VEIN_COUNT: usize = 10;
pub const COAL_DEPOSIT_COUNT: usize = 25;
pub const GEM_VEIN_COUNT: usize = 5;

/// Falloff distance for ore vein abundance (in map units)
pub const VEIN_FALLOFF_DISTANCE: f32 = 500.0;

/// Base extraction rates per day (with level 1 mine)
pub const EXTRACTION_RATES: [(MineralType, f32); 7] = [
    (MineralType::Stone, 10.0),   // Fast extraction
    (MineralType::Coal, 8.0),     // Bulk extraction
    (MineralType::Iron, 5.0),     // Moderate
    (MineralType::Copper, 4.0),   // Moderate  
    (MineralType::Tin, 2.0),      // Slow (rare)
    (MineralType::Gold, 0.5),     // Very slow
    (MineralType::Gems, 0.1),     // Extremely slow
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

// ============================================================================
// ORE VEIN GENERATION
// ============================================================================

/// Represents a concentrated deposit of a mineral
#[derive(Debug, Clone)]
pub struct OreVein {
    pub position: Vec2,
    pub mineral_type: MineralType,
    pub richness: f32,  // 0.5 to 2.0 multiplier
}

/// Generate ore vein centers using clustering algorithm
pub fn generate_ore_veins(
    mineral_type: MineralType,
    count: usize,
    seed: u32,
    provinces: &[Province],
    terrain_bias: fn(&TerrainType, f32) -> f32,
) -> Vec<OreVein> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut veins = Vec::new();
    
    // Filter provinces by terrain suitability
    let suitable_provinces: Vec<&Province> = provinces.iter()
        .filter(|p| terrain_bias(&p.terrain, p.elevation) > 0.0)
        .collect();
    
    if suitable_provinces.is_empty() {
        return veins;
    }
    
    // Generate vein centers biased toward suitable terrain
    for _ in 0..count {
        let province = suitable_provinces.choose(&mut rng).unwrap();
        
        // Add some random offset within the province
        let offset_x = rng.gen_range(-HEX_SIZE_PIXELS..HEX_SIZE_PIXELS);
        let offset_y = rng.gen_range(-HEX_SIZE_PIXELS..HEX_SIZE_PIXELS);
        
        veins.push(OreVein {
            position: Vec2::new(
                province.position.x + offset_x,
                province.position.y + offset_y,
            ),
            mineral_type,
            richness: rng.gen_range(0.5..2.0),
        });
    }
    
    veins
}

/// Terrain bias functions for different minerals
pub fn iron_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    match terrain {
        TerrainType::Mountains => 3.0,
        TerrainType::Hills => 2.0,
        TerrainType::Tundra => 2.5,  // Arctic regions are iron-rich (Siberia, Sweden)
        TerrainType::Desert if elevation > 0.5 => 1.5,
        _ => 0.0,
    }
}

pub fn copper_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    match terrain {
        TerrainType::Mountains => 2.0,
        TerrainType::Hills => 1.5,
        TerrainType::Tundra => 1.8,  // Arctic copper deposits (Alaska, Russia)
        TerrainType::Desert if elevation > 0.4 => 1.0,
        _ => 0.0,
    }
}

pub fn tin_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    // Tin is very rare and specific
    match terrain {
        TerrainType::Mountains if elevation > 0.8 => 4.0,
        TerrainType::Hills if elevation > 0.65 => 1.0,
        TerrainType::Tundra if elevation > 0.6 => 0.8,  // Some arctic tin deposits
        _ => 0.0,
    }
}

pub fn gold_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    // Gold in high mountains, rivers, and arctic regions
    match terrain {
        TerrainType::Mountains if elevation > 0.85 => 5.0,
        TerrainType::River if elevation > 0.4 => 2.0,  // Alluvial gold
        TerrainType::Tundra if elevation > 0.65 => 1.5,  // Arctic gold (Alaska, Yukon)
        _ => 0.0,
    }
}

pub fn coal_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    // Coal in ancient swamps and permafrost regions
    match terrain {
        TerrainType::Forest if elevation < 0.4 => 3.0,
        TerrainType::Plains if elevation < 0.35 => 2.0,
        TerrainType::Tundra if elevation < 0.5 => 2.0,  // Permafrost coal (Siberia)
        TerrainType::Jungle if elevation < 0.3 => 1.5,
        _ => 0.0,
    }
}

pub fn gem_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    // Gems in the highest peaks and frozen tundra
    match terrain {
        TerrainType::Mountains if elevation > 0.9 => 10.0,
        TerrainType::Tundra if elevation > 0.7 => 3.0,  // Arctic diamonds (Canada, Russia)
        _ => 0.0,
    }
}

// ============================================================================
// RESOURCE CALCULATION
// ============================================================================

/// Calculate mineral abundance for a province based on distance to ore veins
pub fn calculate_province_resources(
    province: &Province,
    ore_veins: &HashMap<MineralType, Vec<OreVein>>,
) -> ProvinceResources {
    let mut resources = ProvinceResources::default();
    
    // Stone is everywhere but concentrated in rocky areas
    resources.stone = match province.terrain {
        TerrainType::Mountains => 80,
        TerrainType::Hills => 60,
        TerrainType::Tundra => 50,  // Arctic regions have exposed bedrock
        TerrainType::Desert => 40,
        TerrainType::Beach => 30,
        _ => 20,
    };
    
    // Calculate abundance for each mineral based on distance to veins
    if let Some(iron_veins) = ore_veins.get(&MineralType::Iron) {
        resources.iron = calculate_abundance(province.position, iron_veins, &province.terrain);
    }
    
    if let Some(copper_veins) = ore_veins.get(&MineralType::Copper) {
        resources.copper = calculate_abundance(province.position, copper_veins, &province.terrain);
    }
    
    if let Some(tin_veins) = ore_veins.get(&MineralType::Tin) {
        resources.tin = calculate_abundance(province.position, tin_veins, &province.terrain);
    }
    
    if let Some(gold_veins) = ore_veins.get(&MineralType::Gold) {
        resources.gold = calculate_abundance(province.position, gold_veins, &province.terrain);
    }
    
    if let Some(coal_veins) = ore_veins.get(&MineralType::Coal) {
        resources.coal = calculate_abundance(province.position, coal_veins, &province.terrain);
    }
    
    if let Some(gem_veins) = ore_veins.get(&MineralType::Gems) {
        resources.gems = calculate_abundance(province.position, gem_veins, &province.terrain);
    }
    
    resources
}

/// Calculate abundance based on distance to nearest vein with Gaussian falloff
fn calculate_abundance(
    position: Vec2,
    veins: &[OreVein],
    terrain: &TerrainType,
) -> u8 {
    if veins.is_empty() {
        return 0;
    }
    
    // Find closest vein
    let mut min_distance = f32::MAX;
    let mut closest_richness = 1.0;
    
    for vein in veins {
        let distance = position.distance(vein.position);
        if distance < min_distance {
            min_distance = distance;
            closest_richness = vein.richness;
        }
    }
    
    // Apply Gaussian falloff
    if min_distance > VEIN_FALLOFF_DISTANCE {
        return 0;
    }
    
    let base_abundance = 100.0 * (-min_distance / VEIN_FALLOFF_DISTANCE).exp();
    let terrain_multiplier = get_terrain_extraction_bonus(terrain);
    
    (base_abundance * closest_richness * terrain_multiplier).min(100.0) as u8
}

/// Terrain affects how easy resources are to extract
fn get_terrain_extraction_bonus(terrain: &TerrainType) -> f32 {
    match terrain {
        TerrainType::Mountains => 0.8,  // Harder to extract
        TerrainType::Hills => 1.0,      // Normal
        TerrainType::Plains => 1.2,     // Easier access
        TerrainType::Desert => 0.9,     // Some difficulty
        TerrainType::Forest => 0.95,    // Trees in the way
        TerrainType::Jungle => 0.7,     // Very difficult
        TerrainType::Tundra => 0.6,     // Frozen ground
        TerrainType::Ice => 0.3,        // Nearly impossible
        _ => 1.0,
    }
}

// ============================================================================
// WORLD GENERATION
// ============================================================================

/// Generate mineral resources for the entire world using tectonic data
pub fn generate_world_minerals_with_tectonics(
    seed: u32,
    provinces: &[Province],
    tectonics: &TectonicSystem,
) -> HashMap<u32, ProvinceResources> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut ore_veins: HashMap<MineralType, Vec<OreVein>> = HashMap::new();
    
    // Generate ore veins at tectonic features
    
    // Iron and copper at convergent boundaries (mountain building)
    let mut iron_veins = Vec::new();
    let mut copper_veins = Vec::new();
    for boundary in &tectonics.boundaries {
        if matches!(boundary.boundary_type, BoundaryType::Convergent { .. }) {
            // Place veins along boundary segments
            for segment in &boundary.segments {
                for _ in 0..2 {
                    let t = rng.gen::<f32>();
                    let pos = segment.start.lerp(segment.end, t);
                    iron_veins.push(OreVein {
                        position: pos,
                        mineral_type: MineralType::Iron,
                        richness: rng.gen_range(1.0..2.0),
                    });
                    if rng.gen_bool(0.5) {
                        copper_veins.push(OreVein {
                            position: pos + Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0)),
                            mineral_type: MineralType::Copper,
                            richness: rng.gen_range(0.8..1.5),
                        });
                    }
                }
            }
        }
    }
    
    // Gold at volcanic hotspots
    let mut gold_veins = Vec::new();
    for hotspot in &tectonics.hotspots {
        if rng.gen_bool(0.3) {  // Gold is rare
            gold_veins.push(OreVein {
                position: hotspot.position,
                mineral_type: MineralType::Gold,
                richness: rng.gen_range(1.5..3.0),  // Rich deposits
            });
        }
    }
    
    // Tin in specific mountain regions (rare)
    let mut tin_veins = Vec::new();
    let mountain_provinces: Vec<&Province> = provinces.iter()
        .filter(|p| p.terrain == TerrainType::Mountains && p.elevation > 0.8)
        .collect();
    for _ in 0..TIN_VEIN_COUNT {
        if let Some(province) = mountain_provinces.choose(&mut rng) {
            tin_veins.push(OreVein {
                position: province.position,
                mineral_type: MineralType::Tin,
                richness: rng.gen_range(1.0..2.0),
            });
        }
    }
    
    // Coal in ancient lowlands
    let mut coal_veins = Vec::new();
    let lowland_provinces: Vec<&Province> = provinces.iter()
        .filter(|p| (p.terrain == TerrainType::Forest || p.terrain == TerrainType::Plains) && p.elevation < 0.4)
        .collect();
    for _ in 0..COAL_DEPOSIT_COUNT {
        if let Some(province) = lowland_provinces.choose(&mut rng) {
            coal_veins.push(OreVein {
                position: province.position,
                mineral_type: MineralType::Coal,
                richness: rng.gen_range(0.8..1.5),
            });
        }
    }
    
    // Gems at the highest peaks
    let mut gem_veins = Vec::new();
    let peak_provinces: Vec<&Province> = provinces.iter()
        .filter(|p| p.terrain == TerrainType::Mountains && p.elevation > 0.9)
        .collect();
    for _ in 0..GEM_VEIN_COUNT {
        if let Some(province) = peak_provinces.choose(&mut rng) {
            gem_veins.push(OreVein {
                position: province.position,
                mineral_type: MineralType::Gems,
                richness: rng.gen_range(2.0..3.0),  // Very rich but rare
            });
        }
    }
    
    // Store veins in HashMap
    ore_veins.insert(MineralType::Iron, iron_veins);
    ore_veins.insert(MineralType::Copper, copper_veins);
    ore_veins.insert(MineralType::Tin, tin_veins);
    ore_veins.insert(MineralType::Gold, gold_veins);
    ore_veins.insert(MineralType::Coal, coal_veins);
    ore_veins.insert(MineralType::Gems, gem_veins);
    
    // Calculate resources for each province
    let mut minerals = HashMap::new();
    for province in provinces {
        let resources = calculate_province_resources(province, &ore_veins);
        minerals.insert(province.id, resources);
    }
    
    minerals
}

// ============================================================================
// HELPER FUNCTIONS FOR OVERLAY
// ============================================================================

/// Get mineral abundance for a specific province and mineral type
pub fn get_mineral_abundance(
    province_id: u32,
    mineral_type: MineralType,
    minerals: &HashMap<u32, ProvinceResources>,
) -> u8 {
    minerals.get(&province_id)
        .map(|resources| match mineral_type {
            MineralType::Iron => resources.iron,
            MineralType::Copper => resources.copper,
            MineralType::Tin => resources.tin,
            MineralType::Gold => resources.gold,
            MineralType::Coal => resources.coal,
            MineralType::Stone => resources.stone,
            MineralType::Gems => resources.gems,
            _ => 0,
        })
        .unwrap_or(0)
}

/// Calculate total mineral richness for a province
pub fn calculate_total_richness(resources: &ProvinceResources) -> f32 {
    // Weighted sum of all minerals
    let iron_value = resources.iron as f32 * 1.0;
    let copper_value = resources.copper as f32 * 1.5;
    let tin_value = resources.tin as f32 * 3.0;  // Rare
    let gold_value = resources.gold as f32 * 10.0;  // Very valuable
    let coal_value = resources.coal as f32 * 0.8;
    let stone_value = resources.stone as f32 * 0.2;  // Common
    let gem_value = resources.gems as f32 * 20.0;  // Extremely valuable
    
    (iron_value + copper_value + tin_value + gold_value + coal_value + stone_value + gem_value) / 100.0
}
