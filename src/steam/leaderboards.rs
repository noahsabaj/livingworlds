//! Steam leaderboards integration
//!
//! This module handles Steam leaderboard functionality for competitive
//! aspects of Living Worlds, such as longest observation times and
//! highest world populations achieved.

use bevy::prelude::*;
use bevy_steamworks::*;

use super::types::SteamClient;

/// Leaderboard identifiers
pub mod leaderboard_ids {
    pub const LONGEST_OBSERVATION: &str = "longest_observation";
    pub const MOST_WORLDS: &str = "most_worlds_generated";
    pub const HIGHEST_POPULATION: &str = "highest_world_population";
    pub const LONGEST_PEACE: &str = "longest_peace_era";
}

/// Submit score to a Steam leaderboard
pub fn submit_leaderboard_score(steam: &SteamClient, leaderboard_name: &str, score: i32) {
    let client = &steam.0;
    let user_stats = client.user_stats();

    user_stats.find_or_create_leaderboard(
        leaderboard_name,
        LeaderboardSortMethod::Descending,
        LeaderboardDisplayType::Numeric,
    );

    // Note: Actual submission requires callback handling
    info!(
        "Submitting score {} to leaderboard {}",
        score, leaderboard_name
    );
}

/// Submit longest observation time to leaderboard
pub fn submit_observation_time(steam: &SteamClient, hours: u32) {
    submit_leaderboard_score(steam, leaderboard_ids::LONGEST_OBSERVATION, hours as i32);
}

/// Submit world count to leaderboard
pub fn submit_world_count(steam: &SteamClient, count: u32) {
    submit_leaderboard_score(steam, leaderboard_ids::MOST_WORLDS, count as i32);
}

/// Submit highest population achieved to leaderboard
pub fn submit_population_record(steam: &SteamClient, population: u64) {
    // Convert to millions for leaderboard display
    let population_millions = (population / 1_000_000) as i32;
    submit_leaderboard_score(
        steam,
        leaderboard_ids::HIGHEST_POPULATION,
        population_millions,
    );
}

/// Submit longest peace era to leaderboard
pub fn submit_peace_record(steam: &SteamClient, years: u32) {
    submit_leaderboard_score(steam, leaderboard_ids::LONGEST_PEACE, years as i32);
}
