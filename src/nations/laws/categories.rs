//! Law category definitions and specific laws
//!
//! This module contains all the specific laws in the game, organized by category.
//! Each law has unique effects and prerequisites that shape nation behavior.

use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::types::*;
use crate::nations::GovernmentCategory;
use crate::simulation::PressureType;

// ================================
// ECONOMIC LAWS (IDs 1000-1024)
// ================================

pub static ECONOMIC_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        // Tax System Laws (mutually exclusive)
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

        // Trade Laws
        Law {
            id: LawId::new(1003),
            category: LawCategory::Economic,
            name: "Free Trade".to_string(),
            description: "Minimal restrictions on international commerce".to_string(),
            effects: LawEffects {
                trade_income_modifier: 0.3,
                industrial_output_modifier: -0.1,
                wealth_inequality_change: 0.1,
                diplomatic_reputation_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1004), LawId::new(1005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.9),
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Autocratic, -0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1004),
            category: LawCategory::Economic,
            name: "Protective Tariffs".to_string(),
            description: "High taxes on imports to protect domestic industry".to_string(),
            effects: LawEffects {
                trade_income_modifier: -0.2,
                industrial_output_modifier: 0.2,
                diplomatic_reputation_change: -0.1,
                pressure_modifiers: HashMap::from([
                    (PressureType::EconomicStrain, -0.1),
                ]),
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1003), LawId::new(1005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.6),
                (GovernmentCategory::Socialist, 0.4),
                (GovernmentCategory::Corporate, -0.5),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1005),
            category: LawCategory::Economic,
            name: "Trade Embargo".to_string(),
            description: "Complete prohibition on trade with certain nations".to_string(),
            effects: LawEffects {
                trade_income_modifier: -0.4,
                diplomatic_reputation_change: -0.2,
                stability_change: 0.1,
                pressure_modifiers: HashMap::from([
                    (PressureType::MilitaryVulnerability, -0.1),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::MinimumStability(0.4)],
            conflicts_with: vec![LawId::new(1003), LawId::new(1004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.7),
                (GovernmentCategory::Theocratic, 0.5),
                (GovernmentCategory::Democratic, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },

        // Labor Laws
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

        // Currency Laws
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

        // Market Regulation
        Law {
            id: LawId::new(1013),
            category: LawCategory::Economic,
            name: "Laissez-Faire".to_string(),
            description: "Minimal government intervention in markets".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.25,
                trade_income_modifier: 0.2,
                wealth_inequality_change: 0.3,
                corruption_change: 0.15,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1014), LawId::new(1015)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 1.0),
                (GovernmentCategory::Democratic, 0.3),
                (GovernmentCategory::Socialist, -0.9),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1014),
            category: LawCategory::Economic,
            name: "Mixed Economy".to_string(),
            description: "Balance of private enterprise and government regulation".to_string(),
            effects: LawEffects {
                industrial_output_modifier: 0.1,
                stability_change: 0.1,
                wealth_inequality_change: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1013), LawId::new(1015)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Socialist, 0.3),
                (GovernmentCategory::Corporate, 0.2),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1015),
            category: LawCategory::Economic,
            name: "Planned Economy".to_string(),
            description: "Government controls all production and distribution".to_string(),
            effects: LawEffects {
                industrial_output_modifier: -0.15,
                agricultural_output_modifier: 0.1,
                wealth_inequality_change: -0.4,
                stability_change: 0.2,
                trade_income_modifier: -0.3,
                pressure_modifiers: HashMap::from([
                    (PressureType::EconomicStrain, 0.1),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Socialist)],
            conflicts_with: vec![LawId::new(1013), LawId::new(1014)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 1.0),
                (GovernmentCategory::Autocratic, 0.4),
                (GovernmentCategory::Corporate, -1.0),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.3,
            is_constitutional: true,
            available_from_year: 100,
        },

        // Banking and Finance
        Law {
            id: LawId::new(1016),
            category: LawCategory::Economic,
            name: "Central Banking".to_string(),
            description: "Government-controlled monetary policy".to_string(),
            effects: LawEffects {
                stability_change: 0.2,
                trade_income_modifier: 0.1,
                corruption_change: 0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(1017)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Corporate, 0.4),
                (GovernmentCategory::Anarchist, -0.7),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(1017),
            category: LawCategory::Economic,
            name: "Free Banking".to_string(),
            description: "Unregulated private banking system".to_string(),
            effects: LawEffects {
                trade_income_modifier: 0.25,
                stability_change: -0.15,
                wealth_inequality_change: 0.2,
                corruption_change: 0.2,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1016)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.8),
                (GovernmentCategory::Anarchist, 0.5),
                (GovernmentCategory::Socialist, -0.8),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },

        // Welfare Laws
        Law {
            id: LawId::new(1018),
            category: LawCategory::Economic,
            name: "Basic Income".to_string(),
            description: "All citizens receive unconditional payment".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.3,
                tax_efficiency_modifier: -0.2,
                wealth_inequality_change: -0.3,
                population_growth_modifier: 0.1,
                pressure_modifiers: HashMap::from([
                    (PressureType::PopulationOvercrowding, -0.2),
                    (PressureType::EconomicStrain, 0.15),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(4)],
            conflicts_with: vec![LawId::new(1019)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 0.7),
                (GovernmentCategory::Democratic, 0.4),
                (GovernmentCategory::Corporate, -0.6),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 300,
        },
        Law {
            id: LawId::new(1019),
            category: LawCategory::Economic,
            name: "No Social Safety Net".to_string(),
            description: "Citizens entirely responsible for their welfare".to_string(),
            effects: LawEffects {
                tax_efficiency_modifier: 0.2,
                happiness_modifier: -0.2,
                wealth_inequality_change: 0.35,
                population_growth_modifier: -0.05,
                pressure_modifiers: HashMap::from([
                    (PressureType::PopulationOvercrowding, 0.3),
                ]),
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(1018), LawId::new(1020)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Corporate, 0.7),
                (GovernmentCategory::Anarchist, 0.4),
                (GovernmentCategory::Socialist, -1.0),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.2,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(1020),
            category: LawCategory::Economic,
            name: "Unemployment Benefits".to_string(),
            description: "Temporary support for those without work".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.15,
                stability_change: 0.1,
                tax_efficiency_modifier: -0.1,
                wealth_inequality_change: -0.1,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(1019)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Socialist, 0.9),
                (GovernmentCategory::Corporate, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.65,
            is_constitutional: false,
            available_from_year: 100,
        },
    ]
});

// ================================
// MILITARY LAWS (IDs 2000-2019)
// ================================

pub static MILITARY_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        // Conscription Laws (mutually exclusive)
        Law {
            id: LawId::new(2000),
            category: LawCategory::Military,
            name: "Volunteer Army".to_string(),
            description: "Military service is entirely voluntary".to_string(),
            effects: LawEffects {
                army_morale_modifier: 0.2,
                mobilization_speed_modifier: -0.3,
                happiness_modifier: 0.1,
                tax_efficiency_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2001), LawId::new(2002), LawId::new(2003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Anarchist, 0.6),
                (GovernmentCategory::Autocratic, -0.4),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.8,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(2001),
            category: LawCategory::Military,
            name: "Limited Conscription".to_string(),
            description: "Selective military draft during wartime".to_string(),
            effects: LawEffects {
                mobilization_speed_modifier: 0.2,
                army_morale_modifier: -0.1,
                happiness_modifier: -0.05,
                defensive_bonus_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2000), LawId::new(2002), LawId::new(2003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.4),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Corporate, 0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(2002),
            category: LawCategory::Military,
            name: "Universal Conscription".to_string(),
            description: "All citizens must serve in the military".to_string(),
            effects: LawEffects {
                mobilization_speed_modifier: 0.4,
                army_morale_modifier: -0.15,
                happiness_modifier: -0.15,
                defensive_bonus_modifier: 0.2,
                population_growth_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2000), LawId::new(2001), LawId::new(2003)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.8),
                (GovernmentCategory::Socialist, 0.4),
                (GovernmentCategory::Democratic, -0.3),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(2003),
            category: LawCategory::Military,
            name: "Pacifist Constitution".to_string(),
            description: "Military force prohibited except for defense".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.2,
                stability_change: 0.2,
                expansion_desire_modifier: -0.8,
                defensive_bonus_modifier: -0.2,
                diplomatic_reputation_change: 0.3,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::MinimumStability(0.6)],
            conflicts_with: vec![LawId::new(2000), LawId::new(2001), LawId::new(2002)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Anarchist, 0.7),
                (GovernmentCategory::Autocratic, -0.9),
            ]),
            complexity: LawComplexity::Revolutionary,
            base_popularity: 0.6,
            is_constitutional: true,
            available_from_year: 200,
        },

        // Military Organization
        Law {
            id: LawId::new(2004),
            category: LawCategory::Military,
            name: "Professional Officer Corps".to_string(),
            description: "Military leadership requires formal training".to_string(),
            effects: LawEffects {
                army_morale_modifier: 0.15,
                naval_tradition_modifier: 0.1,
                tax_efficiency_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(2005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Monarchic, 0.7),
                (GovernmentCategory::Democratic, 0.5),
                (GovernmentCategory::Tribal, -0.4),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.5,
            is_constitutional: false,
            available_from_year: 100,
        },
        Law {
            id: LawId::new(2005),
            category: LawCategory::Military,
            name: "Elected Officers".to_string(),
            description: "Military units elect their own commanders".to_string(),
            effects: LawEffects {
                army_morale_modifier: 0.25,
                mobilization_speed_modifier: -0.2,
                stability_change: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2004)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Anarchist, 0.9),
                (GovernmentCategory::Democratic, 0.3),
                (GovernmentCategory::Autocratic, -0.8),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },

        // War Conduct
        Law {
            id: LawId::new(2006),
            category: LawCategory::Military,
            name: "Total War Doctrine".to_string(),
            description: "All resources devoted to warfare when at war".to_string(),
            effects: LawEffects {
                mobilization_speed_modifier: 0.5,
                industrial_output_modifier: 0.3,
                expansion_desire_modifier: 0.3,
                happiness_modifier: -0.2,
                diplomatic_reputation_change: -0.2,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Autocratic)],
            conflicts_with: vec![LawId::new(2007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.9),
                (GovernmentCategory::Democratic, -0.6),
                (GovernmentCategory::Anarchist, -0.8),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.3,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(2007),
            category: LawCategory::Military,
            name: "Laws of War".to_string(),
            description: "Military follows international humanitarian rules".to_string(),
            effects: LawEffects {
                diplomatic_reputation_change: 0.25,
                army_morale_modifier: -0.05,
                stability_change: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2006)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Theocratic, 0.4),
                (GovernmentCategory::Autocratic, -0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 100,
        },

        // Military Technology
        Law {
            id: LawId::new(2008),
            category: LawCategory::Military,
            name: "Chemical Weapons Ban".to_string(),
            description: "Prohibits development and use of chemical weapons".to_string(),
            effects: LawEffects {
                diplomatic_reputation_change: 0.2,
                expansion_desire_modifier: -0.1,
                technology_rate_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(3)],
            conflicts_with: vec![LawId::new(2009)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Theocratic, 0.5),
                (GovernmentCategory::Autocratic, -0.4),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 250,
        },
        Law {
            id: LawId::new(2009),
            category: LawCategory::Military,
            name: "Unrestricted Weapons Development".to_string(),
            description: "No limits on military technology research".to_string(),
            effects: LawEffects {
                technology_rate_modifier: 0.15,
                diplomatic_reputation_change: -0.15,
                expansion_desire_modifier: 0.2,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(2008)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.7),
                (GovernmentCategory::Technocratic, 0.6),
                (GovernmentCategory::Democratic, -0.3),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },

        // Intelligence and Espionage
        Law {
            id: LawId::new(2010),
            category: LawCategory::Military,
            name: "Secret Police".to_string(),
            description: "Internal security forces monitor citizens".to_string(),
            effects: LawEffects {
                stability_change: 0.3,
                happiness_modifier: -0.25,
                corruption_change: 0.2,
                pressure_modifiers: HashMap::from([
                    (PressureType::LegitimacyCrisis, -0.2),
                ]),
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::GovernmentCategory(GovernmentCategory::Autocratic)],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Autocratic, 0.9),
                (GovernmentCategory::Democratic, -0.9),
                (GovernmentCategory::Anarchist, -1.0),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.1,
            is_constitutional: false,
            available_from_year: 0,
        },
    ]
});

// ================================
// SOCIAL LAWS (IDs 3000-3024)
// ================================

pub static SOCIAL_LAWS: Lazy<Vec<Law>> = Lazy::new(|| {
    vec![
        // Healthcare Laws
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

        // Education Laws
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

        // Gender Equality Laws
        Law {
            id: LawId::new(3005),
            category: LawCategory::Social,
            name: "Gender Equality".to_string(),
            description: "Equal rights and opportunities regardless of gender".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.15,
                industrial_output_modifier: 0.1,
                technology_rate_modifier: 0.1,
                population_growth_modifier: -0.05,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![LawId::new(3006)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.8),
                (GovernmentCategory::Socialist, 0.9),
                (GovernmentCategory::Theocratic, -0.5),
            ]),
            complexity: LawComplexity::Complex,
            base_popularity: 0.5,
            is_constitutional: true,
            available_from_year: 200,
        },
        Law {
            id: LawId::new(3006),
            category: LawCategory::Social,
            name: "Traditional Gender Roles".to_string(),
            description: "Strict separation of gender responsibilities".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                population_growth_modifier: 0.1,
                industrial_output_modifier: -0.15,
                happiness_modifier: -0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3005)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Theocratic, 0.7),
                (GovernmentCategory::Tribal, 0.6),
                (GovernmentCategory::Democratic, -0.5),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },

        // Marriage Laws
        Law {
            id: LawId::new(3007),
            category: LawCategory::Social,
            name: "Civil Marriage".to_string(),
            description: "Government recognizes and regulates marriage".to_string(),
            effects: LawEffects {
                stability_change: 0.1,
                happiness_modifier: 0.05,
                population_growth_modifier: 0.05,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3008)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Monarchic, 0.5),
                (GovernmentCategory::Anarchist, -0.3),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        },
        Law {
            id: LawId::new(3008),
            category: LawCategory::Social,
            name: "Free Union".to_string(),
            description: "No government involvement in personal relationships".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.1,
                stability_change: -0.05,
                cultural_conversion_modifier: 0.1,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![LawId::new(3007)],
            government_affinity: HashMap::from([
                (GovernmentCategory::Anarchist, 0.9),
                (GovernmentCategory::Democratic, 0.3),
                (GovernmentCategory::Theocratic, -0.7),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.4,
            is_constitutional: false,
            available_from_year: 0,
        },

        // Welfare Programs
        Law {
            id: LawId::new(3009),
            category: LawCategory::Social,
            name: "Old Age Pension".to_string(),
            description: "Government support for elderly citizens".to_string(),
            effects: LawEffects {
                happiness_modifier: 0.2,
                stability_change: 0.15,
                tax_efficiency_modifier: -0.1,
                wealth_inequality_change: -0.15,
                ..Default::default()
            },
            prerequisites: vec![LawPrerequisite::TechnologyLevel(2)],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 0.9),
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Corporate, -0.5),
            ]),
            complexity: LawComplexity::Moderate,
            base_popularity: 0.8,
            is_constitutional: false,
            available_from_year: 150,
        },
        Law {
            id: LawId::new(3010),
            category: LawCategory::Social,
            name: "Child Support".to_string(),
            description: "Financial assistance for families with children".to_string(),
            effects: LawEffects {
                population_growth_modifier: 0.2,
                happiness_modifier: 0.15,
                tax_efficiency_modifier: -0.08,
                ..Default::default()
            },
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Socialist, 0.8),
                (GovernmentCategory::Democratic, 0.6),
                (GovernmentCategory::Corporate, -0.3),
            ]),
            complexity: LawComplexity::Simple,
            base_popularity: 0.7,
            is_constitutional: false,
            available_from_year: 100,
        },
    ]
});

// Helper functions to access laws

/// Get all laws across all categories
pub fn get_all_laws() -> Vec<&'static Law> {
    let mut laws = Vec::new();
    laws.extend(ECONOMIC_LAWS.iter());
    laws.extend(MILITARY_LAWS.iter());
    laws.extend(SOCIAL_LAWS.iter());
    // Add other categories as they're implemented
    laws
}

/// Get all laws in a specific category
pub fn get_category_laws(category: LawCategory) -> Vec<&'static Law> {
    match category {
        LawCategory::Economic => ECONOMIC_LAWS.iter().collect(),
        LawCategory::Military => MILITARY_LAWS.iter().collect(),
        LawCategory::Social => SOCIAL_LAWS.iter().collect(),
        // Add other categories as they're implemented
        _ => Vec::new(),
    }
}

/// Get a specific law by ID
pub fn get_law_by_id(id: LawId) -> Option<&'static Law> {
    get_all_laws().into_iter().find(|law| law.id == id)
}

// Type aliases for cleaner code
pub type EconomicLaw = Law;
pub type MilitaryLaw = Law;
pub type SocialLaw = Law;
pub type ReligiousLaw = Law;
pub type CriminalLaw = Law;
pub type PropertyLaw = Law;
pub type ImmigrationLaw = Law;
pub type EnvironmentalLaw = Law;
pub type TechnologyLaw = Law;
pub type CulturalLaw = Law;
pub type AdministrativeLaw = Law;
pub type DiplomaticLaw = Law;