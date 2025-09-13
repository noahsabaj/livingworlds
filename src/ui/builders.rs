//! Centralized re-exports for all UI builders
//! 
//! This module provides convenient access to all UI builder patterns in one place.
//! Instead of importing from individual modules, you can use:
//! ```rust
//! use crate::ui::builders::*;
//! ```

// Button builders
pub use super::buttons::{
    ButtonBuilder,
    ButtonStyle,
    ButtonSize,
    // Preset functions
    presets as button_presets,
};

// Dialog builders
pub use super::dialogs::{
    DialogBuilder,
    DialogType,
    DialogButton,
    // Preset dialogs
    presets as dialog_presets,
};

// Slider builders
pub use super::sliders::{
    SliderBuilder,
    ValueFormat,
    slider,  // Convenience function
};

// Text input builders
pub use super::text_inputs::{
    TextInputBuilder,
    InputFilter,
    InputTransform,
    FocusGroupId,
    text_input,  // Convenience function
};

// Component builders (from components.rs)
pub use super::components::{
    PanelBuilder,
    PanelStyle,
    LabelBuilder,
    LabelStyle,
    SeparatorBuilder,
    SeparatorStyle,
    Orientation,
    ProgressBarBuilder,
    ProgressBarStyle,
};

// Form builder
pub use super::form::{
    FormBuilder,
    form,  // Convenience function
    presets as form_presets,
};

// Toolbar builder
pub use super::toolbar::{
    ToolbarBuilder,
    ToolbarOrientation,
    ToolbarStyle,
    toolbar,  // Convenience function
    presets as toolbar_presets,
};

// ============================================================================
// CONVENIENCE FUNCTIONS
// ============================================================================

/// Quick button creation
pub fn button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text)
}

/// Quick primary button
pub fn primary_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Primary)
}

/// Quick danger button
pub fn danger_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Danger)
}

/// Quick dialog creation
pub fn dialog(dialog_type: DialogType) -> DialogBuilder {
    DialogBuilder::new(dialog_type)
}

/// Quick progress bar creation
pub fn progress_bar(value: f32) -> ProgressBarBuilder {
    ProgressBarBuilder::new(value)
}

// Note: PanelBuilder, LabelBuilder, and SeparatorBuilder require parent parameter
// so they can't have simple convenience functions. Use them directly:
// PanelBuilder::new(parent)
// LabelBuilder::new(parent, "text")
// SeparatorBuilder::new(parent)

// ============================================================================
// BUILDER TRAITS (for future extension)
// ============================================================================

/// Common trait for all builders
pub trait UIBuilder {
    type Output;
    
    /// Build the UI element
    fn build(self, parent: &mut bevy::prelude::ChildSpawnerCommands) -> Self::Output;
}

// ============================================================================
// DOCUMENTATION
// ============================================================================

// # UI Builder Pattern Guide
// 
// All UI components in Living Worlds follow the builder pattern for consistent,
// ergonomic construction. This module re-exports all builders for convenience.
// 
// ## Basic Usage
// ```rust
// use crate::ui::builders::*;
// 
// // Create a button
// button("Click Me")
//     .style(ButtonStyle::Primary)
//     .size(ButtonSize::Large)
//     .build(parent);
// 
// // Create a slider
// slider(0.0, 100.0)
//     .with_label("Volume")
//     .with_value(50.0)
//     .percentage()
//     .build(parent);
// 
// // Create a text input
// text_input()
//     .with_placeholder("Enter name...")
//     .with_validation(InputFilter::Alphabetic)
//     .build(parent);
// ```
// 
// ## Available Builders
// 
// - **ButtonBuilder**: Styled buttons with hover effects
// - **DialogBuilder**: Modal dialogs with consistent styling
// - **SliderBuilder**: Draggable sliders with value display
// - **TextInputBuilder**: Text inputs with validation
// - **PanelBuilder**: Container panels with borders
// - **LabelBuilder**: Styled text labels
// - **SeparatorBuilder**: Visual separators/dividers
// 
// ## Design Philosophy
// 
// 1. **Consistency**: All builders follow similar patterns
// 2. **Ergonomics**: Fluent API with method chaining
// 3. **Type Safety**: Compile-time validation where possible
// 4. **Defaults**: Sensible defaults with override options
// 5. **Extensibility**: Easy to add new builders or options