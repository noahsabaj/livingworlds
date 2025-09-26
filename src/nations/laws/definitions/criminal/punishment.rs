//! Criminal punishment law definitions
//!
//! Laws governing criminal punishments and sentencing.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All punishment-related laws
pub static PUNISHMENT_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(5000),
            category: LawCategory::Criminal,
            name: "Death Penalty".to_string(),
            description: "Capital punishment for severe crimes".to_string(),
            effects: LawEffects {
                revolt_risk_change: -0.15,
                happiness_modifier: -0.1,
                stability_change: 0.05,
                diplomatic_reputation_change: -0.1,
                maintenance_cost_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(5001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Authoritarian, 0.8),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Democratic, -0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(5001),
            category: LawCategory::Criminal,
            name: "Rehabilitation Focus".to_string(),
            description: "Emphasis on criminal rehabilitation over punishment".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.15,
                revolt_risk_change: -0.05,
                technology_rate_modifier: 0.05,
                maintenance_cost_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(3)],
            conflicts_with: vec![LawId::new(5000), LawId::new(5002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Socialist, 0.7),
                (GovernmentCategory::Authoritarian, -0.6),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(5002),
            category: LawCategory::Criminal,
            name: "Forced Labor".to_string(),
            description: "Criminals sentenced to hard labor".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.1,
                happiness_modifier: -0.15,
                revolt_risk_change: 0.1,
                maintenance_cost_modifier: -0.1,
                diplomatic_reputation_change: -0.15,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(5001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Authoritarian, 0.9),
                (GovernmentCategory::Corporate, 0.6),
                (GovernmentCategory::Democratic, -0.8),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(5003),
            category: LawCategory::Criminal,
            name: "Corporal Punishment".to_string(),
            description: "Physical punishment for criminal offenses".to_string(),
            effects: LawEffects {
                revolt_risk_change: -0.1,
                happiness_modifier: -0.2,
                stability_change: -0.05,
                maintenance_cost_modifier: -0.15,
                diplomatic_reputation_change: -0.2,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(5001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Tribal, 0.7),
                (GovernmentCategory::Authoritarian, 0.6),
                (GovernmentCategory::Democratic, -0.9),
            ]),
            complexity: LawComplexity::Trivial,
            base_popularity: 0.2,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});