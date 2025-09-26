//! Government history tracking
//!
//! This module tracks the history of government changes for each nation,
//! allowing for historical analysis and storytelling.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::types::GovernmentType;
use super::transitions::TransitionType;

/// Component that tracks a nation's government history
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct GovernmentHistory {
    pub founding_government: Option<GovernmentType>,
    pub changes: Vec<GovernmentChange>,
    pub total_peaceful_transitions: u32,
    pub total_violent_transitions: u32,
    pub longest_stable_period: u32, // In days
    pub most_common_government: Option<GovernmentType>,
}

/// A single government change event
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct GovernmentChange {
    pub from: GovernmentType,
    pub to: GovernmentType,
    pub transition_type: TransitionType,
    pub peaceful: bool,
    pub game_time: u32, // Day when it happened
}

impl GovernmentHistory {
    /// Create a new government history starting with the given government
    pub fn new(founding: GovernmentType) -> Self {
        Self {
            founding_government: Some(founding),
            changes: Vec::new(),
            total_peaceful_transitions: 0,
            total_violent_transitions: 0,
            longest_stable_period: 0,
            most_common_government: Some(founding),
        }
    }

    /// Add a government change to the history
    pub fn record_change(&mut self, change: GovernmentChange) {
        if change.peaceful {
            self.total_peaceful_transitions += 1;
        } else {
            self.total_violent_transitions += 1;
        }

        self.changes.push(change);
        self.update_statistics();
    }

    /// Update statistics based on history
    fn update_statistics(&mut self) {
        // Calculate longest stable period
        if self.changes.len() > 1 {
            let mut max_period = 0;
            for i in 1..self.changes.len() {
                let period = self.changes[i].game_time - self.changes[i - 1].game_time;
                max_period = max_period.max(period);
            }
            self.longest_stable_period = max_period;
        }

        // Find most common government
        let mut government_counts = std::collections::HashMap::new();

        // Count founding government's duration
        if let Some(founding) = self.founding_government {
            let first_change_time = self.changes.first().map(|c| c.game_time).unwrap_or(0);
            *government_counts.entry(founding).or_insert(0) += first_change_time;
        }

        // Count each government's duration
        for (i, change) in self.changes.iter().enumerate() {
            let duration = if i < self.changes.len() - 1 {
                self.changes[i + 1].game_time - change.game_time
            } else {
                365 // Assume at least a year for current government
            };
            *government_counts.entry(change.to).or_insert(0) += duration;
        }

        // Find the government with longest total duration
        self.most_common_government = government_counts
            .into_iter()
            .max_by_key(|(_, duration)| *duration)
            .map(|(gov, _)| gov);
    }

    /// Get the current government type
    pub fn current_government(&self) -> Option<GovernmentType> {
        self.changes
            .last()
            .map(|change| change.to)
            .or(self.founding_government)
    }

    /// Check if the nation has ever had a specific government type
    pub fn has_had_government(&self, government: GovernmentType) -> bool {
        self.founding_government == Some(government) ||
        self.changes.iter().any(|change| change.to == government)
    }

    /// Get the number of times a specific transition type has occurred
    pub fn count_transition_type(&self, transition_type: TransitionType) -> usize {
        self.changes
            .iter()
            .filter(|change| change.transition_type == transition_type)
            .count()
    }

    /// Get the stability score (fewer changes = more stable)
    pub fn stability_score(&self) -> f32 {
        if self.changes.is_empty() {
            1.0
        } else {
            let changes_per_century = self.changes.len() as f32 / 36.5; // Assuming 365 days = 1 year
            1.0 / (1.0 + changes_per_century)
        }
    }

    /// Generate a historical summary string
    pub fn summary(&self) -> String {
        if self.changes.is_empty() {
            if let Some(founding) = self.founding_government {
                return format!("Has remained a {:?} since founding", founding);
            } else {
                return "No government history recorded".to_string();
            }
        }

        let revolutions = self.count_transition_type(TransitionType::Revolution);
        let coups = self.count_transition_type(TransitionType::Coup);
        let reforms = self.count_transition_type(TransitionType::Reform);

        format!(
            "Government changed {} times: {} peacefully, {} violently ({} revolutions, {} coups, {} reforms)",
            self.changes.len(),
            self.total_peaceful_transitions,
            self.total_violent_transitions,
            revolutions,
            coups,
            reforms
        )
    }
}