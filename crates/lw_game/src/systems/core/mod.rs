//! Main Simulation Coordinator System
//! 
//! This system orchestrates all simulation phases in the correct order.
//! It replaces the god object SimulationState methods with a clean system.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use crate::components::simulation::*;
use crate::types::GameTime;

/// Main simulation step coordinator
pub fn simulation_coordinator_system(
    mut simulation: ResMut<SimulationState>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<SimulationPhase>>,
) {
    if simulation.paused {
        return;
    }
    
    let step_start = std::time::Instant::now();
    
    // Execute phases in order
    // Each phase system will check if it's the active phase
    // and execute its logic accordingly
    
    // The actual phase execution happens in the individual phase systems
    // This coordinator just manages the overall flow and timing
    
    let step_duration = step_start.elapsed();
    simulation.last_step_duration = step_duration;
    simulation.current_turn += 1;
    simulation.game_time.advance(Fixed32::ONE);
    
    // Update performance metrics
    update_performance_metrics(&mut simulation, step_duration);
}

/// Update performance tracking metrics
fn update_performance_metrics(
    simulation: &mut SimulationState,
    step_duration: std::time::Duration,
) {
    // Update average step time with exponential moving average
    let new_time = Fixed32::from_float(step_duration.as_secs_f32());
    let alpha = Fixed32::from_float(0.1); // Smoothing factor
    
    simulation.average_step_time = 
        simulation.average_step_time * (Fixed32::ONE - alpha) + new_time * alpha;
    
    // Add to phase timings for analysis
    if simulation.phase_timings.len() > 100 {
        simulation.phase_timings.remove(0); // Keep only recent timings
    }
}

/// System to advance to the next simulation phase
pub fn advance_phase_system(
    mut simulation: ResMut<SimulationState>,
    current_phase: Res<State<SimulationPhase>>,
    mut next_state: ResMut<NextState<SimulationPhase>>,
) {
    let current = current_phase.get();
    
    // Mark current phase as completed
    if !simulation.completed_phases.contains(current) {
        simulation.completed_phases.push(*current);
    }
    
    // Determine next phase based on current phase
    let next_phase = match current {
        // Phase 1: Individual Foundation
        SimulationPhase::IndividualDecisions => SimulationPhase::LocalMarketFormation,
        SimulationPhase::LocalMarketFormation => SimulationPhase::SkillDevelopment,
        SimulationPhase::SkillDevelopment => SimulationPhase::PersonalRelationships,
        SimulationPhase::PersonalRelationships => SimulationPhase::MarketPriceClearance,
        
        // Phase 2: Economic Emergence  
        SimulationPhase::MarketPriceClearance => SimulationPhase::ProductionDecisions,
        SimulationPhase::ProductionDecisions => SimulationPhase::TradeNetworkFormation,
        SimulationPhase::TradeNetworkFormation => SimulationPhase::ResourceAllocation,
        SimulationPhase::ResourceAllocation => SimulationPhase::EconomicCrisis,
        SimulationPhase::EconomicCrisis => SimulationPhase::InformationGathering,
        
        // Phase 3: Government Responses
        SimulationPhase::InformationGathering => SimulationPhase::PolicyDecision,
        SimulationPhase::PolicyDecision => SimulationPhase::PolicyImplementation,
        SimulationPhase::PolicyImplementation => SimulationPhase::BureaucraticExecution,
        SimulationPhase::BureaucraticExecution => SimulationPhase::PublicReaction,
        SimulationPhase::PublicReaction => SimulationPhase::IdeaGeneration,
        
        // Phase 4: Cultural Transmission
        SimulationPhase::IdeaGeneration => SimulationPhase::CulturalContact,
        SimulationPhase::CulturalContact => SimulationPhase::IdeaTransmission,
        SimulationPhase::IdeaTransmission => SimulationPhase::CulturalAdoption,
        SimulationPhase::CulturalAdoption => SimulationPhase::CulturalEvolution,
        SimulationPhase::CulturalEvolution => SimulationPhase::ArmyMovement,
        
        // Phase 5: Military Actions
        SimulationPhase::ArmyMovement => SimulationPhase::SupplyLineManagement,
        SimulationPhase::SupplyLineManagement => SimulationPhase::MoraleCalculation,
        SimulationPhase::MoraleCalculation => SimulationPhase::CombatResolution,
        SimulationPhase::CombatResolution => SimulationPhase::WarExhaustionUpdate,
        SimulationPhase::WarExhaustionUpdate => SimulationPhase::TrustEventProcessing,
        
        // Phase 6: Diplomatic Evolution
        SimulationPhase::TrustEventProcessing => SimulationPhase::DiplomaticContactUpdate,
        SimulationPhase::DiplomaticContactUpdate => SimulationPhase::TreatyCompliance,
        SimulationPhase::TreatyCompliance => SimulationPhase::NegotiationProgress,
        SimulationPhase::NegotiationProgress => SimulationPhase::InternationalReaction,
        SimulationPhase::InternationalReaction => SimulationPhase::GeographicChanges,
        
        // Phase 7: World Changes
        SimulationPhase::GeographicChanges => SimulationPhase::ClimateEvolution,
        SimulationPhase::ClimateEvolution => SimulationPhase::ResourceDepletion,
        SimulationPhase::ResourceDepletion => SimulationPhase::EnvironmentalImpact,
        SimulationPhase::EnvironmentalImpact => SimulationPhase::NaturalDisasters,
        SimulationPhase::NaturalDisasters => SimulationPhase::PopulationGrowth,
        
        // Phase 8: Demographic Transition
        SimulationPhase::PopulationGrowth => SimulationPhase::Migration,
        SimulationPhase::Migration => SimulationPhase::UrbanizationProgress,
        SimulationPhase::UrbanizationProgress => SimulationPhase::GenerationalChange,
        SimulationPhase::GenerationalChange => SimulationPhase::SocialMobility,
        SimulationPhase::SocialMobility => SimulationPhase::SystemSynchronization,
        
        // Meta phases
        SimulationPhase::SystemSynchronization => SimulationPhase::ConflictResolution,
        SimulationPhase::ConflictResolution => SimulationPhase::DataValidation,
        SimulationPhase::DataValidation => SimulationPhase::IndividualDecisions, // Loop back to start
    };
    
    // Update active phases
    simulation.active_phases.clear();
    simulation.active_phases.push(next_phase);
    
    // Transition to next phase
    next_state.set(next_phase);
}

/// Check simulation health and handle critical issues
pub fn simulation_health_check_system(
    mut simulation: ResMut<SimulationState>,
    query: Query<Entity>,
) {
    let entity_count = query.iter().count();
    
    // Check for various health issues
    let mut issues = Vec::new();
    
    if entity_count > 1_000_000 {
        issues.push(SimulationIssue::DataInconsistency { system: format!("Too many entities: {}", entity_count) });
    }
    
    if simulation.average_step_time > Fixed32::from_float(1.0) {
        issues.push(SimulationIssue::SlowPerformance { 
            average_step_time: simulation.average_step_time 
        });
    }
    
    if simulation.completed_phases.is_empty() && simulation.current_turn > 10 {
        issues.push(SimulationIssue::DataInconsistency { system: "Simulation appears stuck".to_string() });
    }
    
    // Update health status
    simulation.simulation_health = if issues.is_empty() {
        SimulationHealth::Healthy
    } else if issues.len() > 5 { // If we have many issues, consider it critical
        SimulationHealth::Critical { 
            critical_errors: vec![CriticalError::SystemDeadlock] 
        }
    } else {
        SimulationHealth::Warning { issues }
    };
}



/// Result of a simulation step
#[derive(Debug, Clone)]
pub enum SimulationStepResult {
    Success {
        step_number: u64,
        duration: std::time::Duration,
        events_generated: u32,
    },
    Paused,
    Error(SimulationError),
}

#[derive(Debug, Clone)]
pub enum SimulationError {
    PhaseExecutionFailed(String),
    DataValidationFailed(String),
    CriticalSystemError(String),
}