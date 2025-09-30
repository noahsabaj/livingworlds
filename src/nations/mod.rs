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
    handle_population_pressure, resolve_nation_actions,
    // Execution systems - actually do it (with reactive cache invalidation!)
    execute_expansion_events,
    // Events and types
    NationAction, NationActionEvent, ReformType, PublicWorkType, ActionType,
};
pub use errors::{
    NationError, NationResult, TransitionError, LawError, TerritoryError,
    DiplomaticError, EconomicError, MilitaryError, IntegrityError,
    RecoveryStrategy, ErrorSeverity,
};
pub use generation::{build_territories_from_provinces, spawn_nations};
pub use governance::{
    BrokenPromise, CorruptionScandal, CrisisFactors, DivineApproval, ElectoralMandate,
    Governance, GovernancePlugin, GovernanceSettings, GovernmentCategory, GovernmentType,
    GovernmentTransition, GovernmentHistory, GovernmentChange, InstitutionalControl,
    LegitimacyEvent, LegitimacyEventType, LegitimacyFactors, LegitimacyWeights,
    MilitaryVictory, PoliticalPressure, RevolutionaryFervor, SeparatistMovement,
    generate_governance_aware_name, get_ruler_title, get_structure_name, build_nation_name,
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
    LawEnactmentEvent, LawRepealEvent, get_all_laws, get_category_laws, get_law_by_id,
    LawPrerequisite,
};
pub use plugin::NationPlugin;
pub use migration::{
    NationMigrationPlugin, MigrationStatus,
    spawn_nations_with_relationships, migrate_ownership_to_relationships,
};
pub use territory_analysis::{NationSizeCategory, TerritoryMetrics, TerritoryMetricsCache};
pub use types::*;
