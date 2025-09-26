//! Law system for Living Worlds
//!
//! This module implements a comprehensive legal framework that allows nations
//! to pass, enforce, and repeal laws based on their government type and pressures.
//! Laws create observable effects on nation behavior, economy, and social dynamics.

// PRIVATE MODULES - Gateway architecture compliance
mod categories; // TODO: Remove once definitions/ is fully migrated
mod debug;
mod definitions;
mod initialization;
mod mechanics;
mod passage;
mod plugin;
mod registry;
mod systems;
mod types;

// Test modules
#[cfg(test)]
mod tests;

// Public exports (controlled API surface)
pub use plugin::LawPlugin;

// Re-export types through gateway
pub use types::{
    // Core types
    Law, LawId, LawCategory, LawPrerequisite,
    // Effects
    LawEffects, PopularityWeights,
    // Status
    LawStatus, LawComplexity, LawPopularity,
    // Events
    LawEnactmentEvent, LawRepealEvent,
};

pub use categories::{
    EconomicLaw, MilitaryLaw, SocialLaw, ReligiousLaw, CriminalLaw,
    PropertyLaw, ImmigrationLaw, EnvironmentalLaw, TechnologyLaw,
    CulturalLaw, AdministrativeLaw, DiplomaticLaw, get_all_laws,
    get_category_laws, get_law_by_id,
};

pub use registry::{
    LawRegistry, NationLaws, ActiveLaws, LawHistory, LawChange, LawChangeType, ProposedLaw,
};

pub use mechanics::{
    calculate_law_effects, apply_law_modifiers, check_law_conflicts,
    evaluate_law_popularity, get_government_law_affinity, calculate_popularity_weights,
    suggest_laws_for_pressures, calculate_law_diplomatic_impact, apply_diminishing_returns,
};

// Debug tools (conditional export for development)
#[cfg(debug_assertions)]
pub use debug::{
    LawDebugPlugin,
    LawDebugCommands,
    toggle_law_debug_overlay,
    validate_law_consistency,
    spawn_test_laws,
};

pub use passage::{
    evaluate_law_passage, trigger_law_vote, process_law_reform,
    emergency_law_powers, revolutionary_law_changes,
};