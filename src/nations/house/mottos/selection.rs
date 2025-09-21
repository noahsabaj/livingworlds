//! Motto selection system with rarity weighting and intelligent filtering
//!
//! This module handles the complex logic of selecting the most appropriate motto
//! from eligible variations, taking into account rarity weights, randomness,
//! and house characteristics.

use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

use super::super::traits::{DominantTrait, HouseTraits};
use super::data::MottoRegistry;
use super::types::{MottoError, MottoVariation};
use crate::name_generator::Culture;

/// Configuration for motto selection behavior
#[derive(Debug, Clone)]
pub struct SelectionConfig {
    /// Boost factor for higher prestige houses when selecting rare mottos
    pub prestige_boost: f32,
    /// Minimum number of eligible variations before applying prestige boost
    pub min_variations_for_boost: usize,
    /// Whether to prefer variations that exactly match trait thresholds
    pub prefer_threshold_matches: bool,
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            prestige_boost: 1.5,
            min_variations_for_boost: 5,
            prefer_threshold_matches: true,
        }
    }
}

/// Intelligent motto selector with configurable behavior
pub struct MottoSelector {
    registry: MottoRegistry,
    config: SelectionConfig,
}

impl MottoSelector {
    /// Create a new motto selector with default configuration
    pub fn new() -> Self {
        Self {
            registry: MottoRegistry::new(),
            config: SelectionConfig::default(),
        }
    }

    /// Create a new motto selector with custom configuration
    pub fn with_config(config: SelectionConfig) -> Self {
        Self {
            registry: MottoRegistry::new(),
            config,
        }
    }

    /// Select the best motto for a house based on its dominant trait
    ///
    /// This is the primary selection method that combines eligibility filtering,
    /// rarity weighting, and intelligent selection heuristics.
    pub fn select_motto_for_trait(
        &mut self,
        trait_type: DominantTrait,
        trait_value: f32,
        culture: &Culture,
        prestige: f32,
        rng: &mut impl Rng,
    ) -> Result<String, MottoError> {
        // Get eligible variations from the registry (clone to avoid borrow issues)
        let eligible_variations = {
            self.registry
                .get_eligible_variations(trait_type, culture, trait_value, prestige)?
                .into_iter()
                .cloned()
                .collect::<Vec<MottoVariation>>()
        };

        if eligible_variations.is_empty() {
            // No variations qualified - use fallback
            let fallback = self.registry.get_fallback(trait_type, culture);
            return Ok(fallback.to_string());
        }

        // Select using weighted rarity system (no borrow conflict now)
        let eligible_refs: Vec<&MottoVariation> = eligible_variations.iter().collect();
        let selected = self.select_by_rarity_with_intelligence(
            &eligible_refs,
            trait_value,
            prestige,
            rng,
        )?;

        Ok(selected.text.to_string())
    }

    /// Advanced selection with prestige boosting and intelligent heuristics
    fn select_by_rarity_with_intelligence<'a>(
        &self,
        variations: &'a [&'a MottoVariation],
        trait_value: f32,
        prestige: f32,
        rng: &mut impl Rng,
    ) -> Result<&'a MottoVariation, MottoError> {
        if variations.is_empty() {
            return Err(MottoError::DataInconsistency {
                message: "No variations provided for selection".to_string(),
            });
        }

        // Calculate weights with intelligent boosting
        let weights: Vec<f32> = variations
            .iter()
            .map(|variation| self.calculate_selection_weight(variation, trait_value, prestige))
            .collect();

        // Use weighted selection
        let dist = WeightedIndex::new(&weights).map_err(|e| MottoError::DataInconsistency {
            message: format!("Failed to create weighted distribution: {}", e),
        })?;

        let selected_index = dist.sample(rng);
        Ok(variations[selected_index])
    }

    /// Calculate selection weight with intelligent boosting based on house characteristics
    fn calculate_selection_weight(
        &self,
        variation: &MottoVariation,
        trait_value: f32,
        prestige: f32,
    ) -> f32 {
        let mut weight = variation.rarity.selection_weight();

        // Apply prestige boost for rare mottos if configured
        if self.config.prestige_boost > 1.0 {
            match variation.rarity {
                super::types::MottoRarity::Rare => {
                    weight *= 1.0 + (prestige * (self.config.prestige_boost - 1.0));
                }
                super::types::MottoRarity::Legendary => {
                    weight *= 1.0 + (prestige * prestige * (self.config.prestige_boost - 1.0));
                }
                _ => {} // No boost for common/uncommon
            }
        }

        // Prefer variations that exactly match trait requirements if configured
        if self.config.prefer_threshold_matches {
            if let Some(min_trait) = variation.min_trait {
                let trait_excess = trait_value - min_trait;
                if trait_excess < 0.1 {
                    // Close to minimum requirement
                    weight *= 1.2; // Slight preference for "just qualified" mottos
                }
            }
        }

        weight
    }

    /// Simple selection using base rarity weights only
    ///
    /// This is a fallback method that uses only the raw rarity weights without
    /// any intelligent boosting. Useful for testing or simplified selection.
    pub fn select_by_rarity_simple<'a>(
        variations: &'a [&'a MottoVariation],
        rng: &mut impl Rng,
    ) -> Result<&'a MottoVariation, MottoError> {
        if variations.is_empty() {
            return Err(MottoError::DataInconsistency {
                message: "No variations provided for selection".to_string(),
            });
        }

        let weights: Vec<f32> = variations
            .iter()
            .map(|v| v.rarity.selection_weight())
            .collect();

        let dist = WeightedIndex::new(&weights).map_err(|e| MottoError::DataInconsistency {
            message: format!("Failed to create weighted distribution: {}", e),
        })?;

        Ok(variations[dist.sample(rng)])
    }

    /// Filter variations to only those eligible for the house's characteristics
    ///
    /// This is a utility method that can be used independently for testing
    /// or when you need the filtered list without immediate selection.
    pub fn filter_eligible_variations<'a>(
        variations: &'a [MottoVariation],
        trait_value: f32,
        prestige: f32,
    ) -> Result<Vec<&'a MottoVariation>, MottoError> {
        // Validate inputs
        if !(0.0..=1.0).contains(&trait_value) {
            return Err(MottoError::InvalidTraitValue { value: trait_value });
        }
        if !(0.0..=1.0).contains(&prestige) {
            return Err(MottoError::InvalidPrestige { value: prestige });
        }

        let eligible: Vec<&MottoVariation> = variations
            .iter()
            .filter(|variation| variation.is_eligible(trait_value, prestige))
            .collect();

        Ok(eligible)
    }

    /// Get selection statistics for analysis and debugging
    ///
    /// Provides insights into how the selection system would behave with
    /// different trait/prestige combinations.
    pub fn get_selection_statistics(
        &mut self,
        trait_type: DominantTrait,
        culture: &Culture,
        trait_value: f32,
        prestige: f32,
    ) -> Result<SelectionStatistics, MottoError> {
        // Clone variations to avoid borrow conflicts
        let all_variations = self.registry
            .get_variations(trait_type, culture)?
            .to_vec();
        let eligible = Self::filter_eligible_variations(&all_variations, trait_value, prestige)?;

        let mut stats = SelectionStatistics {
            total_variations: all_variations.len(),
            eligible_variations: eligible.len(),
            common_eligible: 0,
            uncommon_eligible: 0,
            rare_eligible: 0,
            legendary_eligible: 0,
            average_weight: 0.0,
            max_weight: 0.0,
            min_weight: f32::MAX,
        };

        if !eligible.is_empty() {
            let mut total_weight = 0.0;

            for variation in &eligible {
                let weight = self.calculate_selection_weight(variation, trait_value, prestige);
                total_weight += weight;

                stats.max_weight = stats.max_weight.max(weight);
                stats.min_weight = stats.min_weight.min(weight);

                match variation.rarity {
                    super::types::MottoRarity::Common => stats.common_eligible += 1,
                    super::types::MottoRarity::Uncommon => stats.uncommon_eligible += 1,
                    super::types::MottoRarity::Rare => stats.rare_eligible += 1,
                    super::types::MottoRarity::Legendary => stats.legendary_eligible += 1,
                }
            }

            stats.average_weight = total_weight / eligible.len() as f32;
            stats.min_weight = if stats.min_weight == f32::MAX {
                0.0
            } else {
                stats.min_weight
            };
        }

        Ok(stats)
    }

    /// Get a reference to the underlying registry for advanced operations
    pub fn registry(&mut self) -> &mut MottoRegistry {
        &mut self.registry
    }

    /// Update the selection configuration
    pub fn set_config(&mut self, config: SelectionConfig) {
        self.config = config;
    }

    /// Get the current selection configuration
    pub fn config(&self) -> &SelectionConfig {
        &self.config
    }
}

impl Default for MottoSelector {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about motto selection for a specific scenario
#[derive(Debug, Clone)]
pub struct SelectionStatistics {
    pub total_variations: usize,
    pub eligible_variations: usize,
    pub common_eligible: usize,
    pub uncommon_eligible: usize,
    pub rare_eligible: usize,
    pub legendary_eligible: usize,
    pub average_weight: f32,
    pub max_weight: f32,
    pub min_weight: f32,
}

impl SelectionStatistics {
    /// Calculate the probability of selecting each rarity tier
    pub fn rarity_probabilities(&self) -> RarityProbabilities {
        let total_eligible = self.eligible_variations;

        if total_eligible == 0 {
            return RarityProbabilities::default();
        }

        RarityProbabilities {
            common: self.common_eligible as f32 / total_eligible as f32,
            uncommon: self.uncommon_eligible as f32 / total_eligible as f32,
            rare: self.rare_eligible as f32 / total_eligible as f32,
            legendary: self.legendary_eligible as f32 / total_eligible as f32,
        }
    }

    /// Generate a human-readable report
    pub fn generate_report(&self) -> String {
        let probs = self.rarity_probabilities();

        format!(
            "Selection Statistics:\n\
             Total variations: {}\n\
             Eligible variations: {}\n\
             Rarity distribution: C:{} U:{} R:{} L:{}\n\
             Selection probabilities: C:{:.1}% U:{:.1}% R:{:.1}% L:{:.1}%\n\
             Weight range: {:.1} - {:.1} (avg: {:.1})",
            self.total_variations,
            self.eligible_variations,
            self.common_eligible,
            self.uncommon_eligible,
            self.rare_eligible,
            self.legendary_eligible,
            probs.common * 100.0,
            probs.uncommon * 100.0,
            probs.rare * 100.0,
            probs.legendary * 100.0,
            self.min_weight,
            self.max_weight,
            self.average_weight
        )
    }
}

/// Probability distribution across rarity tiers
#[derive(Debug, Clone, Default)]
pub struct RarityProbabilities {
    pub common: f32,
    pub uncommon: f32,
    pub rare: f32,
    pub legendary: f32,
}
