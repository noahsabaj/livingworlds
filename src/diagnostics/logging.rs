//! Structured Logging Infrastructure
//!
//! This module provides comprehensive logging utilities for Living Worlds,
//! including structured logging, tracing spans, and performance tracking.
//!
//! # Features
//! - Structured logging with context and metadata
//! - Tracing spans for tracking complex operations
//! - Performance timing utilities
//! - Debug output formatting
//!
//! # Usage
//! ```rust
//! use crate::diagnostics::logging::{log_world_gen_step, create_span, log_performance};
//!
//! let span = create_span("world_generation");
//! log_world_gen_step("terrain", 1000000, 2.5);
//! ```

use bevy::log::{debug, error, info, trace, warn};
use std::time::Instant;
use std::fmt::Display;

/// Log levels for structured logging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Performance timing helper
pub struct TimedOperation {
    name: String,
    start: Instant,
    log_level: LogLevel,
}

impl TimedOperation {
    /// Start timing an operation
    pub fn start(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            log_level: LogLevel::Debug,
        }
    }

    /// Start timing with specific log level
    pub fn start_with_level(name: impl Into<String>, level: LogLevel) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            log_level: level,
        }
    }

    /// Complete the operation and log the elapsed time
    pub fn complete(self) -> f32 {
        let elapsed = self.start.elapsed();
        let ms = elapsed.as_secs_f32() * 1000.0;

        match self.log_level {
            LogLevel::Trace => trace!("{} completed in {:.2}ms", self.name, ms),
            LogLevel::Debug => debug!("{} completed in {:.2}ms", self.name, ms),
            LogLevel::Info => info!("{} completed in {:.2}ms", self.name, ms),
            LogLevel::Warn => warn!("{} completed in {:.2}ms", self.name, ms),
            LogLevel::Error => error!("{} completed in {:.2}ms", self.name, ms),
        }

        ms
    }

    /// Complete with additional context
    pub fn complete_with_context<T: Display>(self, context: T) -> f32 {
        let elapsed = self.start.elapsed();
        let ms = elapsed.as_secs_f32() * 1000.0;

        match self.log_level {
            LogLevel::Trace => trace!("{} completed in {:.2}ms - {}", self.name, ms, context),
            LogLevel::Debug => debug!("{} completed in {:.2}ms - {}", self.name, ms, context),
            LogLevel::Info => info!("{} completed in {:.2}ms - {}", self.name, ms, context),
            LogLevel::Warn => warn!("{} completed in {:.2}ms - {}", self.name, ms, context),
            LogLevel::Error => error!("{} completed in {:.2}ms - {}", self.name, ms, context),
        }

        ms
    }
}

/// Log world generation steps with consistent formatting
pub fn log_world_gen_step(step: &str, count: usize, elapsed_ms: f32) {
    info!(
        "[World Generation] {} - {} items processed in {:.2}ms ({:.1} items/ms)",
        step, count, elapsed_ms, count as f32 / elapsed_ms.max(0.001)
    );
}

/// Log world generation progress
pub fn log_world_gen_progress(phase: &str, progress: f32, details: Option<&str>) {
    if let Some(details) = details {
        info!(
            "[World Generation] {} - {:.1}% complete - {}",
            phase, progress * 100.0, details
        );
    } else {
        info!(
            "[World Generation] {} - {:.1}% complete",
            phase, progress * 100.0
        );
    }
}

/// Log nation AI decisions
pub fn log_nation_decision(nation_id: u32, nation_name: &str, decision_type: &str, details: &str) {
    debug!(
        "[Nation AI] {} ({}) - Decision: {} - {}",
        nation_name, nation_id, decision_type, details
    );
}

/// Log nation state changes
pub fn log_nation_state_change(nation_id: u32, nation_name: &str, old_state: &str, new_state: &str, reason: &str) {
    info!(
        "[Nation State] {} ({}) - Transition: {} -> {} (Reason: {})",
        nation_name, nation_id, old_state, new_state, reason
    );
}

/// Log simulation updates with timing
pub fn log_simulation_update(frame: u32, entity_count: usize, elapsed_ms: f32) {
    trace!(
        "[Simulation] Frame {} - {} entities updated in {:.2}ms ({:.1} entities/ms)",
        frame, entity_count, elapsed_ms, entity_count as f32 / elapsed_ms.max(0.001)
    );
}

/// Log performance warnings
pub fn log_performance_warning(system: &str, metric: &str, value: f32, threshold: f32) {
    warn!(
        "[Performance] {} - {} ({:.2}) exceeded threshold ({:.2})",
        system, metric, value, threshold
    );
}

/// Log memory usage
pub fn log_memory_usage(system: &str, bytes: usize) {
    let mb = bytes as f32 / (1024.0 * 1024.0);
    debug!("[Memory] {} - {:.2} MB allocated", system, mb);
}

/// Log relationship operations
pub fn log_relationship_operation(operation: &str, source_entity: bevy::ecs::entity::Entity, target_entity: bevy::ecs::entity::Entity, relationship_type: &str) {
    trace!(
        "[Relationships] {} - Source: {:?}, Target: {:?}, Type: {}",
        operation, source_entity, target_entity, relationship_type
    );
}

/// Log law system events
pub fn log_law_event(nation_id: u32, nation_name: &str, event_type: &str, law_name: &str, details: Option<&str>) {
    if let Some(details) = details {
        info!(
            "[Law System] {} ({}) - {}: {} - {}",
            nation_name, nation_id, event_type, law_name, details
        );
    } else {
        info!(
            "[Law System] {} ({}) - {}: {}",
            nation_name, nation_id, event_type, law_name
        );
    }
}

/// Log error with context
pub fn log_error_with_context(system: &str, error: &str, context: impl Display) {
    error!("[{}] Error: {} - Context: {}", system, error, context);
}

/// Create a debug context string for complex objects
pub fn debug_context<T: std::fmt::Debug>(label: &str, object: &T) -> String {
    format!("{}: {:?}", label, object)
}

/// Batch logging for multiple items
pub struct BatchLogger {
    system: String,
    items: Vec<String>,
    max_items: usize,
}

impl BatchLogger {
    pub fn new(system: impl Into<String>) -> Self {
        Self {
            system: system.into(),
            items: Vec::new(),
            max_items: 100,
        }
    }

    pub fn add(&mut self, item: impl Into<String>) {
        if self.items.len() < self.max_items {
            self.items.push(item.into());
        }
    }

    pub fn log(self, level: LogLevel) {
        if self.items.is_empty() {
            return;
        }

        let message = format!(
            "[{}] Batch operation - {} items:\n{}",
            self.system,
            self.items.len(),
            self.items.join("\n  - ")
        );

        match level {
            LogLevel::Trace => trace!("{}", message),
            LogLevel::Debug => debug!("{}", message),
            LogLevel::Info => info!("{}", message),
            LogLevel::Warn => warn!("{}", message),
            LogLevel::Error => error!("{}", message),
        }
    }
}

/// Performance metrics aggregator
pub struct PerformanceAggregator {
    name: String,
    samples: Vec<f32>,
}

impl PerformanceAggregator {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            samples: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, value: f32) {
        self.samples.push(value);
    }

    pub fn log_summary(self) {
        if self.samples.is_empty() {
            return;
        }

        let sum: f32 = self.samples.iter().sum();
        let avg = sum / self.samples.len() as f32;
        let min = self.samples.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = self.samples.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        info!(
            "[Performance] {} - Samples: {}, Avg: {:.2}ms, Min: {:.2}ms, Max: {:.2}ms",
            self.name,
            self.samples.len(),
            avg,
            min,
            max
        );
    }
}