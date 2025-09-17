//! Ancient culture name generation
//!
//! Lost civilization-inspired naming patterns including lost empires,
//! primordial dynasties, and mysterious ancient house structures.
//!
//! Features three generation systems:
//! 1. **Simple Patterns**: Traditional single-variable replacement (25+ patterns)
//! 2. **Compound Patterns**: Multi-variable combinations with adjectives and geographic modifiers
//! 3. **Weighted Selection**: Realistic distribution with common/uncommon/rare patterns

use super::super::core::NameGenerator;

/// Generate an Ancient-style nation name using compound pattern system
pub fn generate_nation_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::ancient_data::*;

    // Choose generation method: 60% weighted simple, 30% compound, 10% geographic compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=5 => {
            // 60% - Weighted simple patterns for realistic distribution
            generate_weighted_simple_nation(generator)
        }
        6..=8 => {
            // 30% - Compound patterns with adjectives
            generate_compound_nation(generator)
        }
        _ => {
            // 10% - Geographic compound patterns
            generate_geographic_compound_nation(generator)
        }
    }
}

/// Generate an Ancient-style house name using compound pattern system
pub fn generate_house_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::ancient_data::*;

    // Choose generation method: 70% weighted simple, 30% compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=6 => {
            // 70% - Weighted simple patterns
            generate_weighted_simple_house(generator)
        }
        _ => {
            // 30% - Compound patterns with mystical themes
            generate_compound_house(generator)
        }
    }
}

// ========================================================================
// PRIVATE GENERATION FUNCTIONS
// ========================================================================

/// Generate nation name using weighted simple patterns
fn generate_weighted_simple_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::ancient_data::*;

    let root = generator.random_choice(NATION_ROOTS);
    let pattern = generator.weighted_choice(WEIGHTED_NATION_PATTERNS);
    pattern.replace("{}", root)
}

/// Generate nation name using compound patterns (Adjective + Structure + Root)
fn generate_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::ancient_data::*;

    let adjective = generator.random_choice(ANCIENT_ADJECTIVES);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Multiple compound formats for variety
    let formats = [
        "The {} {} of {}",           // "The Sacred Empire of Atlantis"
        "{} {} of {}",               // "Divine Kingdom of Lemuria"
        "{} {}",                     // "Eternal Dominion" (adjective modifies structure)
        "The {} {}",                 // "The Ancient Imperium"
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
            // Geographic adjective + structure
            let geo_adjective = generator.random_choice(GEOGRAPHIC_MODIFIERS);
            format!("The {} {}", geo_adjective, structure)
        }
        _ => format!("{} {} of {}", adjective, structure, root),
    }
}

/// Generate nation name using geographic compound patterns
fn generate_geographic_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::ancient_data::*;

    let geo_modifier = generator.random_choice(GEOGRAPHIC_MODIFIERS);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Geographic compound formats
    let formats = [
        "{} {} of {}",               // "Northern Empire of Atlantis"
        "{} {}",                     // "Highland Kingdom"
        "The {} {} Territories",     // "The Eastern Imperium Territories"
        "{} {} Federation",          // "Coastal Dominion Federation"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "{} {} of {}" => format!("{} {} of {}", geo_modifier, structure, root),
        "{} {}" => {
            // Use a different root as the base name
            let base_name = generator.random_choice(NATION_ROOTS);
            format!("{} {} of {}", geo_modifier, structure, base_name)
        }
        "The {} {} Territories" => format!("The {} {} Territories", geo_modifier, structure),
        "{} {} Federation" => format!("{} {} Federation", geo_modifier, structure),
        _ => format!("{} {} of {}", geo_modifier, structure, root),
    }
}

/// Generate house name using weighted simple patterns
fn generate_weighted_simple_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::ancient_data::*;

    let house = generator.random_choice(HOUSE_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_HOUSE_PATTERNS);
    pattern.replace("{}", house)
}

/// Generate house name using compound mystical patterns
fn generate_compound_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::ancient_data::*;

    // For houses, use mystical/temporal adjectives more often
    let mystical_adjectives = &[
        "Sacred", "Divine", "Eternal", "Ancient", "Primordial", "Celestial",
        "Transcendent", "Enlightened", "Illuminated", "Crystalline", "Astral",
    ];

    let adjective = generator.random_choice(mystical_adjectives);
    let house = generator.random_choice(HOUSE_NAMES);

    // Mystical compound formats for houses
    let formats = [
        "The {} Order of {}",        // "The Sacred Order of Atlantean"
        "{} Lineage of {}",          // "Divine Lineage of Starborn"
        "{} {} Bloodline",           // "Ancient Primordial Bloodline"
        "The {} {} Circle",          // "The Eternal Crystalline Circle"
        "{} Guardians of {}",        // "Celestial Guardians of Orichalcum"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} Order of {}" => format!("The {} Order of {}", adjective, house),
        "{} Lineage of {}" => format!("{} Lineage of {}", adjective, house),
        "{} {} Bloodline" => {
            let second_adjective = generator.random_choice(mystical_adjectives);
            format!("{} {} Bloodline", adjective, second_adjective)
        }
        "The {} {} Circle" => {
            let material = generator.random_choice(&["Crystal", "Obsidian", "Mithril", "Adamantine"]);
            format!("The {} {} Circle", adjective, material)
        }
        "{} Guardians of {}" => format!("{} Guardians of {}", adjective, house),
        _ => format!("The {} House of {}", adjective, house),
    }
}