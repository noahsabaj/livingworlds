//! Performance benchmarks for nation systems
//!
//! Measures performance characteristics of nation systems at scale,
//! particularly focusing on 100+ nation scenarios to verify scalability.

#![cfg(test)]

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::nations::{
    Nation, NationId, NationPersonality, NationRegistry,
    ProvinceOwnershipCache, TerritoryMetricsCache,
    laws::{Law, LawId, LawCategory, LawRegistry, NationLaws},
    governance::{GovernmentType, GovernmentCategory, Governance, PoliticalPressure},
};

/// Create a test world with specified number of nations
fn create_benchmark_world(nation_count: usize, provinces_per_nation: usize) -> (App, Vec<Entity>) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let mut nation_entities = Vec::new();
    let mut ownership_cache = ProvinceOwnershipCache::default();
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // Create nations
    for nation_id in 0..nation_count {
        let nation = Nation {
            id: NationId::new(nation_id as u32),
            name: format!("Nation {}", nation_id),
            adjective: format!("Nation{}", nation_id),
            color: Color::srgb(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            capital_province: (nation_id * provinces_per_nation) as u32,
            treasury: rng.gen_range(100.0..10000.0),
            military_strength: rng.gen_range(10.0..1000.0),
            stability: rng.gen_range(0.3..0.9),
            personality: NationPersonality::random(&mut rng),
        };

        let entity = app.world_mut().spawn(nation).id();
        nation_entities.push(entity);

        // Add provinces to ownership cache
        let province_ids: HashSet<u32> = ((nation_id * provinces_per_nation)
            ..((nation_id + 1) * provinces_per_nation))
            .map(|p| p as u32)
            .collect();
        ownership_cache.by_nation.insert(NationId::new(nation_id as u32), province_ids);
    }

    app.insert_resource(ownership_cache);
    app.insert_resource(NationRegistry {
        nations: Vec::new(), // Not used in benchmarks
        nation_id_counter: std::sync::Arc::new(std::sync::atomic::AtomicU32::new(nation_count as u32)),
    });

    (app, nation_entities)
}

/// Benchmark province ownership lookups
fn bench_ownership_lookups(c: &mut Criterion) {
    let mut group = c.benchmark_group("ownership_lookups");

    for nation_count in [10, 50, 100, 200] {
        let provinces_per_nation = 100;
        let (app, _) = create_benchmark_world(nation_count, provinces_per_nation);
        let ownership_cache = app.world().resource::<ProvinceOwnershipCache>();

        group.bench_with_input(
            BenchmarkId::from_parameter(nation_count),
            &nation_count,
            |b, _| {
                b.iter(|| {
                    // Benchmark looking up provinces for each nation
                    for nation_id in 0..nation_count {
                        let provinces = ownership_cache
                            .get_nation_provinces(NationId::new(nation_id as u32));
                        black_box(provinces);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark law voting across all nations
fn bench_law_voting(c: &mut Criterion) {
    let mut group = c.benchmark_group("law_voting");

    for nation_count in [10, 50, 100, 200] {
        let (mut app, nation_entities) = create_benchmark_world(nation_count, 50);

        // Create law registry with sample laws
        let mut registry = LawRegistry::default();
        for i in 0..20 {
            let law = Law {
                id: LawId::new(i),
                category: LawCategory::Economic,
                name: format!("Law {}", i),
                description: "Test law".to_string(),
                effects: Default::default(),
                prerequisites: vec![],
                conflicts_with: vec![],
                government_affinity: HashMap::new(),
                complexity: crate::nations::laws::LawComplexity::Simple,
                base_popularity: 0.5,
                is_constitutional: false,
                available_from_year: 0,
            };
            registry.register_law(law);
        }

        // Add NationLaws component to each nation
        for &entity in &nation_entities {
            app.world_mut().entity_mut(entity).insert(NationLaws::new(entity));
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(nation_count),
            &nation_count,
            |b, _| {
                b.iter(|| {
                    // Simulate law voting for each nation
                    for &entity in &nation_entities {
                        let mut nation_laws = app.world_mut()
                            .entity_mut(entity)
                            .get_mut::<NationLaws>()
                            .unwrap();

                        // Propose a law
                        nation_laws.propose_law(
                            LawId::new(5),
                            0.6,
                            5.0,
                            Some(crate::simulation::PressureType::Economic),
                        );

                        // Simulate debate progress
                        if let Some(proposal) = nation_laws.proposed_laws.get_mut(0) {
                            proposal.debate_progress += 0.1;
                            proposal.current_support += 0.01;
                        }
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark government transition calculations
fn bench_government_transitions(c: &mut Criterion) {
    let mut group = c.benchmark_group("government_transitions");

    for nation_count in [10, 50, 100, 200] {
        let (mut app, nation_entities) = create_benchmark_world(nation_count, 50);

        // Add governance components
        for &entity in &nation_entities {
            let governance = Governance {
                current_type: GovernmentType::Democracy,
                category: GovernmentCategory::Democratic,
                stability: 0.5,
                legitimacy: 0.7,
                transition_pressure: 0.0,
                last_transition: 0.0,
                institutional_strength: 0.5,
                revolutionary_fervor: 0.0,
            };

            let pressure = PoliticalPressure {
                economic_crisis: 0.3,
                military_defeat: 0.0,
                popular_unrest: 0.2,
                elite_dissatisfaction: 0.1,
                religious_influence: 0.0,
                foreign_influence: 0.1,
                internal_conflict: 0.0,
            };

            app.world_mut().entity_mut(entity)
                .insert((governance, pressure));
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(nation_count),
            &nation_count,
            |b, _| {
                b.iter(|| {
                    // Calculate transition pressure for each nation
                    for &entity in &nation_entities {
                        let (governance, pressure) = app.world()
                            .entity(entity)
                            .get::<(&Governance, &PoliticalPressure)>()
                            .unwrap();

                        let total_pressure = pressure.economic_crisis * 0.3
                            + pressure.military_defeat * 0.5
                            + pressure.popular_unrest * 0.4
                            + pressure.elite_dissatisfaction * 0.3
                            + pressure.religious_influence * 0.2
                            + pressure.foreign_influence * 0.2
                            + pressure.internal_conflict * 0.4;

                        let threshold = governance.stability * 0.5
                            + governance.legitimacy * 0.3
                            + governance.institutional_strength * 0.2;

                        let should_transition = total_pressure > threshold;
                        black_box(should_transition);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark territory metrics calculation
fn bench_territory_metrics(c: &mut Criterion) {
    let mut group = c.benchmark_group("territory_metrics");

    for nation_count in [10, 50, 100] {
        let provinces_per_nation = 100;
        let (app, _) = create_benchmark_world(nation_count, provinces_per_nation);
        let ownership_cache = app.world().resource::<ProvinceOwnershipCache>();

        // Create mock province storage
        let total_provinces = nation_count * provinces_per_nation;
        let mut provinces = Vec::with_capacity(total_provinces);
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        for i in 0..total_provinces {
            provinces.push(MockProvince {
                id: i as u32,
                population: rng.gen_range(100..10000),
                development: rng.gen_range(0.1..1.0),
                terrain_type: if rng.gen_bool(0.3) { TerrainType::Mountain } else { TerrainType::Plains },
            });
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(nation_count),
            &nation_count,
            |b, _| {
                b.iter(|| {
                    let mut metrics_cache = TerritoryMetricsCache::default();

                    // Calculate metrics for all nations
                    for nation_id in 0..nation_count {
                        let nation_id = NationId::new(nation_id as u32);
                        if let Some(province_ids) = ownership_cache.get_nation_provinces(nation_id) {
                            let mut total_population = 0;
                            let mut total_development = 0.0;
                            let mut terrain_counts = HashMap::new();

                            for &province_id in province_ids {
                                if let Some(province) = provinces.get(province_id as usize) {
                                    total_population += province.population;
                                    total_development += province.development;
                                    *terrain_counts.entry(province.terrain_type).or_insert(0) += 1;
                                }
                            }

                            black_box((total_population, total_development, terrain_counts));
                        }
                    }

                    black_box(metrics_cache);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark diplomatic relationship queries
fn bench_diplomatic_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("diplomatic_queries");

    for nation_count in [10, 50, 100] {
        // Create all possible relationships
        let mut alliances: HashSet<(NationId, NationId)> = HashSet::new();
        let mut wars: HashSet<(NationId, NationId)> = HashSet::new();
        let mut trade: HashSet<(NationId, NationId)> = HashSet::new();
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        // Generate random relationships
        for i in 0..nation_count {
            for j in i + 1..nation_count {
                let id1 = NationId::new(i as u32);
                let id2 = NationId::new(j as u32);

                if rng.gen_bool(0.1) {
                    alliances.insert((id1, id2));
                } else if rng.gen_bool(0.05) {
                    wars.insert((id1, id2));
                }

                if rng.gen_bool(0.3) {
                    trade.insert((id1, id2));
                }
            }
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(nation_count),
            &nation_count,
            |b, _| {
                b.iter(|| {
                    // Query all relationships for each nation
                    for i in 0..nation_count {
                        let nation_id = NationId::new(i as u32);

                        // Find all allies
                        let allies: Vec<_> = alliances.iter()
                            .filter(|(n1, n2)| *n1 == nation_id || *n2 == nation_id)
                            .collect();

                        // Find all enemies
                        let enemies: Vec<_> = wars.iter()
                            .filter(|(n1, n2)| *n1 == nation_id || *n2 == nation_id)
                            .collect();

                        // Find all trade partners
                        let partners: Vec<_> = trade.iter()
                            .filter(|(n1, n2)| *n1 == nation_id || *n2 == nation_id)
                            .collect();

                        black_box((allies, enemies, partners));
                    }
                });
            },
        );
    }

    group.finish();
}

// Mock types for benchmarking
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum TerrainType {
    Plains,
    Mountain,
}

struct MockProvince {
    id: u32,
    population: u32,
    development: f32,
    terrain_type: TerrainType,
}

// Note: To run these benchmarks, add to Cargo.toml:
// [[bench]]
// name = "nations"
// harness = false
// path = "src/nations/benches.rs"
//
// [dev-dependencies]
// criterion = "0.5"
//
// Then run: cargo bench --bench nations