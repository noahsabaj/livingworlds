//! Diplomatic trade law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Diplomatic trade laws
pub static TRADE_DIPLOMATIC_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(12002),
            category: LawCategory::Diplomatic,
            name: "Trade Agreements".to_string(),
            description: "Bilateral trade partnerships".to_string(),
            effects: LawEffects {
                trade_income_modifier: 0.2,
                diplomatic_reputation_change: 0.1,
                industrial_output_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.9),
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Anarchist, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 50,
        },
    ]
});