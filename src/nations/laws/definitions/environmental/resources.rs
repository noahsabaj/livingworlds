//! Resource management law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Resource management laws
pub static RESOURCE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(8003),
            category: LawCategory::Environmental,
            name: "Sustainable Harvesting".to_string(),
            description: "Regulated resource extraction for long-term viability".to_string(),
            effects: LawEffects {
                agricultural_output_modifier: -0.05,
                stability_change: 0.1,
                happiness_modifier: 0.05,
                diplomatic_reputation_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Corporate, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 100,
        },
    ]
});