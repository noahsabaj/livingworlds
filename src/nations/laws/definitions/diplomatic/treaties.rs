//! Treaty law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Treaty laws
pub static TREATY_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(12000),
            category: LawCategory::Diplomatic,
            name: "Honor Treaties".to_string(),
            description: "Strict adherence to international agreements".to_string(),
            effects: LawEffects {
                diplomatic_reputation_change: 0.3,
                stability_change: 0.05,
                trade_income_modifier: 0.1,
                military_flexibility_modifier: Some(-0.1),
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(12001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Monarchic, 0.6),
                (GovernmentCategory::Fascist, -0.5),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(12001),
            category: LawCategory::Diplomatic,
            name: "Diplomatic Flexibility".to_string(),
            description: "Treaties subject to reinterpretation".to_string(),
            effects: LawEffects {
                diplomatic_reputation_change: -0.2,
                military_flexibility_modifier: Some(0.2),
                revolt_risk_change: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(12000)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Fascist, 0.7),
                (GovernmentCategory::Authoritarian, 0.5),
                (GovernmentCategory::Democratic, -0.6),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});