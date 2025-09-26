//! Law registry gateway
//!
//! Manages global law registration, per-nation law tracking,
//! and historical law change records.

// Private modules - gateway architecture
mod global;
mod nation;
mod tracking;
mod history;
mod types;

// Public exports - controlled API surface
pub use global::LawRegistry;
pub use nation::NationLaws;
pub use tracking::ActiveLaws;
pub use history::LawHistory;
pub use types::{ProposedLaw, LawChange, LawChangeType};