//! Name validation for government consistency
//!
//! This module ensures that nation names are logically consistent with their
//! government types, preventing contradictions like anarchist "kingdoms".

use crate::nations::governance::types::{GovernmentType, GovernmentCategory};
use super::generator::get_structure_name;

/// Check if a nation name contains words that contradict its government type
pub fn validate_name_consistency(name: &str, government: GovernmentType) -> Result<(), String> {
    let name_lower = name.to_lowercase();
    let category = government.category();

    // Get forbidden words for this government category
    let forbidden_words = get_forbidden_words(category);

    // Check for any forbidden words in the name
    for word in forbidden_words {
        if name_lower.contains(word) {
            return Err(format!(
                "Name '{}' contains '{}' which contradicts {} government type",
                name, word, get_structure_name(&government)
            ));
        }
    }

    // Additional specific validations
    match government {
        // Anarchist governments shouldn't have hierarchical terms
        GovernmentType::AnarchoSyndicalism
        | GovernmentType::AnarchoCommunism
        | GovernmentType::Mutualism
        | GovernmentType::AnarchoPrimitivism
        | GovernmentType::Egoism => {
            if contains_hierarchical_terms(&name_lower) {
                return Err(format!(
                    "Anarchist government '{}' shouldn't have hierarchical terms in name",
                    name
                ));
            }
        }

        // Democratic governments shouldn't have monarchical terms
        GovernmentType::ParliamentaryDemocracy
        | GovernmentType::PresidentialRepublic
        | GovernmentType::DirectDemocracy
        | GovernmentType::LiquidDemocracy
        | GovernmentType::SortitionDemocracy => {
            if contains_monarchical_terms(&name_lower) {
                return Err(format!(
                    "Democratic government '{}' shouldn't have monarchical terms in name",
                    name
                ));
            }
        }

        _ => {} // Other government types have fewer restrictions
    }

    Ok(())
}

/// Get words that should not appear in names for a given government category
fn get_forbidden_words(category: GovernmentCategory) -> &'static [&'static str] {
    match category {
        GovernmentCategory::Anarchist => &[
            "kingdom", "empire", "royal", "imperial", "crown", "throne",
            "sovereign", "majesty", "realm", "dominion", "dynasty",
            "aristocrat", "noble", "lord", "duke", "baron",
        ],

        GovernmentCategory::Socialist => &[
            "royal", "imperial", "crown", "throne", "majesty",
            "aristocrat", "noble", "lord", "dynasty",
        ],

        GovernmentCategory::Democratic => &[
            "empire", "imperial", "autocrat", "despot", "tyrann",
            "totalitarian", "dictator",
        ],

        GovernmentCategory::Autocratic => &[
            "democratic", "free", "liberal", "anarchist", "commune",
            "cooperative", "collective" // except for some specific fascist types
        ],

        _ => &[], // Other categories have fewer restrictions
    }
}

/// Check if a name contains hierarchical terms
fn contains_hierarchical_terms(name: &str) -> bool {
    let hierarchical_terms = [
        "king", "queen", "emperor", "empress", "prince", "princess",
        "duke", "duchess", "baron", "baroness", "count", "countess",
        "lord", "lady", "noble", "royal", "imperial", "sovereign",
        "majesty", "highness", "excellency", "grace",
    ];

    hierarchical_terms.iter().any(|term| name.contains(term))
}

/// Check if a name contains monarchical terms
fn contains_monarchical_terms(name: &str) -> bool {
    let monarchical_terms = [
        "king", "queen", "emperor", "empress", "monarch",
        "royal", "imperial", "crown", "throne", "dynasty",
        "realm", "majesty", "highness",
    ];

    monarchical_terms.iter().any(|term| name.contains(term))
}

/// Clean a nation name to remove any contradictory elements
///
/// This is a more aggressive version of strip_government_structures that
/// removes ANY government-related words, not just structural patterns.
pub fn clean_nation_name(name: &str) -> String {
    let mut result = name.to_string();

    // List of ALL government-related words to remove
    let government_words = [
        // Titles and hierarchy
        "Royal", "Imperial", "Noble", "Grand", "Supreme", "Sovereign",
        "Majestic", "Glorious", "Eternal", "Ancient", "Holy", "Sacred",
        "Divine", "Blessed",
        
        // Political structures
        "Democratic", "Parliamentary", "Constitutional", "Federal",
        "United", "Allied", "Confederate", "Free", "Independent",
        "Autonomous", "Socialist", "Communist", "Anarchist",
        
        // Government types
        "Kingdom", "Empire", "Republic", "Federation", "Union",
        "State", "Commonwealth", "Dominion", "Territory", "Province",
        "Duchy", "Principality", "Sultanate", "Caliphate", "Realm",
        "Monarchy", "Democracy", "Theocracy", "Oligarchy",
    ];

    // Remove each word (case-insensitive)
    for word in government_words {
        // Remove as a standalone word with spaces
        result = result.replace(&format!(" {} ", word), " ");
        // Remove at start with space after
        if result.starts_with(&format!("{} ", word)) {
            result = result[word.len() + 1..].to_string();
        }
        // Remove at end with space before
        if result.ends_with(&format!(" {}", word)) {
            result = result[..result.len() - word.len() - 1].to_string();
        }
    }

    // Clean up any double spaces
    while result.contains("  ") {
        result = result.replace("  ", " ");
    }

    result.trim().to_string()
}