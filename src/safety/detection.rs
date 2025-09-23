//! Race Condition Detection System
//!
//! This module provides monitoring and detection of potential race conditions
//! in parallel operations by tracking concurrent access patterns.

use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Monitor for potential race conditions in parallel operations
#[derive(Resource)]
pub struct RaceConditionDetector {
    /// Track concurrent access to shared data
    access_tracker: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl Default for RaceConditionDetector {
    fn default() -> Self {
        Self {
            access_tracker: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl RaceConditionDetector {
    /// Record access to a shared resource
    pub fn record_access(&self, resource_id: &str) {
        if let Ok(mut tracker) = self.access_tracker.lock() {
            let now = Instant::now();
            let accesses = tracker
                .entry(resource_id.to_string())
                .or_insert_with(Vec::new);
            accesses.push(now);

            // Keep only recent accesses (last 100ms)
            let cutoff = now - Duration::from_millis(100);
            accesses.retain(|&time| time > cutoff);

            // Detect suspicious concurrent access patterns
            if accesses.len() > 10 {
                warn!(
                    "High contention detected on resource '{}': {} accesses in 100ms",
                    resource_id,
                    accesses.len()
                );
            }
        }
    }

    /// Check for potential race conditions
    pub fn check_for_races(&self) -> usize {
        match self.access_tracker.lock() { Ok(tracker) => {
            tracker
                .values()
                .filter(|accesses| accesses.len() > 5)
                .count()
        } _ => {
            0
        }}
    }
}
