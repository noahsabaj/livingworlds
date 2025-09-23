//! Error Context Capture System
//!
//! This module provides comprehensive error context capture for debugging and user feedback.
//! When errors occur (especially during world generation), this system captures relevant
//! diagnostic information to help users understand what went wrong and developers to debug issues.

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// Comprehensive error context captured when failures occur
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorContext {
    /// The actual error message
    pub error_message: String,

    /// Error category for routing
    pub error_type: ErrorType,

    /// When the error occurred
    pub timestamp: SystemTime,

    /// Current game state when error occurred
    pub game_state: String,

    /// World generation metrics if this was a generation error
    pub generation_metrics: Option<GenerationMetrics>,

    /// Performance metrics at time of error
    pub performance_metrics: Option<PerformanceMetrics>,

    /// GPU status if available
    pub gpu_status: Option<GpuStatus>,

    /// Suggested recovery actions
    pub recovery_suggestions: Vec<String>,
}

/// Types of errors for categorization and handling
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ErrorType {
    WorldGeneration,
    RiverGeneration,
    MeshBuilding,
    ResourceLoading,
    SaveLoadFailure,
    ModLoadingFailure,
    GpuError,
    OutOfMemory,
    Unknown,
}

/// Detailed metrics about world generation at point of failure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerationMetrics {
    /// Percentage of world covered by ocean
    pub ocean_percentage: f32,

    /// Percentage of world that is land
    pub land_percentage: f32,

    /// Min and max elevation values
    pub elevation_range: (f32, f32),

    /// Calculated sea level
    pub sea_level: f32,

    /// Number of valid river sources found
    pub river_sources_found: usize,

    /// Number of provinces above mountain threshold
    pub mountain_count: usize,

    /// Number of continent seeds used
    pub continent_seeds: usize,

    /// Total provinces generated
    pub total_provinces: usize,

    /// Time taken to generate (milliseconds)
    pub generation_time_ms: u128,

    /// World size setting
    pub world_size: String,

    /// Ocean coverage setting
    pub ocean_coverage_setting: f32,

    /// River density setting
    pub river_density: f32,
}

/// Performance metrics at time of error
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Current FPS
    pub fps: f32,

    /// Memory usage in MB
    pub memory_usage_mb: f32,

    /// Number of entities
    pub entity_count: usize,

    /// CPU usage percentage (if available)
    pub cpu_usage: Option<f32>,
}

/// GPU status information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GpuStatus {
    /// Whether GPU acceleration was enabled
    pub enabled: bool,

    /// GPU device name
    pub device_name: String,

    /// Available VRAM in MB
    pub vram_available_mb: Option<f32>,

    /// GPU compute capability
    pub compute_capability: Option<String>,
}

impl ErrorContext {
    /// Create a new error context for world generation failures
    pub fn from_generation_error(
        error_message: String,
        metrics: Option<GenerationMetrics>,
        game_state: String,
    ) -> Self {
        let mut recovery_suggestions = Vec::new();

        // Add smart suggestions based on the metrics
        if let Some(ref m) = metrics {
            if m.ocean_percentage > 95.0 {
                recovery_suggestions.push("Try reducing ocean coverage to 60% or less".to_string());
            }
            if m.ocean_percentage < 5.0 {
                recovery_suggestions.push("Try increasing ocean coverage to at least 40%".to_string());
            }
            if m.river_sources_found == 0 && m.mountain_count == 0 {
                recovery_suggestions.push("No mountains found for rivers. Try reducing ocean coverage or changing the seed".to_string());
            }
            if m.elevation_range.1 - m.elevation_range.0 < 0.1 {
                recovery_suggestions.push("Elevation range too flat. Try a different seed or adjust continent count".to_string());
            }
        }

        // Add general suggestions
        if recovery_suggestions.is_empty() {
            recovery_suggestions.push("Try using different world generation settings".to_string());
            recovery_suggestions.push("Consider using a different seed value".to_string());
        }

        Self {
            error_message,
            error_type: ErrorType::WorldGeneration,
            timestamp: SystemTime::now(),
            game_state,
            generation_metrics: metrics,
            performance_metrics: None,
            gpu_status: None,
            recovery_suggestions,
        }
    }

    /// Save error context to a JSON file for debugging
    pub fn save_to_file(&self) -> Result<PathBuf, std::io::Error> {
        use chrono::Local;

        // Create timestamp for filename
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("error_{}_{}.json",
            match self.error_type {
                ErrorType::WorldGeneration => "worldgen",
                ErrorType::RiverGeneration => "river",
                ErrorType::MeshBuilding => "mesh",
                ErrorType::SaveLoadFailure => "saveload",
                _ => "other",
            },
            timestamp
        );

        // Determine save directory
        // Use a local directory for now (dirs crate would need to be added to Cargo.toml)
        let path = std::path::PathBuf::from("./error_logs")
            .join(filename);

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Serialize to JSON with pretty printing
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json)?;

        info!("Error context saved to: {:?}", path);
        Ok(path)
    }

    /// Generate a user-friendly error report
    pub fn format_for_display(&self) -> String {
        let mut report = String::new();

        // Main error
        report.push_str(&format!("Error: {}\n", self.error_message));
        report.push_str(&format!("Type: {:?}\n", self.error_type));

        // Generation metrics if available
        if let Some(ref metrics) = self.generation_metrics {
            report.push_str("\nWorld Generation Details:\n");
            report.push_str(&format!("  • Ocean Coverage: {:.1}%\n", metrics.ocean_percentage));
            report.push_str(&format!("  • Land Coverage: {:.1}%\n", metrics.land_percentage));
            report.push_str(&format!("  • Sea Level: {:.3}\n", metrics.sea_level));
            report.push_str(&format!("  • Elevation Range: {:.3} to {:.3}\n",
                metrics.elevation_range.0, metrics.elevation_range.1));
            report.push_str(&format!("  • River Sources Found: {}\n", metrics.river_sources_found));
            report.push_str(&format!("  • Mountains: {}\n", metrics.mountain_count));
            report.push_str(&format!("  • World Size: {}\n", metrics.world_size));
        }

        // Recovery suggestions
        if !self.recovery_suggestions.is_empty() {
            report.push_str("\nSuggested Solutions:\n");
            for (i, suggestion) in self.recovery_suggestions.iter().enumerate() {
                report.push_str(&format!("  {}. {}\n", i + 1, suggestion));
            }
        }

        report
    }

    /// Generate a GitHub issue body with all context
    pub fn format_for_github_issue(&self) -> String {
        let mut body = String::new();

        body.push_str("## Error Report\n\n");
        body.push_str(&format!("**Error Message:** {}\n", self.error_message));
        body.push_str(&format!("**Error Type:** {:?}\n", self.error_type));
        body.push_str(&format!("**Game State:** {}\n", self.game_state));
        body.push_str(&format!("**Timestamp:** {:?}\n\n", self.timestamp));

        if let Some(ref metrics) = self.generation_metrics {
            body.push_str("## World Generation Metrics\n\n");
            body.push_str("```json\n");
            if let Ok(json) = serde_json::to_string_pretty(metrics) {
                body.push_str(&json);
            }
            body.push_str("\n```\n\n");
        }

        body.push_str("## System Information\n\n");
        body.push_str(&format!("- Platform: {}\n", std::env::consts::OS));
        body.push_str(&format!("- Architecture: {}\n", std::env::consts::ARCH));

        body
    }
}

/// System for capturing performance metrics when errors occur
pub fn capture_performance_metrics(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<Entity>,
) -> PerformanceMetrics {
    // Get FPS from diagnostics
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or(0.0) as f32;

    // Count entities
    let entity_count = query.iter().count();

    // Estimate memory usage (very rough)
    let memory_usage_mb = (entity_count as f32 * 0.001) + 100.0; // Rough estimate

    PerformanceMetrics {
        fps,
        memory_usage_mb,
        entity_count,
        cpu_usage: None, // Would need system-specific implementation
    }
}

/// Collect generation metrics from the current world state
pub fn collect_generation_metrics(
    provinces: &[crate::world::Province],
    sea_level: f32,
    generation_time_ms: u128,
    settings: &crate::world::WorldGenerationSettings,
) -> GenerationMetrics {
    use crate::constants::RIVER_MIN_ELEVATION;

    let total = provinces.len();
    let ocean_count = provinces.iter()
        .filter(|p| p.terrain == crate::world::TerrainType::Ocean)
        .count();
    let land_count = total - ocean_count;

    let ocean_percentage = (ocean_count as f32 / total as f32) * 100.0;
    let land_percentage = (land_count as f32 / total as f32) * 100.0;

    // Find elevation range
    let elevations: Vec<f32> = provinces.iter()
        .map(|p| p.elevation.value())
        .collect();
    let min_elev = elevations.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max_elev = elevations.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

    // Count river sources (provinces above RIVER_MIN_ELEVATION)
    let river_sources_found = provinces.iter()
        .filter(|p| p.elevation.value() > RIVER_MIN_ELEVATION &&
                    p.terrain != crate::world::TerrainType::Ocean)
        .count();

    // Count mountains (using same threshold as rivers for consistency)
    let mountain_count = river_sources_found; // Same as river sources

    GenerationMetrics {
        ocean_percentage,
        land_percentage,
        elevation_range: (min_elev, max_elev),
        sea_level,
        river_sources_found,
        mountain_count,
        continent_seeds: settings.continent_count as usize,
        total_provinces: total,
        generation_time_ms,
        world_size: format!("{:?}", settings.world_size),
        ocean_coverage_setting: settings.ocean_coverage,
        river_density: settings.river_density,
    }
}