//! Territory ownership events
//!
//! Events fired when province ownership changes (expansion, conquest, etc.)

use bevy::prelude::*;

/// Types of ownership changes
#[derive(Debug, Clone, Copy)]
pub enum OwnershipChangeType {
    /// Peaceful settlement of unclaimed land
    Expansion,
    /// Military conquest from another nation
    Conquest,
    /// Diplomatic transfer (vassalage, gift, etc.)
    Diplomatic,
    /// Province abandoned or lost
    Loss,
}

/// Territory ownership has changed (expansion, conquest, etc.)
/// Triggers relationship rebuilds for neighbor detection and other systems
#[derive(Debug, Clone, Message)]
pub struct TerritoryOwnershipChanged {
    pub nation_entity: Entity,
    pub provinces_changed: u32,
    pub change_type: OwnershipChangeType,
}
