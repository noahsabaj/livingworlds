//! Infrastructure Module - System-Level Configuration and Resource Management
//!
//! This module provides system-level infrastructure services for Living Worlds,
//! following the gateway architecture pattern. It handles:
//! - Thread pool configuration and management for parallel world generation
//! - Logging system initialization and configuration
//! - System resource detection and optimization
//!
//! # Gateway Architecture
//! This module controls access to infrastructure components through a clean API.
//! Internal implementation details are private, and external code can only
//! access functionality through the exported interfaces.
//!
//! # Usage
//! ```rust
//! use living_worlds::infrastructure::{ThreadPoolManager, LoggingConfig};
//!
//! // Initialize logging
//! LoggingConfig::initialize(debug_mode);
//!
//! // Setup thread pool for parallel processing
//! ThreadPoolManager::initialize(thread_count)?;
//! ```

// Private module declarations - implementation details hidden from external code
mod logging;
mod threads;

// Public exports - controlled API surface following gateway pattern
pub use logging::LoggingConfig;
pub use threads::ThreadPoolManager;

// Re-export common error type for infrastructure operations
pub use threads::InfrastructureError;
