//! Simulation module for Living Worlds
//! 
//! This module has a single responsibility: managing game time and simulation speed.
//! Future features will include population growth, nation AI, economics, and world events.
//! For now, it provides the time management scaffolding that other systems can use.

use bevy::prelude::*;
use crate::resources::GameTime;
use crate::states::GameState;

// Constants for simulation speeds
const SPEED_PAUSED: f32 = 0.0;
const SPEED_NORMAL: f32 = 1.0;
const SPEED_FAST: f32 = 3.0;
const SPEED_FASTER: f32 = 6.0;
const SPEED_FASTEST: f32 = 9.0;

/// Event sent when simulation speed changes
#[derive(Event)]
pub struct SimulationSpeedChanged {
    pub new_speed: f32,
    pub is_paused: bool,
}

/// Event sent when a new year begins
#[derive(Event)]
pub struct NewYearEvent {
    pub year: u32,
}

/// Plugin that manages the simulation time system
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<GameTime>()
            
            // Add events
            .add_event::<SimulationSpeedChanged>()
            .add_event::<NewYearEvent>()
            
            // Time management systems - only run during gameplay
            .add_systems(Update, (
                handle_time_controls,
                advance_game_time,
                track_year_changes,
            ).chain().run_if(in_state(GameState::InGame)));
    }
}

/// Handle keyboard input for time control
/// Space for pause/resume, number keys 0-4 for speed control
fn handle_time_controls(
    mut game_time: ResMut<GameTime>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut speed_events: EventWriter<SimulationSpeedChanged>,
) {
    let mut speed_changed = false;
    let old_speed = game_time.speed;
    let was_paused = game_time.paused;
    
    // Number keys for speed control
    if keyboard.just_pressed(KeyCode::Digit0) {
        game_time.paused = true;
        game_time.speed = SPEED_PAUSED;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation paused");
    }
    
    if keyboard.just_pressed(KeyCode::Digit1) {
        game_time.paused = false;
        game_time.speed = SPEED_NORMAL;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation speed: Normal (1x)");
    }
    
    if keyboard.just_pressed(KeyCode::Digit2) {
        game_time.paused = false;
        game_time.speed = SPEED_FAST;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation speed: Fast (3x)");
    }
    
    if keyboard.just_pressed(KeyCode::Digit3) {
        game_time.paused = false;
        game_time.speed = SPEED_FASTER;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation speed: Faster (6x)");
    }
    
    if keyboard.just_pressed(KeyCode::Digit4) {
        game_time.paused = false;
        game_time.speed = SPEED_FASTEST;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation speed: Fastest (9x)");
    }
    
    // Space key for pause toggle
    if keyboard.just_pressed(KeyCode::Space) {
        game_time.paused = !game_time.paused;
        if game_time.paused {
            game_time.speed = SPEED_PAUSED;
        } else if game_time.speed == SPEED_PAUSED {
            game_time.speed = SPEED_NORMAL;
        }
        speed_changed = true;
        
        #[cfg(feature = "debug-simulation")]
        println!("Simulation {}", if game_time.paused { "paused" } else { "resumed" });
    }
    
    // Send event if speed changed
    if speed_changed && (old_speed != game_time.speed || was_paused != game_time.paused) {
        speed_events.send(SimulationSpeedChanged {
            new_speed: game_time.speed,
            is_paused: game_time.paused,
        });
    }
}

/// Advance the game time based on real time and speed multiplier
fn advance_game_time(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
) {
    // Don't advance if paused
    if game_time.paused {
        return;
    }
    
    // Advance game time (in days) based on real time and speed multiplier
    // 1 real second = 1 game day at 1x speed
    game_time.current_date += time.delta_secs() * game_time.speed;
}

/// Track year changes and send events
fn track_year_changes(
    game_time: Res<GameTime>,
    mut last_year: Local<u32>,
    mut year_events: EventWriter<NewYearEvent>,
) {
    // Calculate current year (starting from year 1000)
    let current_year = 1000 + (game_time.current_date / 365.0) as u32;
    
    // Check if year changed
    if current_year != *last_year && *last_year > 0 {
        year_events.send(NewYearEvent {
            year: current_year,
        });
        
        #[cfg(feature = "debug-simulation")]
        println!("Year {}", current_year);
        
        *last_year = current_year;
    } else if *last_year == 0 {
        // Initialize on first run
        *last_year = current_year;
    }
}

// ============================================================================
// FUTURE SYSTEMS (To be implemented when nations/civilizations are added)
// ============================================================================
// 
// These systems will be added as the game develops:
// 
// - population_growth_system: Simulate population changes based on:
//   * Agriculture and food production
//   * Fresh water access
//   * Terrain type bonuses
//   * Disease and disasters
//   * War casualties
//
// - nation_ai_system: Make decisions for AI nations:
//   * Diplomacy (alliances, wars, trade)
//   * Expansion and colonization
//   * Military production and movement
//   * Economic development
//   * Technology research
//
// - economic_simulation: Track resources and trade:
//   * Resource extraction from provinces
//   * Trade route establishment
//   * Market prices and supply/demand
//   * Nation treasuries and taxation
//   * Infrastructure development
//
// - technology_progression: Advance through ages:
//   * Research trees for nations
//   * Technology spread between nations
//   * Unlocking new units and buildings
//   * Cultural and scientific achievements
//
// - battle_resolution: Resolve military conflicts:
//   * Army movement and positioning
//   * Combat calculations
//   * Siege warfare
//   * Naval battles
//   * War exhaustion
//
// - event_system: Random and triggered events:
//   * Natural disasters (earthquakes, floods, droughts)
//   * Plagues and diseases
//   * Religious movements
//   * Revolutions and civil wars
//   * Great persons and discoveries
//
// - climate_system: Long-term climate changes:
//   * Ice ages and warming periods
//   * Desertification and reforestation
//   * Sea level changes
//   * Agricultural zone shifts