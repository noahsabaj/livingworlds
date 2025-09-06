//! Culture Logic Systems
//! 
//! All logic for Culture components extracted to follow ECS principles.

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::culture::core::*;
use crate::components::culture::contact::*;
use crate::components::individual::{Individual, MigrationEvent};
use crate::components::economics::{GoodType, trade::TradeEvent};
use crate::types::GameTime;

/// Calculate how attractive a culture appears to an observer
pub fn calculate_culture_attractiveness(
    culture: &Culture,
    observer: &Individual,
) -> Fixed32 {
    // Attractiveness based on:
    // - Economic prosperity of culture members
    // - Military success
    // - Technological advancement
    // - Values alignment with observer
    
    let prosperity_factor = culture.average_wealth;
    let technology_factor = culture.technology_level;
    let prestige_factor = culture.prestige;
    
    // Weight factors based on observer's values
    let weights = match observer.personality.materialism {
        m if m.value() > Fixed32::from_float(0.7) => (Fixed32::from_float(0.5), Fixed32::from_float(0.3), Fixed32::from_float(0.2)),
        _ => (Fixed32::from_float(0.3), Fixed32::from_float(0.3), Fixed32::from_float(0.4)),
    };
    
    prosperity_factor * weights.0 + technology_factor * weights.1 + prestige_factor * weights.2
}

/// Evaluate whether to adopt a cultural practice
pub fn evaluate_practice_adoption(
    culture: &Culture,
    practice: &CulturalPractice,
    individual: &Individual,
) -> AdoptionDecision {
    // Evaluate based on utility, not random spread
    let utility = calculate_practice_utility(practice, individual);
    let social_pressure = calculate_social_pressure(culture, individual);
    let compatibility = calculate_compatibility(practice, &culture.values);
    
    let adoption_score = utility * Fixed32::from_float(0.5) 
        + social_pressure * Fixed32::from_float(0.3)
        + compatibility * Fixed32::from_float(0.2);
    
    if adoption_score > Fixed32::from_float(0.8) {
        AdoptionDecision::Adopt {
            enthusiasm: adoption_score,
            adaptation: practice.clone(),
        }
    } else if adoption_score > Fixed32::from_float(0.6) {
        AdoptionDecision::PartialAdopt {
            modified_practice: practice.clone(),
        }
    } else {
        AdoptionDecision::Reject {
            reasons: vec!["Low utility score".to_string()],
        }
    }
}

/// Process cultural exchange through trade
pub fn process_trade_cultural_exchange_system(
    mut cultures: Query<(&mut Culture, &ContactNetwork)>,
    mut trades: EventReader<TradeEvent>,
) {
    for trade_event in trades.read() {
        // Trade brings:
        // - Knowledge of foreign goods and their uses
        // - Exposure to different business practices
        // - Language borrowing for trade terms
        // - Dietary changes from new food imports
        
        // Process cultural exchange based on trade volume and frequency
    }
}

/// Process cultural impact of migration
pub fn process_migration_cultural_impact_system(
    mut cultures: Query<(&mut Culture, Entity)>,
    mut migrations: EventReader<MigrationEvent>,
) {
    for migration in migrations.read() {
        // Migration brings entire cultural packages
        // Creates cultural enclaves that may or may not integrate
        
        // Process based on:
        // - Size of migrant population
        // - Cultural distance
        // - Economic integration
        // - Government policies
    }
}

/// Helper functions
fn calculate_practice_utility(practice: &CulturalPractice, individual: &Individual) -> Fixed32 {
    // Calculate actual utility of the practice for this individual
    match &practice.practice_type {
        PracticeType::Economic { efficiency_bonus } => *efficiency_bonus,
        PracticeType::Social { cohesion_bonus } => {
            // Value depends on individual's need for social connection
            *cohesion_bonus * individual.personality.sociability
        },
        PracticeType::Religious { spiritual_fulfillment } => {
            *spiritual_fulfillment * individual.personality.spirituality
        },
    }
}

fn calculate_social_pressure(culture: &Culture, individual: &Individual) -> Fixed32 {
    // Pressure from social connections who have adopted the culture
    Fixed32::from_num(individual.knowledge.social_connections.len() as i32) * Fixed32::from_float(0.05)
}

fn calculate_compatibility(practice: &CulturalPractice, values: &Vec<CulturalValue>) -> Fixed32 {
    // How well does this practice fit with existing values?
    Fixed32::from_float(0.5) // Placeholder
}

/// System to handle all culture logic
pub fn culture_evolution_system(
    mut cultures: Query<(&mut Culture, &ContactNetwork, Entity)>,
    time: Res<Time>,
) {
    for (mut culture, contacts, entity) in &mut cultures {
        // Process cultural evolution based on contacts
        // No abstract "culture points" - only actual human contact
        
        // Update prestige based on achievements
        // Update technology level based on innovations
        // Process practice adoption/abandonment
    }
}