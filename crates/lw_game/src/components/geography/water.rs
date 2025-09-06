//! Water features - rivers, lakes, harbors, and water access
//!
//! Water is essential for life and trade. These components track water resources.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Water body component - rivers, lakes, seas
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WaterBody {
    pub water_type: WaterBodyType,
    pub volume: Fixed32,
    pub flow_rate: Option<Fixed32>,  // For rivers
    pub salinity: Salinity,
    pub navigability: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaterBodyType {
    Ocean,
    Sea,
    Lake,
    River,
    Stream,
    Pond,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Salinity {
    Fresh,
    Brackish,
    Salt,
}

/// Harbor component - natural or constructed ports
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Harbor {
    pub harbor_type: HarborType,
    pub natural_protection: Fixed32,
    pub depth: Fixed32,
    pub capacity: Fixed32,
    pub ice_free_months: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarborType {
    Natural,
    Artificial,
    Riverine,
    Protected,
}

/// River system component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RiverSystem {
    pub source: Entity,           // Mountain/spring source
    pub mouth: Entity,            // Ocean/lake destination
    pub tributaries: Vec<Entity>, // Connected rivers
    pub length_km: Fixed32,
    pub average_width: Fixed32,
    pub flood_cycle: FloodCycle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FloodCycle {
    Annual { peak_month: u8, intensity: Fixed32 },
    Irregular { average_interval_years: Fixed32 },
    None,
}

/// Groundwater component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Groundwater {
    pub aquifer_depth: Fixed32,
    pub water_quality: Fixed32,
    pub recharge_rate: Fixed32,
    pub extraction_rate: Fixed32,
    pub total_capacity: Fixed32,
}

/// Water access component - how entities access water
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WaterAccess {
    pub access_type: WaterAccessType,
    pub reliability: Fixed32,
    pub quality: Fixed32,
    pub quantity_available: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaterAccessType {
    Coastal,
    Riverine,
    Lake,
    Well,
    Spring,
    Aqueduct,
    None,
}

/// Irrigation potential
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IrrigationPotential {
    pub water_source: Entity,
    pub irrigable_area: Fixed32,
    pub seasonal_availability: SeasonalWaterAvailability,
    pub infrastructure_required: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalWaterAvailability {
    pub spring: Fixed32,
    pub summer: Fixed32,
    pub autumn: Fixed32,
    pub winter: Fixed32,
}

/// Fishing grounds component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FishingGrounds {
    pub productivity: Fixed32,
    pub seasonal_variation: Fixed32,
    pub overfishing_risk: Fixed32,
    pub commercial_value: Fixed32,
}