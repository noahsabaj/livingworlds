//! Slider builder system for Living Worlds
//! 
//! Provides THE standard way to create sliders with consistent styling,
//! automatic value tracking, and built-in interaction handling.

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use super::styles::{colors, dimensions};
use super::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Main slider component with configuration and state
#[derive(Component, Clone, Debug)]
pub struct Slider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    /// Entity ID of the associated value text display (if any)
    pub value_text_entity: Option<Entity>,
}

impl Slider {
    /// Create a new slider with the given range
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self {
            value: value.clamp(min, max),
            min,
            max,
            step: None,
            value_text_entity: None,
        }
    }
    
    /// Get normalized value (0.0 to 1.0)
    pub fn normalized(&self) -> f32 {
        if self.max == self.min {
            return 0.0;
        }
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }
    
    /// Set value from normalized (0.0 to 1.0)
    pub fn set_normalized(&mut self, normalized: f32) {
        let normalized = normalized.clamp(0.0, 1.0);
        self.value = self.min + (self.max - self.min) * normalized;
        
        // Apply step if configured
        if let Some(step) = self.step {
            let steps = ((self.value - self.min) / step).round();
            self.value = self.min + steps * step;
        }
        
        self.value = self.value.clamp(self.min, self.max);
    }
}

/// Marker for the draggable handle
#[derive(Component)]
pub struct SliderHandle;

/// Component for increment/decrement buttons
#[derive(Component)]
pub struct SliderButtonAction {
    pub slider_entity: Entity,
    pub delta: f32,
}

/// Component for the slider track (clickable area)
#[derive(Component)]
pub struct SliderTrack;

/// Component for the filled portion of the slider
#[derive(Component)]
pub struct SliderFill;

/// Component for the value text display
#[derive(Component)]
pub struct SliderValueText;

/// Component for the label text
#[derive(Component)]
pub struct SliderLabel;

/// Configuration for how the value is displayed
#[derive(Clone, Debug)]
pub enum ValueFormat {
    /// Display as integer
    Integer,
    /// Display with fixed decimal places
    Decimal(usize),
    /// Display as percentage
    Percentage,
    /// Custom formatter function
    Custom(fn(f32) -> String),
}

impl ValueFormat {
    /// Format a value according to this format
    pub fn format(&self, value: f32) -> String {
        match self {
            ValueFormat::Integer => format!("{}", value as i32),
            ValueFormat::Decimal(places) => format!("{:.precision$}", value, precision = places),
            ValueFormat::Percentage => format!("{}%", (value * 100.0) as i32),
            ValueFormat::Custom(formatter) => formatter(value),
        }
    }
}

/// Configuration for slider appearance
#[derive(Component, Clone, Debug)]
pub struct SliderConfig {
    pub show_value: bool,
    pub value_format: ValueFormat,
    pub track_height: f32,
    pub handle_size: f32,
    pub track_color: Color,
    pub fill_color: Color,
    pub handle_color: Color,
}

impl Default for SliderConfig {
    fn default() -> Self {
        Self {
            show_value: true,
            value_format: ValueFormat::Decimal(1),
            track_height: 6.0,
            handle_size: 16.0,
            track_color: Color::srgb(0.1, 0.1, 0.12),
            fill_color: colors::PRIMARY.with_alpha(0.3),
            handle_color: colors::PRIMARY,
        }
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for creating sliders
pub struct SliderBuilder {
    label: Option<String>,
    value: f32,
    min: f32,
    max: f32,
    step: Option<f32>,
    width: Val,
    show_value: bool,
    value_format: ValueFormat,
    track_height: f32,
    handle_size: f32,
    track_color: Color,
    fill_color: Color,
    handle_color: Color,
    margin: UiRect,
    show_buttons: bool,
    button_step: Option<f32>,
}

/// Builder with a marker component
pub struct SliderBuilderWithMarker<M: Component> {
    builder: SliderBuilder,
    marker: M,
}

impl<M: Component> SliderBuilderWithMarker<M> {
    /// Add a value text marker
    pub fn with_value_marker<V: Component>(self, value_marker: V) -> SliderBuilderWithMarkers<M, V> {
        SliderBuilderWithMarkers {
            builder: self.builder,
            slider_marker: self.marker,
            value_marker,
        }
    }
    
    /// Build the slider
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        build_slider_internal(parent, self.builder, Some(self.marker), None::<NoMarker>)
    }
}

/// Builder with two marker components
pub struct SliderBuilderWithMarkers<M: Component, V: Component> {
    builder: SliderBuilder,
    slider_marker: M,
    value_marker: V,
}

impl<M: Component, V: Component> SliderBuilderWithMarkers<M, V> {
    /// Build the slider with both markers
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        build_slider_internal(parent, self.builder, Some(self.slider_marker), Some(self.value_marker))
    }
}

impl SliderBuilder {
    /// Create a new slider builder
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            label: None,
            value: min,
            min,
            max,
            step: None,
            width: Val::Px(200.0),
            show_value: true,
            value_format: ValueFormat::Decimal(1),
            track_height: 6.0,
            handle_size: 16.0,
            track_color: Color::srgb(0.1, 0.1, 0.12),
            fill_color: colors::PRIMARY.with_alpha(0.3),
            handle_color: colors::PRIMARY,
            margin: UiRect::bottom(Val::Px(10.0)),
            show_buttons: false,
            button_step: None,
        }
    }
    
    /// Set the label text
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    
    /// Set the initial value
    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }
    
    /// Set the step size for discrete values
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }
    
    /// Set the slider width
    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    
    /// Set whether to show the value text
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }
    
    /// Set the value format
    pub fn with_format(mut self, format: ValueFormat) -> Self {
        self.value_format = format;
        self
    }
    
    /// Set as integer display
    pub fn integer(mut self) -> Self {
        self.value_format = ValueFormat::Integer;
        self
    }
    
    /// Set as percentage display
    pub fn percentage(mut self) -> Self {
        self.value_format = ValueFormat::Percentage;
        self
    }
    
    /// Set decimal places
    pub fn decimal_places(mut self, places: usize) -> Self {
        self.value_format = ValueFormat::Decimal(places);
        self
    }
    
    /// Set track height
    pub fn track_height(mut self, height: f32) -> Self {
        self.track_height = height;
        self
    }
    
    /// Set handle size
    pub fn handle_size(mut self, size: f32) -> Self {
        self.handle_size = size;
        self
    }
    
    /// Set colors
    pub fn with_colors(mut self, track: Color, fill: Color, handle: Color) -> Self {
        self.track_color = track;
        self.fill_color = fill;
        self.handle_color = handle;
        self
    }
    
    /// Set margin
    pub fn with_margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }
    
    /// Add increment/decrement buttons
    pub fn with_buttons(mut self) -> Self {
        self.show_buttons = true;
        if self.button_step.is_none() {
            self.button_step = self.step.or(Some((self.max - self.min) / 100.0));
        }
        self
    }
    
    /// Set the step size for button increments
    pub fn button_step(mut self, step: f32) -> Self {
        self.button_step = Some(step);
        self
    }
    
    /// Add a marker component to the slider
    pub fn with_marker<M: Component>(self, marker: M) -> SliderBuilderWithMarker<M> {
        SliderBuilderWithMarker {
            builder: self,
            marker,
        }
    }
    
    /// Build the slider
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        build_slider_internal_no_markers(parent, self)
    }
}

// Internal function to build the slider without markers
fn build_slider_internal_no_markers(
    parent: &mut ChildSpawnerCommands,
    builder: SliderBuilder,
) -> Entity {
    build_slider_internal(parent, builder, None::<NoMarker>, None::<NoMarker>)
}

// Dummy component for when no marker is needed
#[derive(Component)]
struct NoMarker;

// Internal function to build the slider
fn build_slider_internal<M: Component, V: Component>(
    parent: &mut ChildSpawnerCommands,
    builder: SliderBuilder,
    slider_marker: Option<M>,
    value_marker: Option<V>,
) -> Entity {
    let container_id = parent.spawn((
        Node {
            width: builder.width,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            margin: builder.margin,
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).id();
    
    // Track the value text entity ID to associate with the slider
    let mut value_text_id = None;
    
    parent.commands().entity(container_id).with_children(|container| {
        // Label and value row
        if builder.label.is_some() || builder.show_value {
            container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|row| {
                // Label
                if let Some(label) = builder.label {
                    row.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_SECONDARY),
                        SliderLabel,
                    ));
                }
                
                // Value text
                if builder.show_value {
                    let mut value_entity = row.spawn((
                        Text::new(builder.value_format.format(builder.value)),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                        SliderValueText,
                    ));
                    
                    value_text_id = Some(value_entity.id());
                    
                    if let Some(marker) = value_marker {
                        value_entity.insert(marker);
                    }
                }
            });
        }
        
        // Slider track
        let mut slider_entity = container.spawn((
            Button,  // For interaction
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(builder.track_height + builder.handle_size),
                padding: UiRect::vertical(Val::Px(builder.handle_size / 2.0 - builder.track_height / 2.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..default()
            },
            BackgroundColor(Color::NONE),
            Interaction::default(),
            RelativeCursorPosition::default(),
            SliderTrack,
        ));
        
        let track_entity = slider_entity.id();
        
        // Add the Slider component and optional marker
        let mut slider = Slider::new(builder.min, builder.max, builder.value);
        slider.step = builder.step;
        slider.value_text_entity = value_text_id;  // Associate the value text with this slider
        
        slider_entity.insert(slider.clone());
        slider_entity.insert(SliderConfig {
            show_value: builder.show_value,
            value_format: builder.value_format.clone(),
            track_height: builder.track_height,
            handle_size: builder.handle_size,
            track_color: builder.track_color,
            fill_color: builder.fill_color,
            handle_color: builder.handle_color,
        });
        
        if let Some(marker) = slider_marker {
            slider_entity.insert(marker);
        }
        
        // Build track visuals
        slider_entity.with_children(|track| {
            // Track background
            track.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(builder.track_height),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(builder.track_color),
                BorderRadius::all(Val::Px(builder.track_height / 2.0)),
            ));
            
            // Filled portion
            let fill_width = slider.normalized() * 100.0;
            track.spawn((
                Node {
                    width: Val::Percent(fill_width),
                    height: Val::Px(builder.track_height),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(builder.fill_color),
                BorderRadius::all(Val::Px(builder.track_height / 2.0)),
                SliderFill,
            ));
            
            // Handle
            let handle_offset = slider.normalized() * 100.0;
            track.spawn((
                Node {
                    width: Val::Px(builder.handle_size),
                    height: Val::Px(builder.handle_size),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(handle_offset),
                    top: Val::Px((builder.handle_size - builder.track_height) / 2.0 - builder.handle_size / 2.0 + builder.track_height / 2.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(builder.handle_color),
                BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                BorderRadius::all(Val::Px(builder.handle_size / 2.0)),
                SliderHandle,
            ));
        });
        
        // Add increment/decrement buttons if requested
        if builder.show_buttons {
            container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|button_row| {
                // Decrement button
                let dec_button = ButtonBuilder::new("-")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(button_row);
                    
                // Store the track entity on the button so we know which slider it controls
                button_row.commands().entity(dec_button).insert(SliderButtonAction {
                    slider_entity: track_entity,
                    delta: -builder.button_step.unwrap_or(builder.step.unwrap_or((builder.max - builder.min) / 100.0)),
                });
                
                // Increment button
                let inc_button = ButtonBuilder::new("+")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Small)
                    .build(button_row);
                    
                button_row.commands().entity(inc_button).insert(SliderButtonAction {
                    slider_entity: track_entity,
                    delta: builder.button_step.unwrap_or(builder.step.unwrap_or((builder.max - builder.min) / 100.0)),
                });
            });
        }
    });
    
    container_id
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin that provides the slider system
pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_slider_interaction,
            update_slider_visuals,
            handle_slider_button_clicks,
        ));
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Track which slider entity is currently being dragged
#[derive(Resource, Default)]
struct DraggedSlider {
    entity: Option<Entity>,
}

/// Handle slider dragging interaction
fn handle_slider_interaction(
    mut sliders: Query<
        (Entity, &Interaction, &mut Slider, &Node, &RelativeCursorPosition, &Children),
        With<SliderTrack>
    >,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut dragged_slider: Local<Option<Entity>>,
) {
    // If mouse was released, stop dragging
    if !mouse_button.pressed(MouseButton::Left) {
        *dragged_slider = None;
    }
    
    for (entity, interaction, mut slider, node, cursor_pos, children) in &mut sliders {
        // Start dragging on press - only if no other slider is being dragged
        if *interaction == Interaction::Pressed && mouse_button.pressed(MouseButton::Left) && dragged_slider.is_none() {
            *dragged_slider = Some(entity);
        }
        
        // Update value only for the slider being dragged
        if *dragged_slider == Some(entity) {
            if let Some(cursor_pos) = cursor_pos.normalized {
                let normalized_x = cursor_pos.x.clamp(0.0, 1.0);
                let old_value = slider.value;
                slider.set_normalized(normalized_x);
                
                // Track if value changed for event emission
                if old_value != slider.value {
                    // You could emit an event here if needed
                }
            }
        }
    }
}

/// Update slider visuals when value changes
fn update_slider_visuals(
    sliders: Query<(&Slider, &SliderConfig, &Children), Changed<Slider>>,
    mut fills: Query<&mut Node, (With<SliderFill>, Without<SliderHandle>)>,
    mut handles: Query<&mut Node, (With<SliderHandle>, Without<SliderFill>)>,
    mut value_texts: Query<&mut Text>,
) {
    for (slider, config, children) in &sliders {
        // Update fill and handle through children
        for child in children.iter() {
            // Update fill width
            if let Ok(mut fill_node) = fills.get_mut(child) {
                fill_node.width = Val::Percent(slider.normalized() * 100.0);
            }
            
            // Update handle position - keep handle within track bounds
            if let Ok(mut handle_node) = handles.get_mut(child) {
                // At 100%, position handle at the right edge but still within track
                // Account for handle width to keep it fully within track
                let handle_offset = slider.normalized() * 100.0;
                // Cap at 95% to ensure handle remains grabbable at max value
                handle_node.left = Val::Percent(handle_offset.min(95.0));
            }
        }
        
        // Update ONLY the value text associated with this specific slider
        if let Some(value_text_entity) = slider.value_text_entity {
            if let Ok(mut text) = value_texts.get_mut(value_text_entity) {
                **text = config.value_format.format(slider.value);
            }
        }
    }
}

/// Handle clicks on slider increment/decrement buttons
fn handle_slider_button_clicks(
    button_query: Query<(&Interaction, &SliderButtonAction), (Changed<Interaction>, With<Button>)>,
    mut slider_query: Query<&mut Slider>,
) {
    for (interaction, action) in &button_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut slider) = slider_query.get_mut(action.slider_entity) {
                let new_value = (slider.value + action.delta).clamp(slider.min, slider.max);
                
                // Apply step if configured
                if let Some(step) = slider.step {
                    let steps = ((new_value - slider.min) / step).round();
                    slider.value = slider.min + steps * step;
                } else {
                    slider.value = new_value;
                }
            }
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convenience function to create a slider builder
pub fn slider(min: f32, max: f32) -> SliderBuilder {
    SliderBuilder::new(min, max)
}

/// Static helper to spawn a slider without using the builder pattern
/// This avoids lifetime issues in preset functions and other contexts
pub fn spawn_slider(
    parent: &mut ChildSpawnerCommands,
    min: f32,
    max: f32,
    value: f32,
) -> Entity {
    spawn_slider_full(
        parent,
        min,
        max,
        value,
        None,  // No label
        false, // No buttons
        None::<Slider>, // No custom marker
    )
}

/// Static helper to spawn a slider with label
pub fn spawn_slider_with_label(
    parent: &mut ChildSpawnerCommands,
    min: f32,
    max: f32,
    value: f32,
    label: impl Into<String>,
) -> Entity {
    spawn_slider_full(
        parent,
        min,
        max,
        value,
        Some(label.into()),
        false, // No buttons
        None::<Slider>, // No custom marker
    )
}

/// Static helper to spawn a slider with all options
pub fn spawn_slider_full<M: Component>(
    parent: &mut ChildSpawnerCommands,
    min: f32,
    max: f32,
    value: f32,
    label: Option<String>,
    show_buttons: bool,
    marker: Option<M>,
) -> Entity {
    // We need to track the slider entity across closures
    let mut actual_slider_entity = Entity::PLACEHOLDER;
    
    // Container for the whole slider
    let container_entity = parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|container| {
        // Add label if provided
        if let Some(label_text) = label {
            container.spawn((
                Text::new(label_text),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));
        }
        
        // Row for slider and value display
        container.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
        )).with_children(|row| {
            // The slider track and handle
            let mut slider_commands = row.spawn((
                Node {
                    flex_grow: 1.0,
                    height: Val::Px(30.0),
                    padding: UiRect::horizontal(Val::Px(5.0)),
                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                Slider::new(min, max, value),
                Interaction::default(),
            ));
            
            // Add custom marker if provided
            if let Some(m) = marker {
                slider_commands.insert(m);
            }
            
            // Spawn slider handle as child before getting ID
            slider_commands.with_children(|track| {
                let handle_position = ((value - min) / (max - min)).clamp(0.0, 1.0);
                track.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(handle_position * 100.0),
                        top: Val::Percent(50.0),
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect {
                            left: Val::Px(-10.0),
                            top: Val::Px(-10.0),
                            ..default()
                        },
                        ..default()
                    },
                    BackgroundColor(colors::PRIMARY),
                    SliderHandle,
                ));
            });
            
            let slider_entity = slider_commands.id();
            actual_slider_entity = slider_entity; // Store for button usage
            
            // Value display
            let value_text_entity = row.spawn((
                Text::new(format!("{:.1}", value)),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    min_width: Val::Px(50.0),
                    ..default()
                },
            )).id();
            
            // Update the slider with the value text entity
            row.commands().entity(slider_entity).insert(Slider {
                min,
                max,
                value,
                value_text_entity: Some(value_text_entity),
                step: None,
            });
        });
        
        // Add buttons if requested
        if show_buttons {
            use super::buttons::{spawn_button, ButtonStyle, ButtonSize};
            
            container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|button_row| {
                // Decrement button
                let dec_entity = spawn_button(button_row, "-", ButtonStyle::Secondary, ButtonSize::Small);
                button_row.commands().entity(dec_entity).insert(SliderButtonAction {
                    slider_entity: actual_slider_entity,
                    delta: -(max - min) * 0.1,
                });
                
                // Increment button
                let inc_entity = spawn_button(button_row, "+", ButtonStyle::Secondary, ButtonSize::Small);
                button_row.commands().entity(inc_entity).insert(SliderButtonAction {
                    slider_entity: actual_slider_entity,
                    delta: (max - min) * 0.1,
                });
            });
        }
    }).id();
    
    container_entity
}