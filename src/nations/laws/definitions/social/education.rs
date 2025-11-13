//! Education law definitions
//!
//! Laws governing schooling and educational systems.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// All education-related laws
pub static EDUCATION_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(3002),
            category: LawCategory::Social,
            name: "Public Education".to_string(),
            description: "Free schooling for all children".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.2,
                happiness_modifier: 0.1,
                tax_efficiency_modifier: -0.1,
                cultural_conversion_modifier: 0.15,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3003), LawId::new(3004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Socialist, 0.9),
                (GovernmentCategory::Tribal, -0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 50,
        },
        Law {
            id: LawId::new(3003),
            category: LawCategory::Social,
            name: "Religious Education".to_string(),
            description: "Religious institutions provide education".to_string(),
            effects: LawEffects {
                legitimacy_change: 0.15,
                technology_rate_modifier: -0.1,
                cultural_conversion_modifier: -0.2,
                stability_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3002), LawId::new(3004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 1.0),
                (GovernmentCategory::Monarchic, 0.4),
                (GovernmentCategory::Democratic, -0.3),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(3004),
            category: LawCategory::Social,
            name: "No Formal Education".to_string(),
            description: "Education is a private family matter".to_string(),
            effects: LawEffects {
                technology_rate_modifier: -0.3,
                tax_efficiency_modifier: 0.05,
                wealth_inequality_change: 0.25,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3002), LawId::new(3003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Anarchist, 0.5),
                (GovernmentCategory::Tribal, 0.7),
                (GovernmentCategory::Democratic, -0.7),
            ]),
            complexity: LawComplexity::Trivial,
            base_popularity: 0.2,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});