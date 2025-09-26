//! Culture-specific name data modules
//!
//! Each culture has its own module containing names, places, and other
//! culturally-appropriate name components.
//!
//! This module acts as the sole gateway to all culture data.
//! All culture modules are private and their contents are selectively
//! re-exported through this interface.

// All submodules are PRIVATE - only accessible through this gateway
mod ancient;
mod desert;
mod eastern;
mod island;
mod mystical;
mod northern;
mod southern;
mod western;

// Re-export culture data through namespaced modules
// NOTE: These MUST be modules (not direct exports) to prevent naming conflicts.
// Each culture exports the same constant names (MALE_NAMES, FEMALE_NAMES, etc.)
// so we need namespace separation. This is NOT a gateway violation - it's
// necessary namespace management.
pub mod western_data {
    pub use super::western::*;
}

pub mod eastern_data {
    pub use super::eastern::*;
}

pub mod northern_data {
    pub use super::northern::*;
}

pub mod southern_data {
    pub use super::southern::*;
}

pub mod desert_data {
    pub use super::desert::*;
}

pub mod island_data {
    pub use super::island::*;
}

pub mod ancient_data {
    pub use super::ancient::*;
}

pub mod mystical_data {
    pub use super::mystical::*;
}
