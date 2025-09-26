//! Content creation plugin for Living Worlds
//!
//! This plugin coordinates all content creation subsystems including viral
//! moment detection, recording, export, and highlight management.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use crate::states::GameState;

use super::detection::{detect_viral_moments, ViralMomentDetector};
use super::export::{handle_export_requests, ExportPipeline};
use super::highlights::{track_highlights, update_highlight_reel, HighlightReel};
use super::recording::{handle_recording_commands, update_recorder, ContentRecorder};
use super::types::{ExportRequest, RecordingCommand, ViralMomentDetected};

define_plugin!(ContentCreationPlugin {
    resources: [
        ViralMomentDetector,
        ContentRecorder,
        HighlightReel,
        ExportPipeline,
    ],

    events: [
        ViralMomentDetected,
        RecordingCommand,
        ExportRequest,
    ],

    update: [
        // Detection systems
        detect_viral_moments.run_if(in_state(GameState::InGame)),

        // Recording systems
        handle_recording_commands.run_if(in_state(GameState::InGame)),
        update_recorder.run_if(in_state(GameState::InGame)),

        // Highlight tracking
        track_highlights.run_if(in_state(GameState::InGame)),
        update_highlight_reel.run_if(in_state(GameState::InGame)),

        // Export pipeline
        handle_export_requests,
    ],
});