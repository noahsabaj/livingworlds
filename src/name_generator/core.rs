//! Core name generator with state management and orchestration
//!
//! This module contains the main NameGenerator struct and orchestrates
//! name generation by delegating to specialized generation modules.

use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashSet;

// Import through parent module gateway
use super::types::*;
use super::utils::*;

/// Main name generator with optional deterministic seeding
#[derive(Resource, Clone)]
pub struct NameGenerator {
    rng: Option<StdRng>,
    used_names: HashSet<String>,
}

impl Default for NameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl NameGenerator {
    pub fn new() -> Self {
        Self {
            rng: None,
            used_names: HashSet::new(),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: Some(StdRng::seed_from_u64(seed)),
            used_names: HashSet::new(),
        }
    }

    /// Clear the used names cache
    pub fn clear_cache(&mut self) {
        self.used_names.clear();
    }

    pub fn names_generated(&self) -> usize {
        self.used_names.len()
    }

    /// Main orchestration method - delegates to specialized generators
    pub fn generate(&mut self, name_type: NameType) -> String {
        let name = match name_type {
            NameType::World => super::world::generate_world_name(self),
            NameType::Nation { culture } => super::cultures::generate_nation_name(self, culture),
            NameType::House { culture } => super::cultures::generate_house_name(self, culture),
            NameType::Province { region, culture } => super::places::generate_province_name(self, region, culture),
            NameType::City { size, culture } => super::places::generate_city_name(self, size, culture),
            NameType::Person {
                gender,
                culture,
                role,
            } => super::people::generate_person_name(self, gender, culture, role),
            NameType::River => super::geographic::generate_river_name(self),
            NameType::Mountain => super::geographic::generate_mountain_name(self),
            NameType::Ocean => super::geographic::generate_ocean_name(self),
            NameType::Desert => super::geographic::generate_desert_name(self),
            NameType::Forest => super::geographic::generate_forest_name(self),
        };

        // Ensure uniqueness by appending Roman numerals if needed
        self.ensure_unique(name)
    }

    pub fn generate_related_name(&mut self, parent_name: &str, relation: NameRelation) -> String {
        let name = match relation {
            NameRelation::NewSettlement => format!("New {}", parent_name),
            NameRelation::OldSettlement => format!("Old {}", parent_name),
            NameRelation::ChildCity => {
                let suffixes = ["ton", "ville", "burg", "shire", "ford", "haven", "bridge"];
                let suffix = self.random_choice(&suffixes);
                format!("{}{}", parent_name, suffix)
            }
            NameRelation::TwinCity => {
                let prefixes = [
                    "North", "South", "East", "West", "Upper", "Lower", "Greater", "Lesser",
                ];
                let prefix = self.random_choice(&prefixes);
                format!("{} {}", prefix, parent_name)
            }
            NameRelation::RivalCity => {
                let prefixes = ["Fort", "Port", "Mount", "Saint", "New", "Royal"];
                let prefix = self.random_choice(&prefixes);
                format!("{} {}", prefix, parent_name)
            }
        };

        self.ensure_unique(name)
    }

    // ========================================================================
    // UTILITY METHODS
    // ========================================================================

    /// Ensure a name is unique by appending Roman numerals if necessary
    pub fn ensure_unique(&mut self, name: String) -> String {
        if !self.used_names.contains(&name) {
            self.used_names.insert(name.clone());
            return name;
        }

        // Name already exists, append Roman numeral
        let mut counter = 2;
        loop {
            let unique_name = format!("{} {}", name, to_roman_numeral(counter));
            if !self.used_names.contains(&unique_name) {
                self.used_names.insert(unique_name.clone());
                return unique_name;
            }
            counter += 1;
            if counter > 50 {
                // Fallback to regular numbers if we exceed Roman numeral range
                let unique_name = format!("{} {}", name, counter);
                self.used_names.insert(unique_name.clone());
                return unique_name;
            }
        }
    }

    /// Random choice from a slice
    pub fn random_choice<'a, T>(&mut self, choices: &'a [T]) -> &'a T {
        let index = self.random_range(0, choices.len());
        &choices[index]
    }

    /// Weighted random choice from a slice of (item, weight) tuples
    pub fn weighted_choice<'a, T>(&mut self, weighted_choices: &'a [(T, u32)]) -> &'a T {
        let total_weight: u32 = weighted_choices.iter().map(|(_, weight)| weight).sum();
        let mut random_value = self.random_range(0, total_weight as usize) as u32;

        for (item, weight) in weighted_choices {
            if random_value < *weight {
                return item;
            }
            random_value -= weight;
        }

        // Fallback to first item if something goes wrong
        &weighted_choices[0].0
    }

    /// Generate random number in range [min, max)
    pub fn random_range(&mut self, min: usize, max: usize) -> usize {
        if let Some(ref mut rng) = self.rng {
            rng.gen_range(min..max)
        } else {
            rand::thread_rng().gen_range(min..max)
        }
    }

    /// Generate random boolean
    pub fn random_bool(&mut self) -> bool {
        if let Some(ref mut rng) = self.rng {
            rng.gen_bool(0.5)
        } else {
            rand::thread_rng().gen_bool(0.5)
        }
    }
}