//! Simulation module for Living Worlds
//! 
//! This module has a single responsibility: managing the game simulation.
//! It handles time progression, world tension calculation, population growth,
//! and all aspects of the simulation tick. No rendering, input handling for
//! non-simulation features, or other concerns belong here.

use bevy::prelude::*;
use crate::components::Province;
use crate::terrain::TerrainType;
use crate::resources::{GameTime, WorldTension};

/// Plugin that manages all simulation systems
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize simulation resources
            .init_resource::<GameTime>()
            .init_resource::<WorldTension>()
            
            // Simulation systems run in order
            .add_systems(Update, (
                time_control_system,
                advance_time_system,
                population_growth_system,
                calculate_world_tension,
            ).chain());
    }
}

/// Handle pause/play and speed controls for the simulation
fn time_control_system(
    mut game_time: ResMut<GameTime>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Space to pause/resume simulation
    if keyboard.just_pressed(KeyCode::Space) {
        game_time.paused = !game_time.paused;
        println!("Simulation {}", if game_time.paused { "paused" } else { "resumed" });
    }
    
    // Number keys for speed control (simulation time multiplier)
    if keyboard.just_pressed(KeyCode::Digit1) {
        game_time.speed = 1.0;
        println!("Simulation speed: 1x");
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        game_time.speed = 3.0;
        println!("Simulation speed: 3x");
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        game_time.speed = 6.0;
        println!("Simulation speed: 6x");
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        game_time.speed = 9.0;
        println!("Simulation speed: 9x");
    }
}

/// Advance the simulation time based on real time and speed multiplier
fn advance_time_system(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    mut last_year: Local<u64>,
) {
    // Don't advance if paused
    if game_time.paused {
        return;
    }
    
    // Advance game time (in days) based on real time and speed multiplier
    game_time.current_date += time.delta().as_secs_f32() * game_time.speed;
    
    // Log year changes for debugging
    let year = 1000 + (game_time.current_date / 365.0) as u64;
    if year != *last_year {
        println!("Year {}", year);
        *last_year = year;
    }
}

/// Simulate population growth for all provinces
fn population_growth_system(
    game_time: Res<GameTime>,
    mut provinces: Query<&mut Province>,
) {
    // Skip if paused
    if game_time.paused {
        return;
    }
    
    // Every 100 days, simulate population growth
    if game_time.current_date as u64 % 100 == 0 {
        for mut province in provinces.iter_mut() {
            // Only land provinces have population that grows
            if province.terrain != TerrainType::Ocean {
                // Base growth rate depends on agriculture (food production)
                let base_growth = 0.001; // 0.1% base growth per 100 days
                
                // Agriculture bonus: more food = more growth
                let agriculture_multiplier = 0.5 + (province.agriculture * 0.5); // 0.5x to 2.0x
                
                // River/Delta bonus: rivers and deltas grow faster
                let terrain_growth_bonus = match province.terrain {
                    TerrainType::Delta => 2.0,  // Deltas grow very fast
                    TerrainType::River => 1.5,  // Rivers grow fast
                    _ => 1.0,
                };
                
                // Fresh water penalty: far from water = slower growth
                let water_penalty = if province.fresh_water_distance <= 2.0 {
                    1.0  // Close to water - no penalty
                } else if province.fresh_water_distance <= 5.0 {
                    0.8  // Moderate distance - small penalty
                } else {
                    0.6  // Far from water - significant penalty
                };
                
                // Calculate total growth rate
                let growth_rate = base_growth * agriculture_multiplier * terrain_growth_bonus * water_penalty;
                
                // Apply growth with soft population cap based on agriculture
                let carrying_capacity = province.agriculture * 100000.0; // Each agriculture point supports 100k people
                if province.population < carrying_capacity {
                    province.population *= 1.0 + growth_rate;
                } else {
                    // Over capacity - very slow growth or decline
                    province.population *= 1.0 + (growth_rate * 0.1);
                }
            }
        }
    }
}

/// Calculate world tension based on simulated conflicts and events
/// 
/// This system simulates the overall tension level of the world, which
/// affects music, AI behavior, and various game mechanics. Tension is
/// calculated based on wars, economic stress, power imbalances, and
/// random crisis events.
fn calculate_world_tension(
    mut tension: ResMut<WorldTension>,
    game_time: Res<GameTime>,
    provinces: Query<&Province>,
) {
    // Skip if paused
    if game_time.paused {
        return;
    }
    
    // Calculate years elapsed for various cycles
    let years = game_time.current_date / 365.0;
    
    // Base tension from civilization development over time
    // As civilizations grow, conflicts naturally emerge
    let time_factor = (years / 1000.0).min(0.3); // Max 30% from time progression
    
    // Major war cycles (roughly 50-year patterns like real history)
    let war_cycle = ((years / 50.0).sin() + 1.0) / 2.0;
    
    // Minor conflicts and border disputes (7-year cycles)
    let minor_conflicts = ((years / 7.0).sin() + 1.0) / 4.0;
    
    // Random major crisis events (world wars, plagues, etc.)
    // Simplified - in full implementation would be event-driven
    let crisis_chance = if years as i32 % 100 < 5 { 0.2 } else { 0.0 };
    
    // Count land provinces as proxy for civilization complexity
    // More provinces = more potential for conflict
    let land_provinces = provinces.iter()
        .filter(|p| p.terrain != TerrainType::Ocean)
        .count() as f32;
    let complexity_factor = (land_provinces / 10000.0).min(0.1); // Max 10% from world size
    
    // Calculate percentage of nations at war (mock calculation)
    // TODO: Replace with actual nation conflict tracking
    let base_war_percentage = (
        time_factor + 
        war_cycle * 0.3 + 
        minor_conflicts * 0.1 + 
        crisis_chance +
        complexity_factor
    ).min(1.0);
    
    // Apply the exponential curve for tension calculation
    // This creates more interesting dynamics than linear tension
    let calculated_tension = WorldTension::calculate_from_war_percentage(base_war_percentage);
    
    // Update contributing factors for debugging and music system
    tension.war_factor = base_war_percentage;
    tension.power_imbalance = war_cycle * 0.2;
    tension.economic_stress = minor_conflicts * 0.15;
    tension.instability_factor = crisis_chance;
    
    // Set target tension (physics system smoothly interpolates)
    tension.target = calculated_tension;
    
    // Periodic debug output
    if game_time.current_date as i32 % 1000 == 0 {
        println!("World Tension: {:.1}% (Target: {:.1}%, Wars: {:.1}%)",
            tension.current * 100.0,
            tension.target * 100.0,
            base_war_percentage * 100.0
        );
    }
}

// Future simulation systems to add:
// - Nation AI decision making
// - Economic simulation
// - Technology progression  
// - Diplomacy resolution
// - Battle resolution
// - Trade route simulation
// - Cultural spread
// - Disease/disaster events
// - Climate change over centuries