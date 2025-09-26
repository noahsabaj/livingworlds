//! Highlight reel management

use bevy::prelude::*;

use crate::content_creation::types::{Highlight, SocialPlatform};
use crate::nations::DramaEvent;

/// Resource tracking the best moments for export
#[derive(Resource)]
pub struct HighlightReel {
    /// Collection of highlights
    pub highlights: Vec<Highlight>,
    /// Maximum number of highlights to keep
    pub max_highlights: usize,
    /// Whether to auto-export highly viral moments
    pub auto_export: bool,
    /// Next highlight ID
    next_id: u32,
}

impl Default for HighlightReel {
    fn default() -> Self {
        Self {
            highlights: Vec::new(),
            max_highlights: 100,
            auto_export: false,
            next_id: 1,
        }
    }
}

impl HighlightReel {
    /// Add a new highlight to the reel
    pub fn add(&mut self, event: DramaEvent, viral_score: f32, caption: String, timestamp: f32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let highlight = Highlight {
            id,
            timestamp,
            event,
            viral_score,
            caption,
            thumbnail_captured: false,
            exported: false,
            platform_exports: Vec::new(),
        };

        self.highlights.push(highlight);
        self.trim_to_max();

        id
    }

    /// Keep only the best highlights up to max_highlights
    fn trim_to_max(&mut self) {
        if self.highlights.len() > self.max_highlights {
            // Sort by viral score (descending)
            self.highlights.sort_by(|a, b|
                b.viral_score.partial_cmp(&a.viral_score).unwrap()
            );
            self.highlights.truncate(self.max_highlights);
        }
    }

    /// Get a highlight by ID
    pub fn get(&self, id: u32) -> Option<&Highlight> {
        self.highlights.iter().find(|h| h.id == id)
    }

    /// Get a mutable highlight by ID
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Highlight> {
        self.highlights.iter_mut().find(|h| h.id == id)
    }

    /// Mark a highlight as exported to a platform
    pub fn mark_exported(&mut self, id: u32, platform: SocialPlatform) {
        if let Some(highlight) = self.get_mut(id) {
            if !highlight.platform_exports.contains(&platform) {
                highlight.platform_exports.push(platform);
            }
            highlight.exported = true;
        }
    }

    /// Get highlights that haven't been exported yet
    pub fn get_unexported(&self) -> Vec<&Highlight> {
        self.highlights.iter()
            .filter(|h| !h.exported)
            .collect()
    }

    /// Get the most viral highlights
    pub fn get_top(&self, count: usize) -> Vec<&Highlight> {
        let mut sorted = self.highlights.clone();
        sorted.sort_by(|a, b| b.viral_score.partial_cmp(&a.viral_score).unwrap());
        sorted.iter().take(count).collect()
    }
}

/// Helper function to add a highlight
pub fn add_highlight(
    reel: &mut HighlightReel,
    event: DramaEvent,
    viral_score: f32,
    caption: String,
    timestamp: f32,
) -> u32 {
    reel.add(event, viral_score, caption, timestamp)
}