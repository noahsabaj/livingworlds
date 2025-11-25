//! Nation and House system
//!
//! This module implements the political entities that control provinces,
//! including nations, dynasties, and their interactions.

// PRIVATE MODULES - Gateway architecture compliance
mod actions;
mod diplomacy;
mod errors;
mod generation;
mod governance;
mod history;
mod house;
mod laws;
mod neighbors;
mod ownership;
mod plugin;
pub mod relationships;  // Public for relationship component access
mod rendering;
mod territory_analysis;
mod types;
mod warfare;

pub use actions::{
    // Resolution systems - decide what to do
    handle_economic_pressure, handle_legitimacy_pressure, handle_military_pressure,
    handle_population_pressure, resolve_nation_actions,
    // Event types
    NationActionEvent, TerritoryOwnershipChanged, OwnershipChangeType,
};
pub use generation::{spawn_nations, build_territories_from_provinces};
pub use governance::{
    Governance, GovernmentCategory, GovernmentType,
    GovernmentTransition, GovernmentHistory, LegitimacyFactors, PoliticalPressure, get_structure_name,
};
pub use history::{
    BattleOutcome, HistoricalEvent, NationHistory, RulerTraits, SuccessionType,
    WarResult, create_initial_history,
};
pub use house::{
    House, HouseTraits, Ruler, RulerPersonality,
    // Drama engine exports
    DramaEnginePlugin, Character, CharacterId, CharacterRole,
    DramaEvent, DramaEventType, DramaEventId, EventImportance, EventVisibility,
    // Relationship system exports
    HasRelationship, RelationshipMetadata, RelationshipType,
};
pub use laws::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawRegistry, NationLaws, LawRepealEvent,
    LawEnactmentEvent, get_all_laws, LawPrerequisite,
};
pub use neighbors::{
    get_neighbor_strengths, rebuild_neighbor_relationships_on_ownership_change,
};
pub use warfare::{
    War, WarGoal, Battle, BattleConfig, BattleResult, WarOutcome, CasusBelli,
    DeclareWarEvent, BattleEvent, WarEndEvent,
    process_war_declarations, process_battle_events, check_war_resolution,
    record_battle_outcome,
};
pub use diplomacy::{
    CasusBelliExt, FabricatingClaim,
    evaluate_available_casus_belli,
    evaluate_war_triggers_from_pressure,
};
pub use ownership::{
    // O(1) ECS-based ownership queries using Controls/ControlledBy relationships
    get_nation_provinces, get_nation_province_count, nation_has_territory,
    nation_owns_province, get_province_owner, get_nation_bounds, get_nation_centroid,
};
pub use territory_analysis::TerritoryMetrics;
pub use rendering::{NationLabel, NationLabelShadow};
pub use plugin::NationPlugin;
pub use relationships::*;
pub use types::*;

/// Convert a Culture enum to a descriptive display name
pub fn culture_to_display_name(culture: crate::name_generator::Culture) -> &'static str {
    use crate::name_generator::Culture;
    match culture {
        Culture::Western => "Western European",
        Culture::Eastern => "East Asian",
        Culture::Northern => "Nordic",
        Culture::Southern => "Mediterranean",
        Culture::Desert => "Desert Nomadic",
        Culture::Island => "Island Seafaring",
        Culture::Ancient => "Ancient Civilizations",
        Culture::Mystical => "Mystical",
    }
}
