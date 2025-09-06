//! Market and price discovery components
//!
//! Markets emerge from individual actions. Prices are discovered through
//! the matching of buy and sell orders, never centrally determined.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use super::goods::GoodType;

/// Marker component indicating economic activity is present
/// Only added when actual economic activity occurs
#[derive(Component, Debug, Default, Clone)]
pub struct EconomicActivityPresent;

/// Local market knowledge component
/// Individuals have limited, local information about prices
#[derive(Component, Debug, Clone)]
pub struct LocalMarketKnowledge {
    pub known_prices: Vec<KnownPrice>,
    pub information_quality: Fixed32,  // 0.0 (rumors) to 1.0 (direct observation)
    pub information_age: u64,           // Ticks since last update
    pub trusted_sources: Vec<Entity>,  // Other individuals we trust for info
}

#[derive(Debug, Clone)]
pub struct KnownPrice {
    pub good: GoodType,
    pub observed_price: Fixed32,
    pub location: Entity,
    pub observation_time: u64,
    pub confidence: Fixed32,  // How sure we are about this price
}

/// Market component - prices emerge here
/// Markets don't set prices, they discover them through order matching
#[derive(Component, Debug, Clone)]
pub struct Market {
    pub location: Entity,                   // Province where market exists
    pub market_type: MarketType,
    pub participants: Vec<Entity>,          // Active traders
    pub liquidity: Fixed32,                 // How active is trading
    pub information_flow: Fixed32,          // How well informed are traders?
    pub transaction_costs: Fixed32,         // Cost to participate
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketType {
    Local,      // Village market
    Regional,   // City market
    National,   // Capital markets
    International, // Cross-border trade
}

/// Market order component - individual buy/sell decisions
#[derive(Component, Debug, Clone)]
pub struct MarketOrder {
    pub trader: Entity,
    pub order_type: OrderType,
    pub good: GoodType,
    pub quantity: Fixed32,
    pub limit_price: Option<Fixed32>,  // Maximum willing to pay / minimum willing to accept
    pub urgency: Fixed32,               // How quickly they need to trade
    pub created_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Buy,
    Sell,
}

/// Entrepreneurial discovery component
/// Some individuals discover new opportunities
#[derive(Component, Debug, Clone)]
pub struct Entrepreneur {
    pub alertness: Fixed32,              // Ability to spot opportunities
    pub risk_tolerance: Fixed32,         // Willingness to try new things
    pub capital_access: Fixed32,         // Resources to exploit opportunities
    pub discovered_opportunities: Vec<Opportunity>,
}

#[derive(Debug, Clone)]
pub struct Opportunity {
    pub opportunity_type: OpportunityType,
    pub expected_profit: Fixed32,
    pub risk_level: Fixed32,
    pub capital_required: Fixed32,
    pub discovered_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpportunityType {
    Arbitrage,      // Buy low, sell high between markets
    Innovation,     // New production method
    MarketGap,      // Unmet demand
    CostReduction,  // More efficient production
}