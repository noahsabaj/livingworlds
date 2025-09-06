//! Individual Decisions Phase
//! 
//! Every individual makes decisions based on their needs, skills, information, and incentives.
//! This is the foundation of Austrian economics - all economic activity emerges from individual actions.

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::individual::*;
use crate::components::simulation::*;
/// Execute the individual decisions phase of simulation
pub fn execute_individual_decisions_phase(
    mut simulation: ResMut<SimulationState>,
    mut individuals: Query<(&mut Individual, &mut DecisionState, Entity)>,
) {
    // TODO: Implement phase management with Bevy state system
    // simulation.set_active_phase(SimulationPhase::IndividualDecisions);
    
    for (mut individual, mut decision_state, entity) in &mut individuals {
        // Each person evaluates their current situation
        let current_needs = assess_needs(&individual);
        let available_opportunities = scan_opportunities(&individual);
        let social_pressures = evaluate_social_pressure(&individual);
        
        // Make decisions based on Austrian economics principles:
        // - Subjective value theory (everyone values things differently)
        // - Marginal utility (diminishing returns)
        // - Time preference (immediate vs future benefits)
        // - Local knowledge (only knows what they can observe)
        
        let optimal_action = calculate_optimal_action(
            &individual,
            &current_needs,
            &available_opportunities,
            &social_pressures,
        );
        
        decision_state.current_action = Some(optimal_action);
        decision_state.last_decision_time = Fixed32::from_num(simulation.game_time.tick as i32);
        
        // Individual decisions create emergent patterns
        // No central planner coordinates this - it emerges naturally
    }
    
    // TODO: Implement phase management with Bevy state system
    // simulation.complete_phase(SimulationPhase::IndividualDecisions);
}

fn assess_needs(individual: &Individual) -> Vec<(Need, Fixed32)> {
    individual.needs.iter().map(|need| {
        let urgency = match need {
            Need::Food { satisfaction } => Fixed32::ONE - satisfaction.value(),
            Need::Shelter { quality } => Fixed32::ONE - quality.value(),
            Need::Safety { threat_level } => threat_level.value(),
            Need::Status { social_rank } => Fixed32::ONE - social_rank.value(),
            Need::Purpose { fulfillment } => Fixed32::ONE - fulfillment.value(),
        };
        (need.clone(), urgency)
    }).collect()
}

fn scan_opportunities(individual: &Individual) -> Vec<Opportunity> {
    // Scan known provinces for opportunities
    vec![] // TODO: Convert job_opportunities from Vec<JobInfo> to Vec<Opportunity>
}

fn evaluate_social_pressure(individual: &Individual) -> Fixed32 {
    Fixed32::from_num(individual.knowledge.social_connections.len() as i32) * Fixed32::from_float(0.05)
}

fn calculate_optimal_action(
    individual: &Individual,
    needs: &[(Need, Fixed32)],
    opportunities: &[Opportunity],
    social_pressure: &Fixed32,
) -> Decision {
    // Calculate utility of different actions
    // Simplified for now - returning a placeholder Continue decision
    Decision::Continue
}