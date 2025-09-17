//! World name generation with complex pattern-based logic
//!
//! This module handles the sophisticated world name generation using
//! multiple patterns including compound names, epic names, and themed variations.

use super::core::NameGenerator;
use super::utils::*;

/// Generate a world name using complex pattern-based logic
pub fn generate_world_name(generator: &mut NameGenerator) -> String {
    use super::data::*;

    let pattern = generator.random_range(0, 6);
    match pattern {
        0 => {
            // Prefix + Root + Suffix pattern
            let prefix = generator.random_choice(PREFIXES);
            let root = generator.random_choice(ROOTS);
            let suffix = generator.random_choice(SUFFIXES);
            join_name_parts(&[prefix, root, suffix])
        }
        1 => {
            // Single epic name
            generator.random_choice(EPIC_NAMES).to_string()
        }
        2 => {
            // The + Adjective + Noun pattern
            let adjective = generator.random_choice(ADJECTIVES);
            let noun = generator.random_choice(NOUNS);
            format!("The {} {}", adjective, noun)
        }
        3 => {
            // Root + numeric designation
            let root = generator.random_choice(ROOTS);
            let number = generator.random_range(1, 13) as u32;
            format!("{} {}", root, to_roman_numeral(number))
        }
        4 => {
            // Compound name (two roots)
            let first = generator.random_choice(ROOTS);
            let second = generator.random_choice(ROOTS);
            if first == second {
                format!("{} Prime", first)
            } else {
                format!("{}{}", first, second.to_lowercase())
            }
        }
        _ => {
            // Just a root with optional prefix
            let root = generator.random_choice(ROOTS);
            if generator.random_bool() {
                let prefix = generator.random_choice(PREFIXES);
                format!("{} {}", prefix, root)
            } else {
                root.to_string()
            }
        }
    }
}