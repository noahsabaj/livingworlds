//! Gateway module for the legitimacy system
//!
//! This module contains all legitimacy-related types and calculations,
//! providing a comprehensive system for tracking government legitimacy.

// Private submodules
mod calculation;
mod events;
mod factors;
mod weights;

// Public re-exports

// Core legitimacy tracking
pub use factors::{
    BrokenPromise, CorruptionScandal, DivineApproval, ElectoralMandate, InstitutionalControl,
    LegitimacyFactors, MilitaryVictory, RevolutionaryFervor, SeparatistMovement,
};

// Legitimacy events
pub use events::{LegitimacyEvent, LegitimacyEventType};

// Weight system
pub use weights::LegitimacyWeights;