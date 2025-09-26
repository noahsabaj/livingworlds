//! Media control law definitions

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects,
};

/// Media control laws
pub static MEDIA_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(10004),
            category: LawCategory::Cultural,
            name: "Free Press".to_string(),
            description: "Unrestricted media and journalism".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.1,
                corruption_change: -0.15,
                stability_change: -0.05,
                diplomatic_reputation_change: 0.15,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(10005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.9),
                (GovernmentCategory::Authoritarian, -0.8),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.7,
            is_constitutional: true,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(10005),
            category: LawCategory::Cultural,
            name: "State Media".to_string(),
            description: "Government-controlled information".to_string(),
            effects: LawEffects {
                stability_change: 0.15,
                legitimacy_change: 0.1,
                corruption_change: 0.2,
                happiness_modifier: -0.1,
                diplomatic_reputation_change: -0.2,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(10004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Authoritarian, 0.9),
                (GovernmentCategory::Fascist, 1.0),
                (GovernmentCategory::Democratic, -0.9),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.2,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});