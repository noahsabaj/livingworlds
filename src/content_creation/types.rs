//! Shared types for content creation system
//!
//! This module contains types that are used across multiple content creation
//! subsystems. Each subsystem has its own types.rs for domain-specific types.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::nations::{DramaEvent, DramaEventId};

/// Social media platforms for content export
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum SocialPlatform {
    TikTok,
    YouTube,
    Twitter,
    Reddit,
    Instagram,
    Discord,
    Twitch,
    Steam,
}

/// Output format for recordings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum OutputFormat {
    MP4,
    GIF,
    WebM,
    PNG,  // For screenshots
    JPEG, // For thumbnails
}

/// Recording modes available in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum RecordingMode {
    Continuous,     // Record everything
    Highlights,     // Only viral moments
    Timelapse,      // Speed up time
    Cinematic,      // Special camera angles
    Screenshot,     // Single frame capture
}

/// How important/shareable an event is
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Reflect)]
pub enum ViralPotential {
    None,        // Not worth sharing
    Low,         // Might interest some
    Medium,      // Worth a screenshot
    High,        // Definitely shareable
    Legendary,   // Goes viral instantly
}

/// A moment that was detected as potentially viral
#[derive(Event, Debug, Clone)]
pub struct ViralMomentDetected {
    pub event: DramaEvent,
    pub viral_score: f32,
    pub potential: ViralPotential,
    pub suggested_caption: String,
    pub recommended_platforms: Vec<SocialPlatform>,
    pub timestamp: f32,
}

/// Command to control recording
#[derive(Event, Debug, Clone)]
pub struct RecordingCommand {
    pub action: RecordingAction,
    pub options: RecordingOptions,
}

#[derive(Debug, Clone)]
pub enum RecordingAction {
    Start,
    Stop,
    SaveBuffer,     // Save last X seconds
    TakeScreenshot,
    ExportHighlight { event_id: DramaEventId },
}

#[derive(Debug, Clone, Default)]
pub struct RecordingOptions {
    pub include_ui: bool,
    pub add_captions: bool,
    pub zoom_to_action: bool,
    pub slow_motion: bool,
    pub export_platform: Option<SocialPlatform>,
}

/// A single highlight moment that's been captured
#[derive(Debug, Clone)]
pub struct Highlight {
    pub id: u32,
    pub timestamp: f32,
    pub event: DramaEvent,
    pub viral_score: f32,
    pub caption: String,
    pub thumbnail_captured: bool,
    pub exported: bool,
    pub platform_exports: Vec<SocialPlatform>,
}

/// Export request for content
#[derive(Event, Debug, Clone)]
pub struct ExportRequest {
    pub highlight_id: u32,
    pub format: OutputFormat,
    pub platforms: Vec<SocialPlatform>,
    pub include_watermark: bool,
}