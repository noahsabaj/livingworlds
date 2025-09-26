//! Core governance-aware name generation
//!
//! This module provides the main entry point for generating nation names
//! that match their government type.

use crate::name_generator::{Culture, NameGenerator};
use crate::nations::governance::types::{Gender, GovernmentType};

use super::utils::generate_base_name;
use super::formatter::format_nation_name;
use super::validation::{validate_name_consistency, clean_nation_name};

/// Generate a nation name and ruler title based on government type and culture
pub fn generate_governance_aware_name(
    generator: &mut NameGenerator,
    culture: Culture,
    government: &GovernmentType,
) -> (String, String) {
    // Get base name for the nation
    let mut base_name = generate_base_name(generator, culture);

    // Extra cleaning: Remove ALL government-related words to prevent conflicts
    base_name = clean_nation_name(&base_name);

    // Get the government structure name
    let structure = government.structure_name();

    // Format the nation name based on government type
    let mut nation_name = format_nation_name(government, structure, &base_name, culture);

    // Validate the name for consistency
    if let Err(error) = validate_name_consistency(&nation_name, *government) {
        // Log the validation error in debug builds
        #[cfg(debug_assertions)]
        log::warn!("Name validation failed: {}", error);

        // If validation fails, use just the clean base name with government structure
        nation_name = format_nation_name(government, structure, &base_name, culture);
    }

    // Get the ruler title (using neutral gender as default)
    let ruler_title = government.ruler_title(Gender::Neutral).to_string();

    (nation_name, ruler_title)
}

/// Get the appropriate ruler title for a government type
pub fn get_ruler_title(government: &GovernmentType, gender: Gender) -> &'static str {
    government.ruler_title(gender)
}

/// Get the structure name for a government type
pub fn get_structure_name(government: &GovernmentType) -> &'static str {
    government.structure_name()
}