//! Technical education law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Technical education laws
pub static EDUCATION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(9003),
            category: LawCategory::Technology,
            name: "Technical Universities".to_string(),
            description: "State-funded engineering and science education".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.15,
                happiness_modifier: 0.05,
                maintenance_cost_modifier: 0.1,
                industrial_output_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Technocratic, 0.8),
                (GovernmentCategory::Democratic, 0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 150,
        },
    ]
});