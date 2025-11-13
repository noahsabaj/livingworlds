//! Military Relationships - Army positioning and military structures
//!
//! This module defines relationships for military units, their positioning,
//! and the military hierarchy within nations.

use bevy::prelude::*;

// ================================================================================================
// ARMY POSITIONING RELATIONSHIPS
// ================================================================================================

/// An army is stationed in a specific province
/// Military unit positioning for strategic and tactical purposes
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = HostsArmies)]
pub struct StationedIn(pub Entity);

/// Reverse relationship: A province hosts military units
/// Automatically maintained by Bevy when `StationedIn` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = StationedIn, linked_spawn)]
pub struct HostsArmies(Vec<Entity>); // Private for safety - Bevy handles internal access

impl HostsArmies {
    /// Get read-only access to armies stationed in this province
    pub fn armies(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of armies stationed here
    pub fn army_count(&self) -> usize {
        self.0.len()
    }

    /// Check if a specific army is stationed here
    pub fn hosts_army(&self, army: Entity) -> bool {
        self.0.contains(&army)
    }

    /// Check if province has any armies
    pub fn has_armies(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// MILITARY ENTITIES
// ================================================================================================

/// Marker component for army entities
#[derive(Component, Debug, Clone)]
pub struct Army {
    pub name: String,
    pub size: u32,              // Number of soldiers
    pub morale: f32,            // 0.0 = broken, 1.0 = excellent
    pub experience: f32,        // 0.0 = green recruits, 1.0 = veterans
    pub equipment_quality: f32, // 0.0 = poor, 1.0 = excellent
    pub army_type: ArmyType,
    pub owner_nation: Entity, // Nation that owns this army
}

/// Marker component for fortifications
#[derive(Component, Debug, Clone)]
pub struct Fortification {
    pub name: String,
    pub fortification_type: FortificationType,
    pub defensive_strength: f32, // Defensive bonus
    pub garrison_capacity: u32,  // Max army size that can be stationed
    pub construction_year: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmyType {
    Infantry, // Basic foot soldiers
    Cavalry,  // Mounted units
    Archers,  // Ranged units
    Siege,    // Siege equipment
    Navy,     // Naval forces
    Elite,    // Special elite units
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FortificationType {
    Palisade,  // Basic wooden fortification
    StoneWall, // Stone wall fortification
    Fortress,  // Major fortress
    Citadel,   // Heavily fortified citadel
}

// ================================================================================================
// MILITARY DATA
// ================================================================================================

/// Provincial military status
#[derive(Component, Debug, Clone)]
pub struct MilitaryStatus {
    /// Total military strength in this province
    pub total_strength: f32,
    /// Number of armies stationed here
    pub army_count: u32,
    /// Defensive value from fortifications
    pub defensive_value: f32,
    /// Whether this is a strategic military position
    pub strategic_importance: StrategicImportance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategicImportance {
    Low,      // Remote location
    Medium,   // Some strategic value
    High,     // Important position
    Critical, // Vital strategic location
}

// ================================================================================================
// QUERY SYSTEMS - Military intelligence
// ================================================================================================

/// System for querying army positions
pub fn query_army_positions_system(armies_query: Query<(Entity, &Army, Option<&StationedIn>)>) {
    for (army_entity, army, stationed_in) in armies_query.iter() {
        let position = stationed_in.map(|si| si.0);
        debug!(
            "Army {:?} ({}) stationed at province {:?}",
            army_entity, army.name, position
        );
    }
}

/// System for querying military strength by province
pub fn query_provincial_military_strength_system(
    provinces_query: Query<(Entity, &HostsArmies)>,
    armies_query: Query<&Army>,
) {
    for (province_entity, hosts_armies) in provinces_query.iter() {
        let total_strength: f32 = hosts_armies
            .armies()
            .iter()
            .filter_map(|&army_entity| armies_query.get(army_entity).ok())
            .map(|army| calculate_army_strength(army))
            .sum();
        debug!(
            "Province {:?} has military strength: {:.1}",
            province_entity, total_strength
        );
    }
}

/// Find armies belonging to a specific nation
pub fn find_nation_armies(
    nation_entity: Entity,
    armies_query: &Query<(Entity, &Army)>,
) -> Vec<Entity> {
    armies_query
        .iter()
        .filter(|(_, army)| army.owner_nation == nation_entity)
        .map(|(entity, _)| entity)
        .collect()
}

/// Find all armies in a province
pub fn find_province_armies(
    province_entity: Entity,
    provinces_query: &Query<&HostsArmies>,
) -> Option<Vec<Entity>> {
    provinces_query
        .get(province_entity)
        .ok()
        .map(|hosts| hosts.0.clone())
}

// ================================================================================================
// MILITARY CALCULATIONS
// ================================================================================================

/// Calculate effective army strength based on all factors
pub fn calculate_army_strength(army: &Army) -> f32 {
    let base_strength = army.size as f32;
    let morale_modifier = 0.5 + (army.morale * 0.5); // 0.5x to 1.0x
    let experience_modifier = 0.7 + (army.experience * 0.3); // 0.7x to 1.0x
    let equipment_modifier = 0.8 + (army.equipment_quality * 0.2); // 0.8x to 1.0x

    base_strength * morale_modifier * experience_modifier * equipment_modifier
}

/// Calculate defensive strength of a province
pub fn calculate_provincial_defense(
    province_entity: Entity,
    fortifications_query: &Query<(&Fortification, &StationedIn)>,
    armies_query: &Query<&Army>,
) -> f32 {
    let mut total_defense = 0.0;

    // Add fortification bonuses
    let fortification_bonus: f32 = fortifications_query
        .iter()
        .filter(|(_, stationed_in)| stationed_in.0 == province_entity)
        .map(|(fortification, _)| fortification.defensive_strength)
        .sum();

    total_defense += fortification_bonus;

    // Add army defensive strength (reduced compared to offensive strength)
    // TODO: This requires a Query<&HostsArmies> parameter to be passed to this function
    // Temporarily disabled until the function signature can be updated
    // if let Some(armies) = find_province_armies(province_entity, provinces_query) {
    //     let army_defense: f32 = armies.iter()
    //         .filter_map(|&army_entity| armies_query.get(army_entity).ok())
    //         .map(|army| calculate_army_strength(army) * 0.8) // Defensive penalty
    //         .sum();
    //     total_defense += army_defense;
    // }

    total_defense
}

// ================================================================================================
// MILITARY SYSTEMS
// ================================================================================================

/// Updates military status for all provinces
pub fn update_military_status(
    mut provinces_query: Query<(Entity, &mut MilitaryStatus, &HostsArmies)>,
    armies_query: Query<&Army>,
    fortifications_query: Query<(&Fortification, &StationedIn)>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (province_entity, mut military_status, hosts_armies) in &mut provinces_query {
            // Count armies and calculate total strength
            military_status.army_count = hosts_armies.army_count() as u32;
            military_status.total_strength = hosts_armies
                .armies()
                .iter()
                .filter_map(|&army_entity| armies_query.get(army_entity).ok())
                .map(|army| calculate_army_strength(army))
                .sum();

            // Calculate defensive value from fortifications
            military_status.defensive_value = fortifications_query
                .iter()
                .filter(|(_, stationed_in)| stationed_in.0 == province_entity)
                .map(|(fortification, _)| fortification.defensive_strength)
                .sum();

            // Determine strategic importance
            military_status.strategic_importance = determine_strategic_importance(
                military_status.total_strength,
                military_status.defensive_value,
                military_status.army_count,
            );
    }
}

/// Determine strategic importance based on military factors
fn determine_strategic_importance(
    total_strength: f32,
    defensive_value: f32,
    army_count: u32,
) -> StrategicImportance {
    let combined_value = total_strength + defensive_value + (army_count as f32 * 10.0);

    match combined_value {
        x if x >= 1000.0 => StrategicImportance::Critical,
        x if x >= 500.0 => StrategicImportance::High,
        x if x >= 100.0 => StrategicImportance::Medium,
        _ => StrategicImportance::Low,
    }
}

// ================================================================================================
// MILITARY EVENTS
// ================================================================================================

/// Event fired when an army moves to a new position
#[derive(Message, Debug, Clone)]
pub struct ArmyMovedEvent {
    pub army: Entity,
    pub from_province: Option<Entity>,
    pub to_province: Entity,
}

/// Event fired when a fortification is constructed
#[derive(Message, Debug, Clone)]
pub struct FortificationBuiltEvent {
    pub fortification: Entity,
    pub province: Entity,
    pub builder_nation: Entity,
}

/// Event fired when armies engage in battle
#[derive(Message, Debug, Clone)]
pub struct BattleEvent {
    pub attacking_armies: Vec<Entity>,
    pub defending_armies: Vec<Entity>,
    pub location: Entity, // Province where battle occurs
    pub outcome: BattleOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleOutcome {
    AttackerVictory,
    DefenderVictory,
    Draw,
    Retreat,
}

// ================================================================================================
// VALIDATION SYSTEMS
// ================================================================================================

/// Validates military positioning
pub fn validate_military_positions(
    armies_query: Query<(Entity, &Army, Option<&StationedIn>)>,
    provinces_query: Query<Entity>,
) {
    let valid_provinces: std::collections::HashSet<Entity> = provinces_query.iter().collect();

    for (army_entity, army, stationed_in) in armies_query.iter() {
        if let Some(stationed_in) = stationed_in {
            if !valid_provinces.contains(&stationed_in.0) {
                warn!(
                    "Army {:?} ({}) is stationed in invalid province {:?}",
                    army_entity, army.name, stationed_in.0
                );
            }
        }
    }
}

/// Validates army ownership
pub fn validate_army_ownership(armies_query: Query<(Entity, &Army)>, nations_query: Query<Entity>) {
    let valid_nations: std::collections::HashSet<Entity> = nations_query.iter().collect();

    for (army_entity, army) in armies_query.iter() {
        if !valid_nations.contains(&army.owner_nation) {
            warn!(
                "Army {:?} ({}) is owned by invalid nation {:?}",
                army_entity, army.name, army.owner_nation
            );
        }
    }
}
