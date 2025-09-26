//! Types specific to recording functionality

use bevy::prelude::*;

/// Configuration for recording system
#[derive(Debug, Clone)]
pub struct RecordingConfig {
    /// Maximum recording duration in seconds
    pub max_duration: f32,
    /// Target frames per second for recording
    pub target_fps: u32,
    /// Video quality (0.0 to 1.0)
    pub quality: f32,
    /// Whether to include game audio
    pub include_audio: bool,
    /// Whether to show watermark
    pub watermark: bool,
}

impl Default for RecordingConfig {
    fn default() -> Self {
        Self {
            max_duration: 300.0, // 5 minutes max
            target_fps: 60,
            quality: 0.8,
            include_audio: true,
            watermark: true,
        }
    }
}