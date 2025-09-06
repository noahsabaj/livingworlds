//! Economics domain module - markets, trade, and transactions
//!
//! Implements Austrian economics principles where prices emerge from
//! individual human actions rather than being centrally determined.

// Submodules
pub mod trade;
pub mod transactions;
pub mod money;
pub mod banking;
pub mod credit;
pub mod goods;
pub mod markets;
pub mod production;

// Re-export key types for convenience
pub use trade::*;
pub use transactions::*;
pub use money::*;
pub use banking::*;
pub use credit::*;
pub use goods::*;
pub use markets::*;
pub use production::*;

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Additional economic components that don't fit in specific modules

/// Economic performance tracking
#[derive(Component, Clone, Debug, Default)]
pub struct EconomicPerformance {
    pub gdp_growth: Fixed32,    // -1 to 1, economic growth rate
    pub inequality: Fixed32,    // Gini coefficient
    pub innovation: Fixed32,    // Rate of technological advancement
    pub stability: Fixed32,     // Economic stability measure
}

/// Economic shock component for crises
#[derive(Component, Clone, Debug)]
pub struct EconomicShock {
    pub shock_type: ShockType,
    pub severity: Fixed32,
    pub duration: u64,
    pub affected_sectors: Vec<IndustryType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShockType {
    SupplyShock,      // Resource shortage
    DemandShock,      // Sudden demand collapse
    FinancialCrisis,  // Banking/credit crisis
    WarDisruption,    // War affecting economy
    NaturalDisaster,  // Earthquake, flood, etc.
    Pandemic,         // Disease outbreak
}

// Helper types that were duplicated - now consolidated

/// Crop types for farming
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum CropType {
    Grain,
    Vegetables,
    Fruit,
    Livestock,
}

/// Craft specializations
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum CraftType {
    Blacksmith,
    Carpenter,
    Tailor,
    Jeweler,
    Potter,
}

/// Processing types for mills
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ProcessingType {
    GrainToFlour,
    OreToMetal,
    WoodToLumber,
    CottonToCloth,
}

/// Ownership structure
#[derive(Clone, Debug)]
pub enum Owner {
    Individual(Entity),      // Private ownership
    State(Entity),          // Government ownership
    Collective(Vec<Entity>), // Worker cooperative
    Noble(Entity),          // Feudal ownership
}

/// Input requirements for production
#[derive(Clone, Debug)]
pub struct InputRequirement {
    pub good: GoodType,
    pub quantity_per_output: Fixed32,
    pub quality_requirement: Fixed32,
}

/// Output products from production
#[derive(Clone, Debug)]
pub struct OutputProduct {
    pub good: GoodType,
    pub quantity: Fixed32,
    pub quality: Fixed32,
}