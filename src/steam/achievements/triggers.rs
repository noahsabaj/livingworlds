//! Achievement trigger logic and unlocking system
//!
//! This module handles the game state monitoring and achievement unlocking logic.

use bevy::prelude::*;
use bevy_steamworks::*;

use super::super::types::{AchievementUnlockedEvent, SteamClient, SteamStats};
use super::{constants as achievements, display::get_achievement_display_name};

/// Check and unlock achievements based on current game state
pub fn handle_achievement_triggers(
    steam: Res<SteamClient>,
    stats: Res<SteamStats>,
    game_time: Res<crate::resources::GameTime>,
    world_tension: Res<crate::resources::WorldTension>,
    mut achievement_events: MessageWriter<AchievementUnlockedEvent>,
) {
    let client = &steam.0;
    let user_stats = client.user_stats();

    let hours_played = stats.total_playtime_minutes / 60.0;

    // Time-based achievements
    if hours_played >= 1.0 {
        unlock_achievement(
            &user_stats,
            achievements::OBSERVER_NOVICE,
            &mut achievement_events,
        );
    }
    if hours_played >= 10.0 {
        unlock_achievement(
            &user_stats,
            achievements::OBSERVER_VETERAN,
            &mut achievement_events,
        );
    }
    if hours_played >= 100.0 {
        unlock_achievement(
            &user_stats,
            achievements::OBSERVER_MASTER,
            &mut achievement_events,
        );
    }

    // Simulation time achievements
    let years = game_time.current_year() as f32;
    if years >= 1000.0 {
        unlock_achievement(
            &user_stats,
            achievements::MILLENNIUM,
            &mut achievement_events,
        );
    }

    // World tension achievements
    if world_tension.current >= 0.95 {
        unlock_achievement(
            &user_stats,
            achievements::APOCALYPSE,
            &mut achievement_events,
        );
    }

    // World generation achievements
    if stats.worlds_generated >= 10 {
        unlock_achievement(
            &user_stats,
            achievements::WORLD_EXPLORER,
            &mut achievement_events,
        );
    }
}

/// Attempt to unlock a specific achievement
pub fn unlock_achievement(
    user_stats: &UserStats,
    achievement_id: &str,
    messages: &mut MessageWriter<AchievementUnlockedEvent>,
) {
    if let Ok(achieved) = user_stats.achievement(achievement_id) {
        if !achieved {
            if user_stats.set_achievement(achievement_id).is_ok() {
                info!("Achievement unlocked: {}", achievement_id);
                messages.write(AchievementUnlockedEvent {
                    achievement_id: achievement_id.to_string(),
                    name: get_achievement_display_name(achievement_id),
                });

                // Store stats to Steam
                let _ = user_stats.store_stats();
            }
        }
    }
}
