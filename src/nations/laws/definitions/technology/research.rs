//! Research funding law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// Research funding laws
pub static RESEARCH_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(9000),
            category: LawCategory::Technology,
            name: "State Research Funding".to_string(),
            description: "Government investment in scientific research".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.3,
                tax_efficiency_modifier: -0.05,
                maintenance_cost_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Technocratic, 1.0),
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Tribal, -0.5),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(9001),
            category: LawCategory::Technology,
            name: "Private Research Incentives".to_string(),
            description: "Tax breaks for corporate research".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.2,
                industrial_output_modifier: 0.1,
                tax_efficiency_modifier: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.9),
                (GovernmentCategory::Technocratic, 0.5),
                (GovernmentCategory::Socialist, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 150,
        },
    ]
});