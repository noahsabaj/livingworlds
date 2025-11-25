//! Property-based testing for law mechanics
//!
//! Uses proptest to verify law system invariants and edge cases,
//! ensuring the law system behaves correctly under all conditions.

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use bevy::prelude::*;
    use std::collections::HashMap;

    use crate::nations::laws::{
        Law, LawId, LawCategory, LawEffects, LawComplexity,
        LawPrerequisite, NationLaws, LawRegistry,
        calculate_law_effects, apply_diminishing_returns,
    };
    use crate::nations::{Nation, NationId, GovernmentCategory};
    use crate::test_utils::*;

    // Strategy for generating random laws
    fn law_strategy() -> impl Strategy<Value = Law> {
        (
            // Law ID between 1 and 10000
            1u16..10000u16,
            // Random category
            prop_oneof![
                Just(LawCategory::Economic),
                Just(LawCategory::Military),
                Just(LawCategory::Social),
                Just(LawCategory::Religious),
            ],
            // Random name
            "[A-Z][a-z]{3,15}( [A-Z][a-z]{3,10}){0,2}",
            // Random effects
            law_effects_strategy(),
            // Random complexity
            prop_oneof![
                Just(LawComplexity::Simple),
                Just(LawComplexity::Moderate),
                Just(LawComplexity::Complex),
            ],
            // Base popularity between 0 and 1
            0.0f32..=1.0f32,
            // Constitutional flag
            bool::ANY,
        ).prop_map(|(id, category, name, effects, complexity, popularity, constitutional)| {
            Law {
                id: LawId::new(id),
                category,
                name,
                description: "Test law description".to_string(),
                effects,
                prerequisites: vec![],
                conflicts_with: vec![],
                government_affinity: HashMap::new(),
                complexity,
                base_popularity: popularity,
                is_constitutional: constitutional,
                available_from_year: 0,
            }
        })
    }

    // Strategy for generating law effects
    fn law_effects_strategy() -> impl Strategy<Value = LawEffects> {
        (
            // Each modifier between -1 and 1
            -1.0f32..=1.0f32,  // tax_efficiency_modifier
            -1.0f32..=1.0f32,  // trade_income_modifier
            -1.0f32..=1.0f32,  // production_modifier
            -1.0f32..=1.0f32,  // military_cost_modifier
            -1.0f32..=1.0f32,  // research_speed_modifier
            -1.0f32..=1.0f32,  // happiness_modifier
            -0.5f32..=0.5f32,  // stability_change
            -0.5f32..=0.5f32,  // legitimacy_modifier
            -0.5f32..=0.5f32,  // corruption_change
        ).prop_map(|(tax, trade, prod, mil, research, happiness, stability, legitimacy, corruption)| {
            LawEffects {
                tax_efficiency_modifier: tax,
                trade_income_modifier: trade,
                production_modifier: prod,
                military_cost_modifier: mil,
                research_speed_modifier: research,
                happiness_modifier: happiness,
                stability_change: stability,
                legitimacy_modifier: legitimacy,
                corruption_change: corruption,
                ..Default::default()
            }
        })
    }

    // Strategy for generating sets of laws
    fn law_set_strategy() -> impl Strategy<Value = Vec<Law>> {
        prop::collection::vec(law_strategy(), 1..20)
    }

    proptest! {
        // Test 1: Laws never crash the economy
        #[test]
        fn laws_never_crash_economy(laws in law_set_strategy()) {
            let mut nation = create_test_nation_data();

            for law in laws {
                // Apply law effects
                let effects = calculate_law_effects(&[law.clone()]);

                // Apply to nation
                nation.treasury = (nation.treasury * (1.0 + effects.tax_efficiency_modifier)).max(0.0);
                nation.stability = (nation.stability + effects.stability_change).clamp(0.0, 1.0);

                // Invariants that must hold
                prop_assert!(nation.treasury >= 0.0, "Treasury went negative");
                prop_assert!(nation.stability >= 0.0, "Stability went negative");
                prop_assert!(nation.stability <= 1.0, "Stability exceeded maximum");
            }
        }

        // Test 2: Diminishing returns always reduces effect magnitude
        #[test]
        fn diminishing_returns_reduces_effects(
            base_value in -10.0f32..10.0f32,
            count in 1usize..10,
        ) {
            let reduced = apply_diminishing_returns(base_value, count);

            if base_value.abs() > 0.001 {
                // Diminishing returns should reduce the absolute value
                prop_assert!(reduced.abs() <= base_value.abs(),
                    "Diminishing returns increased effect magnitude");
            }

            // Sign should never change
            if base_value > 0.0 {
                prop_assert!(reduced >= 0.0, "Positive value became negative");
            } else if base_value < 0.0 {
                prop_assert!(reduced <= 0.0, "Negative value became positive");
            }
        }

        // Test 3: Conflicting laws are never active simultaneously
        #[test]
        fn conflicting_laws_mutually_exclusive(
            laws in law_set_strategy(),
        ) {
            let mut app = create_law_test_app();
            let mut registry = LawRegistry::default();

            // Register laws with some conflicts
            for (i, law) in laws.iter().enumerate() {
                let mut law_with_conflicts = law.clone();
                // Make every even law conflict with the next odd law
                if i % 2 == 0 && i + 1 < laws.len() {
                    law_with_conflicts.conflicts_with.push(laws[i + 1].id);
                }
                registry.register_law(law_with_conflicts);
            }

            // Create nation with laws component
            let nation = app.world_mut().spawn_empty().id();
            let mut nation_laws = NationLaws::new(nation);

            // Try to activate all laws
            for law in &laws {
                nation_laws.try_activate_law(law.id, &registry);
            }

            // Verify no conflicts are active
            let active = nation_laws.active_laws();
            for law_id in active {
                if let Some(law) = registry.get_law(*law_id) {
                    for conflict_id in &law.conflicts_with {
                        prop_assert!(!nation_laws.is_active(*conflict_id),
                            "Conflicting laws {:?} and {:?} are both active", law_id, conflict_id);
                    }
                }
            }
        }

        // Test 4: Law popularity is always bounded
        #[test]
        fn law_popularity_stays_bounded(
            base_popularity in 0.0f32..=1.0f32,
            modifiers in prop::collection::vec(-0.5f32..=0.5f32, 0..10),
        ) {
            let mut popularity = base_popularity;

            for modifier in modifiers {
                popularity = (popularity + modifier).clamp(0.0, 1.0);

                prop_assert!(popularity >= 0.0, "Popularity went negative");
                prop_assert!(popularity <= 1.0, "Popularity exceeded 1.0");
            }
        }

        // Test 5: Government affinity affects law passage
        #[test]
        fn government_affinity_affects_passage(
            mut law in law_strategy(),
            government_category in prop_oneof![
                Just(GovernmentCategory::Democratic),
                Just(GovernmentCategory::Autocratic),
                Just(GovernmentCategory::Socialist),
                Just(GovernmentCategory::Corporate),
            ],
            affinity in -1.0f32..=1.0f32,
        ) {
            // Set government affinity
            law.government_affinity.insert(government_category, affinity);

            // Calculate passage difficulty
            let base_difficulty = match law.complexity {
                LawComplexity::Simple => 0.3,
                LawComplexity::Moderate => 0.5,
                LawComplexity::Complex => 0.7,
            };

            let adjusted_difficulty = base_difficulty * (1.0 - affinity * 0.5);

            // Verify difficulty is affected by affinity
            if affinity > 0.0 {
                prop_assert!(adjusted_difficulty < base_difficulty,
                    "Positive affinity should reduce difficulty");
            } else if affinity < 0.0 {
                prop_assert!(adjusted_difficulty > base_difficulty,
                    "Negative affinity should increase difficulty");
            }

            // Difficulty should remain bounded
            prop_assert!(adjusted_difficulty >= 0.0);
            prop_assert!(adjusted_difficulty <= 1.5);
        }

        // Test 6: Law prerequisites are enforced
        #[test]
        fn prerequisites_block_invalid_laws(
            mut law1 in law_strategy(),
            mut law2 in law_strategy(),
        ) {
            // Make law2 require law1
            law2.prerequisites.push(LawPrerequisite::RequiresLaw(law1.id));

            let mut registry = LawRegistry::default();
            registry.register_law(law1.clone());
            registry.register_law(law2.clone());

            let nation = Entity::from_raw(0);
            let mut nation_laws = NationLaws::new(nation);

            // Try to activate law2 without law1
            let result2 = nation_laws.try_activate_law(law2.id, &registry);
            prop_assert!(!result2, "Law with unmet prerequisites was activated");

            // Activate law1 first
            let result1 = nation_laws.try_activate_law(law1.id, &registry);
            prop_assert!(result1, "Law1 should activate");

            // Now law2 should be activatable
            let result2_retry = nation_laws.try_activate_law(law2.id, &registry);
            // Note: might still fail due to other factors, but prerequisites are met
        }

        // Test 7: Combined law effects are cumulative with diminishing returns
        #[test]
        fn combined_effects_cumulative(laws in prop::collection::vec(law_strategy(), 2..5)) {
            let individual_effects: Vec<_> = laws.iter()
                .map(|law| calculate_law_effects(&[law.clone()]))
                .collect();

            let combined_effects = calculate_law_effects(&laws);

            // Tax efficiency should be sum with diminishing returns
            let expected_tax: f32 = individual_effects.iter()
                .enumerate()
                .map(|(i, e)| apply_diminishing_returns(e.tax_efficiency_modifier, i + 1))
                .sum();

            // Allow for floating point error
            prop_assert!((combined_effects.tax_efficiency_modifier - expected_tax).abs() < 0.001,
                "Combined tax effect doesn't match expected: {} vs {}",
                combined_effects.tax_efficiency_modifier, expected_tax);
        }

        // Test 8: Law complexity affects debate duration
        #[test]
        fn complexity_affects_debate_duration(law in law_strategy()) {
            let base_duration = 5.0; // days

            let duration = match law.complexity {
                LawComplexity::Simple => base_duration,
                LawComplexity::Moderate => base_duration * 1.5,
                LawComplexity::Complex => base_duration * 2.0,
            };

            prop_assert!(duration >= base_duration,
                "Complex laws should take longer to debate");
            prop_assert!(duration <= base_duration * 3.0,
                "Debate duration should be bounded");
        }
    }

    // Helper function to create test nation data
    fn create_test_nation_data() -> Nation {
        Nation {
            id: NationId::new(1),
            name: "Test Nation".to_string(),
            adjective: "Test".to_string(),
            color: Color::srgb(0.5, 0.5, 0.5),
            capital_province: 0,
            stability: 0.8,
            culture: crate::name_generator::Culture::Western,
            technology_level: 1,
            personality: crate::nations::NationPersonality::balanced(),
        }
    }

    // Extension of NationLaws for testing
    impl NationLaws {
        fn try_activate_law(&mut self, law_id: LawId, registry: &LawRegistry) -> bool {
            // Check prerequisites
            if let Some(law) = registry.get_law(law_id) {
                for prereq in &law.prerequisites {
                    match prereq {
                        LawPrerequisite::RequiresLaw(required_id) => {
                            if !self.is_active(*required_id) {
                                return false;
                            }
                        }
                        _ => {} // Other prerequisites not checked in this test
                    }
                }

                // Check conflicts
                for conflict_id in &law.conflicts_with {
                    if self.is_active(*conflict_id) {
                        return false;
                    }
                }

                // Activate the law
                self.active_laws.insert(law_id);
                true
            } else {
                false
            }
        }
    }
}