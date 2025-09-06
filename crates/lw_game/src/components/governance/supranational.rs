//! Supranational entities - unions, blocs, and federations
//!
//! Nations can delegate sovereignty to supranational entities for
//! collective benefits, but this creates new power dynamics.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use crate::components::economics::MoneyType;

/// Supranational entity component - EU, AU, ASEAN-like
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SupranationalEntity {
    pub name: String,
    pub entity_type: SupranationalType,
    pub member_nations: Vec<Entity>,
    pub candidate_nations: Vec<Entity>,
    pub founding_members: Vec<Entity>,
    pub delegated_powers: Vec<DelegatedPower>,
    pub governance: GovernanceStructure,
    pub headquarters: Entity,  // Location
    pub budget: Fixed32,
    pub legitimacy: Fixed32,   // Public support
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupranationalType {
    CustomsUnion {
        common_external_tariff: Fixed32,
        internal_free_trade: bool,
        rules_of_origin: bool,
    },
    CommonMarket {
        goods_mobility: bool,
        services_mobility: bool,
        capital_mobility: bool,
        labor_mobility: bool,
    },
    CurrencyUnion {
        common_currency: MoneyType,
        central_bank: Option<Entity>,
        fiscal_rules: Vec<FiscalRule>,
        bailout_mechanism: bool,
    },
    EconomicCommunity {
        harmonized_regulations: Vec<Regulation>,
        common_standards: bool,
        dispute_resolution: DisputeMechanism,
    },
    PoliticalUnion {
        shared_sovereignty: Vec<SovereigntyArea>,
        common_foreign_policy: bool,
        common_defense: bool,
        federal_structure: bool,
    },
    MilitaryAlliance {
        mutual_defense: bool,
        integrated_command: bool,
        burden_sharing: BurdenSharing,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelegatedPower {
    TradePolicy,
    MonetaryPolicy,
    FiscalPolicy { max_harmonization: Fixed32 },
    RegulatoryPower { sectors: u32 },
    ForeignPolicy,
    DefensePolicy,
    JudicialPower,
    TaxationPower { types: u32 },
    BorderControl,
    CitizenshipRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStructure {
    pub decision_making: DecisionMaking,
    pub institutions: Vec<Institution>,
    pub voting_weights: Vec<(Entity, Fixed32)>,  // Nation -> weight
    pub veto_powers: Vec<Entity>,
    pub rotating_presidency: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionMaking {
    Unanimity,
    QualifiedMajority { threshold: Fixed32 },
    SimpleMajority,
    Consensus,
    Weighted { by_population: bool, by_contribution: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub name: String,
    pub institution_type: InstitutionType,
    pub members: u32,
    pub selection_method: SelectionMethod,
    pub powers: Vec<InstitutionalPower>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstitutionType {
    Parliament,
    Commission,
    Council,
    Court,
    CentralBank,
    Secretariat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMethod {
    DirectElection,
    NationalAppointment,
    Rotation,
    Merit,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstitutionalPower {
    Legislative,
    Executive,
    Judicial,
    Monetary,
    Regulatory,
    Advisory,
}

/// Customs union component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CustomsUnion {
    pub union_entity: Entity,
    pub common_tariff_schedule: TariffSchedule,
    pub trade_diversion: Fixed32,    // Trade moved from efficient non-members
    pub trade_creation: Fixed32,     // New trade between members
    pub revenue_sharing: RevenueSharing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariffSchedule {
    pub agricultural_tariff: Fixed32,
    pub industrial_tariff: Fixed32,
    pub services_restrictions: Fixed32,
    pub exceptions: Vec<TariffException>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariffException {
    pub good_type: String,
    pub special_rate: Fixed32,
    pub beneficiary: Option<Entity>,
    pub duration: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RevenueSharing {
    ByCollection,      // Who collects keeps it
    ByConsumption,     // Where goods consumed
    ByFormula { population_weight: Fixed32, gdp_weight: Fixed32 },
    Pooled,            // Central budget
}

/// Currency union component (Eurozone-like)
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyUnion {
    pub common_currency: MoneyType,
    pub member_nations: Vec<Entity>,
    pub central_bank: Entity,
    pub convergence_criteria: ConvergenceCriteria,
    pub fiscal_compact: Vec<FiscalRule>,
    pub bailout_fund: Option<BailoutFund>,
    pub exit_mechanism: Option<ExitMechanism>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceCriteria {
    pub inflation_limit: Fixed32,
    pub deficit_limit: Fixed32,      // % of GDP
    pub debt_limit: Fixed32,          // % of GDP
    pub interest_rate_band: Fixed32,
    pub exchange_rate_stability: Fixed32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiscalRule {
    pub rule_type: FiscalRuleType,
    pub limit: Fixed32,
    pub automatic_sanctions: bool,
    pub escape_clauses: Vec<EscapeClause>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiscalRuleType {
    StructuralDeficit,
    CyclicalDeficit,
    PrimaryBalance,
    DebtBrake,
    ExpenditureRule,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscapeClause {
    pub trigger: String,
    pub suspension_allowed: bool,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BailoutFund {
    pub total_capacity: Fixed32,
    pub contribution_key: Vec<(Entity, Fixed32)>,
    pub conditionality: ConditionProgram,
    pub seniority: CreditSeniority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionProgram {
    pub fiscal_adjustment: Fixed32,
    pub structural_reforms: Vec<Reform>,
    pub monitoring: MonitoringIntensity,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reform {
    pub reform_type: String,
    pub deadline: u64,
    pub compliance_measure: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoringIntensity {
    Light,
    Regular,
    Enhanced,
    Program,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditSeniority {
    Senior,
    PariPassu,
    Subordinated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitMechanism {
    pub voluntary_exit: bool,
    pub expulsion_possible: bool,
    pub exit_costs: ExitCosts,
    pub transition_period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitCosts {
    pub currency_redenomination: Fixed32,
    pub debt_redenomination: Fixed32,
    pub trade_disruption: Fixed32,
    pub political_costs: Fixed32,
}

/// Power delegation tracking
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PowerDelegation {
    pub from_nation: Entity,
    pub to_entity: Entity,
    pub powers_delegated: Vec<DelegatedPower>,
    pub reservation: Vec<PowerReservation>,  // Powers explicitly kept
    pub sunset_clause: Option<u64>,
    pub revocable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerReservation {
    pub power: DelegatedPower,
    pub conditions: Vec<String>,
}

/// Supranational influence on members
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SupranationalInfluence {
    pub union: Entity,
    pub member: Entity,
    pub policy_constraints: Vec<PolicyConstraint>,
    pub benefits: MembershipBenefits,
    pub costs: MembershipCosts,
    pub compliance: Fixed32,
    pub euroskepticism: Fixed32,  // Public opposition
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConstraint {
    pub policy_area: String,
    pub constraint_type: ConstraintType,
    pub binding_level: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    Prohibition,
    Harmonization,
    MinimumStandard,
    MaximumLimit,
    Coordination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipBenefits {
    pub trade_boost: Fixed32,
    pub investment_inflow: Fixed32,
    pub funding_received: Fixed32,
    pub political_influence: Fixed32,
    pub security_guarantee: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipCosts {
    pub budget_contribution: Fixed32,
    pub sovereignty_loss: Fixed32,
    pub regulatory_burden: Fixed32,
    pub competitive_pressure: Fixed32,
    pub brain_drain: Fixed32,
}

/// Membership negotiation
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MembershipNegotiation {
    pub candidate: Entity,
    pub union: Entity,
    pub chapters: Vec<NegotiationChapter>,
    pub start_date: u64,
    pub expected_duration: u64,
    pub obstacles: Vec<Obstacle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationChapter {
    pub name: String,
    pub status: ChapterStatus,
    pub requirements: Vec<String>,
    pub compliance: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChapterStatus {
    NotOpened,
    Screening,
    Negotiating,
    ProvisionalClosed,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub obstacle_type: String,
    pub severity: Fixed32,
    pub resolution_path: Option<String>,
}

/// Integration level tracking
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationLevel {
    pub economic: Fixed32,
    pub political: Fixed32,
    pub social: Fixed32,
    pub military: Fixed32,
    pub direction: IntegrationDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationDirection {
    Deepening,
    Stable,
    Fragmenting,
    MultiSpeed,  // Different integration speeds
}

/// Sovereignty balance
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyBalance {
    pub nation: Entity,
    pub retained_sovereignty: Vec<SovereigntyArea>,
    pub pooled_sovereignty: Vec<(SovereigntyArea, Entity)>,  // Area -> Union
    pub sovereignty_conflicts: Vec<SovereigntyConflict>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SovereigntyArea {
    LawMaking,
    TaxPolicy,
    MonetaryPolicy,
    BorderControl,
    ForeignPolicy,
    DefensePolicy,
    JudicialSystem,
    SocialPolicy,
    CulturalPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyConflict {
    pub area: SovereigntyArea,
    pub national_position: String,
    pub supranational_position: String,
    pub resolution: ConflictResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    NationalPrevails,
    SupranationalPrevails,
    Compromise,
    Deferred,
    CourtRuling,
}

/// Regulation component
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Regulation {
    pub name: String,
    pub sector: String,
    pub harmonization_level: Fixed32,
    pub enforcement: Fixed32,
}

/// Dispute mechanism
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisputeMechanism {
    Arbitration,
    Court,
    Mediation,
    Diplomatic,
}

/// Military burden sharing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BurdenSharing {
    pub contribution_formula: String,
    pub capability_targets: Vec<(String, Fixed32)>,
    pub common_funding: Fixed32,
}