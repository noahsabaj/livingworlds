//! Natural resource components - deposits and extraction potential
//!
//! Resources are separate entities with their own components, referenced by provinces.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use crate::components::economics::GoodType;

/// Resource deposit component - a specific extractable resource
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDeposit {
    pub resource_type: GoodType,
    pub total_reserves: Fixed32,
    pub remaining_reserves: Fixed32,
    pub extraction_difficulty: Fixed32,
    pub ore_quality: Fixed32,
    pub discovery_date: Option<u64>,
}

/// Fossil fuel deposit - oil, coal, gas
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FossilFuelDeposit {
    pub fuel_type: FossilFuelType,
    pub deposit_size: Fixed32,
    pub extraction_cost: Fixed32,
    pub energy_content: Fixed32,
    pub environmental_impact: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FossilFuelType {
    Coal,
    Oil,
    NaturalGas,
    Peat,
    Shale,
}

/// Metal ore deposit
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MetalDeposit {
    pub metal_type: MetalType,
    pub ore_concentration: Fixed32,
    pub deposit_volume: Fixed32,
    pub mining_depth: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetalType {
    Iron,
    Copper,
    Gold,
    Silver,
    Aluminum,
    Tin,
    Lead,
    Zinc,
    Platinum,
}

/// Rare earth element deposit
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RareEarthDeposit {
    pub element: RareEarthElement,
    pub concentration: Fixed32,
    pub extraction_complexity: Fixed32,
    pub strategic_importance: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RareEarthElement {
    Lithium,
    Cobalt,
    Neodymium,
    Dysprosium,
    Uranium,
    Thorium,
}

/// Agricultural potential component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalPotential {
    pub soil_fertility: Fixed32,
    pub irrigation_potential: Fixed32,
    pub crop_suitability: CropSuitability,
    pub livestock_capacity: Fixed32,
    pub growing_seasons_per_year: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropSuitability {
    pub grains: Fixed32,
    pub vegetables: Fixed32,
    pub fruits: Fixed32,
    pub cash_crops: Fixed32,
}

/// Forest resources component  
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ForestResources {
    pub timber_volume: Fixed32,
    pub regeneration_rate: Fixed32,
    pub biodiversity: Fixed32,
    pub non_timber_products: Fixed32,  // Nuts, berries, medicinal plants
}

/// Marine resources component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MarineResources {
    pub fish_stocks: Fixed32,
    pub regeneration_rate: Fixed32,
    pub commercial_species: Vec<FishSpecies>,
    pub overfishing_risk: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FishSpecies {
    pub name: String,
    pub population: Fixed32,
    pub commercial_value: Fixed32,
}

/// Renewable energy potential
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RenewableEnergyPotential {
    pub solar_irradiance: Fixed32,
    pub wind_speed_average: Fixed32,
    pub hydroelectric_potential: Fixed32,
    pub geothermal_activity: Fixed32,
    pub tidal_energy: Fixed32,
}

/// Construction materials availability
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionMaterials {
    pub stone_quarries: Fixed32,
    pub sand_deposits: Fixed32,
    pub clay_deposits: Fixed32,
    pub limestone: Fixed32,
}

/// Resource extraction infrastructure
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionInfrastructure {
    pub extraction_rate: Fixed32,
    pub efficiency: Fixed32,
    pub technology_level: u8,
    pub environmental_damage: Fixed32,
    pub workers_required: u32,
}