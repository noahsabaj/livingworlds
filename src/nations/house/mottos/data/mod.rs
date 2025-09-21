//! Motto data module - Gateway for all motto content and data access
//!
//! This module provides controlled access to motto variations and data management.
//! It follows the Living Worlds gateway architecture pattern by keeping all
//! implementation details private and exposing only the essential public API.

// Private submodules - implementation details hidden from external code
mod registry;
mod variations;

// Public re-exports - carefully controlled API surface

// Primary data access - the registry is the main interface
pub use registry::{MottoRegistry, MottoStatistics, TraitCultureStats};

// Direct data access for specialized use cases (used by registry internally)
pub use variations::{get_fallback_motto, get_variations_for_trait};

// Note: Raw variation data is NOT exposed - all access goes through the registry
// This ensures validation, caching, and proper error handling
