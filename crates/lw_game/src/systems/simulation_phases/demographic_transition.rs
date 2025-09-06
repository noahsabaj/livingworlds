//! Demographic Transition Phase

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::*;
use crate::components::simulation::*;

pub fn execute_demographic_transition_phase(
    mut simulation: ResMut<SimulationState>,
    mut populations: Query<(&mut Population, &Province)>,
) {
    // TODO: Implement phase management with Bevy state system
    // simulation.set_active_phase(SimulationPhase::PopulationGrowth);
    
    // New generations born, old die
    
    // TODO: Implement phase management with Bevy state system
    // simulation.complete_phase(SimulationPhase::SocialMobility);
}
