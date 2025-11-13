//! Thread Pool Management for Living Worlds
//!
//! This module handles thread pool configuration and initialization for parallel
//! world generation operations. It provides intelligent thread count detection
//! based on deployment profiles and system capabilities.

use bevy::log::{info, warn};

// Thread pool configuration constants
const THREAD_POOL_CPU_PERCENTAGE: f32 = 0.75; // Use 75% of available cores
const MIN_WORKER_THREADS: usize = 2; // Minimum threads for parallelism
const DEFAULT_WORKER_THREADS: usize = 4; // Fallback if detection fails
const MAX_WORKER_THREADS_DESKTOP: usize = 32; // Desktop/laptop cap
const MAX_WORKER_THREADS_SERVER: usize = 128; // Server deployment cap

/// Errors that can occur during infrastructure operations
#[derive(Debug, thiserror::Error)]
pub enum InfrastructureError {
    #[error("Failed to initialize thread pool: {0}")]
    ThreadPoolInit(String),

    #[error("Invalid thread count: {0}")]
    InvalidThreadCount(String),

    #[error("System resource detection failed: {0}")]
    ResourceDetection(String),
}

/// Thread pool profile for different deployment scenarios
#[derive(Debug, Clone, Copy)]
enum ThreadPoolProfile {
    Development, // Conservative for development (2-8 threads)
    Desktop,     // Standard desktop/laptop (2-32 threads)
    Server,      // High-performance server (4-128 threads)
}

impl ThreadPoolProfile {
    fn from_env() -> Self {
        match std::env::var("LIVING_WORLDS_PROFILE").as_deref() {
            Ok("development") | Ok("dev") => Self::Development,
            Ok("server") | Ok("production") => Self::Server,
            _ => Self::Desktop, // Default to desktop profile
        }
    }

    fn max_threads(&self) -> usize {
        match self {
            Self::Development => 8,
            Self::Desktop => MAX_WORKER_THREADS_DESKTOP,
            Self::Server => MAX_WORKER_THREADS_SERVER,
        }
    }

    fn cpu_percentage(&self) -> f32 {
        match self {
            Self::Development => 0.5, // 50% for development
            Self::Desktop => 0.75,    // 75% for desktop
            Self::Server => 0.9,      // 90% for servers (allow hyperthreading)
        }
    }
}

/// Thread pool manager for Living Worlds parallel processing
pub struct ThreadPoolManager;

impl ThreadPoolManager {
    /// Initialize the global rayon thread pool for parallel world generation
    ///
    /// # Arguments
    /// * `requested_threads` - Number of threads to use (0 for auto-detection)
    ///
    /// # Errors
    /// Returns `InfrastructureError` if thread pool initialization fails
    ///
    /// # Example
    /// ```rust
    /// use living_worlds::infrastructure::ThreadPoolManager;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Auto-detect optimal thread count
    /// ThreadPoolManager::initialize(0)?;
    ///
    /// // Use specific thread count
    /// ThreadPoolManager::initialize(8)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn initialize(requested_threads: usize) -> Result<(), InfrastructureError> {
        // Check environment variable first
        let env_threads = std::env::var("RAYON_NUM_THREADS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok());

        // Determine profile
        let profile = ThreadPoolProfile::from_env();

        let num_threads = if let Some(env_count) = env_threads {
            // Environment variable takes highest priority
            info!(
                "Using RAYON_NUM_THREADS environment variable: {}",
                env_count
            );
            env_count.clamp(MIN_WORKER_THREADS, profile.max_threads())
        } else if requested_threads > 0 {
            // Use user-specified thread count with profile limits
            requested_threads.clamp(MIN_WORKER_THREADS, profile.max_threads())
        } else {
            // Auto-detect optimal thread count based on profile
            Self::calculate_optimal_threads(profile)?
        };

        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .thread_name(|i| format!("world-gen-{i}"))
            .build_global()
            .map_err(|e| InfrastructureError::ThreadPoolInit(e.to_string()))?;

        let total_cores = std::thread::available_parallelism()
            .map(std::num::NonZero::get)
            .unwrap_or(DEFAULT_WORKER_THREADS);

        info!(
            "Thread pool initialized: {} worker threads ({}% of {} cores, {:?} profile)",
            num_threads,
            (num_threads * 100) / total_cores,
            total_cores,
            profile
        );

        Ok(())
    }

    /// Calculate optimal number of worker threads based on profile
    fn calculate_optimal_threads(profile: ThreadPoolProfile) -> Result<usize, InfrastructureError> {
        match std::thread::available_parallelism() {
            Ok(cores) => {
                let available = cores.get();
                // Use percentage of cores based on profile
                let optimal = (available as f32 * profile.cpu_percentage()) as usize;
                // Apply min/max bounds from profile
                Ok(optimal.clamp(MIN_WORKER_THREADS, profile.max_threads().min(available * 2)))
            }
            Err(e) => {
                warn!("Failed to detect CPU cores: {}. Using default.", e);
                // Return default on error rather than failing
                Ok(DEFAULT_WORKER_THREADS)
            }
        }
    }
}
