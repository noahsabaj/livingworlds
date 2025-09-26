//! Global law registry
//!
//! Centralized storage and indexing of all laws in the game,
//! with efficient O(1) lookup and category filtering.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::nations::laws::types::{Law, LawId, LawCategory};

/// Global registry of all laws in the game
#[derive(Resource, Default)]
pub struct LawRegistry {
    /// Dense storage of all laws
    laws: Vec<Law>,

    /// Map from LawId to index in laws vec for O(1) lookup
    id_to_index: HashMap<LawId, usize>,

    /// Laws grouped by category for efficient filtering
    category_index: HashMap<LawCategory, Vec<usize>>,

    /// Precomputed law conflicts for quick checking
    conflict_matrix: HashMap<LawId, HashSet<LawId>>,

    /// Laws that are mutually exclusive within their category
    exclusive_groups: HashMap<String, Vec<LawId>>,
}

impl LawRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new law in the system
    pub fn register_law(&mut self, law: Law) {
        let index = self.laws.len();
        let law_id = law.id;
        let category = law.category;

        // Store conflicts for quick lookup
        for &conflict_id in &law.conflicts_with {
            self.conflict_matrix
                .entry(law_id)
                .or_insert_with(HashSet::new)
                .insert(conflict_id);

            // Conflicts are bidirectional
            self.conflict_matrix
                .entry(conflict_id)
                .or_insert_with(HashSet::new)
                .insert(law_id);
        }

        self.laws.push(law);
        self.id_to_index.insert(law_id, index);
        self.category_index
            .entry(category)
            .or_insert_with(Vec::new)
            .push(index);
    }

    /// Register a group of mutually exclusive laws
    pub fn register_exclusive_group(&mut self, group_name: String, law_ids: Vec<LawId>) {
        // Add conflicts between all laws in the group
        for i in 0..law_ids.len() {
            for j in (i + 1)..law_ids.len() {
                self.conflict_matrix
                    .entry(law_ids[i])
                    .or_insert_with(HashSet::new)
                    .insert(law_ids[j]);
                self.conflict_matrix
                    .entry(law_ids[j])
                    .or_insert_with(HashSet::new)
                    .insert(law_ids[i]);
            }
        }

        self.exclusive_groups.insert(group_name, law_ids);
    }

    /// Get a law by its ID
    pub fn get_law(&self, id: LawId) -> Option<&Law> {
        self.id_to_index.get(&id).and_then(|&idx| self.laws.get(idx))
    }

    /// Get all laws in a category
    pub fn get_category_laws(&self, category: LawCategory) -> Vec<&Law> {
        self.category_index
            .get(&category)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&idx| self.laws.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get conflicts for a law
    pub fn get_conflicts(&self, law_id: LawId) -> Vec<LawId> {
        self.conflict_matrix
            .get(&law_id)
            .map(|conflicts| conflicts.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Filter laws by a predicate
    pub fn filter_laws<F>(&self, predicate: F) -> Vec<&Law>
    where
        F: Fn(&Law) -> bool,
    {
        self.laws.iter().filter(|law| predicate(law)).collect()
    }

    /// Check if two laws conflict
    pub fn laws_conflict(&self, law1: LawId, law2: LawId) -> bool {
        self.conflict_matrix
            .get(&law1)
            .map(|conflicts| conflicts.contains(&law2))
            .unwrap_or(false)
    }

    /// Get total number of laws
    pub fn law_count(&self) -> usize {
        self.laws.len()
    }

    /// Get all laws
    pub fn all_laws(&self) -> &[Law] {
        &self.laws
    }
}