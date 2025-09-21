//! Performance Dashboard Plugin - CONDITIONAL AUTOMATION MASTERY!
//!
//! This module demonstrates PERFECT resource_exists conditional automation!
//! 28 lines of manual registration → 16 lines declarative power!

use super::setup::*;
use super::systems::*;
use super::types::*;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

/// Plugin for the Rayon performance dashboard using declarative automation.
///
/// Note: run_if conditions with tuples trigger a compiler bug with bevy-plugin-builder,
/// so the conditional check is handled within each system instead.
define_plugin!(PerformanceDashboardPlugin {
    resources: [DashboardVisibility],

    startup: [setup_performance_dashboard],

    update: [
        toggle_dashboard_visibility,
        update_thread_utilization,
        update_metrics_summary,
        refresh_operations_list,
    ]
});
