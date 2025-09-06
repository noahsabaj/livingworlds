//! Trade infrastructure and connections
//!
//! Trade routes emerge from individual merchant decisions rather than
//! being centrally planned. Routes form where profitable.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Trade connection between locations
/// Emerges from repeated merchant activity
#[derive(Component, Debug, Clone)]
pub struct TradeConnection {
    pub origin: Entity,
    pub destination: Entity,
    pub connection_type: ConnectionType,
    pub trade_volume: Fixed32,       // Emerges from actual trades
    pub transit_time: Fixed32,       // Based on geography and infrastructure
    pub transport_cost: Fixed32,     // Discovered through experience
    pub reliability: Fixed32,         // How often goods arrive safely
    pub merchant_knowledge: Fixed32,  // How well known this route is
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Road,
    River,
    Sea,
    Rail,
    Air,
    Caravan,
    Mixed,
}

/// Event fired when a trade transaction occurs
#[derive(Event, Debug, Clone)]
pub struct TradeEvent {
    pub buyer: Entity,
    pub seller: Entity,
    pub good: crate::components::GoodType,
    pub quantity: Fixed32,
    pub price: Fixed32,
    pub location: Entity,  // Where the trade happened
}

/// Trade route component - a known path for commerce
/// Created by successful merchant journeys
#[derive(Component, Debug, Clone)]
pub struct TradeRoute {
    pub waypoints: Vec<Entity>,        // Provinces along the route
    pub total_distance: Fixed32,
    pub difficulty: Fixed32,           // Terrain, weather, bandits
    pub established_date: u64,         // When first successfully traversed
    pub usage_frequency: Fixed32,      // How often merchants use it
    pub infrastructure_quality: Fixed32, // Improves with use and investment
}

/// Merchant component - individuals who trade between markets
#[derive(Component, Debug, Clone)]
pub struct Merchant {
    pub capital: Fixed32,
    pub risk_tolerance: Fixed32,
    pub known_routes: Vec<Entity>,     // TradeRoute entities they know
    pub reputation: Fixed32,            // Affects trading opportunities
    pub specialization: Option<MerchantSpecialization>,
    pub current_cargo: Vec<CargoItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MerchantSpecialization {
    Bulk,       // Large volume, low margin
    Luxury,     // Small volume, high margin
    Perishable, // Time-sensitive goods
    Regional,   // Knows local markets well
    LongDistance, // International trade
}

#[derive(Debug, Clone)]
pub struct CargoItem {
    pub good_type: super::GoodType,
    pub quantity: Fixed32,
    pub purchase_price: Fixed32,
    pub origin_market: Entity,
    pub destination_market: Option<Entity>, // May not know yet
}

/// Trading post component - facilitates trade
#[derive(Component, Debug, Clone)]
pub struct TradingPost {
    pub post_type: TradingPostType,
    pub storage_capacity: Fixed32,
    pub services_available: TradingServices,
    pub fee_structure: FeeStructure,
    pub information_hub: bool,  // Do merchants share price info here?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradingPostType {
    Roadside,
    RiverPort,
    SeaPort,
    Caravanserai,
    MarketTown,
    FreePort,
}

#[derive(Debug, Clone)]
pub struct TradingServices {
    pub warehousing: bool,
    pub banking: bool,
    pub guides: bool,
    pub security: bool,
    pub repair: bool,
}

#[derive(Debug, Clone)]
pub struct FeeStructure {
    pub storage_fee: Fixed32,
    pub transaction_fee: Fixed32,
    pub security_fee: Fixed32,
    pub information_fee: Fixed32,
}

/// Trade embargo component - political restriction on trade
#[derive(Component, Debug, Clone)]
pub struct TradeEmbargo {
    pub imposing_nation: Entity,
    pub target_nation: Entity,
    pub restricted_goods: Vec<super::GoodType>,
    pub enforcement_level: Fixed32,
    pub economic_impact: Fixed32,  // Discovered over time
}

/// Smuggling operation component - emerges when profitable
#[derive(Component, Debug, Clone)]
pub struct SmugglingOperation {
    pub route: Entity,              // Hidden trade route
    pub goods_smuggled: Vec<super::GoodType>,
    pub profit_margin: Fixed32,     // Higher due to risk
    pub detection_risk: Fixed32,
    pub bribe_costs: Fixed32,
}

/// Market access component - who can trade where
#[derive(Component, Debug, Clone)]
pub struct MarketAccess {
    pub market: Entity,
    pub access_level: AccessLevel,
    pub restrictions: Vec<TradeRestriction>,
    pub required_permits: Vec<Permit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessLevel {
    Open,       // Anyone can trade
    Licensed,   // Need permits
    Restricted, // Only certain groups
    Closed,     // No outside trade
}

#[derive(Debug, Clone)]
pub struct TradeRestriction {
    pub restriction_type: RestrictionType,
    pub affected_goods: Vec<super::GoodType>,
    pub severity: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictionType {
    Tariff,
    Quota,
    Ban,
    Monopoly,
}

#[derive(Debug, Clone)]
pub struct Permit {
    pub permit_type: String,
    pub issuing_authority: Entity,
    pub cost: Fixed32,
    pub duration: u64,
}

/// Caravan component - group traveling together for safety
#[derive(Component, Debug, Clone)]
pub struct Caravan {
    pub merchants: Vec<Entity>,
    pub guards: Vec<Entity>,
    pub current_location: Entity,
    pub destination: Entity,
    pub departure_time: u64,
    pub expected_arrival: u64,
    pub protection_level: Fixed32,
}

/// Trade fair component - periodic gathering of merchants
#[derive(Component, Debug, Clone)]
pub struct TradeFair {
    pub location: Entity,
    pub frequency: TradeFairFrequency,
    pub next_date: u64,
    pub expected_attendance: u32,
    pub specialties: Vec<super::GoodType>,
    pub reputation: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeFairFrequency {
    Weekly,
    Monthly,
    Seasonal,
    Annual,
}

/// Trade guild component - merchant organization
#[derive(Component, Debug, Clone)]
pub struct TradeGuild {
    pub name: String,
    pub members: Vec<Entity>,
    pub controlled_goods: Vec<super::GoodType>,
    pub influence: Fixed32,
    pub wealth: Fixed32,
    pub rules: GuildRules,
}

#[derive(Debug, Clone)]
pub struct GuildRules {
    pub membership_fee: Fixed32,
    pub quality_standards: Fixed32,
    pub price_coordination: bool,  // Do they fix prices?
    pub dispute_resolution: bool,
    pub shared_information: bool,
}