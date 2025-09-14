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
mod types;
mod panel;
mod label;
mod separator;
mod progress_bar;

// PUBLIC RE-EXPORTS - The Gateway's Public API

// Shared types used across components
pub use types::{Orientation, ComponentMarker};

pub use panel::{
    Panel,
    PanelStyle,
    PanelBuilder,
};

// Label component and builder
pub use label::{
    Label,
    LabelStyle,
    LabelBuilder,
};

// Separator component and builder
pub use separator::{
    Separator,
    SeparatorStyle,
    SeparatorBuilder,
};

// Progress bar component and builder
pub use progress_bar::{
    ProgressBar,
    ProgressBarStyle,
    ProgressBarBuilder,
    ProgressBarFill,
    ProgressBarTrack,
    ProgressBarLabel,
};

