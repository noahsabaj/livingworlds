//! Utility functions for name processing
//!
//! This module contains helper functions for generating and processing
//! nation names, including stripping existing government structures.

use crate::name_generator::{Culture, NameGenerator, NameType};

/// Generate the base name component (the actual place/culture name)
pub fn generate_base_name(generator: &mut NameGenerator, culture: Culture) -> String {
    // Use the existing name generator's culture-specific generation
    // but extract just the base name without any government structure
    let full_name = generator.generate(NameType::Nation { culture });

    // Strip common prefixes/suffixes that might have been added
    strip_government_structures(&full_name)
}

/// Strip existing government structures from generated names
pub fn strip_government_structures(name: &str) -> String {
    let mut result = name.to_string();

    // First pass: Remove compound government adjectives
    // These must be removed before the main structures
    let compound_prefixes = [
        "Royal Kingdom of ",
        "Imperial Empire of ",
        "Grand Duchy of ",
        "Noble Republic of ",
        "United Republic of ",
        "Constitutional Monarchy of ",
        "Sovereign State of ",
        "Democratic Union of ",
        "Parliamentary Kingdom of ",
        "Federal Republic of ",
        "Supreme Dominion of ",
        "Free State of ",
        "Allied Federation of ",
    ];

    for prefix in &compound_prefixes {
        if result.starts_with(prefix) {
            result = result[prefix.len()..].to_string();
            break;
        }
    }

    // Second pass: Remove standard government structures with "The"
    let standard_prefixes = [
        "The Kingdom of ",
        "The Empire of ",
        "The Republic of ",
        "The Sultanate of ",
        "The Duchy of ",
        "The Free Cities of ",
        "The Commonwealth of ",
        "The Federation of ",
        "The Union of ",
        "The State of ",
        "The Dominion of ",
        "The Caliphate of ",
        "The Principality of ",
        "The Realm of ",
        "The Territory of ",
        "The Province of ",
    ];

    for prefix in &standard_prefixes {
        if result.starts_with(prefix) {
            result = result[prefix.len()..].to_string();
            break;
        }
    }

    // Third pass: Remove structures without "The" prefix
    let simple_prefixes = [
        "Kingdom of ",
        "Empire of ",
        "Republic of ",
        "Duchy of ",
        "Commonwealth of ",
        "Federation of ",
        "Union of ",
        "State of ",
        "Dominion of ",
        "Caliphate of ",
        "Principality of ",
        "Free Cities of ",
        "Territory of ",
        "Province of ",
    ];

    for prefix in &simple_prefixes {
        if result.starts_with(prefix) {
            result = result[prefix.len()..].to_string();
            break;
        }
    }

    // Fourth pass: Remove any remaining government adjectives at the start
    let adjective_prefixes = [
        "Royal ",
        "Imperial ",
        "Grand ",
        "Noble ",
        "United ",
        "Constitutional ",
        "Sovereign ",
        "Democratic ",
        "Parliamentary ",
        "Federal ",
        "Supreme ",
        "Free ",
        "Allied ",
        "Ancient ",
        "Eternal ",
        "Holy ",
        "Sacred ",
        "Majestic ",
        "Glorious ",
    ];

    // Keep removing adjectives until none are found
    loop {
        let mut removed_any = false;
        for adj in &adjective_prefixes {
            if result.starts_with(adj) {
                result = result[adj.len()..].to_string();
                removed_any = true;
                break;
            }
        }
        if !removed_any {
            break;
        }
    }

    // Fifth pass: Remove suffix patterns (handles patterns like "Britannia Empire")
    let suffixes = [
        " Kingdom",
        " Empire",
        " Republic",
        " Sultanate",
        " Federation",
        " Union",
        " State",
        " Territories",
        " Territory",
        " Dominion",
        " Monarchy",
        " Caliphate",
        " Principality",
        " Realm",
        " Commonwealth",
        " Confederation",
        " Alliance",
    ];

    for suffix in &suffixes {
        if result.ends_with(suffix) {
            result = result[..result.len() - suffix.len()].to_string();
            break;
        }
    }

    // Final cleanup: Remove "The " if it's all that remains at the start
    if result.starts_with("The ") {
        result = result[4..].to_string();
    }

    result
}