//! Policy incentive components - how governments influence without direct control
//!
//! Governments don't set prices or quotas - they create incentive structures
//! that individuals respond to based on their own goals and constraints.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use crate::components::economics::{GoodType, MoneyType};

/// Policy incentive component - how government influences behavior
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PolicyIncentive {
    pub issuing_authority: Entity,
    pub policy_type: PolicyType,
    pub target_behavior: TargetBehavior,
    pub strength: Fixed32,
    pub compliance_cost: Fixed32,
    pub enforcement: Fixed32,
    pub unintended_consequences: Vec<UnintendedConsequence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyType {
    Tax {
        tax_type: TaxType,
        rate: Fixed32,
        exemptions: Vec<Exemption>,
    },
    Subsidy {
        subsidy_type: SubsidyType,
        amount: Fixed32,
        conditions: Vec<Condition>,
    },
    Regulation {
        regulation_type: RegulationType,
        strictness: Fixed32,
        penalties: Vec<Penalty>,
    },
    Information {
        campaign_type: CampaignType,
        credibility: Fixed32,
    },
    Nudge {
        nudge_type: NudgeType,
        subtlety: Fixed32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxType {
    Income,
    Sales,
    Property,
    Capital,
    Carbon,
    Import,
    Export,
    Transaction,
    Wealth,
    Land,
    Sin,  // Alcohol, tobacco
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubsidyType {
    Production,
    Consumption,
    Export,
    Research,
    Employment,
    Infrastructure,
    Agriculture,
    Energy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulationType {
    Environmental,
    Safety,
    Labor,
    Financial,
    Competition,
    Quality,
    Zoning,
    Professional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignType {
    PublicHealth,
    Education,
    Patriotic,
    Economic,
    Environmental,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NudgeType {
    DefaultOption,
    Framing,
    SocialProof,
    Anchoring,
    LossAversion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetBehavior {
    pub behavior: String,
    pub desired_change: BehaviorChange,
    pub affected_groups: Vec<AffectedGroup>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehaviorChange {
    Increase,
    Decrease,
    Redirect,
    Stabilize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedGroup {
    pub group_type: String,
    pub size: u32,
    pub responsiveness: Fixed32,  // How much they react to incentive
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exemption {
    pub exemption_type: String,
    pub beneficiaries: Vec<Entity>,
    pub justification: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: String,
    pub requirement: String,
    pub verification_method: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Penalty {
    pub violation: String,
    pub fine: Fixed32,
    pub other_consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnintendedConsequence {
    pub consequence_type: ConsequenceType,
    pub severity: Fixed32,
    pub discovery_time: u64,  // When it becomes apparent
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceType {
    TaxAvoidance,
    BlackMarket,
    CapitalFlight,
    Corruption,
    Inequality,
    Inefficiency,
    MarketDistortion,
    RentSeeking,
}

/// Government information limitation component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentInformation {
    pub government: Entity,
    pub information_quality: Fixed32,
    pub information_lag: u64,  // How old is the data
    pub blind_spots: Vec<BlindSpot>,
    pub biased_sources: Vec<BiasedSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlindSpot {
    pub area: String,
    pub severity: Fixed32,
    pub reason: BlindSpotReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlindSpotReason {
    Bureaucratic,     // Bureaucracy doesn't report it
    Political,        // Politically inconvenient
    Technical,        // Can't measure it
    Cultural,         // Don't understand it
    Ideological,      // Don't believe it exists
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasedSource {
    pub source: Entity,
    pub bias_direction: BiasDirection,
    pub bias_strength: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiasDirection {
    Optimistic,
    Pessimistic,
    ProGovernment,
    AntiGovernment,
    Partisan,
}

/// Tax incidence - who really pays
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TaxIncidence {
    pub nominal_payer: Entity,     // Who writes the check
    pub actual_burden: Vec<BurdenShare>,  // Who really pays
    pub deadweight_loss: Fixed32,  // Economic inefficiency
    pub behavioral_response: Fixed32,  // How much behavior changes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurdenShare {
    pub bearer: Entity,
    pub share: Fixed32,
}

/// Regulatory capture component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryCapture {
    pub regulator: Entity,
    pub captured_by: Vec<Entity>,
    pub capture_strength: Fixed32,
    pub public_interest_loss: Fixed32,
    pub rent_extraction: Fixed32,
}

/// Policy lag component - time for policies to take effect
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PolicyLag {
    pub recognition_lag: u64,     // Time to recognize problem
    pub decision_lag: u64,        // Time to decide on response
    pub implementation_lag: u64,  // Time to implement
    pub effectiveness_lag: u64,   // Time for effects to show
}

/// Fiscal multiplier - how much economic activity per government spending
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FiscalMultiplier {
    pub spending_type: SpendingType,
    pub multiplier: Fixed32,
    pub time_profile: Vec<(u64, Fixed32)>,  // Effect over time
    pub crowding_out: Fixed32,    // Private investment reduction
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpendingType {
    Infrastructure,
    Transfer,
    Military,
    Education,
    Healthcare,
    Bureaucracy,
}

/// Incentive compatibility component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IncentiveCompatibility {
    pub policy: Entity,
    pub individual_incentives: Vec<IndividualIncentive>,
    pub alignment: Fixed32,        // How well aligned with goals
    pub gaming_potential: Fixed32, // Can it be exploited?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualIncentive {
    pub individual_type: String,
    pub private_benefit: Fixed32,
    pub social_benefit: Fixed32,
    pub divergence: Fixed32,      // Private vs social
}

/// Public choice problems
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PublicChoiceProblem {
    pub problem_type: PublicChoiceType,
    pub severity: Fixed32,
    pub affected_policies: Vec<Entity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicChoiceType {
    ConcentratedBenefits,  // Benefits to few, costs to many
    DiffuseCosts,          // No one fights small costs
    RationalIgnorance,     // Not worth learning about
    SpecialInterests,      // Organized beats disorganized
    Logrolling,            // Vote trading
    PorkBarrel,            // Local benefits, general costs
}