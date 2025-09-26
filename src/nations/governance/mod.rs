//! Gateway module for governance and political systems
//!
//! This module provides the complete political and governance system for Living Worlds,
//! enabling dynamic government types, political transitions, and governance-aware naming.

// Private submodules (gateway architecture)
mod history;
mod legitimacy;
mod naming;
mod plugin;
mod pressure;
mod transitions;
mod types;

// Public exports (controlled API surface)
pub use plugin::GovernancePlugin;

pub use types::{
    BrokenPromise, CorruptionScandal, CrisisFactors, DivineApproval, ElectoralMandate,
    Gender, Governance, GovernanceSettings, GovernmentCategory, GovernmentMechanics,
    GovernmentRestriction, GovernmentType, GovernmentWeights, InstitutionalControl,
    LegitimacyEvent, LegitimacyEventType, LegitimacyFactors, LegitimacyWeights,
    MilitaryVictory, PoliticalPressure, RevolutionaryFervor, SeparatistMovement,
    SuccessionType, UniqueMechanic,
};

pub use naming::{
    generate_governance_aware_name, get_ruler_title, get_structure_name,
    suggest_government_for_culture, DevelopmentLevel,
};

pub use transitions::{can_transition, transition_government, GovernmentTransition};

pub use history::{GovernmentChange, GovernmentHistory};