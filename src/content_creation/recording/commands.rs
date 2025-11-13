//! Recording command handling

use bevy::prelude::*;

use crate::content_creation::types::{RecordingAction, RecordingCommand};

use super::recorder::ContentRecorder;

/// System to handle recording commands from user or automated systems
pub fn handle_recording_commands(
    mut recorder: ResMut<ContentRecorder>,
    mut commands: MessageReader<RecordingCommand>,
    time: Res<Time>,
) {
    for command in commands.read() {
        match &command.action {
            RecordingAction::Start => {
                if !recorder.is_recording {
                    recorder.start_recording(time.elapsed_secs());
                }
            }
            RecordingAction::Stop => {
                if recorder.is_recording {
                    recorder.stop_recording();
                }
            }
            RecordingAction::SaveBuffer => {
                recorder.save_buffer();
            }
            RecordingAction::TakeScreenshot => {
                recorder.take_screenshot();
            }
            RecordingAction::ExportHighlight { event_id } => {
                info!("Exporting highlight for event {:?}", event_id);
                // Export implementation would go here
            }
        }

        // Apply options
        if command.options.slow_motion {
            // Implement slow motion recording
        }

        if command.options.zoom_to_action {
            // Implement camera zoom to action
        }
    }
}