//! Action execution systems
//!
//! This is where nation actions are actually performed - provinces change hands,
//! treasuries are modified, military forces are recruited, etc.
//!
//! This is THE critical missing piece that makes the game state actually change!

use bevy::prelude::*;
use super::resolution::NationActionEvent;
use crate::nations::{Nation, NationHistory};
use crate::world::{ProvinceStorage, MapMode, CachedOverlayColors};
use crate::simulation::GameTime;

/// Execute expansion events - THIS IS WHERE PROVINCES ACTUALLY CHANGE HANDS
///
/// Uses reactive cache invalidation instead of polling (Bevy 0.17 best practice)
pub fn execute_expansion_events(
    mut commands: Commands,
    mut messages: MessageReader<NationActionEvent>,
    mut ownership_events: MessageWriter<super::TerritoryOwnershipChanged>,
    mut province_storage: ResMut<ProvinceStorage>,
    mut overlay_colors: ResMut<CachedOverlayColors>,
    mut nations_query: Query<(&mut Nation, &mut NationHistory)>,
    game_time: Res<GameTime>,
    current_mode: Res<MapMode>,
) {
    for event in messages.read() {
        if let NationActionEvent::ExpansionAttempt {
            nation_entity,
            nation_name,
            target_provinces,
            pressure_level,
        } = event {
            // Direct entity access - O(1) instead of O(N)
            let Ok((mut nation, mut history)) = nations_query.get_mut(*nation_entity) else {
                warn!("Expansion event for unknown nation entity: {}", nation_name);
                continue;
            };

            // Take ownership of target provinces
            let mut provinces_claimed = 0;

            for province_id in target_provinces {
                if let Some(province) = province_storage.provinces.get_mut(province_id.value() as usize) {
                    // Only claim unclaimed provinces
                    if province.owner_entity.is_none() {
                        province.owner_entity = Some(*nation_entity);
                        provinces_claimed += 1;

                        debug!("{} claims province {} at {:?}",
                               nation_name, province_id.value(), province.position);
                    }
                }
            }

            if provinces_claimed > 0 {
                info!("{} claims {} new provinces through expansion (pressure: {:.1})",
                      nation_name, provinces_claimed, pressure_level);

                // REACTIVE CACHE INVALIDATION: Directly invalidate overlay cache
                overlay_colors.cache.remove(&MapMode::Political);
                info!("Ownership changed, invalidated Political map cache");

                if *current_mode == MapMode::Political {
                    debug!("Currently viewing Political mode - will recalculate on next frame");
                }

                // Fire territory ownership changed event for neighbor relationship rebuild
                ownership_events.write(super::TerritoryOwnershipChanged {
                    nation_entity: *nation_entity,
                    provinces_changed: provinces_claimed,
                    change_type: super::OwnershipChangeType::Expansion,
                });

                // Record historical event
                use crate::nations::history::{HistoricalEvent, AcquisitionMethod};
                history.record_event(HistoricalEvent::TerritorialExpansion {
                    year: game_time.current_year(),
                    provinces_gained: provinces_claimed,
                    pressure_level: *pressure_level,
                    method: AcquisitionMethod::Settlement,
                });

                // Update expansion statistics
                history.provinces_gained += provinces_claimed;
                history.expansion_attempts += 1;

                // Small treasury cost for expansion administration
                nation.treasury -= provinces_claimed as f32 * 50.0;

                // Small stability boost from successful expansion
                nation.stability = (nation.stability + 0.02).min(1.0);

                // TODO Phase 7: Recalculate TerritoryMetrics component here
                // commands.entity(*nation_entity).insert(recalculated_metrics);
            } else {
                debug!("{} expansion attempt found no claimable provinces", nation_name);
            }
        }
    }
}

// REMOVED: force_overlay_refresh_on_expansion
//
// This polling system ran EVERY frame checking for ownership changes.
// Replaced with reactive cache invalidation in execute_expansion_events()
// that invalidates immediately when we KNOW ownership changed.
//
// Performance improvement: No longer wastes CPU cycles polling every frame!
// Bevy 0.16 best practice: React to changes, don't poll for them.
