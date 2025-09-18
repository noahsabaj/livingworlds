//! Parallel Safety Plugin - SIMPLICITY PERFECTION!
//!
//! This module demonstrates PERFECT simple plugin automation!
//! 21 lines of manual registration → 12 lines pure elegance!

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::{
    detection::RaceConditionDetector,
    metrics::{ParallelSafetyMetrics, log_safety_metrics},
};

/// Plugin for parallel safety validation using PURE AUTOMATION!
///
/// **AUTOMATION ACHIEVEMENT**: 21 lines manual → 12 lines declarative!
define_plugin!(ParallelSafetyPlugin {
    resources: [ParallelSafetyMetrics, RaceConditionDetector],

    update: [log_safety_metrics]
});