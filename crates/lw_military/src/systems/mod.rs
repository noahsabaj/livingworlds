//! Military Logic Systems
//! 
//! All logic for Army components extracted to follow ECS principles.

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::*;

/// Calculate combat effectiveness for an army
pub fn calculate_combat_effectiveness(army: &Army) -> Fixed32 {
    let base_power = calculate_base_combat_power(army);
    let supply_modifier = calculate_supply_effectiveness(&army.supply_state);
    let morale_modifier = army.morale.current;
    let leadership_modifier = calculate_leadership_effectiveness(&army.leadership);
    let terrain_modifier = Fixed32::ONE; // Would get from terrain component
    
    base_power * supply_modifier * morale_modifier * leadership_modifier * terrain_modifier
}

/// Calculate base combat power from units
pub fn calculate_base_combat_power(army: &Army) -> Fixed32 {
    let infantry_power = army.composition.infantry * Fixed32::from_float(1.0);
    let cavalry_power = army.composition.cavalry * Fixed32::from_float(2.5);
    let artillery_power = army.composition.artillery * Fixed32::from_float(4.0);
    
    infantry_power + cavalry_power + artillery_power
}

/// Calculate supply effectiveness
pub fn calculate_supply_effectiveness(supply_state: &ArmySupplyDetails) -> Fixed32 {
    let food_factor = supply_state.food_days.min(Fixed32::from_num(30)) / Fixed32::from_num(30);
    let ammo_factor = supply_state.ammunition;
    let equipment_factor = supply_state.equipment_quality;
    
    (food_factor + ammo_factor + equipment_factor) / Fixed32::from_num(3)
}

/// Calculate leadership effectiveness
pub fn calculate_leadership_effectiveness(leadership: &Leadership) -> Fixed32 {
    leadership.commander_skill * Fixed32::from_float(0.7) + 
    leadership.officer_quality * Fixed32::from_float(0.3)
}

/// Calculate movement speed based on terrain
pub fn calculate_movement_speed(army: &Army, terrain_difficulty: Fixed32) -> Fixed32 {
    let base_speed = Fixed32::from_float(20.0); // km/day
    let supply_burden = calculate_supply_burden(army);
    let size_penalty = (army.composition.total() / Fixed32::from_num(10000)).min(Fixed32::from_float(0.5));
    
    base_speed * (Fixed32::ONE - supply_burden) * (Fixed32::ONE - size_penalty) / terrain_difficulty
}

/// Calculate supply burden on movement
pub fn calculate_supply_burden(army: &Army) -> Fixed32 {
    let total_units = army.composition.total();
    let supply_train = army.logistics.supply_wagons;
    let ratio = supply_train / total_units.max(Fixed32::ONE);
    
    (Fixed32::from_float(0.5) - ratio).max(Fixed32::ZERO)
}

/// System to update army morale
pub fn update_army_morale_system(
    mut armies: Query<&mut Army>,
    time: Res<Time>,
) {
    let dt = Fixed32::from_float(time.delta().as_secs_f32());
    
    for mut army in &mut armies {
        // Victory increases morale
        if army.morale.recent_victory {
            army.morale.current = (army.morale.current + Fixed32::from_float(0.1)).min(Fixed32::ONE);
            army.morale.recent_victory = false;
        }
        
        // Defeat decreases morale
        if army.morale.recent_defeat {
            army.morale.current = (army.morale.current - Fixed32::from_float(0.2)).max(Fixed32::ZERO);
            army.morale.recent_defeat = false;
        }
        
        // Poor supply decreases morale
        if army.supply_state.food_days < Fixed32::from_num(3) {
            army.morale.current = (army.morale.current - Fixed32::from_float(0.05) * dt).max(Fixed32::ZERO);
        }
        
        // Home territory increases morale
        if army.morale.defending_homeland {
            army.morale.current = (army.morale.current + Fixed32::from_float(0.02) * dt).min(Fixed32::ONE);
        }
        
        // War exhaustion decreases morale over time
        army.morale.current = (army.morale.current - army.morale.war_exhaustion * dt).max(Fixed32::ZERO);
    }
}

/// System to handle army movement
pub fn army_movement_system(
    mut armies: Query<(&mut Army, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut army, mut transform) in &mut armies {
        if let Some(target) = army.movement.target_position {
            let current_pos = transform.translation.truncate();
            let direction = (target - current_pos).normalize();
            
            let speed = calculate_movement_speed(&army, Fixed32::ONE);
            let movement = direction * speed.to_f32() * time.delta().as_secs_f32();
            
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
            
            // Check if reached target
            if current_pos.distance(target) < 1.0 {
                army.movement.target_position = None;
                army.movement.is_moving = false;
            }
        }
    }
}

/// Combat resolution system
pub fn combat_resolution_system(
    mut armies: Query<&mut Army>,
    mut combat_events: EventWriter<CombatResult>,
) {
    // This would handle combat between armies
    // For now, placeholder
}