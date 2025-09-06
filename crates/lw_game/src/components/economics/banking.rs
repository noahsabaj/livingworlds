//! Banking and financial institution components
//!
//! Banks emerge to solve trust and distance problems in trade.
//! Different banking systems have different effects on economic growth and stability.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use super::money::MoneyType;
use std::collections::HashMap;

/// Bank component - financial intermediary
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    pub bank_type: BankType,
    pub deposits: HashMap<Entity, Deposit>,
    pub loans: HashMap<Entity, Loan>,
    pub reserves: Vec<(MoneyType, Fixed32)>,  // Changed from HashMap to avoid Hash requirement
    pub capital: Fixed32,                 // Bank's own money
    pub credibility: Fixed32,             // Public trust
    pub liquidity: Fixed32,               // Can meet withdrawals?
    pub solvency: Fixed32,                // Assets > Liabilities?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankType {
    Commercial,       // Regular banking
    Investment,       // Securities, not deposits
    Central,          // Government monetary authority  
    Cooperative,      // Member-owned
    Islamic,          // Sharia-compliant
    Shadow,           // Non-bank financial
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    pub depositor: Entity,
    pub amount: Fixed32,
    pub money_type: MoneyType,
    pub account_type: AccountType,
    pub interest_rate: Fixed32,
    pub maturity: Option<u64>,
    pub can_withdraw: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Demand,           // Withdraw anytime
    Savings,          // Some restrictions
    Time,             // Fixed term
    Certificate,      // CD
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    pub borrower: Entity,
    pub principal: Fixed32,
    pub outstanding: Fixed32,
    pub interest_rate: Fixed32,
    pub term: u64,
    pub collateral: Option<Collateral>,
    pub payment_schedule: PaymentSchedule,
    pub risk_rating: Fixed32,
    pub default_probability: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collateral {
    pub asset: Entity,
    pub valuation: Fixed32,
    pub liquidation_value: Fixed32,  // If need to sell quickly
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentSchedule {
    Monthly,
    Quarterly,
    Annual,
    Balloon,          // All at end
    InterestOnly,     // Principal at end
}

/// Credit creation through fractional reserve
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CreditCreation {
    pub bank: Entity,
    pub money_created: Fixed32,
    pub backed_by: CreditBacking,
    pub multiplier_effect: Fixed32,
    pub systemic_risk: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreditBacking {
    Deposits { amount: Fixed32 },
    Assets { value: Fixed32 },
    FutureCashFlows { present_value: Fixed32 },
    Nothing,  // Pure credit expansion
}

/// Bank run component - panic withdrawals
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct BankRun {
    pub bank: Entity,
    pub panic_level: Fixed32,
    pub withdrawal_queue: Vec<Entity>,
    pub available_liquidity: Fixed32,
    pub contagion_risk: Fixed32,        // Spread to other banks
    pub government_response: Option<BailoutResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BailoutResponse {
    LiquidityInjection { amount: Fixed32 },
    DepositGuarantee { limit: Fixed32 },
    Nationalization,
    BankHoliday { duration: u64 },
    Nothing,  // Let it fail
}

/// Interbank lending market
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct InterbankMarket {
    pub participants: Vec<Entity>,
    pub overnight_rate: Fixed32,         // LIBOR-like
    pub total_volume: Fixed32,
    pub trust_matrix: HashMap<(Entity, Entity), Fixed32>, // Bilateral trust
    pub systemic_stress: Fixed32,
}

/// Central bank component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CentralBank {
    pub nation: Entity,
    pub independence: Fixed32,           // From political pressure
    pub policy_tools: Vec<PolicyTool>,
    pub base_rate: Fixed32,
    pub money_supply_target: Fixed32,
    pub inflation_target: Fixed32,
    pub mandate: CentralBankMandate,
    pub credibility: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CentralBankMandate {
    SingleMandate(PolicyGoal),          // Price stability only
    DualMandate(PolicyGoal, PolicyGoal), // Price stability + employment
    FlexibleInflationTargeting,
    ExchangeRatePeg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyGoal {
    PriceStability,
    FullEmployment,
    FinancialStability,
    ExchangeRateStability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyTool {
    InterestRates,
    ReserveRequirements,
    OpenMarketOperations,
    QuantitativeEasing,
    ForwardGuidance,
    MacroprudentialRules,
}

/// Financial innovation component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInnovation {
    pub innovation_type: InnovationType,
    pub adopters: Vec<Entity>,
    pub risk_profile: Fixed32,
    pub regulatory_status: RegulatoryStatus,
    pub systemic_importance: Fixed32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InnovationType {
    Securitization,      // Bundle loans
    Derivatives,         // Options, futures
    DigitalCurrency,
    MobileBanking,
    PeerToPeerLending,
    Microfinance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulatoryStatus {
    Unregulated,
    LightTouch,
    FullyRegulated,
    Banned,
}

/// Shadow banking component - non-bank financial
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ShadowBank {
    pub entity_type: ShadowBankType,
    pub assets_under_management: Fixed32,
    pub leverage: Fixed32,
    pub regulatory_arbitrage: bool,     // Avoiding bank regulations
    pub interconnectedness: Fixed32,    // Links to regular banks
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowBankType {
    MoneyMarketFund,
    HedgeFund,
    PrivateEquity,
    InvestmentVehicle,
    Fintech,
}

/// Financial crisis component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FinancialCrisis {
    pub crisis_type: CrisisType,
    pub trigger: CrisisTrigger,
    pub affected_institutions: Vec<Entity>,
    pub contagion_channels: Vec<ContagionChannel>,
    pub severity: Fixed32,
    pub government_response: Vec<CrisisResponse>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrisisType {
    BankingCrisis,
    CurrencyCrisis,
    DebtCrisis,
    SystemicCrisis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrisisTrigger {
    AssetBubbleBurst,
    BankRuns,
    SovereignDefault,
    ExternalShock,
    ContagionFromAbroad,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContagionChannel {
    InterbankExposure,
    AssetFireSales,
    InformationCascade,
    CreditCrunch,
    TradeLinks,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrisisResponse {
    BankBailout { amount: Fixed32 },
    LiquiditySupport,
    DepositInsurance,
    AssetPurchases,
    Recapitalization,
    Nationalization,
    AusterityMeasures,
}

/// Basel-like banking regulations
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct BankingRegulation {
    pub capital_requirements: CapitalRequirements,
    pub liquidity_requirements: Fixed32,
    pub leverage_ratio_limit: Fixed32,
    pub stress_test_frequency: u64,
    pub too_big_to_fail: Vec<Entity>,   // Systemically important
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalRequirements {
    pub tier1_ratio: Fixed32,           // Core capital
    pub tier2_ratio: Fixed32,           // Supplementary
    pub buffer_requirements: Fixed32,    // Extra cushion
    pub risk_weighted: bool,
}

/// Islamic banking component - no interest
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IslamicBank {
    pub sharia_board: Entity,
    pub profit_loss_sharing: HashMap<Entity, Fixed32>,
    pub asset_backed_only: bool,
    pub prohibited_industries: Vec<String>,
    pub zakat_obligations: Fixed32,     // Charitable giving
}

/// Microfinance institution
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MicrofinanceInstitution {
    pub target_population: TargetPopulation,
    pub average_loan_size: Fixed32,
    pub group_lending: bool,             // Peer pressure for repayment
    pub social_collateral: bool,        // Reputation-based
    pub financial_literacy_programs: bool,
    pub repayment_rate: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetPopulation {
    RuralPoor,
    UrbanPoor,
    Women,
    SmallBusiness,
    Farmers,
}