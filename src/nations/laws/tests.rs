//! Integration tests for the law system
//!
//! Tests the complete flow of law proposals, debates, voting, and enactment
//! to ensure all systems work together correctly.

#[cfg(test)]
mod integration_tests {
    use bevy::prelude::*;
    use crate::test_utils::*;
    use crate::nations::laws::{
        plugin::LawPlugin,
        types::{LawId, LawEnactmentEvent, LawRepealEvent},
        registry::{NationLaws, LawRegistry},
        systems::*,
    };
    use crate::nations::governance::types::GovernmentType;
    use crate::simulation::PressureType;

    #[test]
    fn test_law_proposal_and_enactment() {
        // Create test app with law systems
        let mut app = create_law_test_app();

        // Add the law systems we want to test
        app.add_systems(Update, (
            propose_laws_system,
            update_law_debates_system,
            process_law_votes_system,
        ).chain());

        // Create a test nation
        let nation = spawn_test_nation(&mut app, "Test Nation", GovernmentType::Democracy);

        // Get the nation's laws component
        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        // Propose a law
        nation_laws.propose_law(LawId::MinimalTaxation, 0.7, 5.0, Some(PressureType::Economic));

        // Verify the law is proposed
        assert_eq!(nation_laws.proposed_laws.len(), 1);
        assert_eq!(nation_laws.proposed_laws[0].law_id, LawId::MinimalTaxation);
        assert_eq!(nation_laws.proposed_laws[0].current_support, 0.7);

        // Advance time to let the debate finish
        advance_days(&mut app, 6);

        // Check that the law was enacted (support was > 0.5)
        let nation_laws = app.world()
            .entity(nation)
            .get::<NationLaws>()
            .unwrap();

        assert!(nation_laws.is_active(LawId::MinimalTaxation));
        assert_eq!(nation_laws.proposed_laws.len(), 0); // Proposal should be cleared
    }

    #[test]
    fn test_law_conflict_prevention() {
        let mut app = create_law_test_app();

        // Register conflicting laws in the registry
        let mut registry = app.world_mut().resource_mut::<LawRegistry>();
        let slavery = LawId::Slavery;
        let freedom = LawId::PersonalFreedom;

        // Create test laws with conflicts
        let slavery_law = crate::nations::laws::types::Law {
            id: slavery,
            name: "Slavery".to_string(),
            description: "Allows slavery".to_string(),
            category: crate::nations::laws::types::LawCategory::Social,
            effects: crate::nations::laws::types::LawEffects {
                allows_slavery: Some(true),
                ..Default::default()
            },
            conflicts_with: vec![freedom],
            requires: vec![],
            complexity: crate::nations::laws::types::LawComplexity::Simple,
            popularity_weights: Default::default(),
        };

        let freedom_law = crate::nations::laws::types::Law {
            id: freedom,
            name: "Personal Freedom".to_string(),
            description: "Guarantees personal freedom".to_string(),
            category: crate::nations::laws::types::LawCategory::Social,
            effects: crate::nations::laws::types::LawEffects {
                allows_slavery: Some(false),
                ..Default::default()
            },
            conflicts_with: vec![slavery],
            requires: vec![],
            complexity: crate::nations::laws::types::LawComplexity::Simple,
            popularity_weights: Default::default(),
        };

        registry.register_law(slavery, slavery_law);
        registry.register_law(freedom, freedom_law);

        // Create nation and enact slavery
        let nation = spawn_test_nation(&mut app, "Slave State", GovernmentType::Monarchy);

        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        nation_laws.enact_law(slavery, &Default::default(), 1000);
        assert!(nation_laws.is_active(slavery));

        // Try to propose conflicting freedom law - should fail
        nation_laws.propose_law(freedom, 0.8, 5.0, None);

        // The conflicting law should not be proposed
        assert_eq!(nation_laws.proposed_laws.len(), 0);
    }

    #[test]
    fn test_law_repeal() {
        let mut app = create_law_test_app();

        // Create nation and enact a law
        let nation = spawn_test_nation(&mut app, "Reform Nation", GovernmentType::Republic);

        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        let tax_effects = crate::nations::laws::types::LawEffects {
            tax_efficiency_modifier: 0.2,
            ..Default::default()
        };

        // Enact the law
        nation_laws.enact_law(LawId::HeavyTaxation, &tax_effects, 1000);
        assert!(nation_laws.is_active(LawId::HeavyTaxation));
        assert_eq!(nation_laws.combined_effects.tax_efficiency_modifier, 0.2);

        // Repeal the law
        nation_laws.repeal_law(LawId::HeavyTaxation, 1001);
        assert!(!nation_laws.is_active(LawId::HeavyTaxation));
        assert_eq!(nation_laws.combined_effects.tax_efficiency_modifier, 0.0);
    }

    #[test]
    fn test_multiple_law_effects_stack() {
        let mut app = create_law_test_app();
        let nation = spawn_test_nation(&mut app, "Complex Nation", GovernmentType::Democracy);

        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        // Enact first tax law
        let tax1_effects = crate::nations::laws::types::LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        };
        nation_laws.enact_law(LawId::MinimalTaxation, &tax1_effects, 1000);

        // Enact second tax law - should have diminishing returns
        let tax2_effects = crate::nations::laws::types::LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        };
        nation_laws.enact_law(LawId::ModerateTaxation, &tax2_effects, 1000);

        // Check diminishing returns applied
        // First law: 100% = 0.1
        // Second law: 75% = 0.075
        // Total: 0.175
        assert_eq!(nation_laws.combined_effects.tax_efficiency_modifier, 0.175);
    }

    #[test]
    fn test_law_cooldowns() {
        let mut app = create_law_test_app();
        let nation = spawn_test_nation(&mut app, "Cooldown Nation", GovernmentType::Republic);

        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        // Propose a law
        nation_laws.propose_law(LawId::MinimalTaxation, 0.3, 5.0, None); // Low support, will fail

        // Advance time to let it fail
        advance_days(&mut app, 6);

        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        // Law should have failed and be on cooldown
        assert!(!nation_laws.is_active(LawId::MinimalTaxation));

        // Set a cooldown manually (normally done by the voting system)
        nation_laws.proposal_cooldowns.insert(LawId::MinimalTaxation, 30.0);

        // Try to propose again - should fail due to cooldown
        nation_laws.propose_law(LawId::MinimalTaxation, 0.8, 5.0, None);
        assert_eq!(nation_laws.proposed_laws.len(), 0);

        // Update cooldowns
        nation_laws.update_cooldowns(31.0); // Cooldown expires

        // Now should be able to propose
        nation_laws.propose_law(LawId::MinimalTaxation, 0.8, 5.0, None);
        assert_eq!(nation_laws.proposed_laws.len(), 1);
    }

    #[test]
    fn test_law_history_tracking() {
        let mut app = create_law_test_app();
        let nation = spawn_test_nation(&mut app, "Historical Nation", GovernmentType::Monarchy);

        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        // Enact and repeal laws to build history
        nation_laws.enact_law(LawId::MinimalTaxation, &Default::default(), 1000);
        nation_laws.enact_law(LawId::UniversalHealthcare, &Default::default(), 1001);
        nation_laws.repeal_law(LawId::MinimalTaxation, 1002);

        // Check history
        assert_eq!(nation_laws.history.len(), 3);
        assert_eq!(nation_laws.history[0].law_id, LawId::MinimalTaxation);
        assert_eq!(nation_laws.history[0].change_type,
                   crate::nations::laws::registry::LawChangeType::Enacted);
        assert_eq!(nation_laws.history[2].change_type,
                   crate::nations::laws::registry::LawChangeType::Repealed);
    }

    #[test]
    fn test_law_data_consistency() {
        let mut app = create_law_test_app();
        let nation = spawn_test_nation(&mut app, "Consistent Nation", GovernmentType::Democracy);

        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        // Enact several laws
        nation_laws.enact_law(LawId::MinimalTaxation, &Default::default(), 1000);
        nation_laws.enact_law(LawId::ModerateTaxation, &Default::default(), 1001);
        nation_laws.enact_law(LawId::UniversalHealthcare, &Default::default(), 1002);

        // Validate consistency
        let validation_result = nation_laws.validate_consistency();
        assert!(validation_result.is_ok(), "Law data should be consistent: {:?}", validation_result);

        // Check that active_laws and active_law_data are in sync
        assert_eq!(nation_laws.active_laws.len(), nation_laws.active_law_data.len());
        for &law_id in &nation_laws.active_laws {
            assert!(nation_laws.active_law_data.contains_key(&law_id));
        }
    }

    #[test]
    fn test_recalculate_combined_effects() {
        let mut app = create_law_test_app();
        let nation = spawn_test_nation(&mut app, "Recalc Nation", GovernmentType::Republic);

        let mut nation_laws = app.world_mut()
            .entity_mut(nation)
            .get_mut::<NationLaws>()
            .unwrap();

        // Enact laws with specific effects
        let tax_effects = crate::nations::laws::types::LawEffects {
            tax_efficiency_modifier: 0.15,
            stability_change: 0.1,
            ..Default::default()
        };
        nation_laws.enact_law(LawId::ModerateTaxation, &tax_effects, 1000);

        let health_effects = crate::nations::laws::types::LawEffects {
            happiness_modifier: 0.2,
            stability_change: 0.15,
            ..Default::default()
        };
        nation_laws.enact_law(LawId::UniversalHealthcare, &health_effects, 1001);

        // Manually corrupt the combined effects
        nation_laws.combined_effects.tax_efficiency_modifier = 999.0;

        // Get registry for recalculation
        let registry = app.world().resource::<LawRegistry>();

        // Recalculate should fix the corruption
        nation_laws.recalculate_combined_effects(&registry);

        // Effects should be restored correctly
        assert_eq!(nation_laws.combined_effects.tax_efficiency_modifier, 0.15);
        assert_eq!(nation_laws.combined_effects.happiness_modifier, 0.2);
        assert_eq!(nation_laws.combined_effects.stability_change, 0.25); // Direct addition
    }
}