//! Mineral resource system for Living Worlds
//! 
//! This module handles generation, extraction, and processing of mineral resources.
//! Resources drive technological advancement, trade, and conflict between nations.

use bevy::prelude::*;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::collections::HashMap;

use crate::components::{
    Province, ProvinceResources, ProvinceInfrastructure, 
    Nation, NationStockpile, NationTechnology,
    MineralType, TechnologyAge
};
use crate::terrain::TerrainType;
use crate::constants::*;

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
        TerrainType::Desert if elevation > 0.5 => 1.5,
        TerrainType::Tundra if elevation > 0.6 => 1.0,
        _ => 0.0,
    }
}

pub fn copper_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    match terrain {
        TerrainType::Mountains => 2.0,
        TerrainType::Hills => 1.5,
        TerrainType::Desert if elevation > 0.4 => 1.0,
        _ => 0.0,
    }
}

pub fn tin_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    // Tin is very rare and specific
    match terrain {
        TerrainType::Mountains if elevation > 0.8 => 4.0,
        TerrainType::Hills if elevation > 0.65 => 1.0,
        _ => 0.0,
    }
}

pub fn gold_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    // Gold only in high mountains and rivers
    match terrain {
        TerrainType::Mountains if elevation > 0.85 => 5.0,
        TerrainType::River if elevation > 0.4 => 2.0,  // Alluvial gold
        _ => 0.0,
    }
}

pub fn coal_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    // Coal in ancient swamps - low elevation forests/plains
    match terrain {
        TerrainType::Forest if elevation < 0.4 => 3.0,
        TerrainType::Plains if elevation < 0.35 => 2.0,
        TerrainType::Jungle if elevation < 0.3 => 1.5,
        _ => 0.0,
    }
}

pub fn gem_terrain_bias(terrain: &TerrainType, elevation: f32) -> f32 {
    // Gems only in the highest peaks
    match terrain {
        TerrainType::Mountains if elevation > 0.9 => 10.0,
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
// EXTRACTION SYSTEMS
// ============================================================================

/// System that handles resource extraction from mines
pub fn resource_extraction_system(
    mut provinces: Query<(&Province, &ProvinceResources, &mut ProvinceInfrastructure)>,
    mut nations: Query<(&Nation, &mut NationStockpile, &NationTechnology)>,
    time: Res<Time>,
) {
    for (province, resources, infrastructure) in provinces.iter_mut() {
        // Skip if no mine or no nation owns it
        if infrastructure.mine_level == 0 || province.nation_id.is_none() {
            continue;
        }
        
        // Find the owning nation
        let nation_id = province.nation_id.unwrap();
        for (nation, mut stockpile, technology) in nations.iter_mut() {
            if nation.id != nation_id {
                continue;
            }
            
            // Calculate extraction rate based on mine level and technology
            let base_rate = infrastructure.mine_level as f32 * 0.2;
            let tech_bonus = technology.mining_efficiency;
            let worker_efficiency = (infrastructure.workers as f32 / 100.0).min(1.0);
            
            let extraction_multiplier = base_rate * tech_bonus * worker_efficiency * time.delta_secs();
            
            // Extract each resource
            if resources.iron > 0 {
                let extracted = get_extraction_rate(MineralType::Iron) * extraction_multiplier;
                stockpile.iron += extracted;
            }
            
            if resources.copper > 0 {
                let extracted = get_extraction_rate(MineralType::Copper) * extraction_multiplier;
                stockpile.copper += extracted;
            }
            
            if resources.tin > 0 {
                let extracted = get_extraction_rate(MineralType::Tin) * extraction_multiplier;
                stockpile.tin += extracted;
            }
            
            if resources.gold > 0 {
                let extracted = get_extraction_rate(MineralType::Gold) * extraction_multiplier;
                stockpile.gold += extracted;
            }
            
            if resources.coal > 0 {
                let extracted = get_extraction_rate(MineralType::Coal) * extraction_multiplier;
                stockpile.coal += extracted;
            }
            
            if resources.stone > 0 {
                let extracted = get_extraction_rate(MineralType::Stone) * extraction_multiplier;
                stockpile.stone += extracted;
            }
            
            if resources.gems > 0 {
                let extracted = get_extraction_rate(MineralType::Gems) * extraction_multiplier;
                stockpile.gems += extracted;
            }
        }
    }
}

/// Get base extraction rate for a mineral type
fn get_extraction_rate(mineral: MineralType) -> f32 {
    EXTRACTION_RATES.iter()
        .find(|(m, _)| *m == mineral)
        .map(|(_, rate)| *rate)
        .unwrap_or(1.0)
}

// ============================================================================
// TECHNOLOGY PROGRESSION
// ============================================================================

/// System that advances nation technology based on available resources
pub fn technology_progression_system(
    mut nations: Query<(&mut NationTechnology, &NationStockpile)>,
) {
    for (mut technology, stockpile) in nations.iter_mut() {
        // Check if nation can advance to next age
        let can_advance = match technology.age {
            TechnologyAge::StoneAge => {
                // Advance to Copper Age if copper discovered
                stockpile.copper > 10.0
            },
            TechnologyAge::CopperAge => {
                // Advance to Bronze Age if both copper and tin available
                stockpile.copper > 20.0 && stockpile.tin > 10.0
            },
            TechnologyAge::BronzeAge => {
                // Advance to Iron Age if iron available
                stockpile.iron > 30.0
            },
            TechnologyAge::IronAge => {
                // Advance to Steel Age if iron and coal available
                stockpile.iron > 50.0 && stockpile.coal > 50.0
            },
            TechnologyAge::SteelAge => {
                // Advance to Industrial if massive resources
                stockpile.steel > 100.0 && stockpile.coal > 200.0
            },
            _ => false,
        };
        
        if can_advance {
            technology.age = match technology.age {
                TechnologyAge::StoneAge => TechnologyAge::CopperAge,
                TechnologyAge::CopperAge => TechnologyAge::BronzeAge,
                TechnologyAge::BronzeAge => TechnologyAge::IronAge,
                TechnologyAge::IronAge => TechnologyAge::SteelAge,
                TechnologyAge::SteelAge => TechnologyAge::Industrial,
                TechnologyAge::Industrial => TechnologyAge::Modern,
                TechnologyAge::Modern => TechnologyAge::Modern,
            };
            
            // Improve efficiency with each age
            technology.mining_efficiency *= 1.2;
            technology.forge_efficiency *= 1.2;
            
            println!("Nation advanced to {:?}!", technology.age);
        }
    }
}

// ============================================================================
// PROCESSING CHAINS
// ============================================================================

/// System that processes raw materials into alloys
pub fn resource_processing_system(
    mut nations: Query<(&mut NationStockpile, &NationTechnology)>,
    time: Res<Time>,
) {
    for (mut stockpile, technology) in nations.iter_mut() {
        let delta = time.delta_secs();
        
        // Bronze production (requires copper + tin)
        if technology.age as u8 >= TechnologyAge::BronzeAge as u8 {
            let bronze_rate = 0.5 * technology.forge_efficiency * delta;
            let max_bronze = (stockpile.copper / 2.0).min(stockpile.tin);
            if max_bronze > 0.0 {
                let produced = max_bronze.min(bronze_rate);
                stockpile.copper -= produced * 2.0;
                stockpile.tin -= produced;
                stockpile.bronze += produced;
            }
        }
        
        // Steel production (requires iron + coal)
        if technology.age as u8 >= TechnologyAge::SteelAge as u8 {
            let steel_rate = 0.3 * technology.forge_efficiency * delta;
            let max_steel = stockpile.iron.min(stockpile.coal);
            if max_steel > 0.0 {
                let produced = max_steel.min(steel_rate);
                stockpile.iron -= produced;
                stockpile.coal -= produced;
                stockpile.steel += produced;
            }
        }
    }
}

// ============================================================================
// VISUALIZATION HELPERS
// ============================================================================
// Note: The overlay rendering system has been moved to overlay.rs
// This module now only provides mineral-specific calculations and color functions

/// Get abundance value for a specific mineral type
pub fn get_mineral_abundance(resources: &ProvinceResources, mineral_type: MineralType) -> u8 {
    match mineral_type {
        MineralType::Iron => resources.iron,
        MineralType::Copper => resources.copper,
        MineralType::Tin => resources.tin,
        MineralType::Gold => resources.gold,
        MineralType::Coal => resources.coal,
        MineralType::Stone => resources.stone,
        MineralType::Gems => resources.gems,
        _ => 0, // Bronze and Steel are alloys, not raw resources
    }
}



/// Calculate total mineral richness
pub fn calculate_total_richness(resources: &ProvinceResources) -> f32 {
    // Weight different minerals by rarity/value
    let iron_weight = 1.0;
    let copper_weight = 1.2;
    let tin_weight = 3.0;  // Very rare
    let gold_weight = 5.0;
    let coal_weight = 0.8;
    let stone_weight = 0.2;
    let gems_weight = 10.0;
    
    (resources.iron as f32 * iron_weight +
     resources.copper as f32 * copper_weight +
     resources.tin as f32 * tin_weight +
     resources.gold as f32 * gold_weight +
     resources.coal as f32 * coal_weight +
     resources.stone as f32 * stone_weight +
     resources.gems as f32 * gems_weight) / 100.0
}




// ============================================================================
// WORLD GENERATION
// ============================================================================

/// Generate mineral resources for the entire world during setup
/// This centralizes all mineral generation logic that was previously in setup.rs
pub fn generate_world_minerals(
    seed: u32,
    provinces: &[Province],
) -> HashMap<u32, ProvinceResources> {
    println!("Generating mineral resources...");
    
    // Generate ore veins for each mineral type
    let mut ore_veins: HashMap<MineralType, Vec<_>> = HashMap::new();
    
    // Iron - common in mountains and hills
    ore_veins.insert(
        MineralType::Iron,
        generate_ore_veins(
            MineralType::Iron,
            IRON_VEIN_COUNT,
            seed.wrapping_add(1000),
            provinces,
            iron_terrain_bias,
        ),
    );
    
    // Copper - less common, in mountains and hills
    ore_veins.insert(
        MineralType::Copper,
        generate_ore_veins(
            MineralType::Copper,
            COPPER_VEIN_COUNT,
            seed.wrapping_add(2000),
            provinces,
            copper_terrain_bias,
        ),
    );
    
    // Tin - rare, essential for bronze
    ore_veins.insert(
        MineralType::Tin,
        generate_ore_veins(
            MineralType::Tin,
            TIN_VEIN_COUNT,
            seed.wrapping_add(3000),
            provinces,
            tin_terrain_bias,
        ),
    );
    
    // Gold - rare, in high mountains
    ore_veins.insert(
        MineralType::Gold,
        generate_ore_veins(
            MineralType::Gold,
            GOLD_VEIN_COUNT,
            seed.wrapping_add(4000),
            provinces,
            gold_terrain_bias,
        ),
    );
    
    // Coal - in ancient swamps (lowland forests)
    ore_veins.insert(
        MineralType::Coal,
        generate_ore_veins(
            MineralType::Coal,
            COAL_DEPOSIT_COUNT,
            seed.wrapping_add(5000),
            provinces,
            coal_terrain_bias,
        ),
    );
    
    // Gems - very rare, highest peaks only
    ore_veins.insert(
        MineralType::Gems,
        generate_ore_veins(
            MineralType::Gems,
            GEM_VEIN_COUNT,
            seed.wrapping_add(6000),
            provinces,
            gem_terrain_bias,
        ),
    );
    
    // Calculate resources for each province
    let mut province_resources: HashMap<u32, ProvinceResources> = HashMap::new();
    for province in provinces.iter() {
        let resources = calculate_province_resources(province, &ore_veins);
        province_resources.insert(province.id, resources);
    }
    
    // Log mineral distribution statistics
    let total_iron: u32 = province_resources.values().map(|r| r.iron as u32).sum();
    let total_copper: u32 = province_resources.values().map(|r| r.copper as u32).sum();
    let total_tin: u32 = province_resources.values().map(|r| r.tin as u32).sum();
    let total_gold: u32 = province_resources.values().map(|r| r.gold as u32).sum();
    let total_coal: u32 = province_resources.values().map(|r| r.coal as u32).sum();
    let total_gems: u32 = province_resources.values().map(|r| r.gems as u32).sum();
    
    println!("Mineral distribution:");
    println!("  Iron: {} units across provinces", total_iron);
    println!("  Copper: {} units", total_copper);
    println!("  Tin: {} units (rare!)", if total_tin < 60000 { 
        format!("{}", total_tin) 
    } else { 
        total_tin.to_string() 
    });
    println!("  Gold: {} units", total_gold);
    println!("  Coal: {} units", total_coal);
    println!("  Gems: {} units (very rare!)", if total_gems < 20000 { 
        format!("{}", total_gems) 
    } else { 
        total_gems.to_string() 
    });
    
    province_resources
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin that manages all mineral-related systems
pub struct MineralPlugin;

impl Plugin for MineralPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                resource_extraction_system,
                technology_progression_system,
                resource_processing_system,
            ).chain());
    }
}