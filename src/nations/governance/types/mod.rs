//! Gateway module for governance type definitions
//!
//! This module organizes all governance-related types following gateway architecture.
//! Each submodule contains focused, single-responsibility type definitions.

// Private submodules
mod component;
mod crisis;
mod government;
mod legitimacy;
mod mechanics;
mod pressure;
mod settings;
mod succession;

// Public re-exports - The controlled API surface

// Core government types
pub use government::{GovernmentCategory, GovernmentType};

// Mechanics and restrictions
pub use mechanics::{GovernmentMechanics, GovernmentRestriction, UniqueMechanic};

// Main governance component
pub use component::Governance;

// Crisis tracking
pub use crisis::CrisisFactors;

// Political pressure
pub use pressure::PoliticalPressure;

// Settings and configuration
pub use settings::{GovernanceSettings, GovernmentWeights};

// Succession and gender
pub use succession::{Gender, SuccessionType};

// Legitimacy system (from subdirectory)
pub use legitimacy::{
    BrokenPromise, CorruptionScandal, DivineApproval, ElectoralMandate, InstitutionalControl,
    LegitimacyEvent, LegitimacyEventType, LegitimacyFactors, LegitimacyWeights, MilitaryVictory,
    RevolutionaryFervor, SeparatistMovement,
};