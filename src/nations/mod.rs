//! Nation and House system
//!
//! This module implements the political entities that control provinces,
//! including nations, dynasties, and their interactions.

// PRIVATE MODULES - Gateway architecture compliance
mod actions;
mod generation;
mod governance;
mod history;
mod house;
mod laws;
mod plugin;
mod rendering;
mod territory_analysis;
mod types;

pub use actions::{
    handle_economic_pressure, handle_legitimacy_pressure, handle_military_pressure,
    handle_population_pressure, NationAction, NationActionEvent, resolve_nation_actions,
};
pub use generation::{build_territories_from_provinces, spawn_nations};
pub use governance::{
    BrokenPromise, CorruptionScandal, CrisisFactors, DivineApproval, ElectoralMandate,
    Governance, GovernancePlugin, GovernanceSettings, GovernmentCategory, GovernmentType,
    GovernmentTransition, GovernmentHistory, GovernmentChange, InstitutionalControl,
    LegitimacyEvent, LegitimacyEventType, LegitimacyFactors, LegitimacyWeights,
    MilitaryVictory, PoliticalPressure, RevolutionaryFervor, SeparatistMovement,
    generate_governance_aware_name, get_ruler_title, get_structure_name,
};
pub use history::{
    BattleOutcome, HistoricalEvent, NationHistory, RulerInfo, RulerTraits, SuccessionType,
    WarResult, WarStatus, create_initial_history,
};
pub use house::{
    generate_motto, DominantTrait, House, HouseArchetype, HouseTraits, Ruler, RulerPersonality,
    // Drama engine exports
    DramaEnginePlugin, Character, CharacterId, CharacterRole, DramaEvent, DramaEventType,
    DramaEventId, EventImportance, EventVisibility, EventConsequence,
};
pub use laws::{
    LawPlugin, Law, LawId, LawCategory, LawEffects, LawStatus, LawRegistry, NationLaws,
    LawEnactmentEvent, LawRepealEvent,
};
pub use plugin::NationPlugin;
pub use territory_analysis::{NationSizeCategory, TerritoryMetrics, TerritoryMetricsCache};
pub use types::*;
