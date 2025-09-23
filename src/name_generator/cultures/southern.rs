//! Southern culture name generation
//!
//! Mediterranean/African-inspired naming patterns including trade empires,
//! ancient civilizations, and maritime-based house structures with sun and desert themes.
//!
//! Features three generation systems:
//! 1. **Simple Patterns**: Traditional single-variable replacement (32+ patterns)
//! 2. **Compound Patterns**: Multi-variable combinations with trade and maritime themes
//! 3. **Weighted Selection**: Realistic distribution with common/uncommon/rare patterns

use super::super::core::NameGenerator;
use crate::name_generator::data::southern_data::*;

/// Generate a Southern-style nation name using compound pattern system
pub fn generate_nation_name(generator: &mut NameGenerator) -> String {
    

    // Choose generation method: 60% weighted simple, 30% compound, 10% maritime compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=5 => {
            // 60% - Weighted simple patterns for realistic distribution
            generate_weighted_simple_nation(generator)
        }
        6..=8 => {
            // 30% - Compound patterns with trade/ancient themes
            generate_compound_nation(generator)
        }
        _ => {
            // 10% - Maritime compound patterns
            generate_maritime_compound_nation(generator)
        }
    }
}

/// Generate a Southern-style house name using compound pattern system
pub fn generate_house_name(generator: &mut NameGenerator) -> String {
    

    // Choose generation method: 70% weighted simple, 30% compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=6 => {
            // 70% - Weighted simple patterns
            generate_weighted_simple_house(generator)
        }
        _ => {
            // 30% - Compound patterns with trade/noble themes
            generate_compound_house(generator)
        }
    }
}

// ========================================================================
// PRIVATE GENERATION FUNCTIONS
// ========================================================================

/// Generate nation name using weighted simple patterns
fn generate_weighted_simple_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::southern_data::*;

    let root = generator.random_choice(NATION_ROOTS);
    let pattern = generator.weighted_choice(WEIGHTED_NATION_PATTERNS);
    pattern.replace("{}", root)
}

/// Generate nation name using compound patterns (Trade/Ancient + Structure + Root)
fn generate_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::southern_data::*;

    // Focus on trade/ancient adjectives for Southern culture
    let trade_ancient_adjectives = &[
        "Golden",
        "Rich",
        "Prosperous",
        "Wealthy",
        "Merchant",
        "Trading",
        "Commercial",
        "Ancient",
        "Classical",
        "Imperial",
        "Roman",
        "Egyptian",
        "Pharaonic",
        "Eternal",
    ];

    let adjective = generator.random_choice(trade_ancient_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Multiple compound formats for variety
    let formats = [
        "The {} {} of {}", // "The Golden Republic of Carthago"
        "{} {} of {}",     // "Imperial Empire of Roma"
        "{} {}",           // "Wealthy League" (using different root)
        "The {} {}",       // "The Ancient Sultanate"
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

/// Generate nation name using maritime compound patterns
fn generate_maritime_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::southern_data::*;

    // Focus on maritime/coastal adjectives
    let maritime_adjectives = &[
        "Maritime",
        "Coastal",
        "Naval",
        "Seafaring",
        "Oceanic",
        "Harbor",
        "Port",
        "Pearl",
        "Coral",
        "Tidal",
        "Nautical",
        "Marine",
        "Mediterranean",
        "Adriatic",
        "Aegean",
    ];

    let adjective = generator.random_choice(maritime_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Maritime compound formats
    let formats = [
        "{} {} of {}",      // "Maritime Republic of Venetia"
        "{} {}",            // "Coastal Empire"
        "The {} {} League", // "The Naval Republic League"
        "{} {} Alliance",   // "Harbor States Alliance"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "{} {} of {}" => format!("{} {} of {}", adjective, structure, root),
        "{} {}" => {
            // Use different root as base
            let base_name = generator.random_choice(NATION_ROOTS);
            format!("{} {} of {}", adjective, structure, base_name)
        }
        "The {} {} League" => format!("The {} {} League", adjective, structure),
        "{} {} Alliance" => format!("{} {} Alliance", adjective, structure),
        _ => format!("{} {} of {}", adjective, structure, root),
    }
}

/// Generate house name using weighted simple patterns
fn generate_weighted_simple_house(generator: &mut NameGenerator) -> String {
    use super::super::data::southern_data::*;

    let house = generator.random_choice(HOUSE_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_HOUSE_PATTERNS);
    pattern.replace("{}", house)
}

/// Generate house name using compound trade/noble patterns
fn generate_compound_house(generator: &mut NameGenerator) -> String {
    use super::super::data::southern_data::*;

    // For houses, use trade/noble adjectives
    let noble_trade_adjectives = &[
        "Golden",
        "Rich",
        "Wealthy",
        "Merchant",
        "Trading",
        "Commercial",
        "Prosperous",
        "Ancient",
        "Imperial",
        "Classical",
        "Noble",
        "Royal",
        "Patrician",
        "Aristocratic",
    ];

    let adjective = generator.random_choice(noble_trade_adjectives);
    let house = generator.random_choice(HOUSE_NAMES);

    // Trade/noble compound formats for houses
    let formats = [
        "The {} Trading House of {}", // "The Golden Trading House of Medici"
        "{} Merchant Family of {}",   // "Wealthy Merchant Family of Borgia"
        "The {} {} Banking House",    // "The Ancient Medici Banking House"
        "{} Noble House of {}",       // "Imperial Noble House of Aurelius"
        "{} Maritime Family of {}",   // "Prosperous Maritime Family of Visconti"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} Trading House of {}" => format!("The {} Trading House of {}", adjective, house),
        "{} Merchant Family of {}" => format!("{} Merchant Family of {}", adjective, house),
        "The {} {} Banking House" => {
            let second_element = generator.random_choice(HOUSE_NAMES);
            format!("The {} {} Banking House", adjective, second_element)
        }
        "{} Noble House of {}" => format!("{} Noble House of {}", adjective, house),
        "{} Maritime Family of {}" => format!("{} Maritime Family of {}", adjective, house),
        _ => format!("The {} House of {}", adjective, house),
    }
}
