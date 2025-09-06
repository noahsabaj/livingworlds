//! Transportation infrastructure components
//!
//! Roads, rails, ports, and airports that enable movement and trade.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use crate::types::ProvinceId;

/// Road network component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RoadNetwork {
    pub total_length_km: Fixed32,
    pub paved_percentage: Fixed32,
    pub quality: RoadQuality,
    pub capacity: Fixed32,          // Vehicles per day
    pub maintenance_level: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoadQuality {
    Dirt,
    Gravel,
    Paved,
    Highway,
}

/// Railroad connection between provinces
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RailConnection {
    pub origin: ProvinceId,
    pub destination: ProvinceId,
    pub track_gauge: RailGauge,
    pub electrified: bool,
    pub capacity: Fixed32,           // Trains per day
    pub cargo_capacity: Fixed32,     // Tons per day
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RailGauge {
    Narrow,
    Standard,
    Broad,
}

/// Port facility component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub port_type: PortType,
    pub berths: u32,
    pub max_ship_tonnage: Fixed32,
    pub cargo_capacity: Fixed32,    // Tons per day
    pub passenger_capacity: u32,    // People per day
    pub facilities: PortFacilities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortType {
    Fishing,
    Commercial,
    Military,
    DeepWater,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortFacilities {
    pub warehouses: u32,
    pub cranes: u32,
    pub dry_docks: u32,
    pub fuel_storage: Fixed32,
}

/// Airport component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Airport {
    pub airport_class: AirportClass,
    pub runway_length: Fixed32,
    pub runways: u8,
    pub terminal_capacity: u32,     // Passengers per day
    pub cargo_capacity: Fixed32,    // Tons per day
    pub hangar_space: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AirportClass {
    Airstrip,
    Regional,
    National,
    International,
}

/// Bridge component - critical infrastructure
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Bridge {
    pub bridge_type: BridgeType,
    pub length: Fixed32,
    pub load_capacity: Fixed32,
    pub strategic_importance: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeType {
    Footbridge,
    Road,
    Rail,
    Combined,
}

/// Tunnel component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    pub tunnel_type: TunnelType,
    pub length_km: Fixed32,
    pub capacity: Fixed32,
    pub ventilation_quality: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelType {
    Mountain,
    Underwater,
    Urban,
}

/// Canal component - artificial waterway
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Canal {
    pub width: Fixed32,
    pub depth: Fixed32,
    pub lock_count: u32,
    pub ship_capacity: Fixed32,     // Ships per day
    pub strategic_value: Fixed32,   // Suez, Panama level importance
}