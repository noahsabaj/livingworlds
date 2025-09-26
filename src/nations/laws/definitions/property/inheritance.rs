//! Inheritance law definitions
//!
//! Laws governing wealth and property inheritance.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All inheritance laws
pub static INHERITANCE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(6003),
            category: LawCategory::Property,
            name: "Primogeniture".to_string(),
            description: "Eldest child inherits all property".to_string(),
            effects: LawEffects {
                stability_change: 0.15,
                revolt_risk_change: 0.05,
                legitimacy_change: 0.1,
                happiness_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(6004), LawId::new(6005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Monarchic, 0.9),
                (GovernmentCategory::Aristocratic, 0.8),
                (GovernmentCategory::Democratic, -0.3),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(6004),
            category: LawCategory::Property,
            name: "Equal Inheritance".to_string(),
            description: "Property divided equally among heirs".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.15,
                stability_change: -0.05,
                agricultural_output_modifier: -0.05,
                corruption_change: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(6003), LawId::new(6005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Socialist, 0.6),
                (GovernmentCategory::Monarchic, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 50,
        },
        Law {
            id: LawId::new(6005),
            category: LawCategory::Property,
            name: "State Inheritance".to_string(),
            description: "Property reverts to state upon death".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: 0.3,
                happiness_modifier: -0.2,
                revolt_risk_change: 0.1,
                corruption_change: 0.15,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(6003), LawId::new(6004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 0.7),
                (GovernmentCategory::Authoritarian, 0.6),
                (GovernmentCategory::Corporate, -0.8),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.1,
            is_constitutional: false,
            available_from_year: 100,
        },
    ]
});