//! Universal Name Generator System for Living Worlds
//!
//! This module provides culturally diverse, contextually appropriate names
//! for all entities in the game - from ancient civilizations to individual
//! leaders, from mighty rivers to small villages.
//!
//! # Architecture
//!
//! The name generator uses a modular architecture with specialized modules:
//! - `core`: Main NameGenerator struct and orchestration
//! - `world`: Complex world name generation patterns
//! - `places`: Province and city name generation
//! - `people`: Person names with cultural titles
//! - `geographic`: Natural feature names (rivers, mountains, etc.)
//! - `cultures`: Culture-specific generation patterns
//! - `types`: All enum and type definitions
//! - `data`: Name databases organized by culture and type
//! - `utils`: Utility functions
//!
//! # Examples
//!
//! ```rust
//! use living_worlds::name_generator::{NameGenerator, NameType, Culture};
//!
//! // Create a new generator with random seed
//! let mut gen = NameGenerator::new();
//!
//! // Generate a world name
//! let world = gen.generate(NameType::World);
//!
//! // Generate a culturally appropriate nation name
//! let nation = gen.generate(NameType::Nation { culture: Culture::Eastern });
//!
//! // Generate a person with title
//! let ruler = gen.generate(NameType::Person {
//!     gender: Gender::Female,
//!     culture: Culture::Western,
//!     role: PersonRole::Ruler,
//! });
//! ```
//!
//! # Deterministic Generation
//!
//! For reproducible worlds, use a seeded generator:
//!
//! ```rust
//! let mut gen = NameGenerator::with_seed(12345);
//! // Names will be the same for the same seed
//! ```

// Internal modules - ALL PRIVATE, only accessible through this gateway
mod core; // Main NameGenerator struct and orchestration
mod cultures; // Culture-specific generation patterns
mod data; // Name databases
mod geographic; // Natural feature names
mod people; // Person names with titles
mod places; // Province and city name generation
mod types; // Type definitions
mod utils;
mod world; // World name generation patterns // Utility functions

// CONTROLLED PUBLIC API - This is the ONLY way in/out of name_generator
// Re-export only what external code needs
pub use core::NameGenerator;
pub use types::{CitySize, Culture, Gender, NameRelation, NameType, PersonRole, Region};

// Selectively expose utility functions
pub use utils::to_roman_numeral;

// Module documentation for key features
/// The name generator supports 8 distinct cultural styles
pub const SUPPORTED_CULTURES: usize = 8;

/// Maximum number of attempts to generate a unique name before appending numbers
pub const MAX_UNIQUENESS_ATTEMPTS: usize = 50;
