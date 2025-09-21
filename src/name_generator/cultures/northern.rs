//! Northern culture name generation
//!
//! Nordic/Slavic-inspired naming patterns including clans, jarls,
//! and warrior-based house structures with harsh winter themes.
//!
//! Features three generation systems:
//! 1. **Simple Patterns**: Traditional single-variable replacement (32+ patterns)
//! 2. **Compound Patterns**: Multi-variable combinations with warrior and winter themes
//! 3. **Weighted Selection**: Realistic distribution with common/uncommon/rare patterns

use super::super::core::NameGenerator;

/// Generate a Northern-style nation name using compound pattern system
pub fn generate_nation_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::northern_data::*;

    // Choose generation method: 60% weighted simple, 30% compound, 10% winter compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=5 => {
            // 60% - Weighted simple patterns for realistic distribution
            let root = generator.random_choice(NATION_ROOTS);
            let pattern = generator.weighted_choice(WEIGHTED_NATION_PATTERNS);
            pattern.replace("{}", root)
        }
        6..=8 => {
            // 30% - Compound patterns with warrior themes
            let warrior_adjectives = &[
                "Mighty", "Fierce", "Iron", "Thunder", "Storm", "Wolf", "Bear", "Battle",
            ];
            let adjective = generator.random_choice(warrior_adjectives);
            let structure = generator.random_choice(POLITICAL_STRUCTURES);
            let root = generator.random_choice(NATION_ROOTS);
            format!("{} {} of {}", adjective, structure, root)
        }
        _ => {
            // 10% - Winter compound patterns
            let winter_adjectives = &[
                "Frost", "Ice", "Snow", "Winter", "Frozen", "Crystal", "White", "Cold",
            ];
            let adjective = generator.random_choice(winter_adjectives);
            let structure = generator.random_choice(POLITICAL_STRUCTURES);
            let root = generator.random_choice(NATION_ROOTS);
            format!("{} {} of {}", adjective, structure, root)
        }
    }
}

/// Generate a Northern-style house name using compound pattern system
pub fn generate_house_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::northern_data::*;

    // Choose generation method: 70% weighted simple, 30% compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=6 => {
            // 70% - Weighted simple patterns for realistic distribution
            generate_weighted_simple_house(generator)
        }
        _ => {
            // 30% - Compound patterns with warrior/winter themes
            generate_compound_house(generator)
        }
    }
}

// ========================================================================
// PRIVATE GENERATION FUNCTIONS
// ========================================================================

/// Generate house name using weighted simple patterns
fn generate_weighted_simple_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::northern_data::*;

    let house = generator.random_choice(HOUSE_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_HOUSE_PATTERNS);
    pattern.replace("{}", house)
}

/// Generate house name using compound warrior/winter patterns
fn generate_compound_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::northern_data::*;

    // For houses, use warrior/winter adjectives
    let warrior_winter_adjectives = &[
        "Iron", "Steel", "Thunder", "Storm", "Wolf", "Bear", "Battle", "War", "Frost", "Ice",
        "Snow", "Winter", "Frozen", "Crystal", "White", "Cold", "Ancient", "Elder", "First",
        "Great", "Sacred", "Noble", "True", "Mighty",
    ];

    let adjective = generator.random_choice(warrior_winter_adjectives);
    let house = generator.random_choice(HOUSE_NAMES);

    // Warrior/winter compound formats for houses
    let formats = [
        "The {} {} Brotherhood", // "The Iron Wolf Brotherhood"
        "{} House of {}",        // "Storm House of Ironwolf"
        "The {} {} Clan",        // "The Ancient Bear Clan"
        "{} Warriors of {}",     // "Frost Warriors of Stormaxe"
        "{} Guard of {}",        // "Thunder Guard of Winterborn"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} {} Brotherhood" => {
            let second_element = generator.random_choice(HOUSE_NAMES);
            format!("The {} {} Brotherhood", adjective, second_element)
        }
        "{} House of {}" => format!("{} House of {}", adjective, house),
        "The {} {} Clan" => {
            let second_element = generator.random_choice(HOUSE_NAMES);
            format!("The {} {} Clan", adjective, second_element)
        }
        "{} Warriors of {}" => format!("{} Warriors of {}", adjective, house),
        "{} Guard of {}" => format!("{} Guard of {}", adjective, house),
        _ => format!("The {} House of {}", adjective, house),
    }
}
