//! Goods and valuation components
//!
//! Types of goods that can be traded and how individuals value them.
//! Each person values goods differently based on their subjective needs.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Good types that can be traded
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GoodType {
    // Basic necessities
    Food,
    Water,
    Clothing,
    Shelter,
    
    // Raw materials (consolidated from ResourceType)
    Wood,
    Stone,
    IronOre,
    Coal,
    Gold,
    Oil,
    RareEarths,
    Uranium,
    
    // Processed goods
    Tools,
    Weapons,
    Furniture,
    Cloth,
    
    // Luxury items
    Jewelry,
    Art,
    Spices,
    Wine,
    
    // Services (also traded)
    Labor,
    Transport,
    Protection,
    Knowledge,
}

/// Individual valuation of goods
/// Each person values goods differently based on their needs and situation
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IndividualValuation {
    pub valuations: Vec<GoodValuation>,
    pub needs_satisfaction: Fixed32,
    pub wealth_level: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodValuation {
    pub good: GoodType,
    pub subjective_value: Fixed32,  // How much this individual values it
    pub urgency: Fixed32,            // How urgently they need it
    pub quantity_desired: Fixed32,  // How much they want
}

/// Time preference component - Austrian concept
/// How much individuals prefer present goods over future goods
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TimePreference {
    pub discount_rate: Fixed32,     // How much less they value future goods
    pub savings_rate: Fixed32,      // Tendency to save vs consume
    pub investment_horizon: u64,    // How far ahead they plan
}