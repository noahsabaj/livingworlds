//! Western culture name generation
//!
//! European-inspired naming patterns including kingdoms, republics,
//! and classical house structures with titles and dynasties.
//!
//! Features three generation systems:
//! 1. **Simple Patterns**: Traditional single-variable replacement (32+ patterns)
//! 2. **Compound Patterns**: Multi-variable combinations with royal and democratic themes
//! 3. **Weighted Selection**: Realistic distribution with common/uncommon/rare patterns

use super::super::core::NameGenerator;
use crate::name_generator::data::western_data::*;

/// Generate a Western-style nation name using compound pattern system
pub fn generate_nation_name(generator: &mut NameGenerator) -> String {
    

    // Choose generation method: 65% weighted simple, 25% compound, 10% democratic compound
    let generation_type = generator.random_range(0, 20);

    match generation_type {
        0..=12 => {
            // 65% - Weighted simple patterns for realistic distribution
            generate_weighted_simple_nation(generator)
        }
        13..=17 => {
            // 25% - Compound patterns with royal/noble themes
            generate_compound_nation(generator)
        }
        _ => {
            // 10% - Democratic compound patterns
            generate_democratic_compound_nation(generator)
        }
    }
}

/// Generate a Western-style house name using compound pattern system
pub fn generate_house_name(generator: &mut NameGenerator) -> String {
    

    // Choose generation method: 70% weighted simple, 30% compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=6 => {
            // 70% - Weighted simple patterns
            generate_weighted_simple_house(generator)
        }
        _ => {
            // 30% - Compound patterns with knightly/royal themes
            generate_compound_house(generator)
        }
    }
}

// ========================================================================
// PRIVATE GENERATION FUNCTIONS
// ========================================================================

/// Generate nation name using weighted simple patterns
fn generate_weighted_simple_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::western_data::*;

    let root = generator.random_choice(NATION_ROOTS);
    let pattern = generator.weighted_choice(WEIGHTED_NATION_PATTERNS);
    pattern.replace("{}", root)
}

/// Generate nation name using compound patterns (Royal/Noble + Structure + Root)
fn generate_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::western_data::*;

    // Focus on royal/noble adjectives for Western culture
    let royal_adjectives = &[
        "Royal",
        "Imperial",
        "Noble",
        "Grand",
        "Great",
        "High",
        "Supreme",
        "Sovereign",
        "Majestic",
        "Regal",
        "Holy",
        "Sacred",
        "Glorious",
        "Victorious",
        "Ancient",
        "Eternal",
    ];

    let adjective = generator.random_choice(royal_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Multiple compound formats for variety
    let formats = [
        "The {} {} of {}", // "The Royal Kingdom of Britannia"
        "{} {} of {}",     // "Imperial Empire of Germania"
        "{} {}",           // "Sacred Realm" (using different root)
        "The {} {}",       // "The Grand Duchy"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} {} of {}" => format!("The {} {} of {}", adjective, structure, root),
        "{} {} of {}" => format!("{} {} of {}", adjective, structure, root),
        "{} {}" => {
            // Use adjective + structure, pick a different root as the base name
            let base_name = generator.random_choice(NATION_ROOTS);
            format!("{} {} of {}", adjective, structure, base_name)
        }
        "The {} {}" => {
            // Use geographic + structure combination
            let geo_adjective = generator.random_choice(GEOGRAPHIC_MODIFIERS);
            format!("The {} {}", geo_adjective, structure)
        }
        _ => format!("{} {} of {}", adjective, structure, root),
    }
}

/// Generate nation name using democratic compound patterns
fn generate_democratic_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::western_data::*;

    // Focus on democratic/freedom adjectives
    let democratic_adjectives = &[
        "United",
        "Free",
        "Allied",
        "Confederated",
        "Federated",
        "Republican",
        "Democratic",
        "Independent",
        "Parliamentary",
        "Constitutional",
        "Progressive",
        "Modern",
        "Reformed",
    ];

    let adjective = generator.random_choice(democratic_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Democratic compound formats
    let formats = [
        "{} {} of {}",     // "United States of Britannia"
        "{} {}",           // "Federal Republic"
        "The {} {} Union", // "The Democratic Kingdom Union"
        "{} {} Alliance",  // "Allied Republic Alliance"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "{} {} of {}" => format!("{} {} of {}", adjective, structure, root),
        "{} {}" => {
            // Use different root as base
            let base_name = generator.random_choice(NATION_ROOTS);
            format!("{} {} of {}", adjective, structure, base_name)
        }
        "The {} {} Union" => format!("The {} {} Union", adjective, structure),
        "{} {} Alliance" => format!("{} {} Alliance", adjective, structure),
        _ => format!("{} {} of {}", adjective, structure, root),
    }
}

/// Generate house name using weighted simple patterns
fn generate_weighted_simple_house(generator: &mut NameGenerator) -> String {
    use super::super::data::western_data::*;

    let house = generator.random_choice(HOUSE_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_HOUSE_PATTERNS);
    pattern.replace("{}", house)
}

/// Generate house name using compound knightly/royal patterns
fn generate_compound_house(generator: &mut NameGenerator) -> String {
    use super::super::data::western_data::*;

    // For houses, use royal/knightly adjectives
    let noble_adjectives = &[
        "Royal",
        "Noble",
        "Ancient",
        "Elder",
        "First",
        "Grand",
        "Great",
        "High",
        "Sacred",
        "Holy",
        "Blessed",
        "Glorious",
        "Victorious",
        "Heroic",
        "Legendary",
    ];

    let adjective = generator.random_choice(noble_adjectives);
    let house = generator.random_choice(HOUSE_NAMES);

    // Knightly/royal compound formats for houses
    let formats = [
        "The {} Order of {}",    // "The Sacred Order of Lionheart"
        "{} House of {}",        // "Royal House of Windsor"
        "The {} {} Brotherhood", // "The Ancient Greystone Brotherhood"
        "{} Knights of {}",      // "Noble Knights of Ravencrest"
        "{} Guard of {}",        // "Royal Guard of Crownguard"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} Order of {}" => format!("The {} Order of {}", adjective, house),
        "{} House of {}" => format!("{} House of {}", adjective, house),
        "The {} {} Brotherhood" => {
            let second_element = generator.random_choice(HOUSE_NAMES);
            format!("The {} {} Brotherhood", adjective, second_element)
        }
        "{} Knights of {}" => format!("{} Knights of {}", adjective, house),
        "{} Guard of {}" => format!("{} Guard of {}", adjective, house),
        _ => format!("The {} House of {}", adjective, house),
    }
}
