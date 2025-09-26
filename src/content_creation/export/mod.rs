//! Export pipeline subsystem - Gateway
//!
//! This module handles exporting recorded content to various social media
//! platforms with appropriate formatting and captions.

// PRIVATE modules - implementation details hidden
mod captions;
mod formats;
mod platforms;
mod pipeline;

// CONTROLLED PUBLIC EXPORTS
pub use pipeline::{handle_export_requests, ExportPipeline};
pub use captions::generate_caption;
pub use platforms::recommend_platforms;