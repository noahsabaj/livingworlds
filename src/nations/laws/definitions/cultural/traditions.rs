//! Cultural tradition law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Cultural tradition laws
pub static TRADITION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(10003),
            category: LawCategory::Cultural,
            name: "Cultural Preservation".to_string(),
            description: "Protection of traditional practices".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                happiness_modifier: 0.05,
                cultural_conversion_modifier: -0.3,
                technology_rate_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Tribal, 0.8),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Technocratic, -0.5),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});