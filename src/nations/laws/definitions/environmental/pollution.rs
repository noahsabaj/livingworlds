//! Pollution control law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Pollution control laws
pub static POLLUTION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(8002),
            category: LawCategory::Environmental,
            name: "Emission Standards".to_string(),
            description: "Limits on industrial pollution".to_string(),
            effects: LawEffects {
                industrial_output_modifier: -0.1,
                happiness_modifier: 0.15,
                maintenance_cost_modifier: 0.1,
                technology_rate_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Corporate, -0.6),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 200,
        },
        Law {
            id: LawId::new(8004),
            category: LawCategory::Environmental,
            name: "Carbon Tax".to_string(),
            description: "Tax on carbon emissions".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: 0.1,
                industrial_output_modifier: -0.05,
                technology_rate_modifier: 0.1,
                happiness_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Technocratic, 0.7),
                (GovernmentCategory::Corporate, -0.7),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 250,
        },
    ]
});