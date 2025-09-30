//! Main plugin for the simulation module - AUTOMATION POWERED!

use super::input::handle_time_controls;
use super::pressures::{run_pressure_systems_on_timer, PressureSystemTimer};
use super::time::{
    advance_simulation_ticks, interpolate_visual_time, resume_from_pause_menu, track_year_changes,
    GameTick, NewYearEvent, SimulationSpeed, SimulationSpeedChanged, VisualTime,
};
use crate::resources::GameTime;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

/// Plugin that manages the simulation time system using AUTOMATION FRAMEWORK
define_plugin!(SimulationPlugin {
    resources: [GameTime, PressureSystemTimer, VisualTime],

    events: [
        SimulationSpeedChanged,
        NewYearEvent,
        super::history_update::BattleEvent,
        super::history_update::WarStatusEvent,
        crate::nations::NationActionEvent
    ],

    // DETERMINISTIC SIMULATION: Time advances in FixedUpdate at consistent rate
    fixed_update: [
        // Core simulation tick advancement (deterministic)
        advance_simulation_ticks.run_if(in_state(GameState::InGame)),
        track_year_changes.run_if(in_state(GameState::InGame))
    ],

    update: [
        // Input handling (frame-dependent is OK for input)
        handle_time_controls.run_if(in_state(GameState::InGame)),
        // Visual interpolation for smooth display
        interpolate_visual_time.run_if(in_state(GameState::InGame)),
        // History tracking systems
        super::history_update::update_nation_histories.run_if(in_state(GameState::InGame)),
        super::history_update::track_battle_outcomes.run_if(in_state(GameState::InGame)),
        super::history_update::track_war_status.run_if(in_state(GameState::InGame)),
        // PERFORMANCE: Pressure systems run periodically, not every frame!
        run_pressure_systems_on_timer.run_if(in_state(GameState::InGame)),
        // ACTION RESOLUTION: Nations analyze pressures and decide what to do
        crate::nations::resolve_nation_actions.run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [resume_from_pause_menu]
    }
});