//! Nation generation and territory assignment
//!
//! This module handles spawning nations with dynasties and assigning them
//! territory using a Voronoi-like growth algorithm.

use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::{HashMap, VecDeque};

use crate::name_generator::{Culture, NameGenerator, NameType};
use crate::world::{Province, TerrainType};
use crate::math::euclidean_squared_vec2;
use super::types::*;
use super::house::{House, Ruler, RulerPersonality, HouseTraits, generate_motto};

/// Spawn nations into the world with territory and ruling houses
pub fn spawn_nations(
    provinces: &mut [Province],
    settings: &NationGenerationSettings,
    seed: u32,
) -> (Vec<Nation>, Vec<House>) {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut nations = Vec::new();
    let mut houses = Vec::new();
    let mut nation_registry = NationRegistry::default();

    // Find suitable capital locations
    let capital_provinces = select_capital_provinces(provinces, settings.nation_count, &mut rng);

    if capital_provinces.is_empty() {
        warn!("No suitable provinces found for nation capitals!");
        return (nations, houses);
    }

    info!("Spawning {} nations with capitals", capital_provinces.len());

    // Create nations with ruling houses
    for &capital_idx in &capital_provinces {
        let (nation, house) = create_nation_with_house(
            &mut nation_registry,
            capital_idx,
            provinces[capital_idx].position,
            &mut rng,
            settings,
        );
        nations.push(nation);
        houses.push(house);
    }

    // Assign territory using growth algorithm
    assign_territory_to_nations(&mut nations, provinces, settings.nation_density);

    info!(
        "Successfully spawned {} nations with {} ruling houses",
        nations.len(),
        houses.len()
    );

    (nations, houses)
}

/// Select suitable provinces to be nation capitals
fn select_capital_provinces(
    provinces: &[Province],
    nation_count: u32,
    rng: &mut StdRng,
) -> Vec<usize> {
    // Find all land provinces that could be capitals
    let mut suitable_provinces: Vec<usize> = provinces
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            !matches!(
                p.terrain,
                TerrainType::Ocean | TerrainType::River | TerrainType::Alpine
            )
        })
        .map(|(idx, _)| idx)
        .collect();

    if suitable_provinces.len() <= nation_count as usize {
        return suitable_provinces;
    }

    // Select capitals with good spacing
    let mut selected_capitals = Vec::new();
    let min_distance_squared = calculate_min_capital_distance(provinces.len(), nation_count);

    for _ in 0..nation_count {
        if suitable_provinces.is_empty() {
            break;
        }

        // Pick a random candidate
        let candidate_idx = rng.gen_range(0..suitable_provinces.len());
        let province_idx = suitable_provinces[candidate_idx];

        // Check if it's far enough from existing capitals
        let position = provinces[province_idx].position;
        let far_enough = selected_capitals.iter().all(|&other_idx: &usize| {
            euclidean_squared_vec2(position, provinces[other_idx].position) >= min_distance_squared
        });

        if far_enough || selected_capitals.is_empty() {
            selected_capitals.push(province_idx);
            suitable_provinces.remove(candidate_idx);
        } else {
            // Try a few more times with this candidate
            let mut found = false;
            for _ in 0..5 {
                let idx = rng.gen_range(0..suitable_provinces.len());
                let prov_idx = suitable_provinces[idx];
                let pos = provinces[prov_idx].position;

                let ok = selected_capitals.iter().all(|&other| {
                    euclidean_squared_vec2(provinces[other].position, pos) >= min_distance_squared
                });

                if ok {
                    selected_capitals.push(prov_idx);
                    suitable_provinces.remove(idx);
                    found = true;
                    break;
                }
            }

            if !found && suitable_provinces.len() > 0 {
                // Just pick one anyway if we're struggling
                let idx = rng.gen_range(0..suitable_provinces.len());
                selected_capitals.push(suitable_provinces[idx]);
                suitable_provinces.remove(idx);
            }
        }
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

/// Create a nation with its ruling house
fn create_nation_with_house(
    registry: &mut NationRegistry,
    capital_idx: usize,
    capital_position: Vec2,
    rng: &mut StdRng,
    settings: &NationGenerationSettings,
) -> (Nation, House) {
    let nation_id = registry.create_nation_id();

    // Generate nation name using the name generator
    let mut name_gen = NameGenerator::new();
    let culture = pick_culture_for_location(capital_position, rng);
    let nation_name = name_gen.generate(NameType::Nation { culture });

    // Generate adjective from nation name
    let adjective = generate_adjective(&nation_name);

    // Generate house for this nation using our comprehensive name generator
    let house_name = name_gen.generate(NameType::House { culture });
    let ruler_name = name_gen.generate(NameType::Person {
        gender: crate::name_generator::Gender::Male,
        culture,
        role: crate::name_generator::PersonRole::Ruler,
    });
    let ruler_title = pick_ruler_title(settings.starting_development);

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

    (nation, house)
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
fn pick_ruler_title(development: StartingDevelopment) -> String {
    match development {
        StartingDevelopment::Primitive => "Chief".to_string(),
        StartingDevelopment::Medieval => "King".to_string(),
        StartingDevelopment::Renaissance => "Duke".to_string(),
        StartingDevelopment::Mixed => "Lord".to_string(),
    }
}

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

/// Assign territory to nations using a growth algorithm
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

    // Initialize capitals with ownership
    for nation in nations.iter() {
        provinces[nation.capital_province as usize].owner = Some(nation.id);
    }

    // Grow each nation's territory using BFS
    for nation in nations.iter() {
        let mut frontier = VecDeque::new();
        frontier.push_back(nation.capital_province);
        let mut provinces_claimed = 1;

        while let Some(current_province_id) = frontier.pop_front() {
            if provinces_claimed >= growth_limit {
                break;
            }

            // Get neighbors of current province (copy to avoid borrow issues)
            let neighbors = provinces[current_province_id as usize].neighbors;
            for neighbor_opt in neighbors.iter() {
                if let Some(neighbor_id) = neighbor_opt {
                    let neighbor_idx = neighbor_id.value();
                    let neighbor = &mut provinces[neighbor_idx as usize];

                    // Check if this province is unclaimed and is land
                    if neighbor.owner.is_none() {
                        // Don't claim ocean, river, or alpine terrain
                        if !matches!(
                            neighbor.terrain,
                            TerrainType::Ocean | TerrainType::River | TerrainType::Alpine
                        ) {
                            neighbor.owner = Some(nation.id);
                            frontier.push_back(neighbor_idx);
                            provinces_claimed += 1;

                            if provinces_claimed >= growth_limit {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    let owned_provinces = provinces.iter().filter(|p| p.owner.is_some()).count();
    info!(
        "Territory assignment complete. Average provinces per nation: {}",
        owned_provinces / nations.len().max(1)
    );
}