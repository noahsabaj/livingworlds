//! Configuration types and data structures
//!
//! This module contains all the types related to world generation settings,
//! including the main settings struct and all configuration enums.

use crate::name_generator::{NameGenerator, NameType};
use crate::resources::WorldSize;
use rand::Rng;

/// Complete world generation settings
#[derive(Resource, Clone, Debug)]
pub struct WorldGenerationSettings {
    pub world_name: String,
    pub world_size: WorldSize,
    pub custom_dimensions: Option<(u32, u32)>,
    pub seed: u32,
    pub preset: WorldPreset,

    // Advanced - Geography
    pub continent_count: u32,
    pub island_frequency: IslandFrequency,
    pub ocean_coverage: f32,
    pub climate_type: ClimateType,
    pub mountain_density: MountainDensity,
    pub river_density: f32,

    // Advanced - Civilizations
    pub starting_nations: u32,
    pub aggression_level: AggressionLevel,
    pub tech_progression_speed: f32,
    pub empire_stability: f32,
    pub trade_propensity: TradePropensity,

    // Advanced - Resources
    pub resource_abundance: ResourceAbundance,
    pub mineral_distribution: MineralDistribution,
    pub fertility_variance: f32,
}

impl Default for WorldGenerationSettings {
    fn default() -> Self {
        let mut gen = NameGenerator::new();
        Self {
            world_name: gen.generate(NameType::World),
            world_size: WorldSize::Medium,
            custom_dimensions: None,
            seed: rand::thread_rng().gen(),
            preset: WorldPreset::Balanced,

            continent_count: 7,
            island_frequency: IslandFrequency::Moderate,
            ocean_coverage: 0.6,
            climate_type: ClimateType::Mixed,
            mountain_density: MountainDensity::Normal,
            river_density: 1.0,

            starting_nations: 8,
            aggression_level: AggressionLevel::Balanced,
            tech_progression_speed: 1.0,
            empire_stability: 0.5,
            trade_propensity: TradePropensity::Normal,

            resource_abundance: ResourceAbundance::Normal,
            mineral_distribution: MineralDistribution::Clustered,
            fertility_variance: 0.5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WorldPreset {
    Balanced,
    Pangaea,
    Archipelago,
    IceAge,
    DesertWorld,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IslandFrequency {
    None,
    Sparse,
    Moderate,
    Abundant,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClimateType {
    Arctic,
    Temperate,
    Tropical,
    Desert,
    Mixed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MountainDensity {
    Few,
    Normal,
    Many,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AggressionLevel {
    Peaceful,
    Balanced,
    Warlike,
    Chaotic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TradePropensity {
    Isolationist,
    Normal,
    Mercantile,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResourceAbundance {
    Scarce,
    Normal,
    Rich,
    Bountiful,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MineralDistribution {
    Even,
    Clustered,
    Strategic,
}

// Preset application logic
impl WorldGenerationSettings {
    pub fn apply_preset(&mut self) {
        match self.preset {
            WorldPreset::Balanced => {
                self.continent_count = 7;
                self.island_frequency = IslandFrequency::Moderate;
                self.ocean_coverage = 0.6;
                self.climate_type = ClimateType::Mixed;
            }
            WorldPreset::Pangaea => {
                self.continent_count = 1;
                self.island_frequency = IslandFrequency::Sparse;
                self.ocean_coverage = 0.7;
                self.climate_type = ClimateType::Mixed;
            }
            WorldPreset::Archipelago => {
                self.continent_count = 3;
                self.island_frequency = IslandFrequency::Abundant;
                self.ocean_coverage = 0.75;
                self.climate_type = ClimateType::Tropical;
            }
            WorldPreset::IceAge => {
                self.continent_count = 5;
                self.island_frequency = IslandFrequency::Sparse;
                self.ocean_coverage = 0.5;
                self.climate_type = ClimateType::Arctic;
            }
            WorldPreset::DesertWorld => {
                self.continent_count = 4;
                self.island_frequency = IslandFrequency::None;
                self.ocean_coverage = 0.3;
                self.climate_type = ClimateType::Desert;
            }
            WorldPreset::Custom => {
                // Don't change settings for custom
            }
        }
    }
}

// Add Resource derive
use bevy::prelude::Resource;
