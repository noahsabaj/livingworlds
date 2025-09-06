//! Market transactions and order components
//!
//! Prices emerge from individual buy/sell decisions through order matching.
//! No central price setting - only discovered prices from actual trades.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use super::{GoodType, MoneyType};

/// Market order component - an individual's desire to buy or sell
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MarketOrder {
    pub order_type: OrderType,
    pub issuer: Entity,              // Individual placing the order
    pub good: GoodType,
    pub quantity: Fixed32,
    pub limit_price: Option<Fixed32>, // Max for buy, min for sell
    pub payment_money: MoneyType,    // What money they're using/accepting
    pub time_limit: Option<u64>,     // When order expires
    pub location: Entity,             // Market where order is placed
    pub created_at: u64,
    pub urgency: Fixed32,             // How badly they need this trade
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

/// Completed transaction component - record of actual trade
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CompletedTransaction {
    pub buyer: Entity,
    pub seller: Entity,
    pub good: GoodType,
    pub quantity: Fixed32,
    pub price: Fixed32,              // Discovered price from negotiation
    pub money_used: MoneyType,       // What money was actually used
    pub location: Entity,            // Where trade occurred
    pub timestamp: u64,
    pub transaction_costs: Fixed32,  // Fees, transport, etc.
    pub exchange_rate: Option<Fixed32>, // If currency conversion happened
}

/// Order book component - collection of active orders in a market
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub market: Entity,
    pub buy_orders: Vec<Entity>,     // MarketOrder entities
    pub sell_orders: Vec<Entity>,    // MarketOrder entities
    pub last_cleared: u64,           // When last matching occurred
    pub liquidity: Fixed32,          // How active is this market
}

/// Bid-ask spread component - emerges from order book
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct BidAskSpread {
    pub good: GoodType,
    pub best_bid: Option<Fixed32>,   // Highest buy offer
    pub best_ask: Option<Fixed32>,   // Lowest sell offer
    pub spread: Option<Fixed32>,     // Difference between bid and ask
    pub depth: Fixed32,               // Volume available at these prices
}

/// Market maker component - individuals who provide liquidity
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MarketMaker {
    pub individual: Entity,
    pub markets: Vec<Entity>,
    pub inventory: Vec<Inventory>,
    pub target_spread: Fixed32,      // Profit margin goal
    pub risk_limit: Fixed32,         // Max inventory to hold
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub good: GoodType,
    pub quantity: Fixed32,
    pub average_cost: Fixed32,
}

/// Price discovery event - when new price information emerges
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PriceDiscoveryEvent {
    pub good: GoodType,
    pub discovered_price: Fixed32,
    pub volume: Fixed32,
    pub market: Entity,
    pub participants: Vec<Entity>,
    pub timestamp: u64,
}

/// Negotiation component - haggling between individuals
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Negotiation {
    pub buyer: Entity,
    pub seller: Entity,
    pub good: GoodType,
    pub quantity: Fixed32,
    pub buyer_offer: Fixed32,
    pub seller_ask: Fixed32,
    pub rounds: u32,                 // How many offers back and forth
    pub deadline: Option<u64>,
}

/// Transaction cost component - all costs of doing business
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCost {
    pub search_cost: Fixed32,        // Finding trading partners
    pub negotiation_cost: Fixed32,   // Time spent haggling
    pub enforcement_cost: Fixed32,   // Ensuring delivery
    pub transport_cost: Fixed32,     // Moving goods
    pub information_cost: Fixed32,   // Learning prices
}

/// Contract component - formal agreement between parties
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub parties: Vec<Entity>,
    pub terms: ContractTerms,
    pub enforcement_mechanism: EnforcementType,
    pub penalty_clauses: Vec<PenaltyClause>,
    pub signed_date: u64,
    pub expiry_date: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTerms {
    pub goods: Vec<GoodDelivery>,
    pub payment_schedule: PaymentSchedule,
    pub delivery_location: Entity,
    pub quality_requirements: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodDelivery {
    pub good: GoodType,
    pub quantity: Fixed32,
    pub delivery_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentSchedule {
    Immediate,
    OnDelivery,
    Installments { amount: Fixed32, frequency: u64 },
    Deferred { date: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementType {
    Reputation,    // Social pressure
    Guild,         // Trade organization
    Government,    // Legal system
    Private,       // Hired enforcement
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenaltyClause {
    pub violation_type: String,
    pub penalty_amount: Fixed32,
}

/// Barter component - direct exchange without money
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct BarterOffer {
    pub proposer: Entity,
    pub offered_goods: Vec<(GoodType, Fixed32)>,
    pub requested_goods: Vec<(GoodType, Fixed32)>,
    pub location: Entity,
    pub expiry: Option<u64>,
}

/// Auction component - competitive bidding process
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Auction {
    pub auction_type: AuctionType,
    pub seller: Entity,
    pub good: GoodType,
    pub quantity: Fixed32,
    pub current_bid: Option<Fixed32>,
    pub leading_bidder: Option<Entity>,
    pub participants: Vec<Entity>,
    pub end_time: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuctionType {
    English,    // Price goes up
    Dutch,      // Price goes down
    Sealed,     // Hidden bids
    Vickrey,    // Second-price sealed
}

/// Credit component - deferred payment arrangements
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CreditArrangement {
    pub creditor: Entity,
    pub debtor: Entity,
    pub principal: Fixed32,
    pub interest_rate: Fixed32,
    pub repayment_schedule: PaymentSchedule,
    pub collateral: Option<Entity>,
    pub default_risk: Fixed32,
}

/// Market volatility component - price stability measure
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MarketVolatility {
    pub market: Entity,
    pub good: GoodType,
    pub price_variance: Fixed32,
    pub volume_variance: Fixed32,
    pub measurement_period: u64,
    pub shock_events: Vec<ShockEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShockEvent {
    pub event_type: String,
    pub impact: Fixed32,
    pub timestamp: u64,
    pub duration: u64,
}

/// A completed economic transaction
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub buyer: Entity,
    pub seller: Entity,
    pub good: GoodType,
    pub quantity: Fixed32,
    pub price: Fixed32,
    pub money_type: MoneyType,
    pub location: Entity,           // Where transaction occurred
    pub timestamp: u64,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Spot,               // Immediate exchange
    Forward,            // Future delivery
    Barter,             // Good for good
    Credit,             // Buy now, pay later
    Gift,               // No payment expected
}