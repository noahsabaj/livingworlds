//! Currency law definitions
//!
//! Laws governing monetary systems and currency policies.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};

/// All currency-related laws
pub static CURRENCY_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(1010),
            category: LawCategory::Economic,
            name: "Gold Standard".to_string(),
            description: "Currency backed by gold reserves".to_string(),
            effects: LawEffects {
                trade_income_modifier: 0.1,
                stability_change: 0.15,
                industrial_output_modifier: -0.05,
                wealth_inequality_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1011), LawId::new(1012)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Monarchic, 0.7),
                (GovernmentCategory::Corporate, 0.5),
                (GovernmentCategory::Socialist, -0.3),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1011),
            category: LawCategory::Economic,
            name: "Fiat Currency".to_string(),
            description: "Government-issued currency not backed by commodity".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.1,
                trade_income_modifier: -0.05,
                corruption_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(3)],
            conflicts_with: vec![LawId::new(1010), LawId::new(1012)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Socialist, 0.4),
                (GovernmentCategory::Anarchist, -0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 200,
        },
        Law {
            id: LawId::new(1012),
            category: LawCategory::Economic,
            name: "Barter Economy".to_string(),
            description: "No official currency, direct exchange of goods".to_string(),
            effects: LawEffects {
                trade_income_modifier: -0.3,
                tax_efficiency_modifier: -0.4,
                corruption_change: -0.2,
                stability_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1010), LawId::new(1011)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Anarchist, 0.8),
                (GovernmentCategory::Tribal, 0.6),
                (GovernmentCategory::Corporate, -0.9),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.2,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});