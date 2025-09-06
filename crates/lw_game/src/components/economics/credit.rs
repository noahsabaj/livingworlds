//! Credit and debt components
//!
//! Credit allows economic activity beyond current savings, enabling growth
//! but also creating systemic risks through interconnected obligations.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use super::money::MoneyType;

/// Credit relationship between entities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelationship {
    pub creditor: Entity,
    pub debtor: Entity,
    pub terms: CreditTerms,
    pub status: CreditStatus,
    pub payment_history: Vec<Payment>,
    pub trust_level: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditTerms {
    pub principal: Fixed32,
    pub interest_rate: Fixed32,
    pub term_length: u64,
    pub payment_frequency: PaymentFrequency,
    pub collateral: Option<Entity>,
    pub penalties: PenaltyStructure,
    pub renegotiable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annual,
    OnDemand,
    Bullet,  // All at maturity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenaltyStructure {
    pub late_fee: Fixed32,
    pub default_penalty: Fixed32,
    pub acceleration_clause: bool,  // Full amount due on default
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditStatus {
    Current,
    Delinquent { days: u32 },
    InDefault,
    Restructured,
    PaidOff,
    WrittenOff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub amount: Fixed32,
    pub date: u64,
    pub principal_portion: Fixed32,
    pub interest_portion: Fixed32,
    pub late: bool,
}

/// Credit score/rating component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CreditScore {
    pub entity: Entity,
    pub score: Fixed32,
    pub payment_history_weight: Fixed32,
    pub debt_burden_weight: Fixed32,
    pub credit_history_length: u64,
    pub recent_inquiries: u32,
    pub rating: CreditRating,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CreditRating {
    AAA,  // Pristine
    AA,
    A,
    BBB,  // Investment grade cutoff
    BB,
    B,
    CCC,
    CC,
    C,
    D,    // In default
}

/// Debt burden component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct DebtBurden {
    pub total_debt: Fixed32,
    pub debt_to_income: Fixed32,
    pub debt_to_assets: Fixed32,
    pub debt_service_coverage: Fixed32,  // Income / debt payments
    pub sustainable: bool,
}

/// Sovereign debt component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SovereignDebt {
    pub nation: Entity,
    pub total_debt: Fixed32,
    pub debt_to_gdp: Fixed32,
    pub domestic_debt: Fixed32,
    pub foreign_debt: Fixed32,
    pub bond_yields: Vec<BondYield>,
    pub default_risk: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondYield {
    pub maturity: u64,
    pub yield_rate: Fixed32,
    pub spread_vs_benchmark: Fixed32,
}

/// Debt crisis component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct DebtCrisis {
    pub entity: Entity,
    pub crisis_type: DebtCrisisType,
    pub debt_level: Fixed32,
    pub inability_to_pay: bool,
    pub creditor_response: CreditorResponse,
    pub economic_impact: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebtCrisisType {
    Personal,
    Corporate,
    Banking,
    Sovereign,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditorResponse {
    Restructuring,
    Forbearance,
    Foreclosure,
    Bailout,
    HaircutNegotiation { reduction: Fixed32 },
    DebtForEquitySwap,
}

/// Usury laws component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct UsuryLaws {
    pub jurisdiction: Entity,
    pub max_interest_rate: Option<Fixed32>,
    pub religious_basis: bool,
    pub exceptions: Vec<LoanType>,
    pub enforcement: Fixed32,
    pub black_market_premium: Fixed32,  // How much extra illegal lenders charge
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoanType {
    Personal,
    Commercial,
    Mortgage,
    Payday,
    Microcredit,
    Sovereign,
}

/// Informal credit component - outside banking system
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct InformalCredit {
    pub credit_type: InformalCreditType,
    pub trust_basis: TrustBasis,
    pub enforcement_mechanism: EnforcementMechanism,
    pub interest_rate: Fixed32,
    pub accessibility: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InformalCreditType {
    FamilyLoan,
    CommunityLending,
    RotatingCredit,  // Tontines, ROSCAs
    Pawnbroker,
    LoanShark,
    TradeCredit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustBasis {
    Kinship,
    Community,
    Religious,
    Ethnic,
    Professional,
    Criminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementMechanism {
    SocialPressure,
    Reputation,
    Ostracism,
    ReligiousSanction,
    Violence,
    Legal,
}

/// Debt jubilee component - debt forgiveness
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct DebtJubilee {
    pub decreeing_authority: Entity,
    pub jubilee_type: JubileeType,
    pub affected_debts: Vec<Entity>,
    pub forgiveness_percentage: Fixed32,
    pub economic_disruption: Fixed32,
    pub moral_hazard: Fixed32,  // Future lending impact
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JubileeType {
    Religious,   // Year of Jubilee
    Political,   // New ruler
    Economic,    // Crisis response
    Partial,     // Some debts only
    Complete,    // All debts
}

/// Credit bubble component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CreditBubble {
    pub asset_class: String,
    pub credit_growth_rate: Fixed32,
    pub leverage_levels: Fixed32,
    pub speculation_level: Fixed32,
    pub bubble_stage: BubbleStage,
    pub systemic_risk: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BubbleStage {
    Formation,
    Expansion,
    Euphoria,
    ProfitTaking,
    Panic,
    Bust,
}

/// Securitization component - bundling loans
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Securitization {
    pub underlying_assets: Vec<Entity>,
    pub tranches: Vec<Tranche>,
    pub originator: Entity,
    pub servicer: Entity,
    pub rating: CreditRating,
    pub complexity: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tranche {
    pub seniority: u32,  // 1 = most senior
    pub size: Fixed32,
    pub interest_rate: Fixed32,
    pub rating: CreditRating,
    pub first_loss_position: bool,
}

/// Peer-to-peer lending
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct P2PLending {
    pub platform: Entity,
    pub lenders: Vec<Entity>,
    pub borrowers: Vec<Entity>,
    pub average_rate: Fixed32,
    pub default_rate: Fixed32,
    pub platform_fee: Fixed32,
    pub risk_assessment_quality: Fixed32,
}

/// Debt deflation spiral - Fisher dynamics
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct DebtDeflation {
    pub deflation_rate: Fixed32,
    pub real_debt_burden_increase: Fixed32,
    pub forced_liquidations: Vec<Entity>,
    pub price_collapse: Fixed32,
    pub economic_contraction: Fixed32,
    pub intervention_attempts: Vec<InterventionType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterventionType {
    MonetaryStimulus,
    FiscalStimulus,
    DebtRestructuring,
    QuantitativeEasing,
    HelicopterMoney,
}