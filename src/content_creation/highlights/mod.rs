//! Highlight reel subsystem - Gateway
//!
//! This module manages the collection and curation of viral moments
//! into a highlight reel for easy access and sharing.

// PRIVATE modules - implementation details hidden
mod reel;
mod tracking;

// CONTROLLED PUBLIC EXPORTS
pub use reel::{HighlightReel, add_highlight};
pub use tracking::{track_highlights, update_highlight_reel};