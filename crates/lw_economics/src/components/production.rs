//! Production and capital components
//!
//! Components for modeling production capabilities, capital goods,
//! and the structure of production following Austrian capital theory.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use super::goods::GoodType;

/// Production capability component
#[derive(Component, Debug, Clone)]
pub struct ProductionCapability {
    pub specialization: GoodType,
    pub skill_level: Fixed32,
    pub production_rate: Fixed32,
    pub quality_level: Fixed32,
    pub tools_available: Vec<Entity>,  // References to tool entities
}

/// Capital structure component
/// Capital goods used in production processes
#[derive(Component, Debug, Clone)]
pub struct CapitalGood {
    pub capital_type: CapitalType,
    pub production_stage: u32,           // Distance from final consumption
    pub durability: Fixed32,             // How long it lasts
    pub productivity: Fixed32,           // Multiplier on output
    pub maintenance_required: Fixed32,  // Resources needed to maintain
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapitalType {
    Tool,        // Hand tools
    Machine,     // Complex machinery
    Building,    // Production facilities
    Infrastructure, // Roads, bridges, etc.
}

/// Business cycle component - Austrian theory
/// Tracks malinvestment from artificial credit expansion
#[derive(Component, Debug, Clone)]
pub struct BusinessCyclePosition {
    pub credit_expansion_rate: Fixed32,  // How much artificial credit
    pub malinvestment_level: Fixed32,    // Bad investments made
    pub correction_severity: Fixed32,    // How bad the bust will be
    pub time_to_correction: u64,         // When reality hits
}

/// Competition component
/// Tracks competitive dynamics in markets
#[derive(Component, Debug, Clone)]
pub struct Competition {
    pub market_concentration: Fixed32,   // How monopolistic
    pub entry_barriers: Fixed32,        // How hard to compete
    pub innovation_rate: Fixed32,       // How fast things change
}

/// Economic calculation component
/// Can only work with market prices, fails under central planning
#[derive(Component, Debug, Clone)]
pub struct EconomicCalculation {
    pub price_signals_available: bool,   // Do we have market prices?
    pub calculation_accuracy: Fixed32,   // How well can we calculate profit/loss
    pub planning_horizon: u64,           // How far ahead we can plan
}

/// Production facility component
#[derive(Component, Debug, Clone)]
pub struct ProductionFacility {
    pub facility_type: FacilityType,
    pub owner: Entity,                  // Who owns it
    pub workers: Vec<Entity>,           // Employed individuals
    pub input_goods: Vec<GoodType>,     // What it needs
    pub output_good: GoodType,          // What it produces
    pub efficiency: Fixed32,            // How well it operates
    pub working_conditions: Fixed32,    // Affects worker satisfaction
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacilityType {
    Workshop,
    Factory,
    Farm,
    Mine,
    Service,
}

/// Economic system types that emerge from choices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EconomicSystem {
    FreeMarket {
        property_rights_strength: Fixed32,
        contract_enforcement: Fixed32,
    },
    MixedEconomy {
        government_share: Fixed32,
        regulation_level: Fixed32,
    },
    CentralPlanning {
        planning_efficiency: Fixed32,  // Always less than market
        corruption_level: Fixed32,
    },
    Traditional {
        custom_strength: Fixed32,
        change_resistance: Fixed32,
    },
}

/// Industry classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndustryType {
    Agriculture,
    Mining,
    Manufacturing,
    Construction,
    Trade,
    Services,
    Finance,
    Government,
    Military,
}