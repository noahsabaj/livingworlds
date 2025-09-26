//! Law browser UI gateway
//!
//! Provides a comprehensive interface for exploring all available laws
//! in the game, organized by category with detailed information.

// Private modules - gateway architecture
mod browser;
mod categories;
mod details;
mod search;
mod types;

// Re-export public components
pub use browser::{spawn_law_browser, LawBrowserPlugin};
pub use types::{LawBrowserState, SelectedLawCategory, SelectedLawId};