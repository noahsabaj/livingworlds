//! Culture-specific name generation gateway
//!
//! This module acts as the sole gateway to culture-specific generation logic.
//! All culture modules are private and their functionality is selectively
//! re-exported through this controlled interface.

// All culture modules are PRIVATE - only accessible through this gateway
mod ancient;
mod desert;
mod eastern;
mod island;
mod mystical;
mod northern;
mod southern;
mod western;

// Implementation logic is also PRIVATE
mod generator;

// Re-export controlled interface
pub use generator::{generate_house_name, generate_nation_name};
