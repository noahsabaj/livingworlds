//! Nation and House system
//!
//! This module implements the political entities that control provinces,
//! including nations, dynasties, and their interactions.

// PRIVATE MODULES - Gateway architecture compliance
mod actions;
mod errors;
mod generation;
mod governance;
mod history;
mod house;
mod laws;
mod migration;
mod plugin;
mod rendering;
mod territory_analysis;
mod types;

// Test modules
#[cfg(test)]
mod benches;
#[cfg(test)]
mod integration_tests;
#[cfg(test)]
mod property_tests;

pub use actions::{
    // Resolution systems - decide what to do
    handle_economic_pressure, handle_legitimacy_pressure, handle_military_pressure,
    handle_population_pressure, resolve_nation_actions, NationActionEvent,
};
pub use generation::spawn_nations;
pub use governance::{
    Governance, GovernmentCategory, GovernmentType,
    GovernmentTransition, GovernmentHistory, LegitimacyFactors, PoliticalPressure, get_structure_name,
};
pub use history::{
    BattleOutcome, HistoricalEvent, NationHistory, RulerTraits, SuccessionType,
    WarResult, WarStatus, create_initial_history,
};
pub use house::{
    House, HouseTraits, Ruler, RulerPersonality,
    // Drama engine exports
    DramaEnginePlugin, Character, DramaEvent, DramaEventType,
    DramaEventId, EventImportance, EventVisibility,
};
pub use laws::{
    LawId, LawCategory, LawEffects, LawRegistry, NationLaws, LawRepealEvent, get_all_laws,
    LawPrerequisite,
};
pub use plugin::NationPlugin;
pub use types::*;
