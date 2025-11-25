//! Systems for family browser prestige calculation and caching

use bevy::prelude::*;
use crate::nations::{House, Nation};
use crate::nations::Character;
use super::types::{HousePrestige, HousePrestigeCache};

/// Update interval for prestige cache (in seconds)
const CACHE_UPDATE_INTERVAL: f64 = 5.0;

/// Calculate prestige scores for all houses
pub fn update_prestige_cache(
    mut cache: ResMut<HousePrestigeCache>,
    houses: Query<(Entity, &House, Option<&crate::relationships::RulesOver>)>,
    nations: Query<(Entity, &Nation, &crate::nations::NationId)>,
    characters: Query<&Character>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs_f64();

    // Always update on first run (when last_update is 0.0), then throttle
    let should_update = cache.last_update == 0.0
        || (current_time - cache.last_update >= CACHE_UPDATE_INTERVAL);

    if !should_update {
        return;
    }

    cache.houses.clear();
    cache.last_update = current_time;

    let house_count = houses.iter().count();
    info!("Updating prestige cache for {} houses", house_count);

    for (house_entity, house, rules_over) in &houses {
        let prestige = calculate_house_prestige(
            house_entity,
            house,
            rules_over,
            &nations,
            &characters,
        );
        cache.houses.push(prestige);
    }

    // Sort by total prestige descending
    cache.houses.sort_by(|a, b| {
        b.total_prestige
            .partial_cmp(&a.total_prestige)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    info!("Prestige cache updated: {} houses ranked", cache.houses.len());
}

/// Calculate prestige for a single house
fn calculate_house_prestige(
    house_entity: Entity,
    house: &House,
    rules_over: Option<&crate::relationships::RulesOver>,
    nations: &Query<(Entity, &Nation, &crate::nations::NationId)>,
    characters: &Query<&Character>,
) -> HousePrestige {
    // Base prestige from house
    let base_prestige = house.prestige;

    // Wealth score: simplified calculation based on legitimacy
    let wealth_score = house.legitimacy * 100.0;

    // Influence score: sum of all house members' influence
    let influence_score: f32 = characters
        .iter()
        .filter(|c| c.house_id == house_entity)
        .map(|c| c.influence)
        .sum();

    // Ruler count: count how many rulers this house has had
    // For now, simplified to just current ruler = 1
    let ruler_count = 1;

    // Longevity bonus: years in power
    let longevity_bonus = (house.years_in_power as f32) * 0.5;

    // Total prestige calculation
    let total_prestige = base_prestige
        + wealth_score
        + (influence_score * 10.0)
        + (ruler_count as f32 * 50.0)
        + longevity_bonus;

    // Check if currently ruling (simplified: assume all houses are ruling for now)
    let is_ruling = true;

    // Check if extinct (no living characters)
    let is_extinct = !characters
        .iter()
        .any(|c| c.house_id == house_entity);

    // Get nation info via RulesOver relationship
    let mut nation_id = None;
    let mut nation_name = None;

    if let Some(rules_over) = rules_over {
        let nation_entity = rules_over.0;
        if let Ok((_, nation, nid)) = nations.get(nation_entity) {
            nation_id = Some(*nid);
            nation_name = Some(nation.name.clone());
        }
    }

    HousePrestige {
        house_entity,
        house_name: house.name.clone(),
        nation_id,
        nation_name,
        total_prestige,
        wealth_score,
        influence_score,
        ruler_count,
        years_in_power: house.years_in_power,
        is_ruling,
        is_extinct,
    }
}
