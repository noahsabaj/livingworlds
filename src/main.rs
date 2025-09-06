//! Living Worlds - Main entry point
//! 
//! A procedurally generated strategy game built with Bevy

use bevy::prelude::*;
use clap::Parser;
use lw_game::LivingWorldsPlugin;
// TODO: Add lw_simulation dependency to main Cargo.toml when needed
// use lw_simulation::components::SimulationState;
use lw_core::Fixed32;

/// Living Worlds - A procedural civilization simulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Seed for world generation
    #[arg(short, long, default_value_t = 12345)]
    seed: u32,

    /// World size (small=1000, medium=2000, large=5000)
    #[arg(short, long, default_value = "medium")]
    world_size: String,

    /// Run in test mode
    #[arg(long)]
    test: bool,
}

fn main() {
    let args = Args::parse();
    
    println!("Living Worlds - Starting with seed: {}", args.seed);
    println!("World size: {}", args.world_size);
    
    // Build the Bevy app
    App::new()
        // Default plugins provide basic functionality
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Living Worlds".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // Insert world configuration
        .insert_resource(WorldSeed(args.seed))
        .insert_resource(WorldSize::from_str(&args.world_size))
        // Insert simulation state
        // TODO: Create proper SimulationState initialization
        /*
        .insert_resource(SimulationState {
            // SimulationState fields need to be initialized
            current_turn: 0,
            game_time: lw_core::shared_types::GameTime::new(1000, 1, 1),
            paused: false,
            // ... other fields
        })
        */
        // Add our game plugin
        .add_plugins(LivingWorldsPlugin)
        // Add our game systems
        .add_systems(Startup, setup_world)
        .add_systems(Update, (
            handle_input,
            // update_calendar, // TODO: Update to use SimulationState
        ))
        .run();
}

/// Resource holding the world generation seed
#[derive(Resource)]
struct WorldSeed(u32);

/// Resource for world size configuration
#[derive(Resource)]
enum WorldSize {
    Small,  // 1000 provinces
    Medium, // 2000 provinces
    Large,  // 5000 provinces
}

impl WorldSize {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" => WorldSize::Small,
            "large" => WorldSize::Large,
            _ => WorldSize::Medium,
        }
    }
    
    fn province_count(&self) -> usize {
        match self {
            WorldSize::Small => 1000,
            WorldSize::Medium => 2000,
            WorldSize::Large => 5000,
        }
    }
}

/// Initial world setup
fn setup_world(
    mut commands: Commands,
    seed: Res<WorldSeed>,
    size: Res<WorldSize>,
) {
    // Add 2D camera (Bevy 0.16 uses Camera2d component)
    commands.spawn(Camera2d::default());
    
    println!("Generating world with {} provinces...", size.province_count());
    
    // TODO: Generate procedural world
    // This will call into lw_procedural crate
    // - Generate terrain
    // - Create provinces  
    // - Spawn initial nations
    // - Setup economy
    
    println!("World generation complete!");
}

/// Handle keyboard input
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    // TODO: Update to use SimulationState
    // mut simulation: ResMut<SimulationState>,
) {
    // ESC to exit
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    
    // TODO: Update pause and speed controls to use SimulationState
    /*
    // Space to pause
    if keyboard.just_pressed(KeyCode::Space) {
        simulation.paused = !simulation.paused;
        println!("Game {}", if simulation.paused { "paused" } else { "resumed" });
    }
    
    // Number keys for speed control
    if keyboard.just_pressed(KeyCode::Digit1) {
        simulation.simulation_speed = SimulationSpeed::Slow;
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        simulation.simulation_speed = SimulationSpeed::Normal;
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        simulation.simulation_speed = SimulationSpeed::Fast;
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        simulation.simulation_speed = SimulationSpeed::VeryFast;
    }
    */
}

// TODO: Update to use SimulationState
/*
/// Update the calendar system
fn update_calendar(
    time: Res<Time>,
    mut simulation: ResMut<SimulationState>,
) {
    if time_state.is_paused {
        return;
    }
    
    // Advance time based on delta and speed multiplier
    let delta = time.delta().as_secs_f32() * time_state.speed_multiplier;
    
    // Each tick represents one day
    // At 1x speed, advance 1 day per second
    time_state.tick += (delta * 1.0) as u32;
    
    // Update calendar from ticks
    let total_days = time_state.tick;
    time_state.current_year = 1000 + (total_days / 360) as i32; // 360 days per year
    let day_of_year = total_days % 360;
    time_state.current_month = ((day_of_year / 30) + 1) as u8; // 30 days per month
    time_state.current_day = ((day_of_year % 30) + 1) as u8;
}
*/