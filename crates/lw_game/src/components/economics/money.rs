//! Monetary systems - money as emergent social technology
//!
//! Money isn't assumed - it emerges from repeated exchange as certain goods
//! prove better at facilitating trade than direct barter.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use super::GoodType;

/// Money component - what's being used as medium of exchange
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub money_type: MoneyType,
    pub issuer: Option<Entity>,      // None for commodity money
    pub backing: Option<Backing>,    // What backs this money
    pub quantity: Fixed32,
    pub divisibility: Divisibility,  // Can you make change?
    pub authenticity: Fixed32,        // Real vs counterfeit
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoneyType {
    Commodity(CommodityMoney),
    Representative { 
        backed_by: CommodityMoney,
        issuer: Entity,
        convertibility: Fixed32,  // Can you actually redeem it?
    },
    Fiat { 
        issuer: Entity,           // Government that declares value
        legal_tender: bool,       // Must accept for debts
    },
    Credit {
        issuer: Entity,           // Who owes
        holder: Entity,           // Who is owed
        maturity: Option<u64>,    // When due
    },
    Digital {
        algorithm: String,        // Bitcoin-like
        supply_limit: Option<Fixed32>,
        mining_difficulty: Fixed32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommodityMoney {
    Gold,
    Silver,
    Copper,
    Salt,
    Grain,
    Cattle,
    Shells,
    Stones,  // Rai stones
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Backing {
    FullReserve(CommodityMoney),     // 100% backed
    FractionalReserve {
        commodity: CommodityMoney,
        ratio: Fixed32,               // 10% = 10x multiplication possible
    },
    Asset {
        asset_type: String,
        value: Fixed32,
    },
    Nothing,                          // Pure fiat
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Divisibility {
    Continuous,     // Can divide infinitely (digital)
    Discrete(u32),  // Smallest unit (cents)
    Indivisible,    // Can't divide (whole cows)
}

/// Money emergence tracking - how barter evolves to money
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MoneyEmergence {
    pub good: GoodType,
    pub salability: Salability,
    pub adoption_rate: Fixed32,        // What % use it as money
    pub network_effects: Fixed32,      // More users = more useful
    pub discovered_by: Vec<Entity>,    // Who figured this out
    pub competing_monies: Vec<GoodType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Salability {
    pub across_space: Fixed32,   // Easy to transport?
    pub across_time: Fixed32,    // Stores value?
    pub across_scales: Fixed32,  // Works for big and small trades?
}

/// Individual money preferences
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MoneyPreference {
    pub preferred: MoneyType,
    pub will_accept: Vec<AcceptedMoney>,
    pub barter_willingness: Fixed32,   // Will still barter?
    pub trust_in_issuers: Vec<(Entity, Fixed32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptedMoney {
    pub money_type: MoneyType,
    pub discount_rate: Fixed32,  // 0.9 = accept at 90% value
    pub max_quantity: Fixed32,   // Won't accept more than this
}

/// Banking system types
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub enum BankingSystem {
    FreeCompetitive {
        banks: Vec<Entity>,
        note_competition: bool,        // Banks issue own notes
        clearinghouses: Vec<Entity>,   // Coordinate settlements
        reputation_based: bool,
    },
    CentralBank {
        central_authority: Entity,
        member_banks: Vec<Entity>,
        reserve_requirement: Fixed32,
        base_rate: Fixed32,
        lender_of_last_resort: bool,
    },
    FullReserve {
        warehouses: Vec<Entity>,       // Just store, don't lend
        storage_fees: Fixed32,
        bailment_rules: bool,          // Legal protection
    },
    FractionalReserve {
        banks: Vec<Entity>,
        reserve_ratio: Fixed32,
        deposit_insurance: Option<Fixed32>,
        bank_run_probability: Fixed32,
    },
    Islamic {  // No interest
        profit_sharing_ratio: Fixed32,
        asset_backed_only: bool,
        sharia_board: Entity,
    },
    Informal {  // Hawala, etc
        trust_networks: Vec<Entity>,
        no_physical_transfer: bool,
        reputation_enforcement: Fixed32,
    },
}

/// Currency exchange component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange {
    pub pair: (MoneyType, MoneyType),
    pub rate: Fixed32,                  // Discovered through trade
    pub bid_ask_spread: Fixed32,
    pub volume_24h: Fixed32,
    pub volatility: Fixed32,
    pub arbitrage_opportunities: Vec<ArbitrageRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageRoute {
    pub path: Vec<MoneyType>,
    pub profit_potential: Fixed32,
    pub capital_required: Fixed32,
    pub risk_level: Fixed32,
}

/// Monetary policy component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryPolicy {
    pub authority: Entity,
    pub policy_type: MonetaryPolicyType,
    pub money_supply: Fixed32,
    pub target: PolicyTarget,
    pub instruments: Vec<PolicyInstrument>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonetaryPolicyType {
    FixedSupply,          // Gold standard
    DiscretionaryFiat,    // Central bank decides
    RuleBased,            // Taylor rule, etc
    Algorithmic,          // Coded rules
    Competing,            // Multiple issuers
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyTarget {
    PriceStability { inflation_target: Fixed32 },
    FullEmployment,
    ExchangeRateTarget { peg_to: MoneyType, rate: Fixed32 },
    MoneySupplyGrowth { rate: Fixed32 },
    NominalGDP { target_growth: Fixed32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyInstrument {
    OpenMarketOperations,
    ReserveRequirements,
    InterestRates,
    QuantitativeEasing,
    ForeignExchangeIntervention,
    DirectLending,
}

/// Inflation/deflation tracking
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MoneySupplyEffects {
    pub money_type: MoneyType,
    pub total_supply: Fixed32,
    pub velocity: Fixed32,              // How fast money circulates
    pub price_level: Fixed32,           // General price level
    pub inflation_rate: Fixed32,        // Rate of change
    pub inflation_expectations: Fixed32, // What people expect
    pub cantillon_effects: Vec<CantillonEffect>, // Who gets new money first
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CantillonEffect {
    pub beneficiary: Entity,
    pub advantage: Fixed32,  // How much they benefit from getting new money first
    pub time_lag: u64,       // How long before prices adjust
}

/// Seigniorage - profit from money creation
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Seigniorage {
    pub issuer: Entity,
    pub revenue: Fixed32,                // Profit from creating money
    pub debasement_rate: Fixed32,        // How fast reducing value
    pub public_trust: Fixed32,           // Do people still accept it?
    pub alternative_monies: Vec<MoneyType>, // Competition
}

/// Gresham's Law tracking - bad money drives out good
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct GreshamsLaw {
    pub good_money: MoneyType,
    pub bad_money: MoneyType,
    pub hoarding_rate: Fixed32,         // Good money being saved
    pub spending_rate: Fixed32,         // Bad money being spent
    pub law_active: bool,               // Is this happening?
}

/// Monetary union component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryUnion {
    pub common_currency: MoneyType,
    pub member_nations: Vec<Entity>,
    pub central_bank: Option<Entity>,
    pub fiscal_rules: Vec<FiscalRule>,
    pub exit_possibility: bool,         // Can countries leave?
    pub optimal_currency_area: Fixed32, // How well suited for common currency
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalRule {
    pub rule_type: FiscalRuleType,
    pub limit: Fixed32,
    pub enforcement: Fixed32,  // How well enforced
    pub penalties: Vec<Penalty>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiscalRuleType {
    DeficitLimit,      // Max % of GDP
    DebtLimit,         // Max % of GDP
    SpendingLimit,     // Max growth rate
    BalancedBudget,    // Must balance
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    pub violation_level: Fixed32,
    pub penalty_amount: Fixed32,
    pub enforcement_mechanism: String,
}

/// Money multiplier in fractional reserve
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MoneyMultiplier {
    pub base_money: Fixed32,
    pub reserve_ratio: Fixed32,
    pub actual_multiplier: Fixed32,     // Theoretical vs actual
    pub leakage: Fixed32,               // Money leaving system
    pub credit_creation: Fixed32,       // New money from loans
}