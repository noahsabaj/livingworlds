//! World Tension Musical System
//! 
//! Music continuously plays based on world tension (0.0=peace to 1.0=world war)
//! The soundtrack evolves smoothly as tension changes
//! Press T/G to test tension changes, music plays automatically

use bevy::prelude::*;
use bevy::audio::{Pitch, Volume};
use bevy::time::{Timer, TimerMode};
use std::time::Duration;
use crate::resources::WorldTension;

/// Musical moods mapped to tension ranges
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub enum MusicState {
    Silence,        // 0.0 - 0.05: Nearly dead world
    Dawn,           // 0.05 - 0.15: Primitive beginnings
    Peace,          // 0.15 - 0.30: Tranquil growth
    Growth,         // 0.30 - 0.45: Expansion, exploration
    Competition,    // 0.45 - 0.60: Rivalry, tension building
    Conflict,       // 0.60 - 0.75: Active wars
    Crisis,         // 0.75 - 0.90: Major conflicts
    Apocalypse,     // 0.90 - 1.00: World war
}

impl Default for MusicState {
    fn default() -> Self {
        Self::Peace
    }
}

impl MusicState {
    fn from_tension(tension: f32) -> Self {
        match tension {
            t if t < 0.05 => Self::Silence,
            t if t < 0.15 => Self::Dawn,
            t if t < 0.30 => Self::Peace,
            t if t < 0.45 => Self::Growth,
            t if t < 0.60 => Self::Competition,
            t if t < 0.75 => Self::Conflict,
            t if t < 0.90 => Self::Crisis,
            _ => Self::Apocalypse,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            Self::Silence => "The world holds its breath...",
            Self::Dawn => "First stirrings of civilization",
            Self::Peace => "Harmony and gentle growth",
            Self::Growth => "Nations expand and explore",
            Self::Competition => "Rivalry simmers beneath diplomacy",
            Self::Conflict => "Wars break out across the land",
            Self::Crisis => "Major powers clash violently",
            Self::Apocalypse => "Total war consumes the world",
        }
    }
}

/// Setup the music system
fn setup_music_system(
    mut commands: Commands,
    mut pitch_assets: ResMut<Assets<Pitch>>,
) {
    println!("\n🎵 World Tension Music System");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Controls:");
    println!("  T - Increase tension (simulate war)");
    println!("  G - Decrease tension (simulate peace)");
    println!("  Y - Sudden crisis (+0.3 tension)");
    println!("  H - Peace treaty (-0.3 tension)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Startup sound
    let chime = pitch_assets.add(Pitch::new(440.0, Duration::from_millis(100)));
    commands.spawn((
        AudioPlayer(chime),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.1)),
    ));
}

/// Manual tension control for testing
fn update_tension_manual(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tension: ResMut<WorldTension>,
) {
    // Gradual changes
    if keyboard.pressed(KeyCode::KeyT) {
        tension.target = (tension.target + 0.01).min(1.0);
    }
    if keyboard.pressed(KeyCode::KeyG) {
        tension.target = (tension.target - 0.01).max(0.0);
    }
    
    // Sudden events
    if keyboard.just_pressed(KeyCode::KeyY) {
        tension.target = (tension.target + 0.3).min(1.0);
        println!("⚠️ CRISIS! Multiple wars declared!");
    }
    if keyboard.just_pressed(KeyCode::KeyH) {
        tension.target = (tension.target - 0.3).max(0.0);
        println!("☮️ PEACE TREATY! Wars ending...");
    }
}

/// Physics simulation for smooth tension changes
fn calculate_tension_physics(
    mut tension: ResMut<WorldTension>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    
    // Calculate the difference
    let diff = tension.target - tension.current;
    
    // Apply different rates for heating vs cooling
    let rate = if diff > 0.0 {
        tension.heating_rate  // Tension rises quickly
    } else {
        tension.cooling_rate  // Tension falls slowly
    };
    
    // Update velocity with inertia
    tension.velocity = tension.velocity * tension.inertia + diff * rate * dt;
    
    // Apply velocity to current tension
    tension.current = (tension.current + tension.velocity * dt).clamp(0.0, 1.0);
}

/// Map tension to musical mood
fn map_tension_to_mood(
    tension: Res<WorldTension>,
    mut current_mood: ResMut<MusicState>,
) {
    let new_mood = MusicState::from_tension(tension.current);
    
    if new_mood != *current_mood {
        println!("\n🎼 Mood shift: {:?} → {:?}", *current_mood, new_mood);
        println!("   {}", new_mood.description());
        println!("   Tension: {:.1}%", tension.current * 100.0);
        *current_mood = new_mood;
    }
}

// Future Extensions:
// 
// 1. Event-Driven Tension:
//    - War declaration: +0.1 instantly
//    - Nation collapse: +0.2
//    - Trade route: -0.02
//    - Alliance: -0.05
//
// 2. Regional Tension:
//    - Different areas have different tension
//    - Music layers regional themes
//
// 3. Tension Memory:
//    - Recent wars leave "tension residue"
//    - Peace takes time to truly settle
//
// 4. Musical Transitions:
//    - Smooth pitch bending between moods
//    - Crossfade overlapping themes
//    - Tempo gradually changes

/// Music Plugin that manages procedural audio generation
pub struct ProceduralMusicPlugin;

impl Plugin for ProceduralMusicPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<WorldTension>()
            .init_resource::<MusicState>()
            .init_resource::<ContinuousMusicTimer>()
            
            // Startup system
            .add_systems(Startup, setup_music_system)
            
            // Update systems
            .add_systems(Update, (
                update_tension_manual,
                calculate_tension_physics,
                map_tension_to_mood,
                continuous_music_system,
            ).chain());
    }
}

/// Timer resource for continuous music playback
#[derive(Resource)]
struct ContinuousMusicTimer {
    timer: Timer,
    current_phrase_index: usize,
}

impl Default for ContinuousMusicTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            current_phrase_index: 0,
        }
    }
}

/// Play continuous background music based on mood
fn continuous_music_system(
    time: Res<Time>,
    mood: Res<MusicState>,
    tension: Res<WorldTension>,
    mut timer: ResMut<ContinuousMusicTimer>,
    mut commands: Commands,
    mut pitch_assets: ResMut<Assets<Pitch>>,
) {
    timer.timer.tick(time.delta());
    
    if !timer.timer.just_finished() {
        return;
    }
    
    // Skip music in Silence mood
    if matches!(*mood, MusicState::Silence) {
        return;
    }
    
    // Generate musical phrases based on mood
    let phrases = match *mood {
        MusicState::Silence => return,
        
        MusicState::Dawn => {
            // Sparse, lonely notes
            vec![
                vec![(110.0, 800, 0.08)],  // A2
                vec![],                      // Silence
                vec![(165.0, 600, 0.06)],   // E3
                vec![],                      // Silence
            ]
        },
        
        MusicState::Peace => {
            // Gentle arpeggios
            vec![
                vec![(261.63, 400, 0.12), (329.63, 400, 0.12)],  // C4, E4
                vec![(392.00, 400, 0.12), (523.25, 600, 0.15)],  // G4, C5
                vec![(329.63, 400, 0.12), (261.63, 400, 0.12)],  // E4, C4
                vec![(196.00, 800, 0.10)],                        // G3
            ]
        },
        
        MusicState::Growth => {
            // Ascending patterns
            vec![
                vec![(261.63, 300, 0.15), (293.66, 300, 0.15)],  // C4, D4
                vec![(329.63, 300, 0.15), (392.00, 300, 0.15)],  // E4, G4
                vec![(440.00, 300, 0.18), (523.25, 500, 0.20)],  // A4, C5
                vec![(392.00, 300, 0.15), (329.63, 300, 0.15)],  // G4, E4
            ]
        },
        
        MusicState::Competition => {
            // Competing melodies
            vec![
                vec![(261.63, 400, 0.25), (329.63, 200, 0.20)],  // Nation 1
                vec![(293.66, 400, 0.22), (349.23, 200, 0.20)],  // Nation 2
                vec![(261.63, 200, 0.25), (277.18, 200, 0.25)],  // Dissonance
                vec![(246.94, 600, 0.20)],                        // B3 resolution
            ]
        },
        
        MusicState::Conflict => {
            // War drums and tension
            vec![
                vec![(110.00, 200, 0.35), (110.00, 200, 0.35)],  // Drums
                vec![(164.81, 150, 0.25), (185.00, 150, 0.25)],  // E3, F#3
                vec![(110.00, 200, 0.35), (110.00, 200, 0.35)],  // Drums
                vec![(155.56, 400, 0.30)],                        // D#3
            ]
        },
        
        MusicState::Crisis => {
            // Chaotic patterns
            vec![
                vec![(87.31, 300, 0.45), (92.50, 300, 0.45)],    // F2, F#2
                vec![(130.81, 200, 0.35), (138.59, 200, 0.35)],  // C3, C#3
                vec![(87.31, 400, 0.50)],                         // F2
                vec![(82.41, 200, 0.40), (87.31, 200, 0.40)],    // E2, F2
            ]
        },
        
        MusicState::Apocalypse => {
            // Total chaos
            vec![
                vec![(65.41, 1000, 0.60), (69.30, 1000, 0.55)],  // C2, C#2
                vec![(73.42, 1000, 0.55), (77.78, 1000, 0.55)],  // D2, D#2
                vec![(82.41, 1000, 0.60)],                        // E2
                vec![(65.41, 2000, 0.65)],                        // C2 sustain
            ]
        },
    };
    
    // Play the current phrase
    if !phrases.is_empty() {
        let phrase = &phrases[timer.current_phrase_index % phrases.len()];
        
        for &(freq, duration_ms, volume) in phrase {
            let pitch = pitch_assets.add(Pitch::new(freq, Duration::from_millis(duration_ms)));
            commands.spawn((
                AudioPlayer(pitch),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume * 0.5)), // Quieter for background
            ));
        }
        
        // Advance to next phrase
        timer.current_phrase_index = (timer.current_phrase_index + 1) % phrases.len();
    }
    
    // Adjust timer based on tension (faster music when tense)
    let base_duration = 3.0 - (tension.current * 1.5); // 3s at peace, 1.5s at war
    timer.timer.set_duration(Duration::from_secs_f32(base_duration));
}