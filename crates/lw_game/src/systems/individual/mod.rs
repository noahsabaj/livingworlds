//! Individual Logic Systems
//! 
//! All logic for Individual components extracted to follow ECS principles.
//! Components should be pure data - all behavior belongs in systems.

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::individual::*;
use crate::components::common::bounded_types::Percentage;
use crate::components::economics::GoodType;

/// Create a new individual with basic needs and random skills
pub fn create_individual(age: Fixed32, location: Entity, culture: Entity) -> Individual {
    Individual {
        age,
        health: Percentage::new(Fixed32::from_float(0.8)), // Generally healthy
        education: Percentage::new(Fixed32::from_float(0.2)), // Basic education
        
        needs: vec![
            Need::Food { satisfaction: Percentage::new(Fixed32::from_float(0.5)) },
            Need::Shelter { quality: Percentage::new(Fixed32::from_float(0.3)) },
            Need::Safety { threat_level: Percentage::new(Fixed32::from_float(0.2)) },
            Need::Status { social_rank: Percentage::new(Fixed32::from_float(0.1)) },
            Need::Purpose { fulfillment: Percentage::new(Fixed32::from_float(0.4)) },
        ],
        
        skills: vec![
            Skill::Farming { proficiency: Fixed32::from_float(0.3) },
        ],
        
        knowledge: LocalKnowledge {
            known_provinces: vec![location],
            price_memory: Vec::new(),
            job_opportunities: Vec::new(),
            social_connections: Vec::new(),
            rumors: Vec::new(),
        },
        
        incentives: Vec::new(),
        personality: Personality::default(),
        
        current_job: None,
        employer: None,
        wealth: Fixed32::ZERO,
        location,
        
        culture,
        religion: None,
        loyalty: Vec::new(),
    }
}

/// System to assess an individual's current needs
pub fn assess_individual_needs_system(
    mut individuals: Query<(&mut Individual, Entity)>,
) {
    for (individual, entity) in &mut individuals {
        // Assessment logic here - analyze needs array
        for need in &individual.needs {
            match need {
                Need::Food { satisfaction } => {
                    // Process food need
                    if satisfaction.value() < Fixed32::from_float(0.3) {
                        // Individual is hungry
                    }
                },
                Need::Shelter { quality } => {
                    // Process shelter need
                    if quality.value() < Fixed32::from_float(0.2) {
                        // Need better shelter
                    }
                },
                _ => {}
            }
        }
    }
}

/// System to scan local opportunities
pub fn scan_local_opportunities_system(
    individuals: Query<(&Individual, Entity)>,
    markets: Query<&crate::components::economics::Market>,
) {
    for (individual, entity) in &individuals {
        // Scan markets in known provinces
        for province in &individual.knowledge.known_provinces {
            // Look for job opportunities and prices
        }
    }
}

/// System to evaluate social environment
pub fn evaluate_social_environment_system(
    individuals: Query<(&Individual, Entity)>,
) {
    for (individual, entity) in &individuals {
        // Evaluate based on social connections
        let social_pressure = Fixed32::from_num(individual.knowledge.social_connections.len() as i32) 
            * Fixed32::from_float(0.1);
        // Use social pressure in decision making
    }
}

/// System to calculate optimal action for an individual
pub fn calculate_optimal_action_system(
    mut individuals: Query<(&mut Individual, Entity)>,
) {
    for (mut individual, entity) in &mut individuals {
        // Calculate utility of different actions
        // This is where Austrian economics principles apply
        
        // Subjective value theory - everyone values things differently
        let food_value = match individual.personality {
            Personality { risk_tolerance, .. } if risk_tolerance.value() > Fixed32::from_float(0.5) => {
                Fixed32::from_float(0.3)
            },
            _ => Fixed32::from_float(0.7)
        };
        
        // Time preference - immediate vs future benefits
        let time_discount = individual.personality.time_preference;
        
        // Make decision based on marginal utility
        // Action selection logic here
    }
}