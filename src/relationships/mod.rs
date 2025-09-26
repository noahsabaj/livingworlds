//! Relationships module gateway - Entity relationship system for Living Worlds
//!
//! This is a PURE GATEWAY following strict gateway architecture principles.
//! NO IMPLEMENTATION CODE HERE - only module organization and exports.
//!
//! # Architecture
//!
//! The relationships module provides Bevy 0.16 Entity Relationships that replace
//! manual relationship tracking throughout Living Worlds:
//!
//! - **political.rs**: Nation/province ownership, capitals, ruling relationships
//! - **diplomatic.rs**: Inter-nation relations (alliances, wars, trade)
//! - **cultural.rs**: Cultural regions and provincial cultural membership
//! - **administrative.rs**: Governors and provincial administration
//! - **infrastructure.rs**: Roads, trade routes, and connections
//! - **military.rs**: Army positioning and military structures
//! - **religious.rs**: Religious influence and faith spread
//! - **population.rs**: Demographics, population groups, and migration
//! - **plugin.rs**: Bevy plugin that integrates all systems
//!
//! # Revolutionary Benefits
//!
//! - **Automatic Bidirectional Tracking**: When you add `ControlledBy(nation)` to a province,
//!   Bevy automatically creates `Controls(provinces)` on the nation!
//! - **No Manual Synchronization**: Eliminates ProvinceOwnershipCache and all HashMap maintenance
//! - **Graph Database Architecture**: True entity relationship graph with automatic integrity
//! - **Performance**: Entity relationships are optimized for queries and updates

// PRIVATE IMPLEMENTATION MODULES - All implementation details are hidden

mod administrative; // Governor and provincial administration relationships
mod cultural; // Cultural region and provincial cultural membership
mod diplomatic; // Inter-nation diplomatic relationships
mod infrastructure; // Roads, trade routes, and physical connections
mod legislative; // Legislative and law enactment relationships
mod military; // Army positioning and military structures
mod plugin; // Bevy plugin for relationships system
mod political; // Core political relationships (nation ownership, capitals)
mod population; // Demographics, population groups, and migration
mod religious; // Religious influence and faith spread

// SELECTIVE PUBLIC EXPORTS - The controlled API surface

// ================================================================================================
// POLITICAL RELATIONSHIPS - Core governance
// ================================================================================================

pub use political::{
    query_nation_capitals,
    // Political query functions (renamed from _system)
    query_nation_territories,
    query_province_owners,
    validate_capital_assignments,
    // Political validation systems
    validate_province_ownership,
    CapitalOf,
    // Core political relationship components
    ControlledBy,
    Controls,
    HasCapital,
    RuledBy,
    RulesOver,
};

// ================================================================================================
// DIPLOMATIC RELATIONSHIPS - Inter-nation relations
// ================================================================================================

pub use diplomatic::{
    get_diplomatic_partners,
    // Diplomatic query systems
    query_alliances,
    query_trade_relationships,
    query_wars,
    // Diplomatic update systems
    update_diplomatic_state,
    validate_diplomatic_consistency,
    // Diplomatic events
    AllianceFormedEvent,
    // Diplomatic relationship components
    AlliedWith,
    AtWarWith,
    DiplomaticRelationType,
    DiplomaticState,
    PeaceTreatyEvent,
    TradeAgreementEvent,
    TradesWithNation,
    WarDeclaredEvent,
};

// ================================================================================================
// CULTURAL RELATIONSHIPS - Cultural regions and identity
// ================================================================================================

pub use cultural::{
    detect_cultural_tensions,
    find_regions_by_culture,
    get_provinces_in_region,
    // Cultural query functions (renamed from _system)
    query_cultural_regions,
    query_province_regions,
    // Cultural analysis systems
    update_cultural_coherence,
    // Cultural validation systems
    validate_cultural_regions,
    validate_exclusive_cultural_membership,
    // Cultural relationship components
    BelongsToRegion,
    ContainsProvinces,
    CulturalCoherence,
    CulturalRegion,
    // Cultural events
    CulturalTensionEvent,
    CulturalUnificationEvent,
    FragmentationCause,
};

// ================================================================================================
// LEGISLATIVE RELATIONSHIPS - Laws and governance
// ================================================================================================

pub use legislative::{
    // Legislative relationship components
    EnactedBy,
    EnactedLaws,
    ProposedFor,
    ProposedLaws,
    RepealedBy,
    RepealedLaws,
    ConflictsWith,
    ConflictedBy,
    // Law entity components
    LawEntity,
    ProposalStatus,
    RepealInfo,
    // Legislative events
    LawEnactedEvent,
    LawProposedEvent,
    LawRepealedEvent,
};

// ================================================================================================
// ADMINISTRATIVE RELATIONSHIPS - Governance and administration
// ================================================================================================

pub use administrative::{
    find_province_governor,
    get_governor_provinces,
    // Administrative query systems
    query_provincial_administration_system,
    // Administrative update systems
    update_administrative_efficiency,
    validate_administrative_assignments,
    AdministeredBy,
    // Administrative relationship components
    Administers,
    AdministrativeEfficiency,
    CorruptionDetectedEvent,
    DismissalReason,
    Governor,
    // Administrative events
    GovernorAppointedEvent,
    GovernorDismissedEvent,
};

// ================================================================================================
// INFRASTRUCTURE RELATIONSHIPS - Physical connections
// ================================================================================================

pub use infrastructure::{
    calculate_trade_efficiency,
    find_province_roads,
    find_province_trade_routes,
    // Infrastructure query systems
    query_road_network_system,
    query_trade_network_system,
    // Infrastructure calculation systems
    update_infrastructure_status,
    // Infrastructure validation systems
    validate_infrastructure_connections,
    // Infrastructure relationship components
    ConnectedByRoad,
    ConnectedByTrade,
    InfrastructureMaintenanceEvent,
    InfrastructureStatus,
    MaintenanceUrgency,
    Road,
    // Infrastructure events
    RoadConstructedEvent,
    // Infrastructure type enums
    RoadQuality,
    TradeRoute,
    TradeRouteEstablishedEvent,
    TradeRouteType,
};

// ================================================================================================
// MILITARY RELATIONSHIPS - Army positioning and structures
// ================================================================================================

pub use military::{
    // Military calculation systems
    calculate_army_strength,
    calculate_provincial_defense,
    find_nation_armies,
    find_province_armies,
    // Military query systems
    query_army_positions_system,
    query_provincial_military_strength_system,
    update_military_status,
    validate_army_ownership,
    // Military validation systems
    validate_military_positions,
    Army,
    // Military events
    ArmyMovedEvent,
    // Military type enums
    ArmyType,
    BattleEvent,
    BattleOutcome,
    Fortification,
    FortificationBuiltEvent,
    FortificationType,
    HostsArmies,
    MilitaryStatus,
    // Military relationship components
    StationedIn,
    StrategicImportance,
};

// ================================================================================================
// RELIGIOUS RELATIONSHIPS - Faith and influence
// ================================================================================================

pub use religious::{
    find_religion_provinces,
    get_dominant_religion,
    query_provincial_religions,
    // Religious query systems
    query_religious_influence,
    simulate_religious_spread,
    // Religious update systems
    update_religious_status,
    validate_religious_influence_consistency,
    // Religious validation systems
    validate_religious_relationships,
    ConflictType,
    InfluencedByReligions,
    // Religious relationship components
    InfluencesProvince,
    Religion,
    ReligionFoundedEvent,
    // Religious type enums
    ReligionType,
    ReligiousConflictEvent,
    // Religious events
    ReligiousConversionEvent,
    ReligiousInfluence,
    ReligiousStatus,
};

// ================================================================================================
// POPULATION RELATIONSHIPS - Demographics and residence
// ================================================================================================

pub use population::{
    calculate_provincial_population,
    find_province_populations,
    // Population query systems
    query_population_distribution_system,
    query_provincial_demographics_system,
    // Population update systems
    update_provincial_demographics,
    validate_demographic_consistency,
    // Population validation systems
    validate_population_residence,
    CulturalGroup,
    DemographicShiftEvent,
    DemographicShiftType,
    Demographics,
    HostsPopulations,
    // Population events
    MigrationEvent,
    MigrationFlow,
    MigrationType,
    Occupation,
    PopulationChangeEvent,
    PopulationChangeType,
    PopulationGroup,
    PullFactor,
    PushFactor,
    // Population relationship components
    ResidesIn,
    // Population type enums and structures
    SocialClass,
    SocialStratification,
};

// ================================================================================================
// BEVY PLUGIN INTEGRATION
// ================================================================================================

pub use plugin::{
    // Debug utilities
    DebugRelationships,
    // Main plugin
    RelationshipsPlugin,
};

// ================================================================================================
// RELATIONSHIP SYSTEM DOCUMENTATION
// ================================================================================================

/// # Living Worlds Entity Relationships
///
/// This module provides a complete entity relationship system using Bevy 0.16's
/// automatic bidirectional relationship tracking. It replaces manual tracking
/// throughout Living Worlds with a true graph database architecture.
///
/// ## Core Benefits
///
/// 1. **Automatic Synchronization**: No more manual HashMap maintenance
/// 2. **Bidirectional Queries**: Query relationships from either direction
/// 3. **Data Integrity**: Automatic validation and consistency checking
/// 4. **Performance**: Optimized for 3M+ province simulations
///
/// ## Usage Example
///
/// ```rust
/// // Old manual way (REMOVED)
/// province.owner = Some(nation_id);
/// ownership_cache.by_nation.get_mut(&nation_id).unwrap().insert(province.id);
///
/// // New entity relationship way
/// commands.entity(province_entity).insert(ControlledBy(nation_entity));
/// // Bevy automatically creates Controls(provinces) on the nation!
///
/// // Query territories owned by nation
/// if let Ok(controls) = nations_query.get(nation_entity) {
///     for &province_entity in &controls.0 {
///         // Process each controlled province
///     }
/// }
/// ```
///
/// ## Performance Notes
///
/// Entity relationships are designed for Living Worlds' scale:
/// - 3,000,000+ provinces
/// - 20-100 nations
/// - Complex diplomatic networks
/// - Dynamic cultural regions
///
/// The system provides O(1) relationship queries and automatic integrity maintenance.
pub struct RelationshipSystemDocumentation;
