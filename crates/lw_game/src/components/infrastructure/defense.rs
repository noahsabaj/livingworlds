//! Defense infrastructure components
//!
//! Military installations and fortifications built for defense.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Fortification component - defensive structures
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Fortification {
    pub fortification_type: FortificationType,
    pub defense_bonus: Fixed32,
    pub garrison_capacity: u32,
    pub supply_storage: Fixed32,
    pub construction_quality: Fixed32,
    pub siege_resistance: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FortificationType {
    Palisade,
    Wall,
    Castle,
    StarFort,
    Bunker,
    ModernBase,
}

/// Military base component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MilitaryBase {
    pub base_type: MilitaryBaseType,
    pub troop_capacity: u32,
    pub training_facilities: TrainingFacilities,
    pub logistics_capacity: Fixed32,
    pub command_level: CommandLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MilitaryBaseType {
    Camp,
    Fort,
    Garrison,
    TrainingGround,
    HeadQuarters,
    NavalBase,
    AirBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingFacilities {
    pub barracks: u32,
    pub drill_grounds: u32,
    pub weapon_ranges: u32,
    pub officer_academy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandLevel {
    Local,
    Regional,
    Theater,
    Strategic,
}

/// Supply depot component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SupplyDepot {
    pub storage_capacity: Fixed32,
    pub ammunition_storage: Fixed32,
    pub fuel_storage: Fixed32,
    pub food_storage: Fixed32,
    pub medical_supplies: Fixed32,
    pub distribution_efficiency: Fixed32,
}

/// Coastal defense component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CoastalDefense {
    pub battery_count: u32,
    pub gun_caliber: Fixed32,
    pub range_km: Fixed32,
    pub fire_control_quality: Fixed32,
    pub ammunition_supply: Fixed32,
}

/// Air defense component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct AirDefense {
    pub defense_type: AirDefenseType,
    pub coverage_radius: Fixed32,
    pub tracking_quality: Fixed32,
    pub interception_rate: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AirDefenseType {
    AAGuns,
    SAM,
    CIWS,
    LaserDefense,
}

/// Border fortification component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct BorderFortification {
    pub length_km: Fixed32,
    pub fortification_density: Fixed32,
    pub obstacle_type: ObstacleType,
    pub surveillance_coverage: Fixed32,
    pub quick_reaction_forces: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObstacleType {
    Ditch,
    Fence,
    Wall,
    Minefield,
    TankTraps,
    ConcertinWire,
}

/// Radar station component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RadarStation {
    pub radar_type: RadarType,
    pub detection_range: Fixed32,
    pub tracking_capacity: u32,
    pub resolution_quality: Fixed32,
    pub jamming_resistance: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadarType {
    EarlyWarning,
    FireControl,
    Weather,
    AirTraffic,
    OTH,  // Over-the-horizon
}

/// Military communications component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MilitaryCommunications {
    pub comm_type: MilitaryCommType,
    pub encryption_level: Fixed32,
    pub range_km: Fixed32,
    pub bandwidth: Fixed32,
    pub jamming_resistance: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MilitaryCommType {
    Radio,
    Telegraph,
    Telephone,
    Satellite,
    Fiber,
    Quantum,
}