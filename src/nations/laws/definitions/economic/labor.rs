//! Labor law definitions
//!
//! Laws governing employment, wages, and worker rights.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::nations::governance::GovernmentCategory;
use crate::nations::laws::types::{
    Law, LawId, LawCategory, LawComplexity, LawEffects, LawPrerequisite,
};
use crate::simulation::PressureType;

/// All labor-related laws
pub static LABOR_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        Law {
            id: LawId::new(1006),
            category: LawCategory::Economic,
            name: "Minimum Wage".to_string(),
            description: "Legally mandated minimum hourly compensation".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.15,
                wealth_inequality_change: -0.1,
                industrial_output_modifier: -0.05,
                pressure_modifiers: HashMap::from([
                    (PressureType::PopulationOvercrowding, -0.1),
                ]),
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 0.9),
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Corporate, -0.7),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(1007),
            category: LawCategory::Economic,
            name: "Unrestricted Labor Market".to_string(),
            description: "No government intervention in wage negotiations".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.15,
                wealth_inequality_change: 0.2,
                happiness_modifier: -0.1,
                corruption_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1006), LawId::new(1008)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.9),
                (GovernmentCategory::Anarchist, 0.3),
                (GovernmentCategory::Socialist, -0.9),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1008),
            category: LawCategory::Economic,
            name: "Labor Union Rights".to_string(),
            description: "Workers can organize and collectively bargain".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.2,
                wealth_inequality_change: -0.15,
                stability_change: 0.1,
                industrial_output_modifier: -0.1,
                pressure_modifiers: HashMap::from([
                    (PressureType::PopulationOvercrowding, -0.15),
                ]),
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1007), LawId::new(1009)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 1.0),
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Autocratic, -0.8),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 50,
        },
        Law {
            id: LawId::new(1009),
            category: LawCategory::Economic,
            name: "Labor Union Ban".to_string(),
            description: "Collective bargaining is illegal".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.2,
                stability_change: -0.15,
                happiness_modifier: -0.2,
                wealth_inequality_change: 0.25,
                pressure_modifiers: HashMap::from([
                    (PressureType::PopulationOvercrowding, 0.2),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Autocratic)],
            conflicts_with: vec![LawId::new(1008)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.8),
                (GovernmentCategory::Corporate, 0.6),
                (GovernmentCategory::Socialist, -1.0),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.2,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});