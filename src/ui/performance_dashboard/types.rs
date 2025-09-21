//! Performance Dashboard Types

use bevy::prelude::*;

/// Resource controlling dashboard visibility
#[derive(Resource, Default)]
pub struct DashboardVisibility {
    pub visible: bool,
    pub expanded: bool,
    pub show_graph: bool,
    pub show_operations: bool,
    pub max_operations_shown: usize,
}

impl DashboardVisibility {
    pub fn new() -> Self {
        Self {
            visible: false, // Hidden by default
            expanded: false,
            show_graph: true,
            show_operations: true,
            max_operations_shown: 10,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

/// Marker component for the main performance panel
#[derive(Component)]
pub struct PerformancePanel;

/// Marker for thread utilization display
#[derive(Component)]
pub struct ThreadUtilizationDisplay;

/// Marker for operations list
#[derive(Component)]
pub struct OperationsListDisplay;

/// Marker for metrics summary
#[derive(Component)]
pub struct MetricsSummaryDisplay;

/// Display mode for the dashboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Compact,
    Expanded,
    GraphOnly,
    ListOnly,
}
