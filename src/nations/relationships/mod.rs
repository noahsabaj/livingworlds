//! Entity relationship components for the nation system
//!
//! This module contains all Bevy 0.17 relationship components that define
//! connections between nations, territories, and other political entities.
//! These relationships enable idiomatic, non-fragmenting entity queries.

mod warfare;
mod diplomatic_extended;
mod neighbors;
mod historical;

pub use warfare::*;
pub use diplomatic_extended::*;
pub use neighbors::*;
pub use historical::*;
