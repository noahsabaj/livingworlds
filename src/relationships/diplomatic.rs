//! Diplomatic Relationships - Inter-nation relations
//!
//! This module defines relationships between nations: alliances, wars, trade agreements,
//! and other diplomatic interactions that shape the political landscape.

use bevy::prelude::*;

// ================================================================================================
// ALLIANCE RELATIONSHIPS
// ================================================================================================

/// Nation A is allied with Nation B
/// Mutual defense and trade agreements
///
/// Alliances provide mutual defense benefits and trade bonuses.
/// They can be broken under certain conditions (betrayal mechanics).
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = Allies)]
pub struct AlliedWith(pub Entity);

/// Reverse relationship: A nation has allies
/// Automatically maintained by Bevy when `AlliedWith` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = AlliedWith, linked_spawn)]
pub struct Allies(Vec<Entity>); // Private for safety - Bevy handles internal access

impl Allies {
    /// Get read-only access to allies
    pub fn allies(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of allies
    pub fn ally_count(&self) -> usize {
        self.0.len()
    }

    /// Check if allied with a specific nation
    pub fn allied_with(&self, nation: Entity) -> bool {
        self.0.contains(&nation)
    }

    /// Check if nation has any allies
    pub fn has_allies(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// WAR RELATIONSHIPS
// ================================================================================================

/// Nation A is at war with Nation B
/// Active military conflict
///
/// Wars block trade, enable territorial conquest, and affect stability.
/// Wars can be resolved through victory, defeat, or peace treaties.
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = Enemies)]
pub struct AtWarWith(pub Entity);

/// Reverse relationship: A nation has enemies at war
/// Automatically maintained by Bevy when `AtWarWith` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = AtWarWith, linked_spawn)]
pub struct Enemies(Vec<Entity>); // Private for safety - Bevy handles internal access

impl Enemies {
    /// Get read-only access to enemies
    pub fn enemies(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of enemies
    pub fn enemy_count(&self) -> usize {
        self.0.len()
    }

    /// Check if at war with a specific nation
    pub fn at_war_with(&self, nation: Entity) -> bool {
        self.0.contains(&nation)
    }

    /// Check if nation has any enemies
    pub fn has_enemies(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// TRADE RELATIONSHIPS
// ================================================================================================

/// Nation A has a trade agreement with Nation B
/// Economic cooperation and resource exchange
///
/// Trade agreements boost both nations' economies and resource availability.
/// They can be affected by wars, alliances, and economic policies.
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = TradePartners)]
pub struct TradesWithNation(pub Entity);

/// Reverse relationship: A nation has trade partners
/// Automatically maintained by Bevy when `TradesWithNation` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = TradesWithNation, linked_spawn)]
pub struct TradePartners(Vec<Entity>); // Private for safety - Bevy handles internal access

impl TradePartners {
    /// Get read-only access to trade partners
    pub fn trade_partners(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of trade partners
    pub fn partner_count(&self) -> usize {
        self.0.len()
    }

    /// Check if trading with a specific nation
    pub fn trades_with(&self, nation: Entity) -> bool {
        self.0.contains(&nation)
    }

    /// Check if nation has any trade partners
    pub fn has_trade_partners(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// DIPLOMACY STATE TRACKING
// ================================================================================================

/// Comprehensive diplomatic relationship data
/// Attached to nation entities to track diplomatic state
#[derive(Component, Debug, Clone, Default)]
pub struct DiplomaticState {
    /// Number of allies this nation has
    pub ally_count: u32,
    /// Number of enemies this nation is at war with
    pub enemy_count: u32,
    /// Number of trade partners
    pub trade_partner_count: u32,
    /// Overall diplomatic stance (-1.0 = hostile, 1.0 = peaceful)
    pub diplomatic_stance: f32,
}

// ================================================================================================
// QUERY SYSTEMS - Diplomatic intelligence
// ================================================================================================

/// Query all alliances in the world
pub fn query_alliances<'w, 's>(
    nations_query: &'w Query<'w, 's, (Entity, &'w AlliedWith)>,
) -> impl Iterator<Item = (Entity, Entity)> + 'w {
    nations_query
        .iter()
        .map(|(nation_a, allied_with)| (nation_a, allied_with.0))
}

/// Query all active wars
pub fn query_wars<'w, 's>(
    nations_query: &'w Query<'w, 's, (Entity, &'w AtWarWith)>,
) -> impl Iterator<Item = (Entity, Entity)> + 'w {
    nations_query
        .iter()
        .map(|(nation_a, at_war_with)| (nation_a, at_war_with.0))
}

/// Query all trade relationships
pub fn query_trade_relationships<'w, 's>(
    nations_query: &'w Query<'w, 's, (Entity, &'w TradesWithNation)>,
) -> impl Iterator<Item = (Entity, Entity)> + 'w {
    nations_query
        .iter()
        .map(|(nation_a, trades_with)| (nation_a, trades_with.0))
}

/// Get all nations that a specific nation has diplomatic relations with
/// Using entity relationships for comprehensive diplomatic state tracking
pub fn get_diplomatic_partners(
    nation_entity: Entity,
    diplomatic_query: &Query<(Option<&Allies>, Option<&Enemies>, Option<&TradePartners>)>,
) -> Vec<(Entity, DiplomaticRelationType)> {
    let mut partners = Vec::new();

    if let Ok((allies, enemies, trade_partners)) = diplomatic_query.get(nation_entity) {
        // Add all allies
        if let Some(allies) = allies {
            for &ally in allies.allies() {
                partners.push((ally, DiplomaticRelationType::Alliance));
            }
        }

        // Add all enemies
        if let Some(enemies) = enemies {
            for &enemy in enemies.enemies() {
                partners.push((enemy, DiplomaticRelationType::War));
            }
        }

        // Add all trade partners
        if let Some(trade_partners) = trade_partners {
            for &partner in trade_partners.trade_partners() {
                partners.push((partner, DiplomaticRelationType::Trade));
            }
        }
    }

    partners
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiplomaticRelationType {
    Alliance,
    War,
    Trade,
}

// ================================================================================================
// DIPLOMATIC EVENTS - State changes
// ================================================================================================

/// Event fired when a new alliance is formed
#[derive(Message, Debug, Clone)]
pub struct AllianceFormedEvent {
    pub nation_a: Entity,
    pub nation_b: Entity,
}

/// Event fired when a war begins
#[derive(Message, Debug, Clone)]
pub struct WarDeclaredEvent {
    pub aggressor: Entity,
    pub defender: Entity,
}

/// Event fired when a peace treaty is signed
#[derive(Message, Debug, Clone)]
pub struct PeaceTreatyEvent {
    pub nation_a: Entity,
    pub nation_b: Entity,
    pub victor: Option<Entity>, // None for status quo peace
}

/// Event fired when a trade agreement is established
#[derive(Message, Debug, Clone)]
pub struct TradeAgreementEvent {
    pub nation_a: Entity,
    pub nation_b: Entity,
}

// ================================================================================================
// VALIDATION SYSTEMS - Diplomatic consistency
// ================================================================================================

/// Ensures diplomatic relationships are consistent using entity relationships
/// (e.g., nations can't be allied and at war simultaneously)
pub fn validate_diplomatic_consistency(
    nations_query: Query<(Entity, Option<&Allies>, Option<&Enemies>)>,
) {
    for (nation_entity, allies, enemies) in nations_query.iter() {
        if let (Some(allies), Some(enemies)) = (allies, enemies) {
            // Check for conflicts (allied and at war with same nation)
            for &ally in allies.allies() {
                if enemies.at_war_with(ally) {
                    error!(
                        "Diplomatic inconsistency: Nation {:?} is both allied with and at war with {:?}",
                        nation_entity, ally
                    );
                }
            }
        }
    }
}

/// Updates diplomatic state counters using entity relationships
/// Much more efficient with automatic relationship tracking
pub fn update_diplomatic_state(
    mut nations_query: Query<(
        Entity,
        &mut DiplomaticState,
        Option<&Allies>,
        Option<&Enemies>,
        Option<&TradePartners>,
    )>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (nation_entity, mut diplomatic_state, allies, enemies, trade_partners) in &mut nations_query {
        // Count relationships using entity relationship components
        diplomatic_state.ally_count = allies.map(|a| a.ally_count() as u32).unwrap_or(0);

        diplomatic_state.enemy_count = enemies.map(|e| e.enemy_count() as u32).unwrap_or(0);

        diplomatic_state.trade_partner_count = trade_partners
            .map(|tp| tp.partner_count() as u32)
            .unwrap_or(0);

        // Calculate diplomatic stance
        diplomatic_state.diplomatic_stance = calculate_diplomatic_stance(
            diplomatic_state.ally_count,
            diplomatic_state.enemy_count,
            diplomatic_state.trade_partner_count,
        );
    }
}

/// Calculate overall diplomatic stance based on relationship counts
fn calculate_diplomatic_stance(ally_count: u32, enemy_count: u32, trade_count: u32) -> f32 {
    let positive_relations = ally_count as f32 + (trade_count as f32 * 0.5);
    let negative_relations = enemy_count as f32;

    if positive_relations + negative_relations == 0.0 {
        0.0 // Neutral
    } else {
        ((positive_relations - negative_relations) / (positive_relations + negative_relations))
            .clamp(-1.0, 1.0)
    }
}
