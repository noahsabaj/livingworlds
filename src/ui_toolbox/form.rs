//! Form builder system for creating complex forms with consistent layout
//! 
//! FormBuilder provides a high-level API for creating forms that automatically
//! handles layout, spacing, and composition of other ui_toolbox builders.

use bevy::prelude::*;
use super::styles::{colors, dimensions};
use super::components::{PanelBuilder, PanelStyle};
use super::text_inputs::FocusGroupId;
use super::sliders::{Slider, SliderButtonAction};

// ============================================================================
// FORM BUILDER
// ============================================================================

/// High-level builder for creating forms with automatic layout management
pub struct FormBuilder<'a> {
    parent: &'a mut ChildSpawnerCommands<'a>,
    title: Option<String>,
    width: Val,
    padding: UiRect,
    row_gap: Val,
    sections: Vec<FormSection>,
}

/// Represents a section within a form
struct FormSection {
    title: Option<String>,
    fields: Vec<FormField>,
}

/// Represents a field within a form
enum FormField {
    TextInput {
        label: String,
        placeholder: String,
        marker: Option<Box<dyn std::any::Any + Send + Sync>>,
    },
    Slider {
        label: String,
        min: f32,
        max: f32,
        value: f32,
        show_buttons: bool,
    },
    Custom {
        builder_fn: Box<dyn FnOnce(&mut ChildSpawnerCommands)>,
    },
}

impl<'a> FormBuilder<'a> {
    /// Create a new form builder
    pub fn new(parent: &'a mut ChildSpawnerCommands<'a>) -> Self {
        Self {
            parent,
            title: None,
            width: Val::Px(400.0),
            padding: UiRect::all(Val::Px(dimensions::PADDING_LARGE)),
            row_gap: Val::Px(dimensions::MARGIN_MEDIUM),
            sections: Vec::new(),
        }
    }
    
    /// Set the form title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    /// Set the form width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    
    /// Set padding
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }
    
    /// Set row gap
    pub fn row_gap(mut self, gap: Val) -> Self {
        self.row_gap = gap;
        self
    }
    
    /// Add a new section to the form
    pub fn section(mut self, title: impl Into<String>) -> Self {
        self.sections.push(FormSection {
            title: Some(title.into()),
            fields: Vec::new(),
        });
        self
    }
    
    /// Add a text input field to the current section
    pub fn text_field(mut self, label: impl Into<String>, placeholder: impl Into<String>) -> Self {
        if self.sections.is_empty() {
            self.sections.push(FormSection {
                title: None,
                fields: Vec::new(),
            });
        }
        
        if let Some(section) = self.sections.last_mut() {
            section.fields.push(FormField::TextInput {
                label: label.into(),
                placeholder: placeholder.into(),
                marker: None,
            });
        }
        
        self
    }
    
    /// Add a slider field to the current section
    pub fn slider_field(mut self, label: impl Into<String>, min: f32, max: f32, value: f32) -> Self {
        if self.sections.is_empty() {
            self.sections.push(FormSection {
                title: None,
                fields: Vec::new(),
            });
        }
        
        if let Some(section) = self.sections.last_mut() {
            section.fields.push(FormField::Slider {
                label: label.into(),
                min,
                max,
                value,
                show_buttons: false,
            });
        }
        
        self
    }
    
    /// Add a slider with buttons to the current section
    pub fn slider_with_buttons(mut self, label: impl Into<String>, min: f32, max: f32, value: f32) -> Self {
        if self.sections.is_empty() {
            self.sections.push(FormSection {
                title: None,
                fields: Vec::new(),
            });
        }
        
        if let Some(section) = self.sections.last_mut() {
            section.fields.push(FormField::Slider {
                label: label.into(),
                min,
                max,
                value,
                show_buttons: true,
            });
        }
        
        self
    }
    
    /// Add a custom field using a closure
    pub fn custom_field<F>(mut self, builder_fn: F) -> Self 
    where
        F: FnOnce(&mut ChildSpawnerCommands) + 'static,
    {
        if self.sections.is_empty() {
            self.sections.push(FormSection {
                title: None,
                fields: Vec::new(),
            });
        }
        
        if let Some(section) = self.sections.last_mut() {
            section.fields.push(FormField::Custom {
                builder_fn: Box::new(builder_fn),
            });
        }
        
        self
    }
    
    /// Build the form
    pub fn build(self) -> Entity {
        // Use PanelBuilder for the container
        let mut panel_builder = PanelBuilder::new(self.parent)
            .style(PanelStyle::Default)
            .width(self.width)
            .padding(self.padding);
            
        if let Some(title) = self.title {
            panel_builder = panel_builder.with_title(title);
        }
        
        panel_builder.build_with_children(|form_container| {
            // Create a focus group for all text inputs in this form
            let focus_group = FocusGroupId::Custom(rand::random());
            
            // Build each section
            for (section_idx, section) in self.sections.into_iter().enumerate() {
                // Add section separator if not first section
                if section_idx > 0 {
                    form_container.spawn((
                        Node {
                            height: Val::Px(1.0),
                            width: Val::Percent(100.0),
                            margin: UiRect::vertical(Val::Px(dimensions::MARGIN_MEDIUM)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                    ));
                }
                
                // Add section title
                if let Some(title) = section.title {
                    form_container.spawn((
                        Text::new(title),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_LARGE,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                        Node {
                            margin: UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)),
                            ..default()
                        },
                    ));
                }
                
                // Build each field in the section
                for field in section.fields {
                    // Field container
                    form_container.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(5.0),
                            margin: UiRect::bottom(self.row_gap),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    )).with_children(|field_container| {
                        match field {
                            FormField::TextInput { label, placeholder, .. } => {
                                // Label
                                field_container.spawn((
                                    Text::new(label),
                                    TextFont {
                                        font_size: dimensions::FONT_SIZE_NORMAL,
                                        ..default()
                                    },
                                    TextColor(colors::TEXT_PRIMARY),
                                    Node {
                                        margin: UiRect::bottom(Val::Px(2.0)),
                                        ..default()
                                    },
                                ));
                                    
                                // Text input - spawn directly with components
                                field_container.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(40.0),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                                        ..default()
                                    },
                                    BackgroundColor(colors::BACKGROUND_LIGHT),
                                    BorderColor(colors::BORDER_DEFAULT),
                                    bevy_simple_text_input::TextInput,
                                    bevy_simple_text_input::TextInputValue(placeholder),
                                    bevy_simple_text_input::TextInputTextFont(TextFont {
                                        font_size: dimensions::FONT_SIZE_NORMAL,
                                        ..default()
                                    }),
                                    bevy_simple_text_input::TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
                                ));
                            },
                            
                            FormField::Slider { label, min, max, value, show_buttons } => {
                                // Create slider container
                                field_container.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(5.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::NONE),
                                )).with_children(|slider_container| {
                                    // Label
                                    slider_container.spawn((
                                        Text::new(label),
                                        TextFont {
                                            font_size: dimensions::FONT_SIZE_NORMAL,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT_PRIMARY),
                                    ));
                                    
                                    // Slider track and handle
                                    let slider_entity = slider_container.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(30.0),
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(colors::BACKGROUND_LIGHT),
                                        Slider::new(min, max, value),
                                        Interaction::default(),
                                    )).id();
                                    
                                    // Add buttons if requested
                                    if show_buttons {
                                        slider_container.spawn((
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
                                            button_row.spawn((
                                                Button,
                                                Node {
                                                    width: Val::Px(30.0),
                                                    height: Val::Px(30.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                                                    ..default()
                                                },
                                                BackgroundColor(colors::SECONDARY),
                                                BorderColor(colors::BORDER_DEFAULT),
                                                SliderButtonAction {
                                                    slider_entity,
                                                    delta: -(max - min) * 0.1,
                                                },
                                            )).with_children(|btn| {
                                                btn.spawn((
                                                    Text::new("-"),
                                                    TextFont {
                                                        font_size: dimensions::FONT_SIZE_MEDIUM,
                                                        ..default()
                                                    },
                                                    TextColor(colors::TEXT_PRIMARY),
                                                ));
                                            });
                                            
                                            // Increment button
                                            button_row.spawn((
                                                Button,
                                                Node {
                                                    width: Val::Px(30.0),
                                                    height: Val::Px(30.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                                                    ..default()
                                                },
                                                BackgroundColor(colors::SECONDARY),
                                                BorderColor(colors::BORDER_DEFAULT),
                                                SliderButtonAction {
                                                    slider_entity,
                                                    delta: (max - min) * 0.1,
                                                },
                                            )).with_children(|btn| {
                                                btn.spawn((
                                                    Text::new("+"),
                                                    TextFont {
                                                        font_size: dimensions::FONT_SIZE_MEDIUM,
                                                        ..default()
                                                    },
                                                    TextColor(colors::TEXT_PRIMARY),
                                                ));
                                            });
                                        });
                                    }
                                });
                            },
                            
                            FormField::Custom { builder_fn } => {
                                builder_fn(field_container);
                            },
                        }
                    });
                }
            }
            
            // Add submit/cancel buttons at the bottom
            form_container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    column_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(dimensions::MARGIN_LARGE)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|button_row| {
                // Cancel button
                button_row.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                        ..default()
                    },
                    BackgroundColor(colors::SECONDARY),
                    BorderColor(colors::BORDER_DEFAULT),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Cancel"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
                    
                // Submit button
                button_row.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                        ..default()
                    },
                    BackgroundColor(colors::PRIMARY),
                    BorderColor(colors::BORDER_DEFAULT),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Submit"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
            });
        })
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convenience function to create a form builder
pub fn form<'a>(parent: &'a mut ChildSpawnerCommands<'a>) -> FormBuilder<'a> {
    FormBuilder::new(parent)
}

// ============================================================================
// PRESETS
// ============================================================================

pub mod presets {
    use super::*;
    
    /// Create a basic login form
    pub fn login_form(parent: &mut ChildSpawnerCommands) -> Entity {
        // We can't use the builder pattern across lifetime boundaries in presets
        // So we create the form manually with proper structure
        parent.spawn((
            Node {
                width: Val::Px(350.0),
                padding: UiRect::all(Val::Px(dimensions::PADDING_LARGE)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::MARGIN_MEDIUM),
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_MEDIUM),
            BorderColor(colors::BORDER_DEFAULT),
        )).with_children(|form| {
            // Title
            form.spawn((
                Text::new("Login"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_XLARGE,
                    ..default()
                },
                TextColor(colors::TEXT_TITLE),
                Node {
                    margin: UiRect::bottom(Val::Px(dimensions::MARGIN_LARGE)),
                    ..default()
                },
            ));
            
            // Username field
            form.spawn((
                Text::new("Username"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));
            
            form.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                    margin: UiRect::bottom(Val::Px(dimensions::MARGIN_MEDIUM)),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                bevy_simple_text_input::TextInput,
                bevy_simple_text_input::TextInputValue("Enter your username".to_string()),
                bevy_simple_text_input::TextInputTextFont(TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                }),
                bevy_simple_text_input::TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
            ));
            
            // Password field
            form.spawn((
                Text::new("Password"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));
            
            form.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                    margin: UiRect::bottom(Val::Px(dimensions::MARGIN_LARGE)),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                bevy_simple_text_input::TextInput,
                bevy_simple_text_input::TextInputValue("Enter your password".to_string()),
                bevy_simple_text_input::TextInputTextFont(TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                }),
                bevy_simple_text_input::TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
            ));
            
            // Button row
            form.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|buttons| {
                // Cancel button
                buttons.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                        ..default()
                    },
                    BackgroundColor(colors::SECONDARY),
                    BorderColor(colors::BORDER_DEFAULT),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Cancel"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
                
                // Login button
                buttons.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                        ..default()
                    },
                    BackgroundColor(colors::PRIMARY),
                    BorderColor(colors::BORDER_DEFAULT),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Login"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
            });
        }).id()
    }
    
    /// Create a settings form with sliders
    pub fn settings_form(parent: &mut ChildSpawnerCommands) -> Entity {
        // Manual construction due to lifetime constraints
        parent.spawn((
            Node {
                width: Val::Px(450.0),
                padding: UiRect::all(Val::Px(dimensions::PADDING_LARGE)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::MARGIN_MEDIUM),
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_MEDIUM),
            BorderColor(colors::BORDER_DEFAULT),
        )).with_children(|form| {
            // Title
            form.spawn((
                Text::new("Settings"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_XLARGE,
                    ..default()
                },
                TextColor(colors::TEXT_TITLE),
                Node {
                    margin: UiRect::bottom(Val::Px(dimensions::MARGIN_LARGE)),
                    ..default()
                },
            ));
            
            // Audio section
            form.spawn((
                Text::new("Audio"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_LARGE,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)),
                    ..default()
                },
            ));
            
            // Create simplified sliders for the form
            // These would be more complex with actual SliderBuilder but we simplify for the preset
            for (label, value) in &[
                ("Master Volume", 50.0),
                ("Music Volume", 50.0),
                ("SFX Volume", 50.0),
            ] {
                form.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::bottom(Val::Px(dimensions::MARGIN_MEDIUM)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|field| {
                    field.spawn((
                        Text::new(*label),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                    
                    field.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(30.0),
                            ..default()
                        },
                        BackgroundColor(colors::BACKGROUND_LIGHT),
                        Slider::new(0.0, 100.0, *value),
                        Interaction::default(),
                    ));
                });
            }
            
            // Separator
            form.spawn((
                Node {
                    height: Val::Px(1.0),
                    width: Val::Percent(100.0),
                    margin: UiRect::vertical(Val::Px(dimensions::MARGIN_MEDIUM)),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
            ));
            
            // Graphics section
            form.spawn((
                Text::new("Graphics"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_LARGE,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)),
                    ..default()
                },
            ));
            
            // Graphics sliders
            for (label, value) in &[
                ("Brightness", 50.0),
                ("Contrast", 50.0),
            ] {
                form.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::bottom(Val::Px(dimensions::MARGIN_MEDIUM)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|field| {
                    field.spawn((
                        Text::new(*label),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                    
                    field.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(30.0),
                            ..default()
                        },
                        BackgroundColor(colors::BACKGROUND_LIGHT),
                        Slider::new(0.0, 100.0, *value),
                        Interaction::default(),
                    ));
                });
            }
            
            // Button row
            form.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    column_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(dimensions::MARGIN_LARGE)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|buttons| {
                buttons.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                        ..default()
                    },
                    BackgroundColor(colors::SECONDARY),
                    BorderColor(colors::BORDER_DEFAULT),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Cancel"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
                
                buttons.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                        ..default()
                    },
                    BackgroundColor(colors::PRIMARY),
                    BorderColor(colors::BORDER_DEFAULT),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Apply"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
            });
        }).id()
    }
}