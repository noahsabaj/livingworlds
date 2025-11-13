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
mod debug; // Debug utilities for relationship system
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
    CapitalOf,
    // Core political relationship components
    ControlledBy,
    Controls,
    HasCapital,
};

// ================================================================================================
// DIPLOMATIC RELATIONSHIPS - Inter-nation relations
// ================================================================================================


// ================================================================================================
// CULTURAL RELATIONSHIPS - Cultural regions and identity
// ================================================================================================


// ================================================================================================
// LEGISLATIVE RELATIONSHIPS - Laws and governance
// ================================================================================================

pub use legislative::{
    // Legislative relationship components
    EnactedLaws,
    // Law entity components
    LawEntity,
};

// ================================================================================================
// ADMINISTRATIVE RELATIONSHIPS - Governance and administration
// ================================================================================================


// ================================================================================================
// INFRASTRUCTURE RELATIONSHIPS - Physical connections
// ================================================================================================


// ================================================================================================
// MILITARY RELATIONSHIPS - Army positioning and structures
// ================================================================================================


// ================================================================================================
// RELIGIOUS RELATIONSHIPS - Faith and influence
// ================================================================================================


// ================================================================================================
// POPULATION RELATIONSHIPS - Demographics and residence
// ================================================================================================


// ================================================================================================
// BEVY PLUGIN INTEGRATION
// ================================================================================================

pub use plugin::RelationshipsPlugin;

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
/// ```rust,no_run
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
/// ```no_run
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
