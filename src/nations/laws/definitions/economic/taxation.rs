//! Taxation law definitions
//!
//! Contains laws related to tax systems, including income tax,
//! progressive taxation, and tax-free systems.

use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::nations::laws::types::{Law, LawId, LawCategory, LawEffects, LawComplexity};
use crate::nations::GovernmentCategory;

/// Tax system laws
pub static TAX_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        // Flat Tax
        Law {
            id: LawId::new(1000),
            category: LawCategory::Economic,
            name: "Flat Tax".to_string(),
            description: "All citizens pay the same percentage of income".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: 0.15,
                wealth_inequality_change: 0.1,
                happiness_modifier: -0.05,
                stability_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1001), LawId::new(1002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.8),
                (GovernmentCategory::Democratic, 0.3),
                (GovernmentCategory::Socialist, -0.8),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
        // Progressive Tax
        Law {
            id: LawId::new(1001),
            category: LawCategory::Economic,
            name: "Progressive Tax".to_string(),
            description: "Higher earners pay a higher percentage of income".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: 0.05,
                wealth_inequality_change: -0.2,
                happiness_modifier: 0.1,
                stability_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1000), LawId::new(1002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Socialist, 0.9),
                (GovernmentCategory::Corporate, -0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        // No Income Tax
        Law {
            id: LawId::new(1002),
            category: LawCategory::Economic,
            name: "No Income Tax".to_string(),
            description: "Government funded entirely through other means".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: -0.3,
                trade_income_modifier: 0.2,
                happiness_modifier: 0.2,
                stability_change: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1000), LawId::new(1001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Anarchist, 0.9),
                (GovernmentCategory::Corporate, 0.4),
                (GovernmentCategory::Socialist, -0.9),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});