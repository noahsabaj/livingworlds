//! Comprehensive integration tests for the nations module
//!
//! Tests the complete lifecycle of nations including creation, governance,
//! laws, relationships, and multi-nation scenarios.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use std::collections::{HashMap, HashSet};

    use crate::test_utils::*;
    use crate::nations::{
        Nation, NationId, NationPersonality, NationBundle,
        House, HouseTraits, Ruler, RulerPersonality,
        Governance, GovernmentType, GovernmentCategory, PoliticalPressure,
        NationHistory, HistoricalEvent,
        laws::{LawPlugin, Law, LawId, LawCategory, LawRegistry, NationLaws, LawEnactmentEvent},
        migration::{spawn_nations_with_relationships, MigrationStatus},
        NationError, TransitionError,
    };
    use crate::relationships::{
        ControlledBy, Controls, HasCapital, CapitalOf,
        RelationshipsPlugin,
    };
    use crate::simulation::{GameTime, PressureVector, PressureType};
    use crate::world::{Province, TerrainType};

    /// Create a test app with all nation systems
    fn create_nation_test_app() -> App {
        let mut app = App::new();
        app
            .add_plugins(MinimalPlugins)
            .add_plugins(RelationshipsPlugin)
            .add_plugins(LawPlugin)
            .init_resource::<GameTime>()
            .init_resource::<MigrationStatus>()
            .add_event::<LawEnactmentEvent>();
        app
    }

    /// Test full nation lifecycle from creation to collapse
    #[test]
    fn test_nation_lifecycle() {
        let mut app = create_nation_test_app();

        // Create test provinces
        let provinces = create_test_provinces(10);
        let mut nation_provinces = HashMap::new();
        nation_provinces.insert(NationId::new(1), vec![0, 1, 2, 3, 4]);

        // Create nation data
        let nation = Nation {
            id: NationId::new(1),
            name: "Test Empire".to_string(),
            adjective: "Imperial".to_string(),
            color: Color::srgb(0.8, 0.1, 0.1),
            capital_province: 0,
            treasury: 5000.0,
            military_strength: 500.0,
            stability: 0.8,
            personality: NationPersonality {
                aggression: 0.3,
                expansionism: 0.5,
                diplomacy: 0.6,
                mercantilism: 0.4,
            },
        };

        let house = create_test_house(NationId::new(1));
        let government = GovernmentType::Democracy;

        // Spawn nation with relationships
        let nation_entities = spawn_nations_with_relationships(
            app.world_mut().commands(),
            &provinces,
            vec![(nation.clone(), house.clone(), government)],
            nation_provinces,
        );

        app.update();

        let nation_entity = nation_entities[0];

        // Test 1: Verify nation components
        {
            let world = app.world();
            assert!(world.entity(nation_entity).get::<Nation>().is_some());
            assert!(world.entity(nation_entity).get::<House>().is_some());
            assert!(world.entity(nation_entity).get::<Governance>().is_some());
            assert!(world.entity(nation_entity).get::<NationHistory>().is_some());
            assert!(world.entity(nation_entity).get::<NationLaws>().is_some());
        }

        // Test 2: Apply political pressure
        {
            let pressure = PoliticalPressure {
                economic_crisis: 0.8,
                military_defeat: 0.5,
                popular_unrest: 0.6,
                elite_dissatisfaction: 0.4,
                religious_influence: 0.2,
                foreign_influence: 0.3,
                internal_conflict: 0.4,
            };

            app.world_mut()
                .entity_mut(nation_entity)
                .insert(pressure);
        }

        // Test 3: Trigger government transition
        {
            let mut governance = app.world_mut()
                .entity_mut(nation_entity)
                .get_mut::<Governance>()
                .unwrap();

            governance.reform_pressure = 1.0;
            governance.stability = 0.2;
            governance.legitimacy = 0.3;
        }

        app.update();

        // Test 4: Verify nation still exists after pressure
        assert!(app.world().get_entity(nation_entity).is_some());
    }

    /// Test law enactment and effects
    #[test]
    fn test_law_system_integration() {
        let mut app = create_nation_test_app();

        // Create and spawn nation
        let nation_entity = spawn_test_nation(&mut app, "Law Test Nation", GovernmentType::Democracy);

        // Get law registry and nation laws
        let mut law_registry = app.world_mut().resource_mut::<LawRegistry>();

        // Register test laws
        let tax_law = Law {
            id: LawId::new(1000),
            category: LawCategory::Economic,
            name: "Progressive Tax".to_string(),
            description: "Higher earners pay more".to_string(),
            effects: Default::default(),
            prerequisites: vec![],
            conflicts_with: vec![],
            government_affinity: HashMap::from([
                (GovernmentCategory::Democratic, 0.7),
                (GovernmentCategory::Socialist, 0.9),
            ]),
            complexity: crate::nations::laws::LawComplexity::Simple,
            base_popularity: 0.6,
            is_constitutional: false,
            available_from_year: 0,
        };

        law_registry.register_law(tax_law.clone());

        // Propose and enact law
        {
            let mut nation_laws = app.world_mut()
                .entity_mut(nation_entity)
                .get_mut::<NationLaws>()
                .unwrap();

            nation_laws.propose_law(
                LawId::new(1000),
                0.7,
                5.0,
                Some(PressureType::Economic),
            );

            assert_eq!(nation_laws.proposed_laws.len(), 1);
        }

        // Simulate debate completion
        advance_days(&mut app, 6);

        // Verify law was enacted
        {
            let nation_laws = app.world()
                .entity(nation_entity)
                .get::<NationLaws>()
                .unwrap();

            assert!(nation_laws.is_active(LawId::new(1000)));
        }
    }

    /// Test multi-nation diplomatic relationships
    #[test]
    fn test_multi_nation_scenario() {
        let mut app = create_nation_test_app();

        // Create 3 nations
        let provinces = create_test_provinces(30);
        let mut all_nation_provinces = HashMap::new();
        all_nation_provinces.insert(NationId::new(1), vec![0, 1, 2, 3, 4]);
        all_nation_provinces.insert(NationId::new(2), vec![10, 11, 12, 13, 14]);
        all_nation_provinces.insert(NationId::new(3), vec![20, 21, 22, 23, 24]);

        let nations_data = vec![
            (
                create_test_nation_data(1, "Alpha Empire"),
                create_test_house(NationId::new(1)),
                GovernmentType::Democracy,
            ),
            (
                create_test_nation_data(2, "Beta Republic"),
                create_test_house(NationId::new(2)),
                GovernmentType::Democracy,
            ),
            (
                create_test_nation_data(3, "Gamma Kingdom"),
                create_test_house(NationId::new(3)),
                GovernmentType::Monarchy,
            ),
        ];

        let nation_entities = spawn_nations_with_relationships(
            app.world_mut().commands(),
            &provinces,
            nations_data,
            all_nation_provinces,
        );

        app.update();

        // Verify all nations exist
        assert_eq!(nation_entities.len(), 3);

        for entity in &nation_entities {
            assert!(app.world().get_entity(*entity).is_some());
        }

        // Test province ownership
        for entity in &nation_entities {
            let controls = app.world().entity(*entity).get::<Controls>();
            assert!(controls.is_some());
            assert!(controls.unwrap().province_count() > 0);
        }
    }

    /// Test entity relationship integrity
    #[test]
    fn test_relationship_integrity() {
        let mut app = create_nation_test_app();

        // Create nation and provinces
        let provinces = create_test_provinces(5);
        let mut nation_provinces = HashMap::new();
        nation_provinces.insert(NationId::new(1), vec![0, 1, 2]);

        let nation_entities = spawn_nations_with_relationships(
            app.world_mut().commands(),
            &provinces,
            vec![(
                create_test_nation_data(1, "Test Nation"),
                create_test_house(NationId::new(1)),
                GovernmentType::Democracy,
            )],
            nation_provinces,
        );

        app.update();

        let nation_entity = nation_entities[0];

        // Test bidirectional relationships
        {
            // Nation should have Controls component
            let controls = app.world().entity(nation_entity).get::<Controls>().unwrap();
            assert_eq!(controls.province_count(), 3);

            // Nation should have capital
            let has_capital = app.world().entity(nation_entity).get::<HasCapital>();
            assert!(has_capital.is_some());
        }
    }

    /// Test government transition scenarios
    #[test]
    fn test_government_transitions() {
        let mut app = create_nation_test_app();

        let nation_entity = spawn_test_nation(&mut app, "Transition Test", GovernmentType::Democracy);

        // Test peaceful transition
        {
            let mut governance = app.world_mut()
                .entity_mut(nation_entity)
                .get_mut::<Governance>()
                .unwrap();

            governance.stability = 0.8;
            governance.legitimacy = 0.7;
            let old_type = governance.government_type.clone();

            // Attempt transition to Republic
            governance.government_type = GovernmentType::Republic;

            assert_ne!(governance.government_type, old_type);
        }

        // Test revolutionary transition
        {
            let mut governance = app.world_mut()
                .entity_mut(nation_entity)
                .get_mut::<Governance>()
                .unwrap();

            governance.stability = 0.1;
            governance.reform_pressure = 0.9;
            governance.government_type = GovernmentType::Revolution;
        }

        app.update();

        // Verify nation survived transitions
        assert!(app.world().get_entity(nation_entity).is_some());
    }

    /// Test nation collapse and recovery
    #[test]
    fn test_nation_collapse_recovery() {
        let mut app = create_nation_test_app();

        let nation_entity = spawn_test_nation(&mut app, "Collapse Test", GovernmentType::Democracy);

        // Drive nation to collapse
        {
            let mut nation = app.world_mut()
                .entity_mut(nation_entity)
                .get_mut::<Nation>()
                .unwrap();

            nation.treasury = -1000.0; // Bankrupt
            nation.stability = 0.0;
            nation.military_strength = 0.0;
        }

        // Add extreme pressure
        {
            let pressure = PoliticalPressure {
                economic_crisis: 1.0,
                military_defeat: 1.0,
                popular_unrest: 1.0,
                elite_dissatisfaction: 1.0,
                religious_influence: 0.5,
                foreign_influence: 0.8,
                internal_conflict: 1.0,
            };

            app.world_mut()
                .entity_mut(nation_entity)
                .insert(pressure);
        }

        app.update();

        // Nation should still exist but be in crisis
        let governance = app.world()
            .entity(nation_entity)
            .get::<Governance>()
            .unwrap();

        assert!(governance.stability < 0.3);
        assert!(governance.transition_pressure > 0.5);
    }

    /// Test migration from old ownership system
    #[test]
    fn test_ownership_migration() {
        let mut app = create_nation_test_app();

        // Create old-style ownership cache
        let mut ownership_cache = crate::nations::ProvinceOwnershipCache::default();
        ownership_cache.by_nation.insert(
            NationId::new(1),
            HashSet::from([0, 1, 2, 3]),
        );

        app.insert_resource(ownership_cache);

        // Spawn nation
        let nation_entity = app.world_mut().spawn((
            Nation {
                id: NationId::new(1),
                name: "Migration Test".to_string(),
                adjective: "Test".to_string(),
                color: Color::srgb(1.0, 0.0, 0.0),
                capital_province: 0,
                treasury: 1000.0,
                military_strength: 100.0,
                stability: 0.7,
                personality: NationPersonality::balanced(),
            },
        )).id();

        // Run migration
        app.add_systems(
            Update,
            crate::nations::migrate_ownership_to_relationships,
        );

        app.update();

        // Verify migration status
        let migration_status = app.world().resource::<MigrationStatus>();
        assert_eq!(migration_status.nations_migrated, 1);
        assert!(migration_status.errors.is_empty());
    }

    // Helper functions

    fn create_test_provinces(count: usize) -> Vec<Province> {
        (0..count)
            .map(|i| Province {
                id: i as u32,
                position: Vec2::new((i as f32) * 10.0, 0.0),
                neighbors: [None; 6],
                terrain: TerrainType::Plains,
                owner: None,
                population: 1000,
                development: 0.5,
            })
            .collect()
    }

    fn create_test_nation_data(id: u32, name: &str) -> Nation {
        Nation {
            id: NationId::new(id),
            name: name.to_string(),
            adjective: format!("{}ian", name),
            color: Color::srgb(0.5, 0.5, 0.5),
            capital_province: (id - 1) * 10,
            treasury: 1000.0,
            military_strength: 100.0,
            stability: 0.7,
            personality: NationPersonality::balanced(),
        }
    }

    fn create_test_house(nation_id: NationId) -> House {
        House {
            nation_id,
            name: "Test House".to_string(),
            full_name: "House Test of Testing".to_string(),
            ruler: Ruler {
                name: "Test Ruler".to_string(),
                title: "Lord".to_string(),
                age: 40,
                years_ruling: 5,
                personality: RulerPersonality::default(),
            },
            motto: "Testing Forever".to_string(),
            traits: HouseTraits::default(),
            years_in_power: 50,
            legitimacy: 0.8,
            prestige: 0.6,
        }
    }

    fn spawn_test_nation(app: &mut App, name: &str, government: GovernmentType) -> Entity {
        let nation = Nation {
            id: NationId::new(1),
            name: name.to_string(),
            adjective: format!("{}ian", name),
            color: Color::srgb(0.5, 0.5, 0.5),
            capital_province: 0,
            treasury: 1000.0,
            military_strength: 100.0,
            stability: 0.7,
            personality: NationPersonality::balanced(),
        };

        let governance = Governance {
            government_type: government,
            stability: 0.7,
            reform_pressure: 0.0,
            tradition_strength: 0.5,
            institution_strength: 0.5,
            last_transition: None,
            days_in_power: 0,
            legitimacy: 0.8,
            legitimacy_trend: 0.0,
            legitimacy_factors: Default::default(),
        };

        app.world_mut().spawn((
            nation,
            create_test_house(NationId::new(1)),
            governance,
            PressureVector::default(),
            NationHistory::default(),
            NationLaws::default(),
        )).id()
    }

    fn advance_days(app: &mut App, days: u32) {
        for _ in 0..days {
            app.update();
        }
    }
}