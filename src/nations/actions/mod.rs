//! Nation action system - pressure-driven decision making and execution
//!
//! This module implements the complete action loop:
//! 1. Resolution: Analyze pressures and decide what actions to take
//! 2. Execution: Actually perform the actions and update game state

// Private modules
mod resolution;
mod execution;

// Public exports - event types
pub use resolution::{
    NationActionEvent,
    ReformType,
    PublicWorkType,
    NationAction,
    ActionType,
};

// Public exports - systems
pub use resolution::{
    resolve_nation_actions,
    handle_population_pressure,
    handle_economic_pressure,
    handle_military_pressure,
    handle_legitimacy_pressure,
};

pub use execution::{
    execute_expansion_events,
    // force_overlay_refresh_on_expansion REMOVED - now uses reactive invalidation
};
