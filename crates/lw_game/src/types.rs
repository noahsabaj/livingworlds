//! Common type definitions used throughout the game

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use crate::components::economics::GoodType;

// Entity ID types for type safety
pub type NationId = u32;
pub type ProvinceId = u32;
pub type CityId = u32;
pub type IndividualId = u32;
pub type TechnologyId = u32;

/// Game time representation
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GameTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub tick: u64,
}

impl GameTime {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self {
            year,
            month,
            day,
            tick: 0,
        }
    }
    
    pub fn advance(&mut self, days: Fixed32) {
        let total_days = days.integer_part();
        for _ in 0..total_days {
            self.day += 1;
            if self.day > 30 {
                self.day = 1;
                self.month += 1;
                if self.month > 12 {
                    self.month = 1;
                    self.year += 1;
                }
            }
        }
        self.tick += total_days as u64;
    }
}

// ResourceType removed - use GoodType from components/economics/mod.rs instead
// This eliminates the DRY violation where we had duplicate enums for resources/goods
// GoodType is the single source of truth for all tradeable items

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    Transportation,
    Finance,
    Education,
    Healthcare,
    Entertainment,
}

/// Technology types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechArea {
    Agriculture,
    Military,
    Economics,
    Science,
    Culture,
    Industry,
    Medicine,
    Information,
}

/// Industry types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndustryType {
    Agriculture,
    Mining,
    Manufacturing,
    Services,
    Technology,
    Energy,
}

/// Event types for the game
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub enum GameEvent {
    PopulationGrowth {
        province_id: ProvinceId,
        growth: Fixed32,
    },
    EconomicTransaction,
    Migration {
        migration_event: MigrationEvent,
    },
    NaturalDisaster {
        disaster: NaturalDisaster,
    },
    TechnologyDiscovery {
        nation_id: NationId,
        technology: TechnologyId,
    },
    War {
        aggressor: NationId,
        defender: NationId,
    },
    Peace {
        nation_a: NationId,
        nation_b: NationId,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MigrationEvent {
    pub from_province: ProvinceId,
    pub to_province: ProvinceId,
    pub population: Fixed32,
    pub reason: MigrationReason,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MigrationReason {
    Economic,
    War,
    Famine,
    Persecution,
    Opportunity,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NaturalDisaster {
    pub disaster_type: DisasterType,
    pub province_id: ProvinceId,
    pub severity: Fixed32,
    pub duration: GameTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DisasterType {
    Earthquake,
    Flood,
    Drought,
    Hurricane,
    Volcano,
    Plague,
    Fire,
}

/// Priority levels for events
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Placeholder structs that need proper implementation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeTerms {
    pub goods: Vec<(GoodType, Fixed32)>,
    pub duration: GameTime,
    pub price_adjustment: Fixed32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CrisisType {
    Economic,
    Political,
    Environmental,
    Social,
    Military,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarGoal {
    pub goal_type: WarGoalType,
    pub target: Option<ProvinceId>,
    pub completion_criteria: CompletionCriteria,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WarGoalType {
    Conquest,
    Liberation,
    Humiliation,
    Reparations,
    RegimeChange,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionCriteria {
    pub provinces_occupied: Vec<ProvinceId>,
    pub military_defeat_threshold: Fixed32,
    pub economic_damage_threshold: Fixed32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProxyConflict {
    pub location: ProvinceId,
    pub sponsor_a: NationId,
    pub sponsor_b: NationId,
    pub intensity: Fixed32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NeutralityType {
    Armed,
    Friendly,
    Strict,
    Opportunistic,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CooperationArea {
    Trade,
    Defense,
    Research,
    Cultural,
    Environmental,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AllianceType {
    Defensive,
    Offensive,
    Economic,
    Full,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Obligation {
    pub obligation_type: ObligationType,
    pub binding_strength: Fixed32,
    pub penalty_for_violation: PenaltyType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObligationType {
    MilitarySupport,
    EconomicAid,
    TerritorialDefense,
    ResourceSharing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PenaltyType {
    Economic,
    Diplomatic,
    Military,
    Reputation,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TreatyType {
    Peace,
    Trade,
    Alliance,
    NonAggression,
    Border,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreatyTerm {
    pub term_type: TreatyTermType,
    pub duration: Option<GameTime>,
    pub enforcement_level: Fixed32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TreatyTermType {
    TerritorialExchange,
    Reparations,
    TradeRights,
    MilitaryRestrictions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnforcementType {
    Honor,
    Economic,
    Military,
    International,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Consequence {
    pub consequence_type: ConsequenceType,
    pub severity: Fixed32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsequenceType {
    WarDeclaration,
    EconomicSanctions,
    DiplomaticIsolation,
    ReputationLoss,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TreatyDuration {
    Permanent,
    Fixed(GameTime),
    Conditional(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenewalCondition {
    pub condition_type: RenewalConditionType,
    pub threshold: Fixed32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RenewalConditionType {
    MutualBenefit,
    PowerBalance,
    NoViolations,
    ExternalThreat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceRecord {
    pub compliance_date: GameTime,
    pub compliance_level: Fixed32,
    pub violations: Vec<TreatyViolation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreatyViolation {
    pub violation_type: ViolationType,
    pub severity: Fixed32,
    pub violator: NationId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ViolationType {
    TerritorialViolation,
    EconomicViolation,
    MilitaryViolation,
    DiplomaticViolation,
}