//! Refugee law definitions
//!
//! Laws governing asylum and refugee protection.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All refugee-related laws
pub static REFUGEE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(7006),
            category: LawCategory::Immigration,
            name: "Right of Asylum".to_string(),
            description: "Protection for those fleeing persecution".to_string(),
            effects: LawEffects {
                diplomatic_reputation_change: 0.2,
                happiness_modifier: 0.1,
                population_growth_modifier: 0.1,
                maintenance_cost_modifier: 0.1,
                stability_change: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(7007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Socialist, 0.7),
                (GovernmentCategory::Fascist, -0.9),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(7007),
            category: LawCategory::Immigration,
            name: "No Sanctuary".to_string(),
            description: "Rejection of all refugee claims".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                maintenance_cost_modifier: -0.1,
                diplomatic_reputation_change: -0.3,
                population_growth_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(7006)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Fascist, 0.8),
                (GovernmentCategory::Authoritarian, 0.6),
                (GovernmentCategory::Democratic, -0.8),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});