//! Core motto generation orchestrator
//!
//! This module provides the main API for motto generation, coordinating between
//! the specialized selection, compound, and data modules to create the final mottos.
//! It maintains the same public interface as the original system while leveraging
//! the new modular architecture.

use rand::prelude::*;
use rand::Rng;

use super::super::traits::{DominantTrait, HouseTraits};
use super::compound::{CompoundMottoConfig, CompoundMottoGenerator};
use super::selection::{MottoSelector, SelectionConfig};
use super::types::MottoError;
use crate::name_generator::Culture;

/// Main motto generator that coordinates all generation systems
pub struct MottoGenerator {
    selector: MottoSelector,
    compound_generator: CompoundMottoGenerator,
    generation_stats: GenerationStatistics,
}

impl MottoGenerator {
    /// Create a new motto generator with default configuration
    pub fn new() -> Self {
        Self {
            selector: MottoSelector::new(),
            compound_generator: CompoundMottoGenerator::new(),
            generation_stats: GenerationStatistics::default(),
        }
    }

    /// Create a new motto generator with custom configurations
    pub fn with_configs(
        selection_config: SelectionConfig,
        compound_config: CompoundMottoConfig,
    ) -> Self {
        Self {
            selector: MottoSelector::with_config(selection_config),
            compound_generator: CompoundMottoGenerator::with_config(compound_config),
            generation_stats: GenerationStatistics::default(),
        }
    }

    /// Generate a motto based on house traits and culture
    ///
    /// This is the main entry point that maintains compatibility with the original API
    /// while leveraging the new modular architecture internally.
    pub fn generate_motto(&mut self, traits: &HouseTraits, culture: &Culture) -> String {
        let mut rng = rand::thread_rng();
        self.generate_motto_with_rng(traits, culture, &mut rng)
    }

    /// Generate a motto with a provided RNG for deterministic results
    ///
    /// This version allows external control of randomness, useful for testing
    /// or when deterministic generation is required.
    pub fn generate_motto_with_rng(
        &mut self,
        traits: &HouseTraits,
        culture: &Culture,
        rng: &mut impl Rng,
    ) -> String {
        // Update statistics
        self.generation_stats.total_generations += 1;

        // For now, use a random prestige value as in the original
        // TODO: This will be replaced when the house system provides actual prestige
        let house_prestige = rng.gen_range(0.0..1.0);

        // Check if we should generate a compound motto
        if self
            .compound_generator
            .should_generate_compound_motto(traits, rng)
        {
            self.generation_stats.compound_generations += 1;
            self.compound_generator
                .generate_compound_motto(traits, culture, rng)
        } else {
            self.generation_stats.single_generations += 1;
            self.generate_single_motto(traits, culture, house_prestige, rng)
        }
    }

    /// Generate a single-trait motto (internal method)
    fn generate_single_motto(
        &mut self,
        traits: &HouseTraits,
        culture: &Culture,
        prestige: f32,
        rng: &mut impl Rng,
    ) -> String {
        let dominant = traits.dominant_trait();
        let dominant_value = Self::get_trait_value(traits, dominant);

        // Use the selector to choose the best motto
        match self
            .selector
            .select_motto_for_trait(dominant, dominant_value, culture, prestige, rng)
        {
            Ok(motto) => {
                self.generation_stats.successful_selections += 1;
                motto
            }
            Err(_) => {
                self.generation_stats.fallback_selections += 1;
                // Use the registry's fallback system
                self.selector
                    .registry()
                    .get_fallback(dominant, culture)
                    .to_string()
            }
        }
    }

    /// Get the value of a specific trait from house traits
    fn get_trait_value(traits: &HouseTraits, trait_type: DominantTrait) -> f32 {
        match trait_type {
            DominantTrait::Martial => traits.martial,
            DominantTrait::Stewardship => traits.stewardship,
            DominantTrait::Diplomacy => traits.diplomacy,
            DominantTrait::Learning => traits.learning,
            DominantTrait::Intrigue => traits.intrigue,
            DominantTrait::Piety => traits.piety,
        }
    }

    /// Generate multiple mottos for analysis or selection
    ///
    /// Useful for testing, debugging, or allowing users to choose from options.
    pub fn generate_multiple_mottos(
        &mut self,
        traits: &HouseTraits,
        culture: &Culture,
        count: usize,
    ) -> Vec<String> {
        let mut mottos = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            mottos.push(self.generate_motto_with_rng(traits, culture, &mut rng));
        }

        mottos
    }

    /// Generate motto with detailed analysis information
    ///
    /// Returns both the motto and information about how it was generated,
    /// useful for debugging or providing insights to developers.
    pub fn generate_motto_with_analysis(
        &mut self,
        traits: &HouseTraits,
        culture: &Culture,
    ) -> MottoGenerationResult {
        let mut rng = rand::thread_rng();
        let start_time = std::time::Instant::now();

        let should_compound = self
            .compound_generator
            .should_generate_compound_motto(traits, &mut rng);
        let house_prestige = rng.gen_range(0.0..1.0);

        let (motto, generation_method) = if should_compound {
            let motto = self
                .compound_generator
                .generate_compound_motto(traits, culture, &mut rng);
            (motto, GenerationMethod::Compound)
        } else {
            let dominant = traits.dominant_trait();
            let dominant_value = Self::get_trait_value(traits, dominant);

            match self.selector.select_motto_for_trait(
                dominant,
                dominant_value,
                culture,
                house_prestige,
                &mut rng,
            ) {
                Ok(motto) => (motto, GenerationMethod::SingleTrait),
                Err(_) => {
                    let fallback = self
                        .selector
                        .registry()
                        .get_fallback(dominant, culture)
                        .to_string();
                    (fallback, GenerationMethod::Fallback)
                }
            }
        };

        let generation_time = start_time.elapsed();

        MottoGenerationResult {
            motto,
            generation_method,
            dominant_trait: traits.dominant_trait(),
            trait_value: Self::get_trait_value(traits, traits.dominant_trait()),
            prestige: house_prestige,
            culture: *culture,
            generation_time,
        }
    }

    /// Validate the entire motto generation system
    ///
    /// Performs comprehensive validation of all components to ensure system integrity.
    pub fn validate_system(&mut self) -> Result<ValidationReport, MottoError> {
        let mut report = ValidationReport::default();

        // Validate the data registry
        match self.selector.registry().validate_all_data() {
            Ok(()) => report.data_validation_passed = true,
            Err(e) => {
                report.data_validation_passed = false;
                report
                    .validation_errors
                    .push(format!("Data validation failed: {}", e));
            }
        }

        // Test generation for all trait/culture combinations
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

        for &dominant_trait in &traits {
            for &culture in &cultures {
                let test_traits = Self::create_test_traits(dominant_trait);
                let result = self.generate_motto_with_analysis(&test_traits, &culture);

                report.generation_tests_performed += 1;

                if result.motto.is_empty() {
                    report.validation_errors.push(format!(
                        "Empty motto generated for {:?} / {:?}",
                        dominant_trait, culture
                    ));
                } else {
                    report.successful_generations += 1;
                }
            }
        }

        // Calculate success rate
        report.success_rate = if report.generation_tests_performed > 0 {
            report.successful_generations as f32 / report.generation_tests_performed as f32
        } else {
            0.0
        };

        Ok(report)
    }

    /// Create test traits with one dominant trait for validation
    fn create_test_traits(dominant_trait: DominantTrait) -> HouseTraits {
        let mut traits = HouseTraits {
            martial: 0.3,
            stewardship: 0.3,
            diplomacy: 0.3,
            learning: 0.3,
            intrigue: 0.3,
            piety: 0.3,
        };

        // Set the dominant trait to a high value
        match dominant_trait {
            DominantTrait::Martial => traits.martial = 0.8,
            DominantTrait::Stewardship => traits.stewardship = 0.8,
            DominantTrait::Diplomacy => traits.diplomacy = 0.8,
            DominantTrait::Learning => traits.learning = 0.8,
            DominantTrait::Intrigue => traits.intrigue = 0.8,
            DominantTrait::Piety => traits.piety = 0.8,
        }

        traits
    }

    /// Get current generation statistics
    pub fn statistics(&self) -> &GenerationStatistics {
        &self.generation_stats
    }

    /// Reset generation statistics
    pub fn reset_statistics(&mut self) {
        self.generation_stats = GenerationStatistics::default();
    }

    /// Get a reference to the selection system for advanced operations
    pub fn selector(&mut self) -> &mut MottoSelector {
        &mut self.selector
    }

    /// Get a reference to the compound generation system
    pub fn compound_generator(&mut self) -> &mut CompoundMottoGenerator {
        &mut self.compound_generator
    }
}

impl Default for MottoGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// The main public function that maintains API compatibility
///
/// This function provides the same interface as the original motto generation
/// system while using the new modular architecture internally.
pub fn generate_motto(traits: &HouseTraits, culture: &Culture) -> String {
    let mut generator = MottoGenerator::new();
    generator.generate_motto(traits, culture)
}

/// Detailed result of motto generation with analysis information
#[derive(Debug, Clone)]
pub struct MottoGenerationResult {
    pub motto: String,
    pub generation_method: GenerationMethod,
    pub dominant_trait: DominantTrait,
    pub trait_value: f32,
    pub prestige: f32,
    pub culture: Culture,
    pub generation_time: std::time::Duration,
}

/// Method used to generate a motto
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GenerationMethod {
    SingleTrait,
    Compound,
    Fallback,
}

/// Statistics tracking motto generation patterns
#[derive(Debug, Clone, Default)]
pub struct GenerationStatistics {
    pub total_generations: usize,
    pub single_generations: usize,
    pub compound_generations: usize,
    pub successful_selections: usize,
    pub fallback_selections: usize,
}

impl GenerationStatistics {
    /// Calculate the compound motto rate
    pub fn compound_rate(&self) -> f32 {
        if self.total_generations > 0 {
            self.compound_generations as f32 / self.total_generations as f32
        } else {
            0.0
        }
    }

    /// Calculate the fallback rate
    pub fn fallback_rate(&self) -> f32 {
        if self.total_generations > 0 {
            self.fallback_selections as f32 / self.total_generations as f32
        } else {
            0.0
        }
    }

    /// Generate a human-readable report
    pub fn generate_report(&self) -> String {
        format!(
            "Generation Statistics:\n\
             Total generations: {}\n\
             Single trait mottos: {} ({:.1}%)\n\
             Compound mottos: {} ({:.1}%)\n\
             Successful selections: {} ({:.1}%)\n\
             Fallback selections: {} ({:.1}%)",
            self.total_generations,
            self.single_generations,
            if self.total_generations > 0 {
                self.single_generations as f32 / self.total_generations as f32 * 100.0
            } else {
                0.0
            },
            self.compound_generations,
            self.compound_rate() * 100.0,
            self.successful_selections,
            if self.total_generations > 0 {
                self.successful_selections as f32 / self.total_generations as f32 * 100.0
            } else {
                0.0
            },
            self.fallback_selections,
            self.fallback_rate() * 100.0
        )
    }
}

/// Report from system validation
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    pub data_validation_passed: bool,
    pub generation_tests_performed: usize,
    pub successful_generations: usize,
    pub success_rate: f32,
    pub validation_errors: Vec<String>,
}

impl ValidationReport {
    /// Check if the system passed all validation tests
    pub fn is_valid(&self) -> bool {
        self.data_validation_passed && self.validation_errors.is_empty() && self.success_rate > 0.95
    }

    /// Generate a human-readable validation report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Motto System Validation Report ===\n");
        report.push_str(&format!(
            "Data validation: {}\n",
            if self.data_validation_passed {
                "PASSED"
            } else {
                "FAILED"
            }
        ));
        report.push_str(&format!(
            "Generation tests: {} performed, {} successful\n",
            self.generation_tests_performed, self.successful_generations
        ));
        report.push_str(&format!(
            "Success rate: {:.1}%\n",
            self.success_rate * 100.0
        ));
        report.push_str(&format!(
            "Overall status: {}\n",
            if self.is_valid() { "VALID" } else { "INVALID" }
        ));

        if !self.validation_errors.is_empty() {
            report.push_str("\n=== Validation Errors ===\n");
            for error in &self.validation_errors {
                report.push_str(&format!("• {}\n", error));
            }
        }

        report
    }
}
