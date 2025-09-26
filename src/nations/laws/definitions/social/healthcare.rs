//! Healthcare law definitions
//!
//! Laws governing medical care systems and health policies.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};
use crate::simulation::PressureType;

/// All healthcare-related laws
pub static HEALTHCARE_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(3000),
            category: LawCategory::Social,
            name: "Universal Healthcare".to_string(),
            description: "Free medical care for all citizens".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.25,
                population_growth_modifier: 0.15,
                tax_efficiency_modifier: -0.15,
                wealth_inequality_change: -0.1,
                pressure_modifiers: HashMap::from([
                    (PressureType::PopulationOvercrowding, -0.1),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(3001)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 1.0),
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Corporate, -0.7),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.75,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(3001),
            category: LawCategory::Social,
            name: "Private Healthcare".to_string(),
            description: "Medical care available through market".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: 0.1,
                wealth_inequality_change: 0.2,
                population_growth_modifier: -0.05,
                happiness_modifier: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3000)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.8),
                (GovernmentCategory::Democratic, 0.2),
                (GovernmentCategory::Socialist, -0.9),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});