//! Error types for Living Worlds
//!
//! Central error handling using thiserror for derive macros.

use thiserror::Error;

/// Main error type for Living Worlds
#[derive(Error, Debug)]
pub enum Error {
    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Invalid game state
    #[error("Invalid game state: {0}")]
    InvalidState(String),
    
    /// Resource not found
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    /// World generation error
    #[error("World generation failed: {0}")]
    WorldGeneration(String),
    
    /// Save/Load error
    #[error("Save/Load error: {0}")]
    SaveLoad(String),
    
    /// Rendering error
    #[error("Rendering error: {0}")]
    Render(String),
    
    /// Network error (for future multiplayer)
    #[error("Network error: {0}")]
    Network(String),
    
    /// Generic error with context
    #[error("{0}")]
    Other(String),
}

/// Result type alias using our Error
pub type Result<T> = std::result::Result<T, Error>;

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}