//! Migration from manual ownership tracking to entity relationships
//!
//! This module provides systems to migrate from the old Province.owner field
//! to the new entity relationship system using ControlledBy/Controls components.

use bevy::prelude::*;
use std::collections::HashMap;

use super::{Nation, NationId, ProvinceOwnershipCache,
    Governance, GovernmentType, House,
    NationHistory, NationLaws};
use crate::relationships::{ControlledBy, Controls, HasCapital, CapitalOf};
use crate::world::{Province, ProvinceId, ProvinceEntity, ProvinceStorage};
use crate::simulation::PressureVector;

/// Migration result tracking
#[derive(Resource, Default)]
pub struct MigrationStatus {
    pub provinces_migrated: usize,
    pub nations_migrated: usize,
    pub capitals_set: usize,
    pub errors: Vec<String>,
}

/// Spawn nations using entity relationships instead of manual ownership
///
/// Note: In the mega-mesh architecture, provinces are NOT entities - they're stored in ProvinceStorage.
/// This function creates ProvinceEntity components to link provinces to nation ownership.
pub fn spawn_nations_with_relationships(
    mut commands: Commands,
    provinces: &[Province],
    nations_data: Vec<(Nation, House, GovernmentType)>,
    nation_provinces: HashMap<NationId, Vec<u32>>,
) -> Vec<Entity> {
    let mut nation_entities = Vec::new();
    let mut province_entities: HashMap<u32, Entity> = HashMap::new();

    // Create ProvinceEntity markers for provinces that will be owned
    // These are lightweight markers that link to the actual province data
    for province in provinces {
        let entity = commands.spawn(
            ProvinceEntity {
                storage_index: province.id.value() as usize, // Index into ProvinceStorage
                id: province.id,
            }
        ).id();
        province_entities.insert(province.id.value(), entity);
    }

    // Spawn nations with all their components
    for (nation, house, government_type) in nations_data {
        let nation_id = nation.id;
        let capital_province_id = nation.capital_province;

        // Create governance component
        let governance = Governance {
            government_type,
            stability: nation.stability,
            reform_pressure: 0.0,
            tradition_strength: 0.5,
            institution_strength: 0.5,
            last_transition: None,
            days_in_power: 0,
            legitimacy: house.legitimacy,
            legitimacy_trend: 0.0,
            legitimacy_factors: Default::default(),
        };

        // Spawn the nation entity with all components
        let nation_entity = commands
            .spawn((
                nation.clone(),
                house,
                governance,
                PressureVector::default(),
                NationHistory::default(),
                NationLaws::default(),
                Transform::default(),
                Visibility::default(),
            ))
            .id();

        // Set capital relationship
        // Only set HasCapital - CapitalOf will be automatically created by Bevy relationships
        if let Some(&capital_entity) = province_entities.get(&capital_province_id) {
            commands.entity(nation_entity).insert(HasCapital(capital_entity));
            // CapitalOf is automatically added to capital_entity by Bevy's relationship system
        }

        // Set province ownership relationships
        if let Some(owned_provinces) = nation_provinces.get(&nation_id) {
            for &province_id in owned_provinces {
                if let Some(&province_entity) = province_entities.get(&province_id) {
                    // Add ControlledBy to province
                    commands.entity(province_entity).insert(ControlledBy(nation_entity));
                    // Note: Controls component on nation will be automatically created
                    // by the relationship maintenance systems
                }
            }
        }

        nation_entities.push(nation_entity);
    }

    info!(
        "Spawned {} nations with entity relationships",
        nation_entities.len()
    );

    nation_entities
}

/// Migrate existing province ownership to entity relationships
pub fn migrate_ownership_to_relationships(
    mut commands: Commands,
    provinces: Res<ProvinceStorage>,
    cache: Res<ProvinceOwnershipCache>,
    nations_query: Query<(Entity, &Nation)>,
    existing_province_entities: Query<(Entity, &ProvinceEntity)>,
    mut migration_status: ResMut<MigrationStatus>,
) {
    info!("Starting migration from ProvinceOwnershipCache to entity relationships");

    // Create mapping from NationId to Entity
    let nation_id_to_entity: HashMap<NationId, Entity> = nations_query
        .iter()
        .map(|(entity, nation)| (nation.id, entity))
        .collect();

    // Create mapping from province ID to Entity
    // First check for existing ProvinceEntity markers
    let mut province_id_to_entity: HashMap<u32, Entity> = HashMap::new();
    for (entity, province_entity) in existing_province_entities.iter() {
        province_id_to_entity.insert(province_entity.id.value(), entity);
    }

    // For each nation's provinces in the cache
    for (nation_id, province_ids) in &cache.by_nation {
        if let Some(&nation_entity) = nation_id_to_entity.get(nation_id) {
            // Process each province
            for &province_id in province_ids {
                // Find or create province entity marker
                let province_entity = if let Some(&entity) = province_id_to_entity.get(&province_id) {
                    entity
                } else {
                    // Create a new ProvinceEntity marker for this province
                    let entity = commands.spawn(
                        ProvinceEntity {
                            storage_index: province_id as usize,
                            id: ProvinceId::new(province_id),
                        }
                    ).id();
                    province_id_to_entity.insert(province_id, entity);
                    entity
                };

                // Add ownership relationship
                commands.entity(province_entity).insert(ControlledBy(nation_entity));
                migration_status.provinces_migrated += 1;
            }

            migration_status.nations_migrated += 1;
        } else {
            let error = format!("Nation {:?} not found during migration", nation_id);
            error!("{}", error);
            migration_status.errors.push(error);
        }
    }

    // Set capital relationships
    for (nation_entity, nation) in nations_query.iter() {
        let capital_id = nation.capital_province;

        if let Some(&capital_entity) = province_id_to_entity.get(&capital_id) {
            commands.entity(nation_entity).insert(HasCapital(capital_entity));
            // CapitalOf is automatically added to capital_entity by Bevy's relationship system
            migration_status.capitals_set += 1;
        }
    }

    info!(
        "Migration completed: {} provinces, {} nations, {} capitals",
        migration_status.provinces_migrated,
        migration_status.nations_migrated,
        migration_status.capitals_set
    );

    if !migration_status.errors.is_empty() {
        warn!(
            "Migration completed with {} errors",
            migration_status.errors.len()
        );
    }
}

/// Validate that relationships are correctly established
pub fn validate_relationship_migration(
    nations_query: Query<(Entity, &Nation, Option<&Controls>, Option<&HasCapital>)>,
    provinces_query: Query<(Entity, Option<&ControlledBy>, Option<&CapitalOf>), With<ProvinceEntity>>,
    mut migration_status: ResMut<MigrationStatus>,
) {
    let mut validation_errors = Vec::new();

    // Check each nation
    for (nation_entity, nation, controls_opt, capital_opt) in nations_query.iter() {
        // Verify nation has Controls component
        if controls_opt.is_none() {
            validation_errors.push(format!(
                "Nation {:?} (entity {:?}) missing Controls component",
                nation.id, nation_entity
            ));
        } else {
            let controls = controls_opt.unwrap();
            if controls.provinces().is_empty() {
                validation_errors.push(format!(
                    "Nation {:?} has Controls but no provinces",
                    nation.id
                ));
            }
        }

        // Verify nation has capital
        if capital_opt.is_none() {
            validation_errors.push(format!(
                "Nation {:?} missing HasCapital component",
                nation.id
            ));
        }
    }

    // Check provinces
    let mut orphaned_provinces = 0;
    for (province_entity, controlled_by_opt, capital_of_opt) in provinces_query.iter() {
        if controlled_by_opt.is_none() && capital_of_opt.is_none() {
            orphaned_provinces += 1;
        }
    }

    if orphaned_provinces > 0 {
        info!(
            "Found {} orphaned provinces (may be expected for unclaimed territory)",
            orphaned_provinces
        );
    }

    if !validation_errors.is_empty() {
        error!(
            "Validation found {} issues after migration",
            validation_errors.len()
        );
        for error in &validation_errors {
            error!("  - {}", error);
        }
        migration_status.errors.extend(validation_errors);
    } else {
        info!("Migration validation successful - all relationships intact");
    }
}

/// System to remove the old ProvinceOwnershipCache after successful migration
pub fn cleanup_old_ownership_system(world: &mut World) {
    // Remove the ProvinceOwnershipCache resource
    if world.contains_resource::<ProvinceOwnershipCache>() {
        world.remove_resource::<ProvinceOwnershipCache>();
        info!("Removed ProvinceOwnershipCache resource after successful migration");
    }

    // Note: Province.owner field removal would require a schema migration
    // as it's part of the Province struct. This would be done in a separate
    // refactoring pass to update the Province definition.
}

use bevy_plugin_builder::define_plugin;

/// Plugin for migrating to entity relationships
define_plugin!(NationMigrationPlugin {
    resources: [MigrationStatus],

    startup: [
        (
            migrate_ownership_to_relationships,
            validate_relationship_migration.after(migrate_ownership_to_relationships),
        ).run_if(resource_exists::<ProvinceOwnershipCache>)
    ]
});

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_nation_spawning_with_relationships() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create test data
        let test_nation = Nation {
            id: NationId::new(1),
            name: "Test Nation".to_string(),
            adjective: "Test".to_string(),
            color: Color::srgb(1.0, 0.0, 0.0),
            capital_province: 0,
            treasury: 1000.0,
            military_strength: 100.0,
            stability: 0.75,
            personality: NationPersonality::balanced(),
        };

        let test_house = House {
            nation_id: NationId::new(1),
            name: "Test House".to_string(),
            full_name: "House Test of Test Nation".to_string(),
            ruler: Ruler {
                name: "Test Ruler".to_string(),
                title: "King".to_string(),
                age: 40,
                years_ruling: 5,
                personality: RulerPersonality::default(),
            },
            motto: "Test Motto".to_string(),
            traits: HouseTraits::default(),
            years_in_power: 50,
            legitimacy: 0.8,
            prestige: 0.6,
        };

        // Create test province data
        let test_provinces = vec![
            Province {
                id: ProvinceId::new(0),
                position: Vec2::ZERO,
                owner: None,
                culture: None,
                population: 1000,
                max_population: 10000,
                terrain: TerrainType::Plains,
                elevation: Elevation::new(0.5),
                agriculture: Agriculture::new(1.0),
                fresh_water_distance: Distance::new(5.0),
                iron: Abundance::new(0.5),
                copper: Abundance::new(0.5),
                tin: Abundance::new(0.2),
                gold: Abundance::new(0.1),
                coal: Abundance::new(0.5),
                stone: Abundance::new(0.8),
                wood: Abundance::new(0.5),
                is_coastal: false,
                days_since_last_update: 0,
            },
        ];

        let mut nation_provinces = HashMap::new();
        nation_provinces.insert(NationId::new(1), vec![0]);

        // Spawn nation with relationships
        let nation_entities = spawn_nations_with_relationships(
            app.world_mut().commands(),
            &test_provinces,
            vec![(test_nation, test_house, GovernmentType::Democracy)],
            nation_provinces,
        );

        app.update();

        // Verify nation entity was created
        assert_eq!(nation_entities.len(), 1);

        // Verify nation has required components
        let nation_entity = nation_entities[0];
        assert!(app.world().entity(nation_entity).get::<Nation>().is_some());
        assert!(app.world().entity(nation_entity).get::<House>().is_some());
        assert!(app.world().entity(nation_entity).get::<Governance>().is_some());
    }

    #[test]
    fn test_migration_validation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<MigrationStatus>();

        // Create a nation without proper relationships
        let nation_entity = app.world_mut().spawn(Nation {
            id: NationId::new(1),
            name: "Test Nation".to_string(),
            adjective: "Test".to_string(),
            color: Color::srgb(1.0, 0.0, 0.0),
            capital_province: 0,
            treasury: 1000.0,
            military_strength: 100.0,
            stability: 0.75,
            personality: NationPersonality::balanced(),
        }).id();

        // Run validation
        app.add_systems(Update, validate_relationship_migration);
        app.update();

        // Check that validation detected missing components
        let migration_status = app.world().resource::<MigrationStatus>();
        assert!(!migration_status.errors.is_empty());
    }
}