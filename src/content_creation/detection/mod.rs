//! Viral moment detection subsystem - Gateway
//!
//! This module detects potentially viral moments in gameplay that are worth
//! recording and sharing on social media platforms.

// PRIVATE modules - implementation details hidden
mod detector;
mod patterns;
mod scoring;
mod types;

// CONTROLLED PUBLIC EXPORTS
pub use detector::{detect_viral_moments, ViralMomentDetector};
pub use patterns::ViralPattern;
pub use scoring::calculate_viral_score;
pub use types::DetectionConfig;