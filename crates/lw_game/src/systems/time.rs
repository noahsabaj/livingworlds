//! Time system - handles calendar and game speed

use bevy::prelude::*;
use crate::components::TimeState;
use lw_core::Fixed32;

#[derive(Event, Debug)]
pub struct NewDayEvent {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[derive(Event, Debug)]
pub struct NewMonthEvent {
    pub month: u32,
    pub year: i32,
}

#[derive(Event, Debug)]
pub struct NewYearEvent {
    pub year: i32,
}

/// Main time system
pub fn time_system(
    mut time_state: ResMut<TimeState>,
    time: Res<Time>,
    mut day_events: EventWriter<NewDayEvent>,
    mut month_events: EventWriter<NewMonthEvent>,
    mut year_events: EventWriter<NewYearEvent>,
) {
    let dt = time.delta().as_secs_f32();
    
    // Update accumulated time
    let speed_multiplier = time_state.speed_multiplier; // Store to avoid borrow conflict
    time_state.accumulated_time += Fixed32::from_float(dt * speed_multiplier);
    
    // Check for day advance
    let seconds_per_day = 2.0; // 2 real seconds = 1 game day at 1x speed
    while time_state.accumulated_time >= Fixed32::from_float(seconds_per_day) {
        time_state.accumulated_time -= Fixed32::from_float(seconds_per_day);
        time_state.current_day += 1;
        
        // Check for month advance
        if time_state.current_day > time_state.days_per_month {
            time_state.current_day = 1;
            time_state.current_month += 1;
            
            // Check for year advance
            if time_state.current_month > time_state.months_per_year {
                time_state.current_month = 1;
                time_state.current_year += 1;
                
                year_events.send(NewYearEvent {
                    year: time_state.current_year,
                });
            }
            
            month_events.send(NewMonthEvent {
                month: time_state.current_month as u32,
                year: time_state.current_year,
            });
        }
        
        day_events.send(NewDayEvent {
            day: time_state.current_day as u32,
            month: time_state.current_month as u32,
            year: time_state.current_year,
        });
    }
}

/// Speed control system
pub fn speed_control_system(
    mut time_state: ResMut<TimeState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Space to pause/unpause
    if keyboard.just_pressed(KeyCode::Space) {
        time_state.is_paused = !time_state.is_paused;
    }
    
    // Number keys for speed
    if keyboard.just_pressed(KeyCode::Digit1) {
        time_state.speed_multiplier = 1.0;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        time_state.speed_multiplier = 2.0;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        time_state.speed_multiplier = 5.0;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        time_state.speed_multiplier = 10.0;
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        time_state.speed_multiplier = 50.0;
    }
    
    // Apply pause
    if time_state.is_paused {
        time_state.speed_multiplier = 0.0;
    }
}