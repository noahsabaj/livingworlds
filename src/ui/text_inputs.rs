//! Complete text input creation and management system
//! 
//! This module provides THE standard way to create text inputs in Living Worlds,
//! similar to how ButtonBuilder is the standard for buttons. It eliminates 90% of
//! boilerplate code and includes built-in focus management, consistent styling,
//! and extensibility for future features.

use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputSettings, TextInputValue,
    TextInputTextFont, TextInputTextColor, TextInputInactive
};
use super::styles::{colors, dimensions};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Defines how a text input participates in focus management
#[derive(Component, Clone, Debug)]
pub enum TextInputFocus {
    /// This input doesn't affect other inputs when focused
    Independent,
    /// This input is part of an exclusive group - only one in the group can be focused
    ExclusiveGroup(FocusGroupId),
}

/// Identifies groups of text inputs where only one can be focused at a time
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FocusGroupId {
    /// World configuration screen (Name and Seed inputs)
    WorldConfig,
    /// Save game dialog
    SaveDialog,
    /// Mod browser search
    ModBrowser,
    /// Custom group for extensions
    Custom(u32),
}

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for creating text inputs with managed focus
pub struct TextInputBuilder {
    value: String,
    placeholder: Option<String>,
    font_size: f32,
    width: Val,
    height: Val,
    padding: UiRect,
    focus_type: TextInputFocus,
    inactive: bool,
    retain_on_submit: bool,
}

/// Builder with a single marker component
pub struct TextInputBuilderWithMarker<M: Component> {
    builder: TextInputBuilder,
    marker: M,
}

impl<M: Component> TextInputBuilderWithMarker<M> {
    /// Add another marker component
    pub fn and_marker<N: Component>(self, marker2: N) -> TextInputBuilderWithTwoMarkers<M, N> {
        TextInputBuilderWithTwoMarkers {
            builder: self.builder,
            marker1: self.marker,
            marker2,
        }
    }
    
    /// Build and spawn the text input entity with the marker
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let mut entity_commands = parent.spawn((
            // Node components for layout
            Node {
                width: self.builder.width,
                height: self.builder.height,
                padding: self.builder.padding,
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
            BorderColor(colors::BORDER_DEFAULT),
            BorderRadius::all(Val::Px(5.0)),
            
            // Text input components
            TextInput,
            TextInputValue(if self.builder.value.is_empty() && self.builder.placeholder.is_some() {
                self.builder.placeholder.unwrap()
            } else {
                self.builder.value
            }),
            TextInputTextFont(TextFont {
                font_size: self.builder.font_size,
                ..default()
            }),
            TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
            TextInputSettings {
                retain_on_submit: self.builder.retain_on_submit,
                ..default()
            },
            
            // Focus management
            self.builder.focus_type.clone(),
            
            // Make it a butfn so it can be clicked
            Button,
            
            // The custom marker
            self.marker,
        ));

        // Add inactive state if requested
        if self.builder.inactive {
            entity_commands.insert(TextInputInactive(true));
        }

        entity_commands.id()
    }
}

/// Builder with two marker components
pub struct TextInputBuilderWithTwoMarkers<M: Component, N: Component> {
    builder: TextInputBuilder,
    marker1: M,
    marker2: N,
}

impl<M: Component, N: Component> TextInputBuilderWithTwoMarkers<M, N> {
    /// Build and spawn the text input entity with both markers
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let mut entity_commands = parent.spawn((
            // Node components for layout
            Node {
                width: self.builder.width,
                height: self.builder.height,
                padding: self.builder.padding,
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
            BorderColor(colors::BORDER_DEFAULT),
            BorderRadius::all(Val::Px(5.0)),
            
            // Text input components
            TextInput,
            TextInputValue(if self.builder.value.is_empty() && self.builder.placeholder.is_some() {
                self.builder.placeholder.unwrap()
            } else {
                self.builder.value
            }),
            TextInputTextFont(TextFont {
                font_size: self.builder.font_size,
                ..default()
            }),
            TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
            TextInputSettings {
                retain_on_submit: self.builder.retain_on_submit,
                ..default()
            },
            
            // Focus management
            self.builder.focus_type.clone(),
            
            // Make it a button so it can be clicked
            Button,
            
            // The custom markers
            self.marker1,
            self.marker2,
        ));

        // Add inactive state if requested
        if self.builder.inactive {
            entity_commands.insert(TextInputInactive(true));
        }

        entity_commands.id()
    }
}

impl TextInputBuilder {
    /// Create a new text input builder
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            font_size: 16.0,
            width: Val::Px(300.0),
            height: Val::Px(40.0),
            padding: UiRect::all(Val::Px(10.0)),
            focus_type: TextInputFocus::Independent,
            inactive: false,
            retain_on_submit: true,
        }
    }

    /// Set the initial value
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Set placeholder text (currently just sets initial value)
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set the font size
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the width
    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Set the height
    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    /// Set padding
    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    /// Make this input part of an exclusive focus group
    pub fn with_focus_group(mut self, group: FocusGroupId) -> Self {
        self.focus_type = TextInputFocus::ExclusiveGroup(group);
        self
    }

    /// Make this input independent (doesn't affect other inputs)
    pub fn independent(mut self) -> Self {
        self.focus_type = TextInputFocus::Independent;
        self
    }

    /// Start with the input inactive (not focused)
    pub fn inactive(mut self) -> Self {
        self.inactive = true;
        self
    }

    /// Set whether to retain text on submit
    pub fn retain_on_submit(mut self, retain: bool) -> Self {
        self.retain_on_submit = retain;
        self
    }

    /// Add a marker component to the input
    pub fn with_marker<M: Component>(self, marker: M) -> TextInputBuilderWithMarker<M> {
        TextInputBuilderWithMarker {
            builder: self,
            marker,
        }
    }

    /// Build and spawn the text input entity
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let mut entity_commands = parent.spawn((
            // Node components for layout
            Node {
                width: self.width,
                height: self.height,
                padding: self.padding,
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
            BorderColor(colors::BORDER_DEFAULT),
            BorderRadius::all(Val::Px(5.0)),
            
            // Text input components
            TextInput,
            TextInputValue(if self.value.is_empty() && self.placeholder.is_some() {
                self.placeholder.unwrap()
            } else {
                self.value
            }),
            TextInputTextFont(TextFont {
                font_size: self.font_size,
                ..default()
            }),
            TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
            TextInputSettings {
                retain_on_submit: self.retain_on_submit,
                ..default()
            },
            
            // Focus management
            self.focus_type.clone(),
            
            // Make it a button so it can be clicked
            Button,
        ));

        // Add inactive state if requested
        if self.inactive {
            entity_commands.insert(TextInputInactive(true));
        }

        entity_commands.id()
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin that provides the complete text input system for the application
pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_text_input_focus,
            handle_click_outside_unfocus,
        ));
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Handle clicking on text inputs to manage focus
fn handle_text_input_focus(
    mut commands: Commands,
    interactions: Query<
        (Entity, &Interaction, &TextInputFocus),
        (Changed<Interaction>, With<TextInput>)
    >,
    all_inputs: Query<(Entity, &TextInputFocus), With<TextInput>>,
) {
    for (clicked_entity, interaction, focus_type) in &interactions {
        if *interaction == Interaction::Pressed {
            match focus_type {
                TextInputFocus::Independent => {
                    // Just focus this one input
                    commands.entity(clicked_entity).insert(TextInputInactive(false));
                }
                TextInputFocus::ExclusiveGroup(group_id) => {
                    // Focus this input, unfocus others in the same group
                    for (entity, other_focus) in &all_inputs {
                        match other_focus {
                            TextInputFocus::ExclusiveGroup(other_group) 
                                if other_group == group_id => {
                                // Same group - manage focus
                                if entity == clicked_entity {
                                    commands.entity(entity).insert(TextInputInactive(false));
                                } else {
                                    commands.entity(entity).insert(TextInputInactive(true));
                                }
                            }
                            _ => {} // Different group or independent - ignore
                        }
                    }
                }
            }
        }
    }
}

/// Unfocus all text inputs when clicking outside any input
fn handle_click_outside_unfocus(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    interactions: Query<&Interaction, With<TextInput>>,
    all_inputs: Query<Entity, With<TextInput>>,
) {
    // Check if left mouse was just pressed
    if mouse_button.just_pressed(MouseButton::Left) {
        // Check if any text input is being interacted with
        let clicking_on_input = interactions.iter().any(|i| *i != Interaction::None);
        
        // If not clicking on any input, unfocus all
        if !clicking_on_input {
            for entity in &all_inputs {
                commands.entity(entity).insert(TextInputInactive(true));
            }
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convenience function to create a text input builder
pub fn text_input() -> TextInputBuilder {
    TextInputBuilder::new()
}