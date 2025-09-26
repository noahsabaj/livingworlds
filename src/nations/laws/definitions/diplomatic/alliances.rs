//! Alliance law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Alliance laws
pub static ALLIANCE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(12003),
            category: LawCategory::Diplomatic,
            name: "Defensive Alliances".to_string(),
            description: "Mutual defense pacts with other nations".to_string(),
            effects: LawEffects {
                diplomatic_reputation_change: 0.15,
                military_strength_modifier: 0.1,
                maintenance_cost_modifier: 0.05,
                stability_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(12004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Fascist, -0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(12004),
            category: LawCategory::Diplomatic,
            name: "Isolationism".to_string(),
            description: "Avoid foreign entanglements".to_string(),
            effects: LawEffects {
                stability_change: 0.15,
                maintenance_cost_modifier: -0.1,
                trade_income_modifier: -0.1,
                diplomatic_reputation_change: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(12003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Anarchist, 0.5),
                (GovernmentCategory::Tribal, 0.4),
                (GovernmentCategory::Corporate, -0.6),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});