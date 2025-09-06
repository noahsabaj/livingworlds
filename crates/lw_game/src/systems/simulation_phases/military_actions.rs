//! Military Actions Phase

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::military::*;
use crate::components::simulation::*;

pub fn execute_military_actions_phase(
    mut simulation: ResMut<SimulationState>,
    mut armies: Query<(&mut Army, &SupplyChain, &mut MoraleState)>,
) {
    // TODO: Implement phase management with Bevy state system
    // simulation.set_active_phase(SimulationPhase::ArmyMovement);
    
    // War is about supply lines, morale, terrain, and leadership
    
    // TODO: Implement phase management with Bevy state system
    // simulation.complete_phase(SimulationPhase::CombatResolution);
}
