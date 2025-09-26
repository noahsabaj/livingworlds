//! Recording subsystem - Gateway
//!
//! This module handles gameplay recording, frame buffering, and capture
//! functionality for creating shareable content.

// PRIVATE modules - implementation details hidden
mod buffer;
mod commands;
mod recorder;
mod types;

// CONTROLLED PUBLIC EXPORTS
pub use commands::handle_recording_commands;
pub use recorder::{update_recorder, ContentRecorder};
pub use types::RecordingConfig;