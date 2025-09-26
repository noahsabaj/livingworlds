//! Conservation law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// Conservation laws
pub static CONSERVATION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(8000),
            category: LawCategory::Environmental,
            name: "Protected Lands".to_string(),
            description: "Designated areas protected from development".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.1,
                industrial_output_modifier: -0.05,
                agricultural_output_modifier: -0.05,
                stability_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(3)],
            conflicts_with: vec![LawId::new(8001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Corporate, -0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(8001),
            category: LawCategory::Environmental,
            name: "Unrestricted Exploitation".to_string(),
            description: "No environmental protection regulations".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.2,
                agricultural_output_modifier: 0.1,
                happiness_modifier: -0.15,
                revolt_risk_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(8000)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.8),
                (GovernmentCategory::Socialist, -0.6),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});