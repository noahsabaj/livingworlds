//! Cross-nation law tracking
//!
//! Tracks which nations have which laws for efficient global queries.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::nations::NationId;
use crate::nations::laws::types::LawId;

/// Resource tracking active laws across all nations for quick queries
#[derive(Resource, Default)]
pub struct ActiveLaws {
    /// Which nations have which laws active
    pub by_nation: HashMap<NationId, HashSet<LawId>>,

    /// Which laws are active in which nations
    pub by_law: HashMap<LawId, HashSet<NationId>>,

    /// Count of how many nations have each law
    pub adoption_counts: HashMap<LawId, u32>,
}

impl ActiveLaws {
    /// Update when a nation enacts a law
    pub fn on_law_enacted(&mut self, nation_id: NationId, law_id: LawId) {
        self.by_nation
            .entry(nation_id)
            .or_insert_with(HashSet::new)
            .insert(law_id);

        self.by_law
            .entry(law_id)
            .or_insert_with(HashSet::new)
            .insert(nation_id);

        *self.adoption_counts.entry(law_id).or_insert(0) += 1;
    }

    /// Update when a nation repeals a law
    pub fn on_law_repealed(&mut self, nation_id: NationId, law_id: LawId) {
        if let Some(laws) = self.by_nation.get_mut(&nation_id) {
            laws.remove(&law_id);
        }

        if let Some(nations) = self.by_law.get_mut(&law_id) {
            nations.remove(&nation_id);
        }

        if let Some(count) = self.adoption_counts.get_mut(&law_id) {
            *count = count.saturating_sub(1);
        }
    }

    /// Get all laws active in a nation
    pub fn get_nation_laws(&self, nation_id: NationId) -> Vec<LawId> {
        self.by_nation
            .get(&nation_id)
            .map(|laws| laws.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Get all nations with a specific law
    pub fn get_law_adopters(&self, law_id: LawId) -> Vec<NationId> {
        self.by_law
            .get(&law_id)
            .map(|nations| nations.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Get adoption rate of a law (0.0 to 1.0)
    pub fn get_adoption_rate(&self, law_id: LawId, total_nations: u32) -> f32 {
        if total_nations == 0 {
            return 0.0;
        }

        self.adoption_counts
            .get(&law_id)
            .map(|&count| count as f32 / total_nations as f32)
            .unwrap_or(0.0)
    }
}