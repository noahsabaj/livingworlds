//! Marriage law definitions
//!
//! Laws governing marriage and personal relationships.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// All marriage-related laws
pub static MARRIAGE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(3007),
            category: LawCategory::Social,
            name: "Civil Marriage".to_string(),
            description: "Government recognizes and regulates marriage".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                happiness_modifier: 0.05,
                population_growth_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3008)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Anarchist, -0.3),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(3008),
            category: LawCategory::Social,
            name: "Free Union".to_string(),
            description: "No government involvement in personal relationships".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.1,
                stability_change: -0.05,
                cultural_conversion_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Anarchist, 0.9),
                (GovernmentCategory::Democratic, 0.3),
                (GovernmentCategory::Theocratic, -0.7),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});