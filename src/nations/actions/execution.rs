//! Action execution systems
//!
//! This is where nation actions are actually performed - provinces change hands,
//! treasuries are modified, military forces are recruited, etc.
//!
//! This is THE critical missing piece that makes the game state actually change!

use bevy::prelude::*;
use super::resolution::NationActionEvent;
use crate::nations::{Nation, NationId, NationHistory};
use crate::nations::types::ProvinceOwnershipCache;
use crate::nations::territory_analysis::TerritoryMetricsCache;
use crate::world::{ProvinceStorage, ProvinceId, MapMode, CachedOverlayColors};
use crate::simulation::GameTime;

/// Execute expansion events - THIS IS WHERE PROVINCES ACTUALLY CHANGE HANDS
///
/// Uses reactive cache invalidation instead of polling (Bevy 0.16 best practice)
pub fn execute_expansion_events(
    mut messages: MessageReader<NationActionEvent>,
    mut province_storage: ResMut<ProvinceStorage>,
    mut ownership_cache: ResMut<ProvinceOwnershipCache>,
    mut territory_cache: ResMut<TerritoryMetricsCache>,
    mut overlay_colors: ResMut<CachedOverlayColors>,
    mut nations_query: Query<(&mut Nation, &mut NationHistory)>,
    game_time: Res<GameTime>,
    current_mode: Res<MapMode>,
) {
    for event in messages.read() {
        if let NationActionEvent::ExpansionAttempt {
            nation_id,
            nation_name,
            target_provinces,
            pressure_level,
        } = event {
            // Find the nation in the query
            let nation_result = nations_query.iter_mut()
                .find(|(nation, _)| nation.id == *nation_id);

            if let Some((mut nation, mut history)) = nation_result {
                // Take ownership of target provinces
                let mut provinces_claimed = 0;

                for province_id in target_provinces {
                    if let Some(province) = province_storage.provinces.get_mut(province_id.value() as usize) {
                        // Only claim unclaimed provinces
                        if province.owner.is_none() {
                            province.owner = Some(*nation_id);
                            provinces_claimed += 1;

                            debug!("{} claims province {} at {:?}",
                                   nation_name, province_id.value(), province.position);
                        }
                    }
                }

                if provinces_claimed > 0 {
                    info!("{} claims {} new provinces through expansion (pressure: {:.1})",
                          nation_name, provinces_claimed, pressure_level);

                    // Rebuild ownership cache to reflect new territories
                    ownership_cache.rebuild(&province_storage.provinces);

                    // Invalidate territory metrics to force recalculation of labels/bounds
                    territory_cache.invalidate_nation(*nation_id);

                    // REACTIVE CACHE INVALIDATION: Directly invalidate overlay cache
                    // This replaces the old polling system that checked every frame
                    overlay_colors.cache.remove(&MapMode::Political);
                    info!("Ownership changed (v{}), invalidated Political map cache",
                          ownership_cache.version);

                    if *current_mode == MapMode::Political {
                        debug!("Currently viewing Political mode - will recalculate on next frame");
                    }

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
                } else {
                    debug!("{} expansion attempt found no claimable provinces", nation_name);
                }
            } else {
                warn!("Expansion event for unknown nation: {} (ID {})",
                      nation_name, nation_id.value());
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
