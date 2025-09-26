//! Law type definitions gateway
//!
//! Provides access to all law type definitions organized into
//! focused modules for clarity and maintainability.

// Private modules - gateway architecture
mod core;
mod effects;
mod events;
mod status;

// Test module
#[cfg(test)]
mod tests;

// Re-export all types through the gateway
pub use core::{Law, LawId, LawCategory, LawPrerequisite};
pub use effects::{LawEffects, PopularityWeights};
pub use events::{LawEnactmentEvent, LawRepealEvent};
pub use status::{LawStatus, LawComplexity, LawPopularity};