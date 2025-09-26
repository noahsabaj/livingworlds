//! Gender law definitions
//!
//! Laws governing gender equality and social roles.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All gender-related laws
pub static GENDER_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(3005),
            category: LawCategory::Social,
            name: "Gender Equality".to_string(),
            description: "Equal rights and opportunities regardless of gender".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.15,
                industrial_output_modifier: 0.1,
                technology_rate_modifier: 0.1,
                population_growth_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(3006)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Socialist, 0.9),
                (GovernmentCategory::Theocratic, -0.5),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: true,
            available_from_year: 200,
        },
        Law {
            id: LawId::new(3006),
            category: LawCategory::Social,
            name: "Traditional Gender Roles".to_string(),
            description: "Strict separation of gender responsibilities".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                population_growth_modifier: 0.1,
                industrial_output_modifier: -0.15,
                happiness_modifier: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 0.7),
                (GovernmentCategory::Tribal, 0.6),
                (GovernmentCategory::Democratic, -0.5),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});