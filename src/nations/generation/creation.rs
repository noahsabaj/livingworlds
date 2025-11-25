//! Nation and house creation
//!
//! Handles creating nations with their ruling houses, governance, and unique attributes.

use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng as StdRng;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::name_generator::{Culture, NameGenerator, NameType};
use crate::world::Province;
use super::super::house::{generate_motto, House, HouseTraits, Ruler, RulerPersonality};
use super::super::types::*;
use super::colors::generate_nation_color;

/// Create a nation with its ruling house and governance (parallel version)
///
/// This version uses a pre-computed seed for thread-safe parallel execution.
pub fn create_nation_with_house_parallel(
    registry: &Arc<NationRegistry>,
    capital_idx: usize,
    capital_position: Vec2,
    seed: u64,
    settings: &NationGenerationSettings,
) -> (NationId, Nation, House, super::super::governance::GovernmentType) {
    let mut rng = StdRng::seed_from_u64(seed);
    let nation_id = registry.create_nation_id(); // Thread-safe with atomic operations

    // Generate nation name using the name generator (thread-local)
    let mut name_gen = NameGenerator::with_seed(seed);
    let culture = pick_culture_for_location(capital_position, &mut rng);

    // Determine government type based on culture and development
    let development_level = super::super::governance::DevelopmentLevel::from(settings.starting_development);
    let government_type = super::super::governance::suggest_government_for_culture(
        culture,
        development_level,
        &mut rng,
    );

    // Generate governance-aware nation name and ruler title
    let (nation_name, ruler_title) = super::super::governance::generate_governance_aware_name(
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
        role: crate::name_generator::PersonRole::Noble,
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
        name: nation_name.clone(),
        adjective,
        color,
        capital_province: capital_idx as u32,
        treasury: 1000.0,
        tax_rate: rng.r#gen_range(0.15..0.35), // Start with 15%-35% tax rate
        military_strength: 100.0,
        stability: 0.75,
        culture,
        technology_level: 1,
        personality,
    };

    // Create the ruling house
    let house = House {
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

    (nation_id, nation, house, government_type)
}

/// Ensure all nation names are unique, regenerating duplicates if necessary
pub fn ensure_unique_nation_names(
    nation_data: &mut Vec<(NationId, Nation, House, super::super::governance::GovernmentType)>,
    capital_provinces: &[usize],
    provinces: &[Province],
    settings: &NationGenerationSettings,
) {
    // Track which names are used and their indices
    let mut name_counts: HashMap<String, Vec<usize>> = HashMap::new();

    // Build index of duplicate names
    for (idx, (_, nation, _, _)) in nation_data.iter().enumerate() {
        name_counts.entry(nation.name.clone())
            .or_insert_with(Vec::new)
            .push(idx);
    }

    // Find duplicates (names that appear more than once)
    let duplicates: Vec<_> = name_counts.iter()
        .filter(|(_, indices)| indices.len() > 1)
        .collect();

    if duplicates.is_empty() {
        return; // No duplicates, nothing to do
    }

    info!("Found {} duplicate nation names, regenerating...", duplicates.len());

    // Track all used names for O(1) lookups
    let mut used_names: HashSet<String> = name_counts.keys().cloned().collect();

    // For each set of duplicates, regenerate all but the first
    for (original_name, indices) in duplicates {
        info!("Duplicate nation name '{}' appears {} times", original_name, indices.len());

        // Skip first occurrence, regenerate the rest
        for &idx in &indices[1..] {
            let (_, nation, house, government) = &mut nation_data[idx];
            let capital_idx = capital_provinces[idx];
            let capital_position = provinces[capital_idx].position;

            // Generate new unique name with modified seed
            let base_seed = idx as u64;
            let mut attempt = 0;
            const MAX_ATTEMPTS: u32 = 100;

            let (new_name, new_adjective, new_ruler_title) = loop {
                attempt += 1;
                if attempt > MAX_ATTEMPTS {
                    warn!("Failed to generate unique name after {} attempts for nation index {}",
                          MAX_ATTEMPTS, idx);
                    // Fallback: original name + number
                    let fallback_name = format!("{} {}", original_name, attempt);
                    let fallback_adj = generate_adjective(&fallback_name);
                    break (fallback_name, fallback_adj, house.ruler.title.clone());
                }

                // Create new generator with modified seed
                let modified_seed = base_seed + (attempt as u64 * 10000);
                let mut rng = StdRng::seed_from_u64(modified_seed);
                let mut name_gen = NameGenerator::with_seed(modified_seed);
                let culture = pick_culture_for_location(capital_position, &mut rng);

                // Generate new name
                let (candidate_name, ruler_title) = super::super::governance::generate_governance_aware_name(
                    &mut name_gen,
                    culture,
                    government,
                );

                // Check if unique
                if !used_names.contains(&candidate_name) {
                    let adjective = generate_adjective(&candidate_name);
                    debug!("  {} -> {} (attempt {})", original_name, candidate_name, attempt);
                    break (candidate_name, adjective, ruler_title);
                }
            };

            // Update nation
            let old_name = nation.name.clone();
            nation.name = new_name.clone();
            nation.adjective = new_adjective.clone();

            // Update house
            house.full_name = format!("House {} of {}", house.name, new_name);
            house.ruler.title = new_ruler_title;

            // Track new name as used, remove old duplicate
            used_names.insert(new_name.clone());
            used_names.remove(&old_name);
        }
    }
}

/// Create a nation with its ruling house and governance (legacy sequential version)
#[allow(dead_code)]
pub fn create_nation_with_house(
    registry: &mut NationRegistry,
    capital_idx: usize,
    capital_position: Vec2,
    rng: &mut StdRng,
    settings: &NationGenerationSettings,
) -> (NationId, Nation, House, super::super::governance::GovernmentType) {
    let nation_id = registry.create_nation_id();

    // Generate nation name using the name generator
    let mut name_gen = NameGenerator::new();
    let culture = pick_culture_for_location(capital_position, rng);

    // Determine government type based on culture and development
    let development_level = super::super::governance::DevelopmentLevel::from(settings.starting_development);
    let government_type = super::super::governance::suggest_government_for_culture(
        culture,
        development_level,
        rng,
    );

    // Generate governance-aware nation name and ruler title
    let (nation_name, ruler_title) = super::super::governance::generate_governance_aware_name(
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
        role: crate::name_generator::PersonRole::Noble,
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
        name: nation_name.clone(),
        adjective,
        color,
        capital_province: capital_idx as u32,
        treasury: 1000.0,
        tax_rate: rng.r#gen_range(0.15..0.35), // Start with 15%-35% tax rate
        military_strength: 100.0,
        stability: 0.75,
        culture,
        technology_level: 1,
        personality,
    };

    // Create the ruling house
    let house = House {
        name: house_name.clone(),
        full_name: format!("House {} of {}", house_name, nation_name),
        ruler: Ruler {
            name: ruler_name,
            title: ruler_title,
            age: rng.r#gen_range(25..65),
            years_ruling: rng.r#gen_range(1..15),
            personality: RulerPersonality::random(rng),
        },
        motto,
        traits: house_traits,
        years_in_power: rng.r#gen_range(10..200),
        legitimacy: 0.75 + rng.r#gen_range(-0.2..0.2),
        prestige: 0.5 + rng.r#gen_range(-0.3..0.3),
    };

    (nation_id, nation, house, government_type)
}

/// Pick a culture based on geographic location
pub fn pick_culture_for_location(position: Vec2, rng: &mut StdRng) -> Culture {
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
pub fn generate_adjective(nation_name: &str) -> String {
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
