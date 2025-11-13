//! Content creation system for viral game moments - Gateway
//!
//! This module manages the detection, recording, and export of viral
//! game moments for social media sharing. It integrates with the Drama
//! Engine to capture clippable content automatically.

// PRIVATE modules - implementation details hidden
mod plugin;
mod types;
mod detection;
mod recording;
mod export;
mod highlights;

// CONTROLLED PUBLIC EXPORTS

// Main plugin for Bevy integration
pub use plugin::ContentCreationPlugin;

// Core types that external code needs

// Detection API - only what's needed for external integration

// Recording API - minimal public interface

// Export API - external export triggers
pub use export::recommend_platforms;

// Highlights API - reel management
