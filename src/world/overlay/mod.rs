//! Overlay feature module gateway

// PRIVATE MODULES
mod cache;
mod rendering;
mod types;

// PUBLIC EXPORTS
pub use cache::CachedOverlayColors;
pub use rendering::{update_province_colors, OverlayPlugin};
pub use types::MapMode;
