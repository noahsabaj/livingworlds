//! Compound motto generation system for multi-trait houses
//!
//! This module handles the complex logic of generating compound mottos for houses
//! that excel in multiple areas. It includes special combinations, cultural variations,
//! and sophisticated trait analysis.

use rand::Rng;
use std::collections::HashMap;

use super::types::TraitCombination;

// Re-export types for external use
use super::super::traits::{DominantTrait, HouseTraits};
pub use super::types::CompoundMottoConfig;
use crate::name_generator::Culture;

/// Generator for compound mottos that combine multiple traits
pub struct CompoundMottoGenerator {
    config: super::types::CompoundMottoConfig,
    special_combinations: HashMap<TraitCombination, SpecialCombination>,
}

impl CompoundMottoGenerator {
    /// Create a new compound motto generator with default configuration
    pub fn new() -> Self {
        Self {
            config: super::types::CompoundMottoConfig::default(),
            special_combinations: Self::initialize_special_combinations(),
        }
    }

    /// Create a new compound motto generator with custom configuration
    pub fn with_config(config: super::types::CompoundMottoConfig) -> Self {
        Self {
            config,
            special_combinations: Self::initialize_special_combinations(),
        }
    }

    /// Check if a house should generate a compound motto
    ///
    /// This analyzes the house's trait distribution to determine if they excel
    /// in multiple areas sufficiently to warrant a compound motto.
    pub fn should_generate_compound_motto(&self, traits: &HouseTraits, rng: &mut impl Rng) -> bool {
        let strong_traits = self.count_strong_traits(traits);

        // Must have at least the minimum number of strong traits
        if strong_traits < self.config.min_strong_traits {
            return false;
        }

        // Apply probability based on configuration
        rng.gen_bool(self.config.compound_probability as f64)
    }

    /// Generate a compound motto combining the house's strongest traits
    ///
    /// This method analyzes the trait distribution, identifies the best combination,
    /// and generates appropriate compound text based on cultural context.
    pub fn generate_compound_motto(
        &self,
        traits: &HouseTraits,
        culture: &Culture,
        rng: &mut impl Rng,
    ) -> String {
        let trait_pairs = self.analyze_trait_combinations(traits);

        if trait_pairs.is_empty() {
            // Fallback to a generic compound motto
            return "Excellence in All Things".to_string();
        }

        // Select the best trait combination
        let (primary, secondary, _combined_strength) = trait_pairs[0];
        let combination = TraitCombination::new(primary, secondary);

        // Generate motto based on whether this combination has special handling
        if let Some(special) = self.special_combinations.get(&combination) {
            self.generate_special_combination_motto(special, culture, rng)
        } else {
            self.generate_generic_compound_motto(primary, secondary, culture, rng)
        }
    }

    /// Count traits that exceed the "strong" threshold
    fn count_strong_traits(&self, traits: &HouseTraits) -> usize {
        let trait_values = [
            traits.martial,
            traits.stewardship,
            traits.diplomacy,
            traits.learning,
            traits.intrigue,
            traits.piety,
        ];

        trait_values
            .iter()
            .filter(|&&value| value > self.config.strong_trait_threshold)
            .count()
    }

    /// Analyze all possible trait combinations and rank them by strength
    ///
    /// Returns combinations sorted by their combined strength, allowing selection
    /// of the most powerful trait pairing for compound motto generation.
    fn analyze_trait_combinations(
        &self,
        traits: &HouseTraits,
    ) -> Vec<(DominantTrait, DominantTrait, f32)> {
        let trait_list = [
            (DominantTrait::Martial, traits.martial),
            (DominantTrait::Stewardship, traits.stewardship),
            (DominantTrait::Diplomacy, traits.diplomacy),
            (DominantTrait::Learning, traits.learning),
            (DominantTrait::Intrigue, traits.intrigue),
            (DominantTrait::Piety, traits.piety),
        ];

        let mut combinations = Vec::new();

        // Analyze all possible pairs
        for i in 0..trait_list.len() {
            for j in (i + 1)..trait_list.len() {
                let (trait_a, value_a) = trait_list[i];
                let (trait_b, value_b) = trait_list[j];

                // Only consider combinations where both traits are reasonably strong
                if value_a > 0.6 && value_b > 0.6 {
                    let combined_strength = (value_a + value_b) / 2.0;
                    combinations.push((trait_a, trait_b, combined_strength));
                }
            }
        }

        // Sort by combined strength, strongest first
        combinations.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        combinations
    }

    /// Generate motto for a special trait combination
    fn generate_special_combination_motto(
        &self,
        special: &SpecialCombination,
        culture: &Culture,
        rng: &mut impl Rng,
    ) -> String {
        let cultural_mottos = special.get_cultural_mottos(culture);

        if cultural_mottos.is_empty() {
            // Fallback to theme-based motto
            special.generate_theme_motto(rng)
        } else {
            // Select from cultural variations
            let index = rng.gen_range(0..cultural_mottos.len());
            cultural_mottos[index].to_string()
        }
    }

    /// Generate a generic compound motto for any trait combination
    fn generate_generic_compound_motto(
        &self,
        primary: DominantTrait,
        secondary: DominantTrait,
        culture: &Culture,
        rng: &mut impl Rng,
    ) -> String {
        let primary_word = self.get_trait_word(primary, culture);
        let secondary_word = self.get_trait_word(secondary, culture);

        // Various generic compound patterns
        let patterns = [
            format!("{} and {}", primary_word, secondary_word),
            format!("{} Through {}", primary_word, secondary_word),
            format!("Masters of {} and {}", primary_word, secondary_word),
            format!("The {} of {}", primary_word, secondary_word),
            format!("{} United with {}", primary_word, secondary_word),
        ];

        let index = rng.gen_range(0..patterns.len());
        patterns[index].clone()
    }

    /// Get a culturally appropriate word representing a trait
    fn get_trait_word(&self, trait_type: DominantTrait, culture: &Culture) -> &'static str {
        match trait_type {
            DominantTrait::Martial => match culture {
                Culture::Western => "Steel",
                Culture::Eastern => "Blade",
                Culture::Northern => "Iron",
                Culture::Southern => "Sun-Sword",
                Culture::Desert => "Scimitar",
                Culture::Island => "Storm",
                Culture::Ancient => "Legion",
                Culture::Mystical => "War-Magic",
            },
            DominantTrait::Stewardship => match culture {
                Culture::Western => "Gold",
                Culture::Eastern => "Jade",
                Culture::Northern => "Wealth",
                Culture::Southern => "Harvest",
                Culture::Desert => "Trade",
                Culture::Island => "Pearl",
                Culture::Ancient => "Treasury",
                Culture::Mystical => "Alchemy",
            },
            DominantTrait::Diplomacy => match culture {
                Culture::Western => "Peace",
                Culture::Eastern => "Harmony",
                Culture::Northern => "Accord",
                Culture::Southern => "Unity",
                Culture::Desert => "Oasis",
                Culture::Island => "Bridge",
                Culture::Ancient => "Alliance",
                Culture::Mystical => "Bond",
            },
            DominantTrait::Learning => match culture {
                Culture::Western => "Knowledge",
                Culture::Eastern => "Wisdom",
                Culture::Northern => "Truth",
                Culture::Southern => "Light",
                Culture::Desert => "Stars",
                Culture::Island => "Depth",
                Culture::Ancient => "Memory",
                Culture::Mystical => "Secrets",
            },
            DominantTrait::Intrigue => match culture {
                Culture::Western => "Shadow",
                Culture::Eastern => "Silk",
                Culture::Northern => "Whisper",
                Culture::Southern => "Mask",
                Culture::Desert => "Mirage",
                Culture::Island => "Current",
                Culture::Ancient => "Veil",
                Culture::Mystical => "Illusion",
            },
            DominantTrait::Piety => match culture {
                Culture::Western => "Faith",
                Culture::Eastern => "Heaven",
                Culture::Northern => "Gods",
                Culture::Southern => "Divine",
                Culture::Desert => "Prophet",
                Culture::Island => "Spirit",
                Culture::Ancient => "Ancestor",
                Culture::Mystical => "Beyond",
            },
        }
    }

    /// Initialize the special trait combinations with their unique mottos
    fn initialize_special_combinations() -> HashMap<TraitCombination, SpecialCombination> {
        let mut combinations = HashMap::new();

        // Martial + Piety: Holy warriors, crusaders, paladins
        combinations.insert(
            TraitCombination::new(DominantTrait::Martial, DominantTrait::Piety),
            SpecialCombination::new(
                "Holy Warrior",
                vec![
                    ("Faith and Steel United", &[Culture::Western]),
                    ("Holy Warriors Eternal", &[Culture::Western]),
                    ("God's Own Sword", &[Culture::Western]),
                    ("Heaven's Blade", &[Culture::Eastern]),
                    ("The Divine Warrior", &[Culture::Eastern]),
                    ("Celestial and Martial", &[Culture::Eastern]),
                    ("By Axe and Altar", &[Culture::Northern]),
                    ("Warriors of the Old Gods", &[Culture::Northern]),
                    ("Sacred Berserkers", &[Culture::Northern]),
                    ("Blessed Blades", &[Culture::Southern, Culture::Desert]),
                    ("Sacred War Eternal", &[]), // Universal fallback
                ],
            ),
        );

        // Stewardship + Intrigue: Merchant spies, economic manipulators
        combinations.insert(
            TraitCombination::new(DominantTrait::Stewardship, DominantTrait::Intrigue),
            SpecialCombination::new(
                "Shadow Merchant",
                vec![
                    ("Gold in Shadow", &[Culture::Western]),
                    ("The Merchant Spies", &[Culture::Western]),
                    ("Wealth Through Secrets", &[Culture::Western]),
                    ("Silk and Shadows", &[Culture::Eastern]),
                    ("The Jade Conspiracy", &[Culture::Eastern]),
                    ("Hidden Wealth", &[Culture::Eastern]),
                    ("Secrets Buy Kingdoms", &[]), // Universal
                ],
            ),
        );

        // Learning + Piety: Scholar-priests, theologians, wise clerics
        combinations.insert(
            TraitCombination::new(DominantTrait::Learning, DominantTrait::Piety),
            SpecialCombination::new(
                "Sacred Scholar",
                vec![
                    ("Knowledge of the Divine", &[Culture::Western]),
                    ("Sacred Wisdom", &[Culture::Western]),
                    ("The Scholar Priests", &[Culture::Western]),
                    ("Celestial Knowledge", &[Culture::Eastern]),
                    ("Heaven's Library", &[Culture::Eastern]),
                    ("The Enlightened Path", &[Culture::Eastern]),
                    ("Truth Through Faith", &[]), // Universal
                ],
            ),
        );

        // Martial + Stewardship: Merchant knights, warrior-traders
        combinations.insert(
            TraitCombination::new(DominantTrait::Martial, DominantTrait::Stewardship),
            SpecialCombination::new(
                "Warrior Merchant",
                vec![
                    ("War Feeds the Treasury", &[]),
                    ("Gold and Iron", &[]),
                    ("Conquest and Commerce", &[]),
                    ("The Merchant Knights", &[Culture::Western]),
                    ("Steel and Gold United", &[Culture::Western]),
                    ("The Warrior's Purse", &[Culture::Eastern]),
                ],
            ),
        );

        // Diplomacy + Intrigue: Master manipulators, political schemers
        combinations.insert(
            TraitCombination::new(DominantTrait::Diplomacy, DominantTrait::Intrigue),
            SpecialCombination::new(
                "Shadow Diplomat",
                vec![
                    ("Smiles Hide Daggers", &[]),
                    ("The Velvet Conspiracy", &[Culture::Western]),
                    ("Peace Through Deception", &[]),
                    ("Friends and Spies", &[]),
                    ("The Silk Tongue", &[Culture::Eastern]),
                    ("Honey-Poisoned Words", &[Culture::Southern]),
                ],
            ),
        );

        combinations
    }

    /// Get configuration reference
    pub fn config(&self) -> &super::types::CompoundMottoConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: super::types::CompoundMottoConfig) {
        self.config = config;
    }
}

impl Default for CompoundMottoGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Special combination with cultural variations and thematic generation
#[derive(Debug, Clone)]
struct SpecialCombination {
    theme: String,
    cultural_mottos: Vec<(String, Vec<Culture>)>,
}

impl SpecialCombination {
    fn new(theme: &str, mottos: Vec<(&str, &[Culture])>) -> Self {
        Self {
            theme: theme.to_string(),
            cultural_mottos: mottos
                .into_iter()
                .map(|(motto, cultures)| (motto.to_string(), cultures.to_vec()))
                .collect(),
        }
    }

    /// Get all mottos appropriate for a specific culture
    fn get_cultural_mottos(&self, culture: &Culture) -> Vec<&str> {
        self.cultural_mottos
            .iter()
            .filter(|(_, cultures)| cultures.is_empty() || cultures.contains(culture))
            .map(|(motto, _)| motto.as_str())
            .collect()
    }

    /// Generate a theme-based motto when no cultural variations are available
    fn generate_theme_motto(&self, rng: &mut impl Rng) -> String {
        let theme_variations = vec![
            format!("Masters of {}", self.theme),
            format!("The {} Path", self.theme),
            format!("{} Eternal", self.theme),
            format!("Born of {}", self.theme),
            format!("The {} Legacy", self.theme),
        ];

        let index = rng.gen_range(0..theme_variations.len());
        theme_variations[index].clone()
    }
}

/// Statistics about compound motto generation patterns
#[derive(Debug, Clone, Default)]
pub struct CompoundStatistics {
    pub total_combinations_analyzed: usize,
    pub strong_combinations_found: usize,
    pub special_combinations_used: usize,
    pub generic_combinations_used: usize,
    pub most_common_primary_trait: Option<DominantTrait>,
    pub most_common_secondary_trait: Option<DominantTrait>,
}

impl CompoundStatistics {
    /// Generate a human-readable report
    pub fn generate_report(&self) -> String {
        format!(
            "Compound Motto Statistics:\n\
             Total combinations analyzed: {}\n\
             Strong combinations found: {}\n\
             Special combinations used: {}\n\
             Generic combinations used: {}\n\
             Most common primary trait: {:?}\n\
             Most common secondary trait: {:?}",
            self.total_combinations_analyzed,
            self.strong_combinations_found,
            self.special_combinations_used,
            self.generic_combinations_used,
            self.most_common_primary_trait,
            self.most_common_secondary_trait
        )
    }
}
