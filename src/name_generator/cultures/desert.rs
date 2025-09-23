//! Desert culture name generation
//!
//! Middle Eastern-inspired naming patterns including caliphates, emirates,
//! and nomadic tribes with oasis and trade route themes.
//!
//! Features three generation systems:
//! 1. **Simple Patterns**: Traditional single-variable replacement (32+ patterns)
//! 2. **Compound Patterns**: Multi-variable combinations with desert and trade themes
//! 3. **Weighted Selection**: Realistic distribution with common/uncommon/rare patterns

use super::super::core::NameGenerator;
use crate::name_generator::data::desert_data::*;

/// Generate a Desert-style nation name using compound pattern system
pub fn generate_nation_name(generator: &mut NameGenerator) -> String {
    

    // Choose generation method: 60% weighted simple, 30% compound, 10% trade compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=5 => {
            // 60% - Weighted simple patterns for realistic distribution
            generate_weighted_simple_nation(generator)
        }
        6..=8 => {
            // 30% - Compound patterns with desert/nomadic themes
            generate_compound_nation(generator)
        }
        _ => {
            // 10% - Trade compound patterns
            generate_trade_compound_nation(generator)
        }
    }
}

/// Generate a Desert-style house name using compound pattern system
pub fn generate_house_name(generator: &mut NameGenerator) -> String {
    

    // Choose generation method: 70% weighted simple, 30% compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=6 => {
            // 70% - Weighted simple patterns
            generate_weighted_simple_house(generator)
        }
        _ => {
            // 30% - Compound patterns with tribal/trade themes
            generate_compound_house(generator)
        }
    }
}

// ========================================================================
// PRIVATE GENERATION FUNCTIONS
// ========================================================================

/// Generate nation name using weighted simple patterns
fn generate_weighted_simple_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::desert_data::*;

    let root = generator.random_choice(NATION_ROOTS);
    let pattern = generator.weighted_choice(WEIGHTED_NATION_PATTERNS);
    pattern.replace("{}", root)
}

/// Generate nation name using compound patterns (Desert/Nomadic + Structure + Root)
fn generate_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::desert_data::*;

    // Focus on desert/nomadic adjectives for Desert culture
    let desert_nomadic_adjectives = &[
        "Golden",
        "Sandy",
        "Scorching",
        "Blazing",
        "Sun-blessed",
        "Desert",
        "Oasis",
        "Wandering",
        "Nomadic",
        "Roaming",
        "Free",
        "Tribal",
        "Proud",
        "Ancient",
        "Mighty",
    ];

    let adjective = generator.random_choice(desert_nomadic_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Multiple compound formats for variety
    let formats = [
        "The {} {} of {}", // "The Golden Caliphate of Babylon"
        "{} {} of {}",     // "Desert Emirate of Arabia"
        "{} {}",           // "Nomadic Federation" (using different root)
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

/// Generate nation name using trade compound patterns
fn generate_trade_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::desert_data::*;

    // Focus on trade/wealth adjectives
    let trade_wealth_adjectives = &[
        "Wealthy",
        "Rich",
        "Merchant",
        "Trading",
        "Silk",
        "Spice",
        "Incense",
        "Pearl",
        "Gold",
        "Jeweled",
        "Prosperous",
        "Thriving",
        "Caravan",
        "Golden",
        "Abundant",
    ];

    let adjective = generator.random_choice(trade_wealth_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Trade compound formats
    let formats = [
        "{} {} of {}",      // "Merchant Caliphate of Samarkand"
        "{} {}",            // "Spice Empire"
        "The {} {} League", // "The Golden Caravan League"
        "{} {} Alliance",   // "Silk Emirate Alliance"
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
    use super::super::data::desert_data::*;

    let house = generator.random_choice(HOUSE_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_HOUSE_PATTERNS);
    pattern.replace("{}", house)
}

/// Generate house name using compound tribal/trade patterns
fn generate_compound_house(generator: &mut NameGenerator) -> String {
    use super::super::data::desert_data::*;

    // For houses, use tribal/trade adjectives
    let tribal_trade_adjectives = &[
        "Ancient", "Noble", "Mighty", "Proud", "Sacred", "Holy", "Blessed", "Faithful", "Golden",
        "Wealthy", "Rich", "Merchant", "Trading", "Silk", "Spice", "Desert",
    ];

    let adjective = generator.random_choice(tribal_trade_adjectives);
    let house = generator.random_choice(HOUSE_NAMES);

    // Tribal/trade compound formats for houses
    let formats = [
        "The {} Trading House of {}", // "The Golden Trading House of Al-Rashid"
        "{} Merchant Tribe of {}",    // "Wealthy Merchant Tribe of Sassanid"
        "The {} {} Confederation",    // "The Ancient Al-Saud Confederation"
        "{} Desert Clan of {}",       // "Sacred Desert Clan of Umayyad"
        "{} Caravan House of {}",     // "Mighty Caravan House of Abbasid"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} Trading House of {}" => format!("The {} Trading House of {}", adjective, house),
        "{} Merchant Tribe of {}" => format!("{} Merchant Tribe of {}", adjective, house),
        "The {} {} Confederation" => {
            let second_element = generator.random_choice(HOUSE_NAMES);
            format!("The {} {} Confederation", adjective, second_element)
        }
        "{} Desert Clan of {}" => format!("{} Desert Clan of {}", adjective, house),
        "{} Caravan House of {}" => format!("{} Caravan House of {}", adjective, house),
        _ => format!("The {} House of {}", adjective, house),
    }
}
