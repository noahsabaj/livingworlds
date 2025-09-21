//! Mystical culture name generation
//!
//! Fantasy/magical-inspired naming patterns including arcane orders,
//! mystical covenants, and enchanted house structures with elemental and celestial themes.
//!
//! Features three generation systems:
//! 1. **Simple Patterns**: Traditional single-variable replacement (32+ patterns)
//! 2. **Compound Patterns**: Multi-variable combinations with arcane and elemental themes
//! 3. **Weighted Selection**: Realistic distribution with common/uncommon/rare patterns

use super::super::core::NameGenerator;

/// Generate a Mystical-style nation name using compound pattern system
pub fn generate_nation_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::mystical_data::*;

    // Choose generation method: 60% weighted simple, 30% compound, 10% planar compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=5 => {
            // 60% - Weighted simple patterns for realistic distribution
            generate_weighted_simple_nation(generator)
        }
        6..=8 => {
            // 30% - Compound patterns with arcane/elemental themes
            generate_compound_nation(generator)
        }
        _ => {
            // 10% - Planar compound patterns
            generate_planar_compound_nation(generator)
        }
    }
}

/// Generate a Mystical-style house name using compound pattern system
pub fn generate_house_name(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::mystical_data::*;

    // Choose generation method: 70% weighted simple, 30% compound
    let generation_type = generator.random_range(0, 10);

    match generation_type {
        0..=6 => {
            // 70% - Weighted simple patterns
            generate_weighted_simple_house(generator)
        }
        _ => {
            // 30% - Compound patterns with mystical/power themes
            generate_compound_house(generator)
        }
    }
}

// ========================================================================
// PRIVATE GENERATION FUNCTIONS
// ========================================================================

/// Generate nation name using weighted simple patterns
fn generate_weighted_simple_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::mystical_data::*;

    let root = generator.random_choice(NATION_ROOTS);
    let pattern = generator.weighted_choice(WEIGHTED_NATION_PATTERNS);
    pattern.replace("{}", root)
}

/// Generate nation name using compound patterns (Arcane/Elemental + Structure + Root)
fn generate_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::mystical_data::*;

    // Focus on arcane/elemental adjectives for Mystical culture
    let arcane_elemental_adjectives = &[
        "Arcane",
        "Mystical",
        "Magical",
        "Enchanted",
        "Sacred",
        "Divine",
        "Blessed",
        "Cursed",
        "Fire",
        "Shadow",
        "Lightning",
        "Crystal",
        "Void",
        "Ethereal",
        "Astral",
        "Celestial",
    ];

    let adjective = generator.random_choice(arcane_elemental_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Multiple compound formats for variety
    let formats = [
        "The {} {} of {}", // "The Arcane Covenant of Mystara"
        "{} {} of {}",     // "Mystical Order of Ethereal"
        "{} {}",           // "Enchanted Conclave" (using different root)
        "The {} {}",       // "The Sacred Sanctum"
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

/// Generate nation name using planar compound patterns
fn generate_planar_compound_nation(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::mystical_data::*;

    // Focus on planar/dimensional adjectives
    let planar_adjectives = &[
        "Planar",
        "Dimensional",
        "Otherworldly",
        "Transcendent",
        "Ascendant",
        "Supreme",
        "Ultimate",
        "Eternal",
        "Infinite",
        "Timeless",
        "Omnipotent",
        "All-knowing",
        "Immortal",
        "Everlasting",
    ];

    let adjective = generator.random_choice(planar_adjectives);
    let structure = generator.random_choice(POLITICAL_STRUCTURES);
    let root = generator.random_choice(NATION_ROOTS);

    // Planar compound formats
    let formats = [
        "{} {} of {}",        // "Transcendent Covenant of Elysium"
        "{} {}",              // "Ultimate Order"
        "The {} {} Dominion", // "The Eternal Conclave Dominion"
        "{} {} Ascendancy",   // "Supreme Sanctum Ascendancy"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "{} {} of {}" => format!("{} {} of {}", adjective, structure, root),
        "{} {}" => {
            // Use different root as base
            let base_name = generator.random_choice(NATION_ROOTS);
            format!("{} {} of {}", adjective, structure, base_name)
        }
        "The {} {} Dominion" => format!("The {} {} Dominion", adjective, structure),
        "{} {} Ascendancy" => format!("{} {} Ascendancy", adjective, structure),
        _ => format!("{} {} of {}", adjective, structure, root),
    }
}

/// Generate house name using weighted simple patterns
fn generate_weighted_simple_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::mystical_data::*;

    let house = generator.random_choice(HOUSE_NAMES);
    let pattern = generator.weighted_choice(WEIGHTED_HOUSE_PATTERNS);
    pattern.replace("{}", house)
}

/// Generate house name using compound mystical/power patterns
fn generate_compound_house(generator: &mut NameGenerator) -> String {
    use super::super::data::cultures::mystical_data::*;

    // For houses, use mystical/power adjectives
    let mystical_power_adjectives = &[
        "Arcane",
        "Mystical",
        "Sacred",
        "Divine",
        "Ancient",
        "Eternal",
        "Immortal",
        "Supreme",
        "Fire",
        "Shadow",
        "Crystal",
        "Void",
        "Astral",
        "Celestial",
        "All-knowing",
        "Omnipotent",
    ];

    let adjective = generator.random_choice(mystical_power_adjectives);
    let house = generator.random_choice(HOUSE_NAMES);

    // Mystical/power compound formats for houses
    let formats = [
        "The {} Arcane House of {}", // "The Sacred Arcane House of Shadowmere"
        "{} Mystical Circle of {}",  // "Divine Mystical Circle of Starweaver"
        "The {} {} Covenant",        // "The Ancient Nightfall Covenant"
        "{} Spellbound Order of {}", // "Eternal Spellbound Order of Crystalborn"
        "{} Enchanted House of {}",  // "Supreme Enchanted House of Dragonborn"
    ];

    let format = generator.random_choice(&formats);
    match *format {
        "The {} Arcane House of {}" => format!("The {} Arcane House of {}", adjective, house),
        "{} Mystical Circle of {}" => format!("{} Mystical Circle of {}", adjective, house),
        "The {} {} Covenant" => {
            let second_element = generator.random_choice(HOUSE_NAMES);
            format!("The {} {} Covenant", adjective, second_element)
        }
        "{} Spellbound Order of {}" => format!("{} Spellbound Order of {}", adjective, house),
        "{} Enchanted House of {}" => format!("{} Enchanted House of {}", adjective, house),
        _ => format!("The {} House of {}", adjective, house),
    }
}
