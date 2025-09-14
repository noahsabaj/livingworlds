//! Simulation module for Living Worlds
//! 
//! This module has a single responsibility: managing game time and simulation speed.
//! Future features will include population growth, nation AI, economics, and world events.
//! For now, it provides the time management scaffolding that other systems can use.

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Serialize, Deserialize};
use crate::resources::GameTime;
use crate::states::GameState;
use crate::constants::{SIMULATION_STARTING_YEAR, SIMULATION_DAYS_PER_YEAR_F32};

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
            ).chain().run_if(in_state(GameState::InGame)))
            
            .add_systems(OnEnter(GameState::InGame), resume_from_pause_menu);
            
            // Example population growth system with events (will be enabled when nations are added)
            // .add_systems(Update, simulate_population_growth
            //     .run_if(in_state(GameState::InGame)));
    }
}

/// Handle keyboard input for time control
/// Space for pause/resume, number keys 1-5 for speed control, +/- for speed increment/decrement
fn handle_time_controls(
    mut game_time: ResMut<GameTime>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut speed_events: EventWriter<SimulationSpeedChanged>,
) {
    let mut speed_changed = false;
    let old_speed = game_time.speed;
    let was_paused = game_time.paused;
    
    // Plus/Minus keys for incremental speed control
    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        // Increase speed by one level
        let new_speed = if game_time.paused {
            // If paused, go to normal speed
            SPEED_NORMAL
        } else if game_time.speed < SPEED_NORMAL + 0.1 {
            SPEED_FAST
        } else if game_time.speed < SPEED_FAST + 0.1 {
            SPEED_FASTER
        } else if game_time.speed < SPEED_FASTER + 0.1 {
            SPEED_FASTEST
        } else {
            game_time.speed // Already at max
        };
        
        if new_speed != game_time.speed || game_time.paused {
            game_time.paused = false;
            game_time.speed = new_speed;
            game_time.speed_before_pause = new_speed;
            speed_changed = true;
            #[cfg(feature = "debug-simulation")]
            println!("Speed increased to: {}x", new_speed);
        }
    }
    
    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        // Decrease speed by one level
        let new_speed = if game_time.speed > SPEED_FASTEST - 0.1 {
            SPEED_FASTER
        } else if game_time.speed > SPEED_FASTER - 0.1 {
            SPEED_FAST
        } else if game_time.speed > SPEED_FAST - 0.1 {
            SPEED_NORMAL
        } else if game_time.speed > SPEED_NORMAL - 0.1 {
            SPEED_PAUSED
        } else {
            game_time.speed // Already at min (paused)
        };
        
        if new_speed != game_time.speed {
            if new_speed == SPEED_PAUSED {
                game_time.paused = true;
                game_time.speed_before_pause = SPEED_NORMAL; // Default to normal when unpausing
            } else {
                game_time.paused = false;
                game_time.speed_before_pause = new_speed;
            }
            game_time.speed = new_speed;
            speed_changed = true;
            #[cfg(feature = "debug-simulation")]
            println!("Speed decreased to: {}x", new_speed);
        }
    }
    
    // Number keys for direct speed control (1 = pause, 2-5 = increasing speeds)
    if keyboard.just_pressed(KeyCode::Digit1) {
        if !game_time.paused {
            game_time.speed_before_pause = game_time.speed;
        }
        game_time.paused = true;
        game_time.speed = SPEED_PAUSED;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation paused");
    }
    
    if keyboard.just_pressed(KeyCode::Digit2) {
        game_time.paused = false;
        game_time.speed = SPEED_NORMAL;
        game_time.speed_before_pause = SPEED_NORMAL;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation speed: Normal (1x)");
    }
    
    if keyboard.just_pressed(KeyCode::Digit3) {
        game_time.paused = false;
        game_time.speed = SPEED_FAST;
        game_time.speed_before_pause = SPEED_FAST;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation speed: Fast (3x)");
    }
    
    if keyboard.just_pressed(KeyCode::Digit4) {
        game_time.paused = false;
        game_time.speed = SPEED_FASTER;
        game_time.speed_before_pause = SPEED_FASTER;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation speed: Faster (6x)");
    }
    
    if keyboard.just_pressed(KeyCode::Digit5) {
        game_time.paused = false;
        game_time.speed = SPEED_FASTEST;
        game_time.speed_before_pause = SPEED_FASTEST;
        speed_changed = true;
        #[cfg(feature = "debug-simulation")]
        println!("Simulation speed: Fastest (9x)");
    }
    
    // Space key for pause toggle
    if keyboard.just_pressed(KeyCode::Space) {
        game_time.paused = !game_time.paused;
        if game_time.paused {
            game_time.speed_before_pause = game_time.speed;
            game_time.speed = SPEED_PAUSED;
        } else {
            // Restore the speed we had before pausing
            game_time.speed = game_time.speed_before_pause;
        }
        speed_changed = true;
        
        #[cfg(feature = "debug-simulation")]
        println!("Simulation {} (speed: {}x)", 
            if game_time.paused { "paused" } else { "resumed" }, 
            if game_time.paused { 0.0 } else { game_time.speed });
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
    let current_year = SIMULATION_STARTING_YEAR + (game_time.current_date / SIMULATION_DAYS_PER_YEAR_F32) as u32;
    
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

/// Resume from pause menu - restore the game speed
fn resume_from_pause_menu(
    mut game_time: ResMut<GameTime>,
    mut speed_events: EventWriter<SimulationSpeedChanged>,
) {
    // When transitioning from Paused to InGame via the menu, restore the speed
    if game_time.paused {
        game_time.paused = false;
        game_time.speed = game_time.speed_before_pause;
        
        speed_events.send(SimulationSpeedChanged {
            new_speed: game_time.speed,
            is_paused: false,
        });
        
        #[cfg(feature = "debug-simulation")]
        println!("Resumed from pause menu at speed: {}x", game_time.speed);
    }
}


// FUTURE SYSTEMS (To be implemented when nations/civilizations are added)
// These systems will be added as the game develops:
// - population_growth_system: Simulate population changes based on:
//   * Agriculture and food production
//   * Fresh water access
//   * Terrain type bonuses
//   * Disease and disasters
//   * War casualties
// - nation_ai_system: Make decisions for AI nations:
//   * Diplomacy (alliances, wars, trade)
//   * Expansion and colonization
//   * Military production and movement
//   * Economic development
//   * Technology research
// - economic_simulation: Track resources and trade:
//   * Resource extraction from provinces
//   * Trade route establishment
//   * Market prices and supply/demand
//   * Nation treasuries and taxation
//   * Infrastructure development
// - technology_progression: Advance through ages:
//   * Research trees for nations
//   * Technology spread between nations
//   * Unlocking new units and buildings
//   * Cultural and scientific achievements
// - battle_resolution: Resolve military conflicts:
//   * Army movement and positioning
//   * Combat calculations
//   * Siege warfare
//   * Naval battles
//   * War exhaustion
// - event_system: Random and triggered events:
//   * Natural disasters (earthquakes, floods, droughts)
//   * Plagues and diseases
//   * Religious movements
//   * Revolutions and civil wars
//   * Great persons and discoveries
// - climate_system: Long-term climate changes:
//   * Ice ages and warming periods
//   * Desertification and reforestation
//   * Sea level changes
//   * Agricultural zone shifts

// ===== WORLD TENSION SYSTEM =====

/// World Tension - Global metric tracking conflict and instability
///
/// Tension ranges from 0.0 (perfect peace) to 1.0 (world war).
/// It rises quickly with conflicts but falls slowly during peace,
/// simulating how real-world tensions have momentum.
#[derive(Resource, Reflect, Clone, Serialize, Deserialize)]
pub struct WorldTension {
    /// Current tension level (0.0 to 1.0)
    pub current: f32,
    /// Target tension based on world state
    pub target: f32,
    /// Rate of change
    pub velocity: f32,

    // Contributing factors (each 0.0 to 1.0)
    /// Percentage of nations at war
    pub war_factor: f32,
    /// Power imbalance (one nation too dominant)
    pub power_imbalance: f32,
    /// Economic disruption (trade routes broken)
    pub economic_stress: f32,
    /// Recent collapses or disasters
    pub instability_factor: f32,

    // Physics parameters
    /// How fast tension rises (default: 2.0)
    pub heating_rate: f32,
    /// How slowly tension falls (default: 0.3)
    pub cooling_rate: f32,
    /// Resistance to change (default: 0.8)
    pub inertia: f32,
}

impl Default for WorldTension {
    fn default() -> Self {
        Self {
            current: 0.0,  // Start at perfect peace
            target: 0.0,
            velocity: 0.0,

            war_factor: 0.0,
            power_imbalance: 0.0,
            economic_stress: 0.0,
            instability_factor: 0.0,

            heating_rate: 2.0,    // Wars escalate quickly
            cooling_rate: 0.3,    // Peace returns slowly
            inertia: 0.8,         // Smooth transitions
        }
    }
}

impl WorldTension {
    /// Calculate tension from war percentage using exponential curve
    ///
    /// This uses a power function to make tension rise exponentially:
    /// - 10% at war = ~18% tension (local conflicts)
    /// - 25% at war = ~40% tension (regional wars)
    /// - 50% at war = ~70% tension (world crisis)
    /// - 75% at war = ~90% tension (near apocalypse)
    /// - 100% at war = 100% tension (total war)
    pub fn calculate_from_war_percentage(war_percentage: f32) -> f32 {
        // Use square root for exponential growth
        // This makes small conflicts barely register but large wars escalate rapidly
        war_percentage.sqrt().clamp(0.0, 1.0)
    }
}