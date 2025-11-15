//! Export pipeline subsystem - Gateway
//!
//! This module handles exporting recorded content to various social media
//! platforms with appropriate formatting and captions.

// PUBLIC modules for internal use within content_creation
pub mod captions;
pub mod platforms;

// PRIVATE modules - implementation details hidden
mod formats;
mod pipeline;

// CONTROLLED PUBLIC EXPORTS
pub use pipeline::{handle_export_requests, ExportPipeline};
pub use platforms::recommend_platforms;