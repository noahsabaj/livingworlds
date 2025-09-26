//! Main recording system implementation

use bevy::prelude::*;

use crate::content_creation::types::{OutputFormat, RecordingMode};

use super::buffer::FrameBuffer;
use super::types::RecordingConfig;

/// Resource for managing gameplay recording
#[derive(Resource)]
pub struct ContentRecorder {
    pub is_recording: bool,
    pub recording_mode: RecordingMode,
    pub output_format: OutputFormat,
    pub config: RecordingConfig,
    pub frame_buffer: FrameBuffer,
    pub recording_start_time: Option<f32>,
    pub auto_record_viral: bool,
}

impl Default for ContentRecorder {
    fn default() -> Self {
        Self {
            is_recording: false,
            recording_mode: RecordingMode::Highlights,
            output_format: OutputFormat::MP4,
            config: RecordingConfig::default(),
            frame_buffer: FrameBuffer::new(30.0), // 30 second buffer
            recording_start_time: None,
            auto_record_viral: true,
        }
    }
}

impl ContentRecorder {
    /// Start recording
    pub fn start_recording(&mut self, current_time: f32) {
        self.is_recording = true;
        self.recording_start_time = Some(current_time);
        info!("Started recording in {:?} mode", self.recording_mode);
    }

    /// Stop recording
    pub fn stop_recording(&mut self) -> Option<f32> {
        self.is_recording = false;
        let duration = self.recording_start_time.map(|start| {
            // Would calculate actual duration here
            0.0
        });
        self.recording_start_time = None;
        info!("Stopped recording");
        duration
    }

    /// Check if we should auto-record this moment
    pub fn should_auto_record(&self, viral_score: f32) -> bool {
        self.auto_record_viral && viral_score >= 0.8
    }

    /// Save the current buffer to file
    pub fn save_buffer(&self) {
        info!("Saving {} seconds of buffered frames", self.frame_buffer.buffer_duration);
        // Integration with bevy_capture_media would go here
    }

    /// Take a screenshot
    pub fn take_screenshot(&self) {
        info!("Taking screenshot");
        // Screenshot implementation would go here
    }
}

/// System to update the recorder state
pub fn update_recorder(
    mut recorder: ResMut<ContentRecorder>,
    time: Res<Time>,
) {
    if recorder.is_recording {
        // Update recording state
        // In a real implementation, we'd capture frames here
    }

    // Update frame buffer
    recorder.frame_buffer.update(time.delta_secs());
}