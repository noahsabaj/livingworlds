//! Steam Integration for Living Worlds
//! 
//! This module handles all Steam-specific features including:
//! - Achievements
//! - Cloud saves
//! - Workshop support for mods
//! - Rich presence
//! - Statistics tracking
//! 
//! IMPORTANT: Requires Steam client to be running and user to own the game

use bevy::prelude::*;
use bevy_steamworks::*;
use std::sync::Arc;
use std::path::PathBuf;

// Your Steam App ID (replace with actual ID from Valve)
const STEAM_APP_ID: u32 = 480; // Using Spacewar ID for testing - REPLACE WITH YOUR ACTUAL APP ID

// ============================================================================
// PLUGIN
// ============================================================================

/// Steam integration plugin for Living Worlds
pub struct SteamPlugin;

impl Plugin for SteamPlugin {
    fn build(&self, app: &mut App) {
        // Initialize Steamworks - MUST be before RenderPlugin
        let (client, single) = match Client::init_app(STEAM_APP_ID) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to initialize Steam: {:?}", e);
                eprintln!("Running in offline mode - Steam features disabled");
                return;
            }
        };

        // Store the client as a resource
        app.insert_resource(SteamClient(Arc::new(client)));
        app.insert_resource(single);
        
        // Steam systems
        app.add_systems(Startup, (
            setup_steam_callbacks,
            initialize_stats,
        ))
        .add_systems(Update, (
            poll_steam_callbacks,
            update_rich_presence,
            handle_achievement_triggers,
            sync_cloud_saves,
        ))
        .add_systems(OnExit(bevy::app::AppExit::Success), cleanup_steam);
        
        // Steam events
        app.add_event::<AchievementUnlockedEvent>()
           .add_event::<WorkshopItemDownloadedEvent>()
           .add_event::<CloudSaveSyncedEvent>();
        
        println!("Steam integration initialized successfully!");
        println!("Steam App ID: {}", STEAM_APP_ID);
        
        // Get Steam user info
        if let Ok(user) = client.user() {
            let steam_id = user.steam_id();
            let name = client.friends().get_friend_name(steam_id);
            println!("Logged in as: {} ({})", name, steam_id.raw());
        }
    }
}

// ============================================================================
// RESOURCES
// ============================================================================

/// Wrapper for the Steam client
#[derive(Resource, Clone)]
pub struct SteamClient(pub Arc<Client>);

/// Steam statistics for Living Worlds
#[derive(Resource, Default)]
pub struct SteamStats {
    pub total_playtime_minutes: f32,
    pub worlds_generated: u32,
    pub provinces_explored: u32,
    pub years_simulated: u32,
    pub nations_witnessed: u32,
    pub wars_observed: u32,
    pub peak_world_population: u64,
}

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Event)]
pub struct AchievementUnlockedEvent {
    pub achievement_id: String,
    pub name: String,
}

#[derive(Event)]
pub struct WorkshopItemDownloadedEvent {
    pub item_id: PublishedFileId,
    pub title: String,
}

#[derive(Event)]
pub struct CloudSaveSyncedEvent {
    pub success: bool,
    pub files_synced: u32,
}

// ============================================================================
// ACHIEVEMENTS
// ============================================================================

/// Achievement IDs for Living Worlds
pub mod achievements {
    pub const FIRST_WORLD: &str = "FIRST_WORLD";
    pub const OBSERVER_NOVICE: &str = "OBSERVER_NOVICE";        // Watch for 1 hour
    pub const OBSERVER_VETERAN: &str = "OBSERVER_VETERAN";      // Watch for 10 hours
    pub const OBSERVER_MASTER: &str = "OBSERVER_MASTER";        // Watch for 100 hours
    pub const WITNESS_WAR: &str = "WITNESS_WAR";                // See first war
    pub const WITNESS_PEACE: &str = "WITNESS_PEACE";            // See 100 years of peace
    pub const WORLD_EXPLORER: &str = "WORLD_EXPLORER";          // Generate 10 worlds
    pub const LARGE_WORLD: &str = "LARGE_WORLD";                // Generate large world
    pub const MILLENNIUM: &str = "MILLENNIUM";                  // Simulate 1000 years
    pub const POPULATION_BOOM: &str = "POPULATION_BOOM";        // Reach 1B population
    pub const RISE_AND_FALL: &str = "RISE_AND_FALL";           // Witness nation collapse
    pub const GOLDEN_AGE: &str = "GOLDEN_AGE";                  // Low tension for 500 years
    pub const APOCALYPSE: &str = "APOCALYPSE";                  // Max world tension
    pub const SPEED_DEMON: &str = "SPEED_DEMON";                // Use fastest speed
    pub const PHOTOGRAPHER: &str = "PHOTOGRAPHER";              // Take 100 screenshots
    pub const MODDER: &str = "MODDER";                          // Subscribe to workshop item
}

/// Check and unlock achievements based on game state
fn handle_achievement_triggers(
    steam: Res<SteamClient>,
    stats: Res<SteamStats>,
    game_time: Res<crate::resources::GameTime>,
    world_tension: Res<crate::resources::WorldTension>,
    mut achievement_events: EventWriter<AchievementUnlockedEvent>,
) {
    let client = &steam.0;
    let user_stats = client.user_stats();
    
    // Check time-based achievements
    let hours_played = stats.total_playtime_minutes / 60.0;
    
    if hours_played >= 1.0 {
        unlock_achievement(&user_stats, achievements::OBSERVER_NOVICE, &mut achievement_events);
    }
    if hours_played >= 10.0 {
        unlock_achievement(&user_stats, achievements::OBSERVER_VETERAN, &mut achievement_events);
    }
    if hours_played >= 100.0 {
        unlock_achievement(&user_stats, achievements::OBSERVER_MASTER, &mut achievement_events);
    }
    
    // Check simulation achievements
    let years = game_time.current_date / 365.0;
    if years >= 1000.0 {
        unlock_achievement(&user_stats, achievements::MILLENNIUM, &mut achievement_events);
    }
    
    // Check tension achievements
    if world_tension.current >= 0.95 {
        unlock_achievement(&user_stats, achievements::APOCALYPSE, &mut achievement_events);
    }
    
    // Check world generation achievements
    if stats.worlds_generated >= 10 {
        unlock_achievement(&user_stats, achievements::WORLD_EXPLORER, &mut achievement_events);
    }
}

fn unlock_achievement(
    user_stats: &UserStats,
    achievement_id: &str,
    events: &mut EventWriter<AchievementUnlockedEvent>,
) {
    if let Ok(achieved) = user_stats.achievement(achievement_id) {
        if !achieved {
            if user_stats.set_achievement(achievement_id).is_ok() {
                println!("Achievement unlocked: {}", achievement_id);
                events.send(AchievementUnlockedEvent {
                    achievement_id: achievement_id.to_string(),
                    name: get_achievement_display_name(achievement_id),
                });
                
                // Store stats to Steam
                let _ = user_stats.store_stats();
            }
        }
    }
}

fn get_achievement_display_name(id: &str) -> String {
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
    }.to_string()
}

// ============================================================================
// RICH PRESENCE
// ============================================================================

/// Update Steam Rich Presence to show what the player is doing
fn update_rich_presence(
    steam: Res<SteamClient>,
    game_state: Res<State<crate::states::GameState>>,
    game_time: Option<Res<crate::resources::GameTime>>,
    world_size: Option<Res<crate::resources::WorldSize>>,
) {
    let client = &steam.0;
    let friends = client.friends();
    
    // Build presence string based on game state
    let status = match **game_state {
        crate::states::GameState::MainMenu => "In Main Menu",
        crate::states::GameState::WorldConfiguration => "Configuring New World",
        crate::states::GameState::LoadingWorld => "Loading World",
        crate::states::GameState::InGame => {
            if let Some(time) = game_time {
                let year = 1000 + (time.current_date / 365.0) as i32;
                &format!("Observing Year {}", year)
            } else {
                "Observing History"
            }
        }
        crate::states::GameState::Paused => "Paused",
        _ => "Playing",
    };
    
    // Set rich presence
    friends.set_rich_presence("status", status);
    
    if let Some(size) = world_size {
        let size_str = format!("{:?}", size);
        friends.set_rich_presence("world_size", &size_str);
    }
    
    // Steam will display: "Playing Living Worlds - Observing Year 1453"
}

// ============================================================================
// CLOUD SAVES
// ============================================================================

/// Sync save files with Steam Cloud
fn sync_cloud_saves(
    steam: Res<SteamClient>,
    mut sync_events: EventWriter<CloudSaveSyncedEvent>,
) {
    // This runs periodically to sync saves
    // Steam Cloud is configured in Steamworks partner site
    // We just need to ensure saves are in the right location
    
    // Steam Cloud automatically syncs files in the save directory
    // Configuration in app_build_config.vdf:
    // "ufs"
    // {
    //     "quota" "104857600" // 100MB
    //     "path" "saves"
    //     "pattern" "*.lws"
    // }
}

// ============================================================================
// WORKSHOP SUPPORT
// ============================================================================

/// Workshop item types for Living Worlds
#[derive(Debug, Clone, Copy)]
pub enum WorkshopItemType {
    WorldPreset,     // Custom world generation settings
    ColorScheme,     // Custom terrain/nation colors
    BalanceMod,      // Modified simulation parameters
}

/// Subscribe to a workshop item
pub fn subscribe_to_workshop_item(
    steam: &SteamClient,
    item_id: PublishedFileId,
) {
    let client = &steam.0;
    let ugc = client.ugc();
    
    ugc.subscribe_item(item_id);
    println!("Subscribed to workshop item: {:?}", item_id);
}

/// Get list of subscribed workshop items
pub fn get_subscribed_items(steam: &SteamClient) -> Vec<PublishedFileId> {
    let client = &steam.0;
    let ugc = client.ugc();
    
    ugc.subscribed_items()
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Initialize Steam statistics
fn initialize_stats(
    steam: Res<SteamClient>,
) {
    let client = &steam.0;
    let user_stats = client.user_stats();
    
    // Request current stats from Steam
    user_stats.request_current_stats();
    
    println!("Steam statistics initialized");
}

/// Update and store statistics to Steam
pub fn update_steam_stats(
    steam: &SteamClient,
    stats: &SteamStats,
) {
    let client = &steam.0;
    let user_stats = client.user_stats();
    
    // Update stats
    let _ = user_stats.set_stat("total_playtime_minutes", stats.total_playtime_minutes);
    let _ = user_stats.set_stat("worlds_generated", stats.worlds_generated as f32);
    let _ = user_stats.set_stat("provinces_explored", stats.provinces_explored as f32);
    let _ = user_stats.set_stat("years_simulated", stats.years_simulated as f32);
    let _ = user_stats.set_stat("nations_witnessed", stats.nations_witnessed as f32);
    let _ = user_stats.set_stat("wars_observed", stats.wars_observed as f32);
    let _ = user_stats.set_stat("peak_world_population", stats.peak_world_population as f32);
    
    // Store to Steam
    let _ = user_stats.store_stats();
}

// ============================================================================
// SYSTEM FUNCTIONS
// ============================================================================

fn setup_steam_callbacks(
    steam: Res<SteamClient>,
) {
    println!("Setting up Steam callbacks...");
    // Callbacks are handled automatically by bevy_steamworks
}

fn poll_steam_callbacks(
    single: Res<SingleClient>,
) {
    // Poll Steam for callbacks
    single.run_callbacks();
}

fn cleanup_steam(
    steam: Res<SteamClient>,
    stats: Res<SteamStats>,
) {
    println!("Cleaning up Steam integration...");
    
    // Final stats update
    update_steam_stats(&steam, &stats);
    
    // Steam cleanup is handled automatically
}

// ============================================================================
// LEADERBOARDS (Future Feature)
// ============================================================================

/// Leaderboard IDs
pub mod leaderboards {
    pub const LONGEST_OBSERVATION: &str = "longest_observation";
    pub const MOST_WORLDS: &str = "most_worlds_generated";
    pub const HIGHEST_POPULATION: &str = "highest_world_population";
    pub const LONGEST_PEACE: &str = "longest_peace_era";
}

/// Submit score to leaderboard
pub fn submit_leaderboard_score(
    steam: &SteamClient,
    leaderboard_name: &str,
    score: i32,
) {
    let client = &steam.0;
    let user_stats = client.user_stats();
    
    // Find or create leaderboard
    user_stats.find_or_create_leaderboard(
        leaderboard_name,
        LeaderboardSortMethod::Descending,
        LeaderboardDisplayType::Numeric,
    );
    
    // Note: Actual submission requires callback handling
    println!("Submitting score {} to leaderboard {}", score, leaderboard_name);
}