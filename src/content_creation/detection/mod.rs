//! Viral moment detection subsystem - Gateway
//!
//! This module detects potentially viral moments in gameplay that are worth
//! recording and sharing on social media platforms.

// PUBLIC modules for internal use within content_creation
pub mod scoring;

// PRIVATE modules - implementation details hidden
mod detector;
mod patterns;
mod types;

// CONTROLLED PUBLIC EXPORTS
pub use detector::{detect_viral_moments, ViralMomentDetector};
