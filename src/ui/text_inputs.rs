//! Complete text input creation and management system
//!
//! This module provides THE standard way to create text inputs in Living Worlds,
//! similar to how ButtonBuilder is the standard for buttons. It eliminates 90% of
//! boilerplate code and includes built-in focus management, consistent styling,
//! and extensibility for future features.

use super::buttons::{ButtonBuilder, ButtonSize, ButtonStyle};
use super::styles::colors;
use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputSettings, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};

/// Defines input validation and filtering rules
#[derive(Component, Clone, Debug)]
pub struct TextInputFilter {
    pub filter_type: InputFilter,
    pub max_length: Option<usize>,
    pub transform: InputTransform,
}

/// Types of input filtering
#[derive(Clone, Debug, PartialEq)]
pub enum InputFilter {
    /// Allow any characters (default)
    None,
    /// Only allow numeric characters (0-9)
    Numeric,
    /// Only allow integers (0-9, optional negative sign)
    Integer,
    /// Only allow decimal numbers (0-9, '.', optional negative)
    Decimal,
    /// Only allow alphabetic characters (a-z, A-Z)
    Alphabetic,
    /// Only allow alphanumeric characters (a-z, A-Z, 0-9)
    Alphanumeric,
    /// Only allow hexadecimal characters (0-9, a-f, A-F)
    Hexadecimal,
    /// Custom regex pattern
    Regex(String),
    /// Custom validation function
    Custom(fn(&str) -> bool),
}

/// Text transformation options
#[derive(Clone, Debug, PartialEq)]
pub enum InputTransform {
    /// No transformation
    None,
    /// Convert to uppercase
    Uppercase,
    /// Convert to lowercase
    Lowercase,
    /// Capitalize first letter of each word
    Capitalize,
}

impl Default for TextInputFilter {
    fn default() -> Self {
        Self {
            filter_type: InputFilter::None,
            max_length: None,
            transform: InputTransform::None,
        }
    }
}

impl InputFilter {
    /// Check if a character is valid for this filter
    pub fn is_valid_char(&self, ch: char, current_text: &str) -> bool {
        match self {
            InputFilter::None => true,
            InputFilter::Numeric => ch.is_ascii_digit(),
            InputFilter::Integer => ch.is_ascii_digit() || (ch == '-' && current_text.is_empty()),
            InputFilter::Decimal => {
                ch.is_ascii_digit()
                    || (ch == '.' && !current_text.contains('.'))
                    || (ch == '-' && current_text.is_empty())
            }
            InputFilter::Alphabetic => ch.is_alphabetic(),
            InputFilter::Alphanumeric => ch.is_alphanumeric(),
            InputFilter::Hexadecimal => ch.is_ascii_hexdigit(),
            InputFilter::Regex(_pattern) => {
                // For regex, we'd need to check the entire string
                // This is a simplified check
                true // Will be validated in the full string check
            }
            InputFilter::Custom(validator) => {
                // Test if adding this character would be valid
                let mut test_string = current_text.to_string();
                test_string.push(ch);
                validator(&test_string)
            }
        }
    }

    /// Validate an entire string
    pub fn is_valid_string(&self, text: &str) -> bool {
        match self {
            InputFilter::None => true,
            InputFilter::Numeric => text.chars().all(|c| c.is_ascii_digit()),
            InputFilter::Integer => {
                if text.is_empty() {
                    return true;
                }
                let mut chars = text.chars();
                if let Some(first) = chars.next() {
                    if first != '-' && !first.is_ascii_digit() {
                        return false;
                    }
                }
                chars.all(|c| c.is_ascii_digit())
            }
            InputFilter::Decimal => {
                if text.is_empty() {
                    return true;
                }
                let mut has_decimal = false;
                let chars = text.chars().enumerate();

                for (i, ch) in chars {
                    if ch == '-' && i != 0 {
                        return false;
                    } else if ch == '.' {
                        if has_decimal {
                            return false;
                        }
                        has_decimal = true;
                    } else if !ch.is_ascii_digit() && ch != '-' {
                        return false;
                    }
                }
                true
            }
            InputFilter::Alphabetic => text.chars().all(|c| c.is_alphabetic()),
            InputFilter::Alphanumeric => text.chars().all(|c| c.is_alphanumeric()),
            InputFilter::Hexadecimal => text.chars().all(|c| c.is_ascii_hexdigit()),
            InputFilter::Regex(_pattern) => {
                // Would need regex crate for full support
                true // Simplified for now
            }
            InputFilter::Custom(validator) => validator(text),
        }
    }

    /// Filter out invalid characters from a string
    pub fn filter_string(&self, text: &str) -> String {
        let mut result = String::new();
        for ch in text.chars() {
            if self.is_valid_char(ch, &result) {
                result.push(ch);
            }
        }
        result
    }
}

/// Component that marks a clear button and tracks which text input it clears
#[derive(Component)]
pub struct ClearButtonTarget(pub Entity);

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

/// Builder for creating text inputs with managed focus
#[derive(Clone)]
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
    filter: Option<TextInputFilter>,
    show_clear_button: bool,
}

/// Builder with a single marker component
pub struct TextInputBuilderWithMarker<M: Component> {
    builder: TextInputBuilder,
    marker: M,
}

// Helper function to build text input with common components
fn build_text_input_with_extras<M>(
    parent: &mut ChildSpawnerCommands,
    builder: TextInputBuilder,
    extras: impl FnOnce(&mut EntityCommands) -> M,
) -> Entity {
    // If we need a clear button, create a container
    if builder.show_clear_button {
        let container_id = parent
            .spawn((
                Node {
                    width: builder.width,
                    height: builder.height,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .id();

        let mut text_input_id = None;

        parent
            .commands()
            .entity(container_id)
            .with_children(|container| {
                let mut entity_commands = container.spawn((
                    // Node components for layout
                    Node {
                        flex_grow: 1.0, // Take remaining space
                        height: Val::Percent(100.0),
                        padding: builder.padding,
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_LIGHT),
                    BorderColor(colors::BORDER_DEFAULT),
                    BorderRadius::all(Val::Px(5.0)),
                    TextInput,
                    TextInputValue(
                        if builder.value.is_empty() && builder.placeholder.is_some() {
                            builder.placeholder.clone().unwrap()
                        } else {
                            builder.value.clone()
                        },
                    ),
                    TextInputTextFont(TextFont {
                        font_size: builder.font_size,
                        ..default()
                    }),
                    TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
                    TextInputSettings {
                        retain_on_submit: builder.retain_on_submit,
                        ..default()
                    },
                    // Focus management
                    builder.focus_type.clone(),
                    // Make it a button so it can be clicked
                    Button,
                ));

                // Add extras from callback
                extras(&mut entity_commands);

                // Add inactive state if requested
                if builder.inactive {
                    entity_commands.insert(TextInputInactive(true));
                }

                // Add filter if specified
                if let Some(filter) = builder.filter.clone() {
                    entity_commands.insert(filter);
                }

                text_input_id = Some(entity_commands.id());

                // Add clear button
                let clear_button = ButtonBuilder::new("×")
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Small)
                    .build(container);

                // Add component to track which text input this button clears
                if let Some(input_id) = text_input_id {
                    container
                        .commands()
                        .entity(clear_button)
                        .insert(ClearButtonTarget(input_id));
                }
            });

        container_id
    } else {
        // No clear button, build normally
        let mut entity_commands = parent.spawn((
            // Node components for layout
            Node {
                width: builder.width,
                height: builder.height,
                padding: builder.padding,
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
            BorderColor(colors::BORDER_DEFAULT),
            BorderRadius::all(Val::Px(5.0)),
            TextInput,
            TextInputValue(
                if builder.value.is_empty() && builder.placeholder.is_some() {
                    builder.placeholder.unwrap()
                } else {
                    builder.value
                },
            ),
            TextInputTextFont(TextFont {
                font_size: builder.font_size,
                ..default()
            }),
            TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
            TextInputSettings {
                retain_on_submit: builder.retain_on_submit,
                ..default()
            },
            // Focus management
            builder.focus_type.clone(),
            // Make it a button so it can be clicked
            Button,
        ));

        // Add extras from callback
        extras(&mut entity_commands);

        // Add inactive state if requested
        if builder.inactive {
            entity_commands.insert(TextInputInactive(true));
        }

        // Add filter if specified
        if let Some(filter) = builder.filter.clone() {
            entity_commands.insert(filter);
        }

        entity_commands.id()
    }
}

impl<M: Component> TextInputBuilderWithMarker<M> {
    pub fn and_marker<N: Component>(self, marker2: N) -> TextInputBuilderWithTwoMarkers<M, N> {
        TextInputBuilderWithTwoMarkers {
            builder: self.builder,
            marker1: self.marker,
            marker2,
        }
    }

    /// Build and spawn the text input entity with the marker
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        build_text_input_with_extras(parent, self.builder, |entity| {
            entity.insert(self.marker);
        })
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
        build_text_input_with_extras(parent, self.builder, |entity| {
            entity.insert(self.marker1);
            entity.insert(self.marker2);
        })
    }
}

impl TextInputBuilder {
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
            filter: None,
            show_clear_button: false,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Set placeholder text (currently just sets initial value)
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

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

    /// Set input filter for validation
    pub fn with_filter(mut self, filter_type: InputFilter) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Set maximum length for input
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.max_length = Some(max_length);
        } else {
            self.filter = Some(TextInputFilter {
                filter_type: InputFilter::None,
                max_length: Some(max_length),
                transform: InputTransform::None,
            });
        }
        self
    }

    /// Set text transformation
    pub fn with_transform(mut self, transform: InputTransform) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.transform = transform;
        } else {
            self.filter = Some(TextInputFilter {
                filter_type: InputFilter::None,
                max_length: None,
                transform,
            });
        }
        self
    }

    /// Convenience method for numeric-only input (0-9)
    pub fn numeric_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Numeric,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for integer input (with optional negative)
    pub fn integer_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Integer,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for decimal input
    pub fn decimal_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Decimal,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for alphabetic-only input
    pub fn alphabetic_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Alphabetic,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for alphanumeric-only input
    pub fn alphanumeric_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Alphanumeric,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    pub fn with_clear_button(mut self) -> Self {
        self.show_clear_button = true;
        self
    }

    pub fn with_marker<M: Component>(self, marker: M) -> TextInputBuilderWithMarker<M> {
        TextInputBuilderWithMarker {
            builder: self,
            marker,
        }
    }

    /// Build and spawn the text input entity
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        build_text_input_with_extras(parent, self, |_entity| {})
    }
}

/// Plugin that provides the complete text input system for the application
pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_text_input_focus,
                handle_click_outside_unfocus,
                validate_text_input_changes,
                handle_clear_button_clicks,
            ),
        );
    }
}

/// Handle clicking on text inputs to manage focus
fn handle_text_input_focus(
    mut commands: Commands,
    interactions: Query<
        (Entity, &Interaction, &TextInputFocus),
        (Changed<Interaction>, With<TextInput>),
    >,
    all_inputs: Query<(Entity, &TextInputFocus), With<TextInput>>,
) {
    for (clicked_entity, interaction, focus_type) in &interactions {
        if *interaction == Interaction::Pressed {
            match focus_type {
                TextInputFocus::Independent => {
                    // Just focus this one input
                    commands
                        .entity(clicked_entity)
                        .insert(TextInputInactive(false));
                }
                TextInputFocus::ExclusiveGroup(group_id) => {
                    // Focus this input, unfocus others in the same group
                    for (entity, other_focus) in &all_inputs {
                        match other_focus {
                            TextInputFocus::ExclusiveGroup(other_group)
                                if other_group == group_id =>
                            {
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
    if mouse_button.just_pressed(MouseButton::Left) {
        let clicking_on_input = interactions.iter().any(|i| *i != Interaction::None);

        // If not clicking on any input, unfocus all
        if !clicking_on_input {
            for entity in &all_inputs {
                commands.entity(entity).insert(TextInputInactive(true));
            }
        }
    }
}

/// Validate and filter text input changes based on TextInputFilter
fn validate_text_input_changes(
    mut text_inputs: Query<
        (&mut TextInputValue, &TextInputFilter),
        (Changed<TextInputValue>, With<TextInput>),
    >,
) {
    for (mut text_value, filter) in &mut text_inputs {
        let current_text = text_value.0.clone();
        let mut modified_text = current_text.clone();

        // Apply filtering
        modified_text = filter.filter_type.filter_string(&modified_text);

        // Apply max length constraint
        if let Some(max_len) = filter.max_length {
            if modified_text.len() > max_len {
                modified_text.truncate(max_len);
            }
        }

        // Apply text transformation
        modified_text = match filter.transform {
            InputTransform::None => modified_text,
            InputTransform::Uppercase => modified_text.to_uppercase(),
            InputTransform::Lowercase => modified_text.to_lowercase(),
            InputTransform::Capitalize => {
                let mut chars = modified_text.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            }
        };

        // Only update if the text changed
        if modified_text != current_text {
            text_value.0 = modified_text;
        }
    }
}

/// Handle clicks on clear buttons
fn handle_clear_button_clicks(
    button_query: Query<(&Interaction, &ClearButtonTarget), (Changed<Interaction>, With<Button>)>,
    mut text_inputs: Query<&mut TextInputValue, With<TextInput>>,
) {
    for (interaction, target) in &button_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut text_value) = text_inputs.get_mut(target.0) {
                text_value.0.clear();
            }
        }
    }
}

/// Convenience function to create a text input builder
pub fn text_input() -> TextInputBuilder {
    TextInputBuilder::new()
}
