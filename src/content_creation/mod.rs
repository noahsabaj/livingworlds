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
pub use types::{
    OutputFormat,
    SocialPlatform,
    RecordingMode,
    ViralPotential,
    ViralMomentDetected,
    RecordingCommand,
    RecordingAction,
    RecordingOptions,
    Highlight,
    ExportRequest,
};

// Detection API - only what's needed for external integration
pub use detection::{
    ViralMomentDetector,
    detect_viral_moments,
    calculate_viral_score,
    ViralPattern,
};

// Recording API - minimal public interface
pub use recording::{
    ContentRecorder,
    handle_recording_commands,
    update_recorder,
};

// Export API - external export triggers
pub use export::{
    ExportPipeline,
    handle_export_requests,
    recommend_platforms,
    generate_caption,
};

// Highlights API - reel management
pub use highlights::{
    HighlightReel,
    add_highlight,
    track_highlights,
    update_highlight_reel,
};