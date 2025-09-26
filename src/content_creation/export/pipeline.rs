//! Export pipeline implementation

use bevy::prelude::*;

use crate::content_creation::types::{ExportRequest, OutputFormat, SocialPlatform};

use super::formats::convert_format;
use super::platforms::optimize_for_platform;

/// Main export pipeline resource
#[derive(Resource, Default)]
pub struct ExportPipeline {
    /// Queue of pending export requests
    pending_exports: Vec<ExportRequest>,
    /// Currently processing export
    current_export: Option<ExportRequest>,
    /// Export statistics
    total_exports: u32,
    exports_by_platform: std::collections::HashMap<SocialPlatform, u32>,
}

impl ExportPipeline {
    /// Add an export request to the queue
    pub fn queue_export(&mut self, request: ExportRequest) {
        self.pending_exports.push(request);
    }

    /// Process the next export in the queue
    pub fn process_next(&mut self) {
        if self.current_export.is_none() && !self.pending_exports.is_empty() {
            self.current_export = Some(self.pending_exports.remove(0));

            if let Some(ref request) = self.current_export {
                info!("Processing export request for highlight {}", request.highlight_id);

                // Process each platform
                for platform in &request.platforms {
                    self.export_to_platform(request, *platform);
                    *self.exports_by_platform.entry(*platform).or_insert(0) += 1;
                }

                self.total_exports += 1;
                self.current_export = None;
            }
        }
    }

    /// Export content to a specific platform
    fn export_to_platform(&self, request: &ExportRequest, platform: SocialPlatform) {
        // Convert format if needed
        let format = optimize_for_platform(platform, request.format);

        info!("Exporting to {:?} in {:?} format", platform, format);

        // In a real implementation:
        // 1. Get the highlight data
        // 2. Convert to appropriate format
        // 3. Apply platform-specific optimizations
        // 4. Add watermark if requested
        // 5. Save to appropriate location
    }

    /// Get export statistics
    pub fn get_stats(&self) -> (u32, usize) {
        (self.total_exports, self.pending_exports.len())
    }
}

/// System to handle export requests
pub fn handle_export_requests(
    mut pipeline: ResMut<ExportPipeline>,
    mut export_events: EventReader<ExportRequest>,
) {
    // Queue new export requests
    for request in export_events.read() {
        pipeline.queue_export(request.clone());
    }

    // Process pending exports
    pipeline.process_next();
}