//! Parallel Safety Plugin
//!
//! Plugin for parallel safety validation and race condition detection.

use super::{
    detection::RaceConditionDetector,
    metrics::{log_safety_metrics, ParallelSafetyMetrics},
};
use bevy_plugin_builder::define_plugin;

// Plugin for parallel safety validation.
define_plugin!(ParallelSafetyPlugin {
    resources: [ParallelSafetyMetrics, RaceConditionDetector],

    update: [log_safety_metrics]
});
