//! Nation generation and territory assignment
//!
//! This module handles spawning nations with dynasties and assigning them
//! territory using a Voronoi-like growth algorithm.

use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::name_generator::{Culture, NameGenerator, NameType};
use crate::world::{Province, TerrainType};
// Distance calculations use Bevy's Vec2 methods directly
use super::house::{generate_motto, House, HouseTraits, Ruler, RulerPersonality};
use super::types::*;

/// Spawn nations into the world with territory, ruling houses, and governance
pub fn spawn_nations(
    provinces: &mut [Province],
    settings: &NationGenerationSettings,
    seed: u32,
) -> (Vec<Nation>, Vec<House>, Vec<super::governance::GovernmentType>) {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut nations = Vec::new();
    let mut houses = Vec::new();
    let nation_registry = NationRegistry::default();

    // Find suitable capital locations
    let capital_provinces = select_capital_provinces(provinces, settings.nation_count, &mut rng);

    if capital_provinces.is_empty() {
        warn!("No suitable provinces found for nation capitals!");
        return (nations, houses, Vec::new());
    }

    info!("Spawning {} nations with capitals", capital_provinces.len());

    // Pre-generate seeds for parallel nation creation (avoids RNG contention)
    let nation_seeds: Vec<u64> = (0..capital_provinces.len()).map(|_| rng.r#gen()).collect();

    // Create nations with ruling houses and governance in parallel
    let nation_registry_arc = Arc::new(nation_registry);
    let nation_data: Vec<(Nation, House, super::governance::GovernmentType)> = capital_provinces
        .par_iter()
        .zip(nation_seeds.par_iter())
        .map(|(&capital_idx, &nation_seed)| {
            create_nation_with_house_parallel(
                &nation_registry_arc,
                capital_idx,
                provinces[capital_idx].position,
                nation_seed,
                settings,
            )
        })
        .collect();

    let mut governments = Vec::new();
    for (nation, house, government) in nation_data {
        nations.push(nation);
        houses.push(house);
        governments.push(government);
    }

    // Assign territory using growth algorithm
    assign_territory_to_nations(&mut nations, provinces, settings.nation_density);

    info!(
        "Successfully spawned {} nations with {} ruling houses and {} governments",
        nations.len(),
        houses.len(),
        governments.len()
    );

    (nations, houses, governments)
}

/// Select suitable provinces to be nation capitals using parallel evaluation
fn select_capital_provinces(
    provinces: &[Province],
    nation_count: u32,
    rng: &mut StdRng,
) -> Vec<usize> {
    // Parallel filter to find all land provinces that could be capitals
    let suitable_provinces: Vec<usize> = provinces
        .par_iter()
        .enumerate()
        .filter_map(|(idx, p)| {
            if !matches!(
                p.terrain,
                TerrainType::Ocean | TerrainType::River | TerrainType::Alpine
            ) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    if suitable_provinces.len() <= nation_count as usize {
        return suitable_provinces;
    }

    // Calculate minimum distance for good spacing
    let min_distance_squared = calculate_min_capital_distance(provinces.len(), nation_count);

    // Use spatial partitioning with parallel evaluation for capital selection
    let mut selected_capitals = Vec::new();
    let mut remaining_candidates = suitable_provinces.clone();

    for _ in 0..nation_count {
        if remaining_candidates.is_empty() {
            break;
        }

        // Parallel evaluate all remaining candidates for distance constraints
        let scores: Vec<(usize, f32)> = remaining_candidates
            .par_iter()
            .map(|&candidate_idx| {
                let position = provinces[candidate_idx].position;

                // Calculate minimum distance to any existing capital
                let min_dist = if selected_capitals.is_empty() {
                    f32::MAX
                } else {
                    selected_capitals
                        .iter()
                        .map(|&other_idx: &usize| {
                            position.distance_squared(provinces[other_idx].position)
                        })
                        .min_by(|a: &f32, b: &f32| a.total_cmp(b))
                        .unwrap_or(f32::MAX)
                };

                (candidate_idx, min_dist)
            })
            .collect();

        // Find candidates that meet distance requirements
        let valid_candidates: Vec<usize> = scores
            .into_iter()
            .filter(|&(_, dist)| dist >= min_distance_squared || selected_capitals.is_empty())
            .map(|(idx, _)| idx)
            .collect();

        // Select a random valid candidate or fallback to any candidate
        let selected = if !valid_candidates.is_empty() {
            let idx = rng.gen_range(0..valid_candidates.len());
            valid_candidates[idx]
        } else if !remaining_candidates.is_empty() {
            // Fallback: pick the candidate with maximum minimum distance
            let best = remaining_candidates
                .par_iter()
                .map(|&idx| {
                    let pos = provinces[idx].position;
                    let min_dist = selected_capitals
                        .iter()
                        .map(|&other| provinces[other].position.distance_squared(pos))
                        .min_by(|a, b| a.total_cmp(b))
                        .unwrap_or(0.0);
                    (idx, min_dist)
                })
                .max_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(idx, _)| idx)
                .unwrap_or(remaining_candidates[rng.gen_range(0..remaining_candidates.len())]);
            best
        } else {
            break;
        };

        selected_capitals.push(selected);
        remaining_candidates.retain(|&x| x != selected);
    }

    selected_capitals
}

/// Calculate minimum distance between capitals based on world size
fn calculate_min_capital_distance(province_count: usize, nation_count: u32) -> f32 {
    let world_area = province_count as f32;
    let area_per_nation = world_area / nation_count as f32;
    let radius = (area_per_nation / std::f32::consts::PI).sqrt();
    radius * radius * 0.5 // Squared distance, with some overlap allowed
}

/// Create a nation with its ruling house and governance (parallel version)
fn create_nation_with_house_parallel(
    registry: &Arc<NationRegistry>,
    capital_idx: usize,
    capital_position: Vec2,
    seed: u64,
    settings: &NationGenerationSettings,
) -> (Nation, House, super::governance::GovernmentType) {
    let mut rng = StdRng::seed_from_u64(seed);
    let nation_id = registry.create_nation_id(); // Thread-safe with atomic operations

    // Generate nation name using the name generator (thread-local)
    let mut name_gen = NameGenerator::with_seed(seed);
    let culture = pick_culture_for_location(capital_position, &mut rng);

    // Determine government type based on culture and development
    let development_level = super::governance::DevelopmentLevel::from(settings.starting_development);
    let government_type = super::governance::suggest_government_for_culture(
        culture,
        development_level,
        &mut rng,
    );

    // Generate governance-aware nation name and ruler title
    let (nation_name, ruler_title) = super::governance::generate_governance_aware_name(
        &mut name_gen,
        culture,
        &government_type,
    );

    // Generate adjective from nation name
    let adjective = generate_adjective(&nation_name);

    // Generate house for this nation using our comprehensive name generator
    let house_name = name_gen.generate(NameType::House { culture });
    let ruler_name = name_gen.generate(NameType::Person {
        gender: crate::name_generator::Gender::Male,
        culture,
        role: crate::name_generator::PersonRole::Ruler,
    });

    // Create personality influenced by aggression setting
    let mut personality = NationPersonality::random(&mut rng);
    personality.aggression *= settings.aggression_level;

    // Create house traits
    let house_traits = HouseTraits::random(&mut rng);
    let motto = generate_motto(&house_traits, &culture);

    // Pick a unique color for this nation
    let color = generate_nation_color(nation_id.value(), &mut rng);

    // Create the nation
    let nation = Nation {
        id: nation_id,
        name: nation_name.clone(),
        adjective,
        color,
        capital_province: capital_idx as u32,
        treasury: 1000.0,
        military_strength: 100.0,
        stability: 0.75,
        personality,
    };

    // Create the ruling house
    let house = House {
        nation_id,
        name: house_name.clone(),
        full_name: format!("House {} of {}", house_name, nation_name),
        ruler: Ruler {
            name: ruler_name,
            title: ruler_title,
            age: rng.gen_range(25..65),
            years_ruling: rng.gen_range(1..15),
            personality: RulerPersonality::random(&mut rng),
        },
        motto,
        traits: house_traits,
        years_in_power: rng.gen_range(10..200),
        legitimacy: 0.75 + rng.gen_range(-0.2..0.2),
        prestige: 0.5 + rng.gen_range(-0.3..0.3),
    };

    (nation, house, government_type)
}

/// Create a nation with its ruling house and governance (legacy sequential version)
fn create_nation_with_house(
    registry: &mut NationRegistry,
    capital_idx: usize,
    capital_position: Vec2,
    rng: &mut StdRng,
    settings: &NationGenerationSettings,
) -> (Nation, House, super::governance::GovernmentType) {
    let nation_id = registry.create_nation_id();

    // Generate nation name using the name generator
    let mut name_gen = NameGenerator::new();
    let culture = pick_culture_for_location(capital_position, rng);

    // Determine government type based on culture and development
    let development_level = super::governance::DevelopmentLevel::from(settings.starting_development);
    let government_type = super::governance::suggest_government_for_culture(
        culture,
        development_level,
        rng,
    );

    // Generate governance-aware nation name and ruler title
    let (nation_name, ruler_title) = super::governance::generate_governance_aware_name(
        &mut name_gen,
        culture,
        &government_type,
    );

    // Generate adjective from nation name
    let adjective = generate_adjective(&nation_name);

    // Generate house for this nation using our comprehensive name generator
    let house_name = name_gen.generate(NameType::House { culture });
    let ruler_name = name_gen.generate(NameType::Person {
        gender: crate::name_generator::Gender::Male,
        culture,
        role: crate::name_generator::PersonRole::Ruler,
    });

    // Create personality influenced by aggression setting
    let mut personality = NationPersonality::random(rng);
    personality.aggression *= settings.aggression_level;

    // Create house traits
    let house_traits = HouseTraits::random(rng);
    let motto = generate_motto(&house_traits, &culture);

    // Pick a unique color for this nation
    let color = generate_nation_color(nation_id.value(), rng);

    // Create the nation
    let nation = Nation {
        id: nation_id,
        name: nation_name.clone(),
        adjective,
        color,
        capital_province: capital_idx as u32,
        treasury: 1000.0,
        military_strength: 100.0,
        stability: 0.75,
        personality,
    };

    // Create the ruling house
    let house = House {
        nation_id,
        name: house_name.clone(),
        full_name: format!("House {} of {}", house_name, nation_name),
        ruler: Ruler {
            name: ruler_name,
            title: ruler_title,
            age: rng.gen_range(25..65),
            years_ruling: rng.gen_range(1..15),
            personality: RulerPersonality::random(rng),
        },
        motto,
        traits: house_traits,
        years_in_power: rng.gen_range(10..200),
        legitimacy: 0.75 + rng.gen_range(-0.2..0.2),
        prestige: 0.5 + rng.gen_range(-0.3..0.3),
    };

    (nation, house, government_type)
}

/// Pick a culture based on geographic location
fn pick_culture_for_location(position: Vec2, rng: &mut StdRng) -> Culture {
    // Simple heuristic: different regions get different cultures
    // This could be made much more sophisticated
    let region_x = (position.x / 1000.0) as i32;
    let region_y = (position.y / 1000.0) as i32;

    let cultures = [
        Culture::Western,
        Culture::Eastern,
        Culture::Northern,
        Culture::Southern,
        Culture::Desert,
        Culture::Island,
    ];

    let index = ((region_x.abs() + region_y.abs()) as usize + rng.gen_range(0..2)) % cultures.len();
    cultures[index]
}

/// Generate an adjective form from a nation name
fn generate_adjective(nation_name: &str) -> String {
    // Simple rules for generating adjectives
    if nation_name.ends_with("ia") {
        format!("{}n", &nation_name[..nation_name.len() - 2])
    } else if nation_name.ends_with("land") {
        format!("{}ic", &nation_name[..nation_name.len() - 4])
    } else if nation_name.ends_with("burg") || nation_name.ends_with("berg") {
        format!("{}ian", &nation_name[..nation_name.len() - 4])
    } else {
        format!("{}ian", nation_name)
    }
}

/// Pick a ruler title based on development level
// Ruler titles are now determined by government type via the governance system

/// Generate a unique color for a nation
fn generate_nation_color(nation_id: u32, rng: &mut StdRng) -> Color {
    // Use nation ID to ensure consistent colors
    let hue = (nation_id as f32 * 137.5) % 360.0; // Golden angle for good distribution
    let saturation = 0.6 + rng.gen_range(0.0..0.3);
    let lightness = 0.4 + rng.gen_range(0.0..0.3);

    // Convert HSL to RGB
    hsl_to_rgb(hue, saturation, lightness)
}

/// Convert HSL color to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let h = h / 360.0;
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l;
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    Color::srgb(r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

/// Assign territory to nations using parallel growth algorithm with atomic operations
fn assign_territory_to_nations(
    nations: &mut Vec<Nation>,
    provinces: &mut [Province],
    density: NationDensity,
) {
    let growth_limit = match density {
        NationDensity::Sparse => provinces.len() / nations.len() * 2,
        NationDensity::Balanced => provinces.len() / nations.len(),
        NationDensity::Fragmented => provinces.len() / nations.len() / 2,
    };

    // Create atomic ownership array (0 = unclaimed, nation_id + 1 = claimed)
    let atomic_owners: Arc<Vec<AtomicU32>> =
        Arc::new((0..provinces.len()).map(|_| AtomicU32::new(0)).collect());

    // Initialize capitals with atomic ownership
    for nation in nations.iter() {
        let capital_idx = nation.capital_province as usize;
        atomic_owners[capital_idx].store(nation.id.value() + 1, Ordering::SeqCst);
    }

    // Parallel territory expansion for all nations
    let nation_claims: Vec<Vec<u32>> = nations
        .par_iter()
        .map(|nation| {
            let mut claimed_provinces = vec![nation.capital_province];
            let mut frontier = VecDeque::new();
            frontier.push_back(nation.capital_province);
            let nation_id_atomic = nation.id.value() + 1; // +1 because 0 means unclaimed

            while claimed_provinces.len() < growth_limit && !frontier.is_empty() {
                // Process current frontier level
                let current_level_size = frontier.len();
                let mut next_frontier = Vec::new();

                for _ in 0..current_level_size {
                    if let Some(current_province_id) = frontier.pop_front() {
                        let neighbors = provinces[current_province_id as usize].neighbors;

                        for neighbor_opt in neighbors.iter() {
                            if let Some(neighbor_id) = neighbor_opt {
                                let neighbor_idx = neighbor_id.value() as usize;
                                let neighbor = &provinces[neighbor_idx];

                                // Check terrain suitability
                                if matches!(
                                    neighbor.terrain,
                                    TerrainType::Ocean | TerrainType::River | TerrainType::Alpine
                                ) {
                                    continue;
                                }

                                // Try to claim the province atomically
                                let result = atomic_owners[neighbor_idx].compare_exchange(
                                    0,                // Expected: unclaimed
                                    nation_id_atomic, // New value: our nation ID
                                    Ordering::SeqCst,
                                    Ordering::SeqCst,
                                );

                                if result.is_ok() {
                                    // Successfully claimed!
                                    claimed_provinces.push(neighbor_idx as u32);
                                    next_frontier.push(neighbor_idx as u32);

                                    if claimed_provinces.len() >= growth_limit {
                                        break;
                                    }
                                }
                            }
                        }

                        if claimed_provinces.len() >= growth_limit {
                            break;
                        }
                    }
                }

                // Move to next level of BFS
                frontier.extend(next_frontier);
            }

            claimed_provinces
        })
        .collect();

    // Apply the atomic ownership results back to provinces
    for (idx, atomic_owner) in atomic_owners.iter().enumerate() {
        let owner_val = atomic_owner.load(Ordering::SeqCst);
        if owner_val > 0 {
            // Convert back from atomic value to NationId (subtract 1)
            provinces[idx].owner = Some(NationId::new(owner_val - 1));
        }
    }

    // Log statistics
    let owned_provinces = provinces.iter().filter(|p| p.owner.is_some()).count();
    let total_claimed: usize = nation_claims.iter().map(|v| v.len()).sum();

    info!(
        "Parallel territory assignment complete. {} provinces claimed by {} nations (avg: {})",
        owned_provinces,
        nations.len(),
        owned_provinces / nations.len().max(1)
    );

    debug!(
        "Territory distribution: min={}, max={}, median={}",
        nation_claims.iter().map(|v| v.len()).min().unwrap_or(0),
        nation_claims.iter().map(|v| v.len()).max().unwrap_or(0),
        {
            // Optimize: pre-allocate Vec with known capacity to avoid reallocations
            let nation_count = nation_claims.len();
            let mut sizes: Vec<_> = Vec::with_capacity(nation_count);
            sizes.extend(nation_claims.iter().map(|v| v.len()));
            sizes.sort();
            sizes.get(sizes.len() / 2).copied().unwrap_or(0)
        }
    );
}

/// Build Territory entities from contiguous province regions (parallel version)
/// Returns a map of nation_id -> vec of territories for that nation
pub fn build_territories_from_provinces(
    provinces: &[Province],
) -> HashMap<NationId, Vec<Territory>> {
    // Parallel grouping: Group provinces by owner nation first
    let provinces_by_nation: HashMap<NationId, Vec<usize>> = provinces
        .par_iter()
        .enumerate()
        .filter_map(|(idx, province)| province.owner.map(|owner| (owner, idx)))
        .fold(HashMap::new, |mut acc, (nation_id, province_idx)| {
            acc.entry(nation_id)
                .or_insert_with(Vec::new)
                .push(province_idx);
            acc
        })
        .reduce(HashMap::new, |mut acc1, acc2| {
            for (nation_id, mut indices) in acc2 {
                acc1.entry(nation_id)
                    .or_insert_with(Vec::new)
                    .append(&mut indices);
            }
            acc1
        });

    // CRITICAL FIX: Build HashMap for O(1) province lookups to prevent O(n²) complexity
    // This prevents the "Green Ocean Disaster" pattern of linear search inside parallel loops
    let province_lookup: HashMap<u32, &Province> = provinces
        .iter()
        .map(|p| (p.id.value(), p))
        .collect();
    let province_lookup = Arc::new(province_lookup);

    // Parallel territory building: Process each nation's provinces in parallel
    let nation_territories: HashMap<NationId, Vec<Territory>> = provinces_by_nation
        .par_iter()
        .map(|(&nation_id, province_indices)| {
            let province_map = Arc::clone(&province_lookup);
            let mut territories = Vec::new();
            let mut visited = HashSet::with_capacity(province_indices.len());

            // For this nation, find all contiguous territories
            for &province_idx in province_indices {
                let province = &provinces[province_idx];

                if visited.contains(&province.id.value()) {
                    continue;
                }

                // Start a new territory from this province
                let mut territory_provinces = HashSet::new();
                let mut frontier = VecDeque::new();
                frontier.push_back(province.id.value());

                let mut sum_x = 0.0;
                let mut sum_y = 0.0;
                let mut count = 0;

                // BFS to find all contiguous provinces owned by same nation
                while let Some(current_id) = frontier.pop_front() {
                    if visited.contains(&current_id) {
                        continue;
                    }

                    // O(1) HashMap lookup instead of O(n) linear search!
                    if let Some(&current_province) = province_map.get(&current_id) {
                        if current_province.owner == Some(nation_id) {
                            visited.insert(current_id);
                            territory_provinces.insert(current_id);

                            sum_x += current_province.position.x;
                            sum_y += current_province.position.y;
                            count += 1;

                            // Add unvisited neighbors
                            for neighbor_opt in current_province.neighbors {
                                if let Some(neighbor_id) = neighbor_opt {
                                    if !visited.contains(&neighbor_id.value()) {
                                        frontier.push_back(neighbor_id.value());
                                    }
                                }
                            }
                        }
                    }
                }

                // Create territory if we found provinces
                if !territory_provinces.is_empty() {
                    let center = Vec2::new(sum_x / count as f32, sum_y / count as f32);

                    let territory = Territory {
                        provinces: territory_provinces,
                        nation_id,
                        center,
                        is_core: true, // TODO: Determine core vs conquered based on culture/history
                    };

                    territories.push(territory);
                }
            }

            (nation_id, territories)
        })
        .collect();

    info!("Built territories for {} nations", nation_territories.len());

    for (nation_id, territories) in &nation_territories {
        let total_provinces: usize = territories.iter().map(|t| t.provinces.len()).sum();
        info!(
            "Nation {} has {} territories with {} total provinces",
            nation_id.value(),
            territories.len(),
            total_provinces
        );
    }

    nation_territories
}
