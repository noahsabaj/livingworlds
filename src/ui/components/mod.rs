//! UI Components Module - Gateway Architecture
//!
//! This module provides reusable UI components with consistent styling
//! that can be used throughout the game interface. Each component type
//! lives in its own submodule with its definition, builder, and logic.
//!
//! ## Architecture
//!
//! Following gateway architecture, this mod.rs is a thin orchestrator that:
//! - Re-exports component types and builders
//! - Provides a clean public API
//! - Never contains implementation logic
//!
//! ## Available Components
//!
//! - **Panel**: Container components with various styles
//! - **Label**: Text display components with semantic styles
//! - **Separator**: Visual dividers for UI layout
//! - **ProgressBar**: Progress indicators with animation
//!
//! ## Usage
//!
//! ```rust
//! use crate::ui::components::{PanelBuilder, LabelBuilder, ProgressBarBuilder};
//!
//! // All builders are available through this gateway
//! PanelBuilder::new(parent)
//!     .style(PanelStyle::Bordered)
//!     .build();
//! ```

// Submodules - all private, exposed through controlled re-exports
mod label;
mod panel;
mod progress_bar;
mod separator;
mod types;

// PUBLIC RE-EXPORTS - The Gateway's Public API

// Shared types used across components
pub use types::Orientation;

pub use panel::{PanelBuilder, PanelStyle};

// Label component and builder
pub use label::{LabelBuilder, LabelStyle};

// Separator component and builder
pub use separator::SeparatorBuilder;

// Progress bar component and builder
pub use progress_bar::{ProgressBar, ProgressBarBuilder, ProgressBarPlugin};
