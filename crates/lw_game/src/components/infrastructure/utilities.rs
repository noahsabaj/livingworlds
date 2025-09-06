//! Utility infrastructure components
//!
//! Power generation, water systems, sewage, and communications.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Power generation facility
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PowerPlant {
    pub plant_type: PowerPlantType,
    pub capacity_mw: Fixed32,
    pub efficiency: Fixed32,
    pub fuel_consumption: Fixed32,
    pub emissions: Fixed32,
    pub workers_required: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerPlantType {
    Coal,
    Gas,
    Oil,
    Nuclear,
    Hydro,
    Solar,
    Wind,
    Geothermal,
}

/// Electrical grid component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalGrid {
    pub transmission_capacity: Fixed32,
    pub distribution_efficiency: Fixed32,
    pub coverage_percentage: Fixed32,
    pub grid_stability: Fixed32,
}

/// Water infrastructure component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WaterSystem {
    pub source_type: WaterSourceType,
    pub treatment_capacity: Fixed32,
    pub distribution_network_km: Fixed32,
    pub storage_capacity: Fixed32,
    pub water_quality: Fixed32,
    pub coverage_percentage: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaterSourceType {
    River,
    Lake,
    Groundwater,
    Desalination,
    Recycled,
}

/// Sewage system component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SewageSystem {
    pub collection_network_km: Fixed32,
    pub treatment_capacity: Fixed32,
    pub treatment_level: TreatmentLevel,
    pub coverage_percentage: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatmentLevel {
    None,
    Primary,
    Secondary,
    Tertiary,
}

/// Communications infrastructure
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationsNetwork {
    pub network_type: NetworkType,
    pub coverage: Fixed32,
    pub bandwidth: Fixed32,
    pub reliability: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    Telegraph,
    Telephone,
    Radio,
    Television,
    Internet,
    Satellite,
}

/// Waste management system
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WasteManagement {
    pub collection_coverage: Fixed32,
    pub landfill_capacity: Fixed32,
    pub recycling_rate: Fixed32,
    pub composting_rate: Fixed32,
    pub incineration_capacity: Fixed32,
}

/// Dam component - multipurpose infrastructure
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Dam {
    pub dam_type: DamType,
    pub height: Fixed32,
    pub reservoir_capacity: Fixed32,
    pub power_generation_mw: Fixed32,
    pub irrigation_capacity: Fixed32,
    pub flood_control_capacity: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamType {
    Gravity,
    Arch,
    Embankment,
    Run_of_River,
}