//! Format conversion utilities for export

use crate::content_creation::types::OutputFormat;

/// Convert content from one format to another
pub fn convert_format(from: OutputFormat, to: OutputFormat) -> Result<(), String> {
    if from == to {
        return Ok(());
    }

    match (from, to) {
        (OutputFormat::PNG, OutputFormat::JPEG) => {
            // Convert PNG to JPEG
            Ok(())
        }
        (OutputFormat::MP4, OutputFormat::GIF) => {
            // Convert video to GIF
            Ok(())
        }
        (OutputFormat::MP4, OutputFormat::WebM) => {
            // Convert MP4 to WebM
            Ok(())
        }
        _ => Err(format!("Unsupported conversion from {:?} to {:?}", from, to)),
    }
}

/// Get the best format for a given file size limit (in MB)
pub fn format_for_size_limit(size_limit_mb: f32) -> OutputFormat {
    match size_limit_mb {
        s if s < 3.0 => OutputFormat::GIF,
        s if s < 10.0 => OutputFormat::WebM,
        _ => OutputFormat::MP4,
    }
}

/// Get recommended quality settings for a format
pub fn quality_for_format(format: OutputFormat) -> f32 {
    match format {
        OutputFormat::PNG => 1.0,  // Lossless
        OutputFormat::JPEG => 0.85, // Good quality/size balance
        OutputFormat::MP4 => 0.8,   // Good quality video
        OutputFormat::WebM => 0.75, // Slightly lower for web
        OutputFormat::GIF => 0.6,   // Limited color palette
    }
}