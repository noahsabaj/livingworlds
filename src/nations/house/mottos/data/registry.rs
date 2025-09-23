//! Data access abstraction layer for motto variations
//!
//! This module provides a clean, validated interface to motto data, isolating generation
//! logic from direct data access. It includes data validation, caching, and error handling.

use super::super::super::traits::DominantTrait;
use super::super::types::{MottoError, MottoVariation};
use super::variations::{get_fallback_motto, get_variations_for_trait};
use crate::name_generator::Culture;
use std::collections::HashMap;

/// Registry providing validated access to motto data
///
/// This abstraction layer ensures data consistency, provides caching for performance,
/// and offers a clean API that isolates the generation logic from raw data access.
pub struct MottoRegistry {
    /// Cache of loaded variations to avoid repeated allocation
    variation_cache: HashMap<(DominantTrait, Culture), Vec<MottoVariation>>,
    /// Whether the registry has been validated
    validated: bool,
}

impl MottoRegistry {
    /// Create a new motto registry
    ///
    /// The registry starts unvalidated - call `validate()` or any data access method
    /// to trigger validation and caching.
    pub fn new() -> Self {
        Self {
            variation_cache: HashMap::new(),
            validated: false,
        }
    }

    /// Get all variations for a specific trait and culture
    ///
    /// This is the primary data access method. It handles caching, validation,
    /// and provides a consistent interface regardless of the underlying data structure.
    pub fn get_variations(
        &mut self,
        trait_type: DominantTrait,
        culture: &Culture,
    ) -> Result<&[MottoVariation], MottoError> {
        if !self.validated {
            self.validate_all_data()?;
        }

        let key = (trait_type, *culture);

        // Use cached data if available
        if !self.variation_cache.contains_key(&key) {
            let variations = get_variations_for_trait(trait_type, culture);
            if variations.is_empty() {
                return Err(MottoError::NoEligibleVariations {
                    trait_type,
                    culture: *culture,
                });
            }
            self.variation_cache.insert(key, variations);
        }

        self.variation_cache.get(&key)
            .map(|v| v.as_slice())
            .ok_or_else(|| MottoError::NoEligibleVariations { trait_type, culture: culture.clone() })
    }

    /// Get eligible variations based on trait value and prestige
    ///
    /// This method filters the full variation set to only those that meet the
    /// house's specific requirements, returning a subset ready for selection.
    pub fn get_eligible_variations(
        &mut self,
        trait_type: DominantTrait,
        culture: &Culture,
        trait_value: f32,
        prestige: f32,
    ) -> Result<Vec<&MottoVariation>, MottoError> {
        // Validate input parameters
        if !(0.0..=1.0).contains(&trait_value) {
            return Err(MottoError::InvalidTraitValue { value: trait_value });
        }
        if !(0.0..=1.0).contains(&prestige) {
            return Err(MottoError::InvalidPrestige { value: prestige });
        }

        let all_variations = self.get_variations(trait_type, culture)?;

        let eligible: Vec<&MottoVariation> = all_variations
            .iter()
            .filter(|variation| variation.is_eligible(trait_value, prestige))
            .collect();

        if eligible.is_empty() {
            // Even if no variations qualify, we can provide a fallback
            // This is handled at a higher level, so we return the empty list
            Ok(eligible)
        } else {
            Ok(eligible)
        }
    }

    /// Get a fallback motto for cases where no variations qualify
    ///
    /// Fallback mottos are simple, culturally neutral options that work for any house.
    /// They ensure the system never fails to generate a motto.
    pub fn get_fallback(&self, trait_type: DominantTrait, culture: &Culture) -> &'static str {
        get_fallback_motto(trait_type, culture)
    }

    /// Validate all motto data for consistency and completeness
    ///
    /// This method performs comprehensive validation of the motto data, checking for:
    /// - Coverage across all trait/culture combinations
    /// - Balanced rarity distribution
    /// - Valid requirement values
    /// - Text quality standards
    pub fn validate_all_data(&mut self) -> Result<(), MottoError> {
        let traits = [
            DominantTrait::Martial,
            DominantTrait::Stewardship,
            DominantTrait::Diplomacy,
            DominantTrait::Learning,
            DominantTrait::Intrigue,
            DominantTrait::Piety,
        ];

        let cultures = [
            Culture::Western,
            Culture::Eastern,
            Culture::Northern,
            Culture::Southern,
            Culture::Desert,
            Culture::Island,
            Culture::Ancient,
            Culture::Mystical,
        ];

        for trait_type in &traits {
            for culture in &cultures {
                self.validate_trait_culture_combination(*trait_type, culture)?;
            }
        }

        self.validated = true;
        Ok(())
    }

    /// Validate a specific trait/culture combination
    fn validate_trait_culture_combination(
        &mut self,
        trait_type: DominantTrait,
        culture: &Culture,
    ) -> Result<(), MottoError> {
        let variations = get_variations_for_trait(trait_type, culture);

        if variations.is_empty() {
            return Err(MottoError::DataInconsistency {
                message: format!("No variations found for {:?} / {:?}", trait_type, culture),
            });
        }

        // Check for minimum coverage requirements
        let common_count = variations
            .iter()
            .filter(|v| matches!(v.rarity, super::super::types::MottoRarity::Common))
            .count();
        if common_count < 3 {
            return Err(MottoError::DataInconsistency {
                message: format!(
                    "Insufficient common variations for {:?} / {:?}: {} (minimum 3)",
                    trait_type, culture, common_count
                ),
            });
        }

        // Validate individual variations
        for variation in &variations {
            self.validate_single_variation(variation, trait_type, *culture)?;
        }

        // Cache the validated variations
        let key = (trait_type, *culture);
        self.variation_cache.insert(key, variations);

        Ok(())
    }

    /// Validate a single motto variation
    fn validate_single_variation(
        &self,
        variation: &MottoVariation,
        trait_type: DominantTrait,
        culture: Culture,
    ) -> Result<(), MottoError> {
        // Check text quality
        if variation.text.is_empty() {
            return Err(MottoError::DataInconsistency {
                message: format!("Empty motto text for {:?} / {:?}", trait_type, culture),
            });
        }

        if variation.text.len() > 50 {
            return Err(MottoError::DataInconsistency {
                message: format!(
                    "Motto text too long for {:?} / {:?}: '{}' ({} chars, max 50)",
                    trait_type,
                    culture,
                    variation.text,
                    variation.text.len()
                ),
            });
        }

        // Validate requirement values
        if let Some(min_trait) = variation.min_trait {
            if !(0.0..=1.0).contains(&min_trait) {
                return Err(MottoError::DataInconsistency {
                    message: format!(
                        "Invalid min_trait value for '{}': {} (must be 0.0-1.0)",
                        variation.text, min_trait
                    ),
                });
            }
        }

        if let Some(min_prestige) = variation.min_prestige {
            if !(0.0..=1.0).contains(&min_prestige) {
                return Err(MottoError::DataInconsistency {
                    message: format!(
                        "Invalid min_prestige value for '{}': {} (must be 0.0-1.0)",
                        variation.text, min_prestige
                    ),
                });
            }
        }

        Ok(())
    }

    /// Get statistics about the motto data
    ///
    /// Useful for debugging, balancing, and ensuring adequate coverage across
    /// all trait/culture combinations.
    pub fn get_statistics(&mut self) -> Result<MottoStatistics, MottoError> {
        if !self.validated {
            self.validate_all_data()?;
        }

        let mut stats = MottoStatistics::default();

        for ((trait_type, culture), variations) in &self.variation_cache {
            let trait_culture_stats = TraitCultureStats {
                trait_type: *trait_type,
                culture: *culture,
                total_variations: variations.len(),
                common_count: variations
                    .iter()
                    .filter(|v| matches!(v.rarity, super::super::types::MottoRarity::Common))
                    .count(),
                uncommon_count: variations
                    .iter()
                    .filter(|v| matches!(v.rarity, super::super::types::MottoRarity::Uncommon))
                    .count(),
                rare_count: variations
                    .iter()
                    .filter(|v| matches!(v.rarity, super::super::types::MottoRarity::Rare))
                    .count(),
                legendary_count: variations
                    .iter()
                    .filter(|v| matches!(v.rarity, super::super::types::MottoRarity::Legendary))
                    .count(),
                variations_with_requirements: variations
                    .iter()
                    .filter(|v| v.min_trait.is_some() || v.min_prestige.is_some())
                    .count(),
            };

            stats.total_variations += trait_culture_stats.total_variations;
            stats.trait_culture_breakdown.push(trait_culture_stats);
        }

        stats.trait_culture_combinations = stats.trait_culture_breakdown.len();
        Ok(stats)
    }

    /// Check if the registry has been validated
    pub fn is_validated(&self) -> bool {
        self.validated
    }

    /// Clear the cache and reset validation status
    ///
    /// Useful for testing or if the underlying data has been modified.
    pub fn reset(&mut self) {
        self.variation_cache.clear();
        self.validated = false;
    }
}

impl Default for MottoRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about motto data coverage and distribution
#[derive(Debug, Clone, Default)]
pub struct MottoStatistics {
    pub total_variations: usize,
    pub trait_culture_combinations: usize,
    pub trait_culture_breakdown: Vec<TraitCultureStats>,
}

/// Statistics for a specific trait/culture combination
#[derive(Debug, Clone)]
pub struct TraitCultureStats {
    pub trait_type: DominantTrait,
    pub culture: Culture,
    pub total_variations: usize,
    pub common_count: usize,
    pub uncommon_count: usize,
    pub rare_count: usize,
    pub legendary_count: usize,
    pub variations_with_requirements: usize,
}

impl MottoStatistics {
    /// Generate a human-readable report of the motto data
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("=== Motto Data Statistics ===\n"));
        report.push_str(&format!("Total variations: {}\n", self.total_variations));
        report.push_str(&format!(
            "Trait/Culture combinations: {}\n",
            self.trait_culture_combinations
        ));
        report.push_str(&format!(
            "Average variations per combination: {:.1}\n",
            self.total_variations as f32 / self.trait_culture_combinations as f32
        ));

        report.push_str(&format!("\n=== Coverage by Trait/Culture ===\n"));
        for stats in &self.trait_culture_breakdown {
            report.push_str(&format!(
                "{:?} / {:?}: {} total (C:{} U:{} R:{} L:{}) - {} with requirements\n",
                stats.trait_type,
                stats.culture,
                stats.total_variations,
                stats.common_count,
                stats.uncommon_count,
                stats.rare_count,
                stats.legendary_count,
                stats.variations_with_requirements
            ));
        }

        report
    }
}
