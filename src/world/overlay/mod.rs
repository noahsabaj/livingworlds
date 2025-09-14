//! Overlay feature module gateway

// PRIVATE MODULES
mod types;
mod rendering;
mod cache;

// PUBLIC EXPORTS
pub use types::{OverlayMode, ResourceOverlay};
pub use cache::CachedOverlayColors;
pub use rendering::{
    OverlayPlugin,
    update_province_colors,
};