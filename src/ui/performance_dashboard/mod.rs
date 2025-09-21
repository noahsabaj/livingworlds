//! Performance Dashboard - Gateway Module
//!
//! Real-time visualization of Rayon parallel operation metrics.

// Submodules (all private - gateway pattern)
mod plugin;
mod setup;
mod systems;
mod types;

// Public exports - controlled API surface
pub use plugin::PerformanceDashboardPlugin;
pub use types::{DashboardVisibility, PerformancePanel};
