//! Eastern culture name generation
//!
//! Asian-inspired naming patterns including dynasties, celestial empires,
//! and honor-based house structures with traditional elements.
//!
//! Features three generation systems:
//! 1. **Simple Patterns**: Traditional single-variable replacement (32+ patterns)
//! 2. **Compound Patterns**: Multi-variable combinations with celestial and elemental themes
//! 3. **Weighted Selection**: Realistic distribution with common/uncommon/rare patterns

use super::super::core::NameGenerator;

/// Generate an Eastern-style nation name using compound pattern system
pub fn generate_nation_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::eastern_data::*;

    // Choose generation method: 60% weighted simple, 30% compound, 10% elemental compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=5 => {
            // 60% - Weighted simple patterns for realistic distribution
            generate_weighted_simple_nation(generator)
        }
        6..=8 => {
            // 30% - Compound patterns with celestial/honor themes
            generate_compound_nation(generator)
        }
        _ => {
            // 10% - Elemental compound patterns
            generate_elemental_compound_nation(generator)
        }
    }
}

/// Generate an Eastern-style house name using compound pattern system
pub fn generate_house_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::eastern_data::*;

    // Choose generation method: 70% weighted simple, 30% compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=6 => {
            // 70% - Weighted simple patterns
            generate_weighted_simple_house(generator)
        }
        _ => {
            // 30% - Compound patterns with honor/way themes
            generate_compound_house(generator)
        }
    }
}

// ========================================================================
// PRIVATE GENERATION FUNCTIONS
// ========================================================================

/// Generate nation name using weighted simple patterns
fn generate_weighted_simple_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::eastern_data::*;

    // Use dynasties for Eastern naming (more appropriate than generic roots)
    let dynasty = generator.random_choice(DYNASTY_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_NATION_PATTERNS);
    pattern.replace("{}", dynasty)
}

/// Generate nation name using compound patterns (Celestial/Honor + Structure + Dynasty)
fn generate_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::eastern_data::*;

    // Focus on celestial/honor adjectives for Eastern culture
    let celestial_adjectives = &[
        "Celestial",
        "Divine",
        "Heavenly",
        "Sacred",
        "Holy",
        "Blessed",
        "Eternal",
        "Noble",
        "Righteous",
        "Harmonious",
        "Peaceful",
        "Wise",
        "Virtuous",
        "Pure",
    ];

    let adjective = generator.random_choice(celestial_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let dynasty = generator.random_choice(DYNASTY_NAMES);

    // Multiple compound formats for variety
    let formats = [
        "The {} {} of {}", // "The Celestial Empire of Ming"
        "{} {} of {}",     // "Divine Dynasty of Tang"
        "{} {}",           // "Sacred Harmony" (using different base)
        "The {} {}",       // "The Heavenly Court"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} {} of {}" => format!("The {} {} of {}", adjective, structure, dynasty),
        "{} {} of {}" => format!("{} {} of {}", adjective, structure, dynasty),
        "{} {}" => {
            // Use adjective + structure, pick a different dynasty as base
            let base_dynasty = generator.random_choice(DYNASTY_NAMES);
            format!("{} {} of {}", adjective, structure, base_dynasty)
        }
        "The {} {}" => {
            // Use celestial + structure combination
            let celestial_mod = generator.random_choice(GEOGRAPHIC_MODIFIERS);
            format!("The {} {}", celestial_mod, structure)
        }
        _ => format!("{} {} of {}", adjective, structure, dynasty),
    }
}

/// Generate nation name using elemental compound patterns
fn generate_elemental_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::eastern_data::*;

    // Focus on elemental adjectives
    let elemental_adjectives = &[
        "Fire",
        "Water",
        "Earth",
        "Metal",
        "Wood",
        "Thunder",
        "Lightning",
        "Storm",
        "Wind",
        "Rain",
        "Snow",
        "Ice",
        "Dragon",
        "Phoenix",
        "Tiger",
        "Crane",
    ];

    let element = generator.random_choice(elemental_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let dynasty = generator.random_choice(DYNASTY_NAMES);

    // Elemental compound formats
    let formats = [
        "{} {} of {}",        // "Fire Dynasty of Ming"
        "{} {}",              // "Dragon Empire"
        "The {} {} Alliance", // "The Thunder Kingdom Alliance"
        "{} {} Harmony",      // "Water Empire Harmony"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "{} {} of {}" => format!("{} {} of {}", element, structure, dynasty),
        "{} {}" => {
            // Use different dynasty as base
            let base_dynasty = generator.random_choice(DYNASTY_NAMES);
            format!("{} {} of {}", element, structure, base_dynasty)
        }
        "The {} {} Alliance" => format!("The {} {} Alliance", element, structure),
        "{} {} Harmony" => format!("{} {} Harmony", element, structure),
        _ => format!("{} {} of {}", element, structure, dynasty),
    }
}

/// Generate house name using weighted simple patterns
fn generate_weighted_simple_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::eastern_data::*;

    let house = generator.random_choice(HOUSE_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_HOUSE_PATTERNS);
    pattern.replace("{}", house)
}

/// Generate house name using compound honor/way patterns
fn generate_compound_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::eastern_data::*;

    // For houses, use honor/virtue adjectives
    let honor_adjectives = &[
        "Sacred",
        "Noble",
        "Ancient",
        "Elder",
        "Wise",
        "Righteous",
        "Honorable",
        "Virtuous",
        "Pure",
        "Peaceful",
        "Harmonious",
        "Celestial",
        "Divine",
        "Holy",
    ];

    let adjective = generator.random_choice(honor_adjectives);
    let house = generator.random_choice(HOUSE_NAMES);

    // Honor/way compound formats for houses
    let formats = [
        "The {} Way of {}", // "The Sacred Way of Golden Dragon"
        "{} School of {}",  // "Noble School of Jade Emperor"
        "The {} {} Circle", // "The Ancient Tiger Circle"
        "{} Order of {}",   // "Wise Order of Phoenix"
        "{} Path of {}",    // "Righteous Path of Lotus"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} Way of {}" => format!("The {} Way of {}", adjective, house),
        "{} School of {}" => format!("{} School of {}", adjective, house),
        "The {} {} Circle" => {
            let second_element = generator.random_choice(HOUSE_NAMES);
            format!("The {} {} Circle", adjective, second_element)
        }
        "{} Order of {}" => format!("{} Order of {}", adjective, house),
        "{} Path of {}" => format!("{} Path of {}", adjective, house),
        _ => format!("The {} House of {}", adjective, house),
    }
}
