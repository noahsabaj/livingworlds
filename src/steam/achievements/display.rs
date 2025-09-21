//! Achievement display names and descriptions
//!
//! This module provides user-friendly names and descriptions for Steam achievements.

use super::constants as achievements;

/// Get the display name for an achievement by its ID
pub fn get_achievement_display_name(id: &str) -> String {
    match id {
        achievements::FIRST_WORLD => "New Observer",
        achievements::OBSERVER_NOVICE => "Novice Observer",
        achievements::OBSERVER_VETERAN => "Veteran Observer",
        achievements::OBSERVER_MASTER => "Master Observer",
        achievements::WITNESS_WAR => "Witness to War",
        achievements::WITNESS_PEACE => "Era of Peace",
        achievements::WORLD_EXPLORER => "World Explorer",
        achievements::LARGE_WORLD => "Grand Scale",
        achievements::MILLENNIUM => "Millennium Watcher",
        achievements::POPULATION_BOOM => "Population Explosion",
        achievements::RISE_AND_FALL => "Cycles of History",
        achievements::GOLDEN_AGE => "Golden Age",
        achievements::APOCALYPSE => "End Times",
        achievements::SPEED_DEMON => "Time Lord",
        achievements::PHOTOGRAPHER => "Chronicler",
        achievements::MODDER => "Community Member",
        _ => "Unknown Achievement",
    }
    .to_string()
}
