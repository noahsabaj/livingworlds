//! Island culture name generation
//!
//! Polynesian/Caribbean-inspired naming patterns including maritime kingdoms,
//! tropical paradises, and seafaring-based house structures with volcanic themes.
//!
//! Features three generation systems:
//! 1. **Simple Patterns**: Traditional single-variable replacement (32+ patterns)
//! 2. **Compound Patterns**: Multi-variable combinations with maritime and tropical themes
//! 3. **Weighted Selection**: Realistic distribution with common/uncommon/rare patterns

use super::super::core::NameGenerator;

/// Generate an Island-style nation name using compound pattern system
pub fn generate_nation_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::island_data::*;

    // Choose generation method: 60% weighted simple, 30% compound, 10% volcanic compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=5 => {
            // 60% - Weighted simple patterns for realistic distribution
            generate_weighted_simple_nation(generator)
        }
        6..=8 => {
            // 30% - Compound patterns with maritime/tropical themes
            generate_compound_nation(generator)
        }
        _ => {
            // 10% - Volcanic compound patterns
            generate_volcanic_compound_nation(generator)
        }
    }
}

/// Generate an Island-style house name using compound pattern system
pub fn generate_house_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::island_data::*;

    // Choose generation method: 70% weighted simple, 30% compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=6 => {
            // 70% - Weighted simple patterns
            generate_weighted_simple_house(generator)
        }
        _ => {
            // 30% - Compound patterns with seafaring/tribal themes
            generate_compound_house(generator)
        }
    }
}

// ========================================================================
// PRIVATE GENERATION FUNCTIONS
// ========================================================================

/// Generate nation name using weighted simple patterns
fn generate_weighted_simple_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::island_data::*;

    let root = generator.random_choice(NATION_ROOTS);
    let pattern = generator.weighted_choice(WEIGHTED_NATION_PATTERNS);
    pattern.replace("{}", root)
}

/// Generate nation name using compound patterns (Maritime/Tropical + Structure + Root)
fn generate_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::island_data::*;

    // Focus on maritime/tropical adjectives for Island culture
    let maritime_tropical_adjectives = &[
        "Maritime", "Ocean", "Sea", "Tidal", "Coral", "Pearl", "Blue", "Turquoise",
        "Tropical", "Paradise", "Golden", "Sun", "Sunset", "Palm", "Coconut", "Sacred",
    ];

    let adjective = generator.random_choice(maritime_tropical_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Multiple compound formats for variety
    let formats = [
        "The {} {} of {}",           // "The Maritime Kingdom of Samoa"
        "{} {} of {}",               // "Ocean Empire of Fiji"
        "{} {}",                     // "Tropical Federation" (using different root)
        "The {} {}",                 // "The Sacred Islands"
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
            let geo_modifier = generator.random_choice(GEOGRAPHIC_MODIFIERS);
            format!("The {} {}", geo_modifier, structure)
        }
        _ => format!("{} {} of {}", adjective, structure, root),
    }
}

/// Generate nation name using volcanic compound patterns
fn generate_volcanic_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::island_data::*;

    // Focus on volcanic/geological adjectives
    let volcanic_adjectives = &[
        "Volcanic", "Fire", "Lava", "Magma", "Ash", "Crater", "Hot", "Steam",
        "Geyser", "Thermal", "Rock", "Stone", "Cliff", "Peak", "Ridge", "Sacred",
    ];

    let adjective = generator.random_choice(volcanic_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Volcanic compound formats
    let formats = [
        "{} {} of {}",               // "Volcanic Kingdom of Hawaii"
        "{} {}",                     // "Fire Empire"
        "The {} {} Chain",           // "The Lava Island Chain"
        "{} {} Alliance",            // "Steam Kingdom Alliance"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "{} {} of {}" => format!("{} {} of {}", adjective, structure, root),
        "{} {}" => {
            // Use different root as base
            let base_name = generator.random_choice(NATION_ROOTS);
            format!("{} {} of {}", adjective, structure, base_name)
        }
        "The {} {} Chain" => format!("The {} {} Chain", adjective, structure),
        "{} {} Alliance" => format!("{} {} Alliance", adjective, structure),
        _ => format!("{} {} of {}", adjective, structure, root),
    }
}

/// Generate house name using weighted simple patterns
fn generate_weighted_simple_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::island_data::*;

    let house = generator.random_choice(HOUSE_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_HOUSE_PATTERNS);
    pattern.replace("{}", house)
}

/// Generate house name using compound seafaring/tribal patterns
fn generate_compound_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::island_data::*;

    // For houses, use seafaring/tribal adjectives
    let seafaring_tribal_adjectives = &[
        "Seafaring", "Navigator", "Voyager", "Explorer", "Mariner", "Captain", "Admiral",
        "Noble", "Chief", "Royal", "Sacred", "Ancient", "Warrior", "Mighty", "Great",
    ];

    let adjective = generator.random_choice(seafaring_tribal_adjectives);
    let house = generator.random_choice(HOUSE_NAMES);

    // Seafaring/tribal compound formats for houses
    let formats = [
        "The {} Navigation House of {}",  // "The Seafaring Navigation House of Kamehameha"
        "{} Voyager Clan of {}",          // "Navigator Voyager Clan of Coral"
        "The {} {} Ohana",                // "The Sacred Pearl Ohana"
        "{} Ocean Family of {}",          // "Captain Ocean Family of Turtle"
        "{} Islander House of {}",        // "Mighty Islander House of Palm"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} Navigation House of {}" => format!("The {} Navigation House of {}", adjective, house),
        "{} Voyager Clan of {}" => format!("{} Voyager Clan of {}", adjective, house),
        "The {} {} Ohana" => {
            let second_element = generator.random_choice(HOUSE_NAMES);
            format!("The {} {} Ohana", adjective, second_element)
        }
        "{} Ocean Family of {}" => format!("{} Ocean Family of {}", adjective, house),
        "{} Islander House of {}" => format!("{} Islander House of {}", adjective, house),
        _ => format!("The {} House of {}", adjective, house),
    }
}