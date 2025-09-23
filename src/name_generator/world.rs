//! World name generation for single-word planet names
//!
//! This module generates planet names as single words, similar to
//! real celestial bodies like Earth, Mars, Venus, Jupiter, etc.

use super::core::NameGenerator;

/// Generate a single-word planet name
pub fn generate_world_name(generator: &mut NameGenerator) -> String {
    use super::data::*;

    let pattern = generator.random_range(0, 5);
    match pattern {
        0 => {
            // Direct root word (most common)
            generator.random_choice(ROOTS).to_string()
        }
        1 => {
            // Direct epic name
            generator.random_choice(EPIC_NAMES).to_string()
        }
        2 => {
            // Compound without spaces (concatenate two roots)
            let first = generator.random_choice(ROOTS);
            let second = generator.random_choice(ROOTS);
            if first == second {
                // If same root selected twice, just use it once
                first.to_string()
            } else {
                // Combine them into one word
                format!("{}{}", first, second.to_lowercase())
            }
        }
        3 => {
            // Root with simple suffix (no space)
            let root = generator.random_choice(ROOTS);
            let suffixes = ["is", "on", "os", "us", "ia", "ara", "ium", "ion"];
            let suffix = generator.random_choice(&suffixes);
            format!("{}{}", root, suffix)
        }
        _ => {
            // Modified root (slight variation)
            let root = generator.random_choice(ROOTS);
            // Remove last character and add new ending
            if root.len() > 3 {
                let truncated = &root[..root.len()-1];
                let endings = ["ix", "ox", "ar", "or", "an", "en"];
                let ending = generator.random_choice(&endings);
                format!("{}{}", truncated, ending)
            } else {
                root.to_string()
            }
        }
    }
}
