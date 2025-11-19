//! Diplomacy module - Casus belli, war justification, and diplomatic systems
//!
//! This module implements:
//! - Casus belli (CB) justification system
//! - CB validation and cost calculation
//! - Pressure-triggered war declarations
//! - Available CB evaluation for AI decision making

mod casus_belli;
mod systems;
mod war_triggers;

pub use casus_belli::{CasusBelliExt, FabricatingClaim};
pub use systems::evaluate_available_casus_belli;
pub use war_triggers::evaluate_war_triggers_from_pressure;
