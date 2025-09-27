//! Time control using the shortcuts registry system

use bevy::prelude::*;
use crate::simulation::{GameTime, SimulationSpeed, SimulationSpeedChanged};
use crate::ui::shortcuts::{ShortcutEvent, ShortcutId};

/// Handle time control through shortcut events
pub fn handle_time_controls(
    mut game_time: ResMut<GameTime>,
    mut shortcut_events: EventReader<ShortcutEvent>,
    mut speed_events: EventWriter<SimulationSpeedChanged>,
) {
    let old_speed = game_time.get_speed();
    let was_paused = game_time.is_paused();
    let mut speed_changed = false;

    for event in shortcut_events.read() {
        match event.shortcut_id {
            ShortcutId::Pause => {
                game_time.toggle_pause();
                speed_changed = true;
                info!(
                    "Simulation {} (speed: {})",
                    if game_time.is_paused() { "paused" } else { "resumed" },
                    game_time.get_speed().name()
                );
            }

            ShortcutId::Speed1 => {
                game_time.set_speed(SimulationSpeed::Normal);
                speed_changed = true;
                info!("Simulation speed: Normal (1x)");
            }
            ShortcutId::Speed2 => {
                game_time.set_speed(SimulationSpeed::Fast);
                speed_changed = true;
                info!("Simulation speed: Fast (2x)");
            }
            ShortcutId::Speed3 => {
                game_time.set_speed(SimulationSpeed::Faster);
                speed_changed = true;
                info!("Simulation speed: Faster (6x)");
            }
            ShortcutId::Speed4 => {
                game_time.set_speed(SimulationSpeed::Fastest);
                speed_changed = true;
                info!("Simulation speed: Fastest (9x)");
            }
            ShortcutId::Speed5 => {
                game_time.set_speed(SimulationSpeed::Fastest);
                speed_changed = true;
                info!("Simulation speed: Fastest (9x) - Max");
            }

            ShortcutId::SpeedUp => {
                let current = game_time.get_speed();
                let new_speed = match current {
                    SimulationSpeed::Paused => SimulationSpeed::Normal,
                    SimulationSpeed::Normal => SimulationSpeed::Fast,
                    SimulationSpeed::Fast => SimulationSpeed::Faster,
                    SimulationSpeed::Faster => SimulationSpeed::Fastest,
                    SimulationSpeed::Fastest => SimulationSpeed::Fastest,
                };
                if new_speed != current {
                    game_time.set_speed(new_speed);
                    speed_changed = true;
                    info!("Speed increased to: {}", new_speed.name());
                }
            }

            ShortcutId::SlowDown => {
                let current = game_time.get_speed();
                let new_speed = match current {
                    SimulationSpeed::Fastest => SimulationSpeed::Faster,
                    SimulationSpeed::Faster => SimulationSpeed::Fast,
                    SimulationSpeed::Fast => SimulationSpeed::Normal,
                    SimulationSpeed::Normal => SimulationSpeed::Paused,
                    SimulationSpeed::Paused => SimulationSpeed::Paused,
                };
                if new_speed != current {
                    game_time.set_speed(new_speed);
                    speed_changed = true;
                    info!("Speed decreased to: {}", new_speed.name());
                }
            }

            _ => {} // Other shortcuts not handled here
        }
    }

    // Send event if speed changed
    if speed_changed && (old_speed != game_time.get_speed() || was_paused != game_time.is_paused()) {
        speed_events.send(SimulationSpeedChanged {
            new_speed: game_time.get_speed().multiplier(),
            is_paused: game_time.is_paused(),
        });
    }
}
