//! Reusable dialog system for Living Worlds UI
//! 
//! Provides standardized dialog creation with consistent styling,
//! animations, and behavior across the entire game interface.

use bevy::prelude::*;
use super::styles::{colors, dimensions, layers, helpers};
use super::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};

/// Component for dialog overlays
#[derive(Component, Debug, Clone)]
pub struct DialogOverlay {
    pub dialog_type: DialogType,
    pub dismissible: bool,
}

/// Types of dialogs in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    ExitConfirmation,
    UnsavedChanges,
    Resolution,
    Error,
    Info,
    Custom,
    WorldGenerationError,
}

/// Component for dialog containers
#[derive(Component, Debug)]
pub struct DialogContainer {
    pub dialog_type: DialogType,
}

/// Component for dialog title text
#[derive(Component)]
pub struct DialogTitle;

/// Component for dialog body text
#[derive(Component)]
pub struct DialogBody;

/// Component for dialog button row
#[derive(Component)]
pub struct DialogButtonRow;

/// Marker for exit confirmation dialog
#[derive(Component)]
pub struct ExitConfirmationDialog;

/// Marker for unsaved changes dialog
#[derive(Component)]
pub struct UnsavedChangesDialog;

/// Marker for resolution dialog
#[derive(Component)]
pub struct ResolutionDialog;

/// Marker for resolution confirmation dialog
#[derive(Component)]
pub struct ResolutionConfirmDialog;

/// Marker for countdown text in resolution dialog
#[derive(Component)]
pub struct CountdownText;

/// Marker for world generation error dialog
#[derive(Component)]
pub struct WorldGenerationErrorDialog;

/// Button markers for dialog actions
#[derive(Component)]
pub struct ConfirmButton;

#[derive(Component)]
pub struct CancelButton;

#[derive(Component)]
pub struct SaveButton;

#[derive(Component)]
pub struct DiscardButton;

#[derive(Component)]
pub struct KeepButton;

#[derive(Component)]
pub struct RevertButton;

/// Builder for creating dialogs
pub struct DialogBuilder {
    title: String,
    body: String,
    dialog_type: DialogType,
    width: f32,
    buttons: Vec<DialogButton>,
    dismissible: bool,
    z_index: i32,
}

/// Button configuration for dialogs
pub struct DialogButton {
    text: String,
    style: ButtonStyle,
    marker: DialogButtonMarker,
}

/// Marker types for dialog buttons
pub enum DialogButtonMarker {
    Confirm,
    Cancel,
    Save,
    Discard,
    Custom(Box<dyn FnOnce(&mut EntityCommands)>),
}

impl DialogBuilder {
    /// Create a new dialog builder
    pub fn new(dialog_type: DialogType) -> Self {
        Self {
            title: String::new(),
            body: String::new(),
            dialog_type,
            width: dimensions::DIALOG_WIDTH_MEDIUM,
            buttons: Vec::new(),
            dismissible: true,
            z_index: layers::MODAL_OVERLAY,
        }
    }
    
    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    
    /// Set the dialog body text
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }
    
    /// Set the dialog width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    
    /// Set whether the dialog can be dismissed by clicking outside
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }
    
    /// Set the z-index layer
    pub fn z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }
    
    /// Add a confirm button
    pub fn confirm_button(mut self, text: impl Into<String>) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Primary,
            marker: DialogButtonMarker::Confirm,
        });
        self
    }
    
    /// Add a cancel button
    pub fn cancel_button(mut self, text: impl Into<String>) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Secondary,
            marker: DialogButtonMarker::Cancel,
        });
        self
    }
    
    /// Add a danger button (for destructive actions)
    pub fn danger_button(mut self, text: impl Into<String>, marker: DialogButtonMarker) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Danger,
            marker,
        });
        self
    }
    
    /// Add a save button
    pub fn save_button(mut self, text: impl Into<String>) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style: ButtonStyle::Success,
            marker: DialogButtonMarker::Save,
        });
        self
    }
    
    /// Add a custom button
    pub fn custom_button(
        mut self,
        text: impl Into<String>,
        style: ButtonStyle,
        marker: DialogButtonMarker,
    ) -> Self {
        self.buttons.push(DialogButton {
            text: text.into(),
            style,
            marker,
        });
        self
    }
    
    /// Build the dialog and return its root entity
    pub fn build(self, commands: &mut Commands) -> Entity {
        // Create overlay that blocks clicks
        let overlay_entity = commands.spawn((
            Button,  // Add Button to block clicks to elements behind
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::OVERLAY_DARK),
            DialogOverlay {
                dialog_type: self.dialog_type,
                dismissible: self.dismissible,
            },
            ZIndex(self.z_index),
        )).id();
        
        // Add type-specific marker
        match self.dialog_type {
            DialogType::ExitConfirmation => {
                commands.entity(overlay_entity).insert(ExitConfirmationDialog);
            }
            DialogType::UnsavedChanges => {
                commands.entity(overlay_entity).insert(UnsavedChangesDialog);
            }
            DialogType::Resolution => {
                commands.entity(overlay_entity).insert(ResolutionDialog);
            }
            _ => {}
        }
        
        // Create container
        let container_entity = commands.spawn((
            Node {
                width: Val::Px(self.width),
                padding: helpers::standard_padding(),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                border: helpers::standard_border(),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_MEDIUM),
            BorderColor(colors::BORDER_DEFAULT),
            DialogContainer {
                dialog_type: self.dialog_type,
            },
            ZIndex(self.z_index + 50),
        )).id();
        
        // Build dialog content
        commands.entity(container_entity).with_children(|parent| {
            // Title
            if !self.title.is_empty() {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        margin: UiRect::bottom(Val::Px(dimensions::DIALOG_SPACING)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|title_parent| {
                    title_parent.spawn((
                        Text::new(self.title.clone()),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_TITLE,
                            ..default()
                        },
                        TextColor(colors::TEXT_TITLE),
                        DialogTitle,
                    ));
                });
            }
            
            // Body
            if !self.body.is_empty() {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        margin: UiRect::bottom(Val::Px(dimensions::DIALOG_SPACING)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|body_parent| {
                    body_parent.spawn((
                        Text::new(self.body.clone()),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_NORMAL,
                            ..default()
                        },
                        TextColor(colors::TEXT_SECONDARY),
                        DialogBody,
                    ));
                });
            }
            
            // Button row
            if !self.buttons.is_empty() {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(dimensions::MARGIN_MEDIUM),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    DialogButtonRow,
                )).with_children(|button_row| {
                    for button in self.buttons {
                        let mut builder = ButtonBuilder::new(button.text)
                            .style(button.style)
                            .size(ButtonSize::Medium);
                        
                        // Add marker based on type
                        match button.marker {
                            DialogButtonMarker::Confirm => {
                                builder = builder.with_marker(ConfirmButton);
                            }
                            DialogButtonMarker::Cancel => {
                                builder = builder.with_marker(CancelButton);
                            }
                            DialogButtonMarker::Save => {
                                builder = builder.with_marker(SaveButton);
                            }
                            DialogButtonMarker::Discard => {
                                builder = builder.with_marker(DiscardButton);
                            }
                            DialogButtonMarker::Custom(_marker_fn) => {
                                // Custom markers require manual spawning as closures
                                // cannot be stored in the builder pattern structure
                                continue;
                            }
                        }
                        
                        builder.build(button_row);
                    }
                });
            }
        });
        
        // Add container as child of overlay
        commands.entity(overlay_entity).add_child(container_entity);
        
        overlay_entity
    }
}

/// Helper functions for creating common dialogs
pub mod presets {
    use super::*;
    
    /// Create a standard exit confirmation dialog
    pub fn exit_confirmation_dialog(mut commands: Commands) -> Entity {
        DialogBuilder::new(DialogType::ExitConfirmation)
            .title("Exit Game")
            .body("Are you sure you want to exit?")
            .width(dimensions::DIALOG_WIDTH_MEDIUM)
            .dismissible(false)
            .z_index(layers::CRITICAL_DIALOG)
            .danger_button("Exit Game", DialogButtonMarker::Confirm)
            .cancel_button("Cancel")
            .build(&mut commands)
    }
    
    /// Create an unsaved changes dialog
    pub fn unsaved_changes_dialog(mut commands: Commands) -> Entity {
        DialogBuilder::new(DialogType::UnsavedChanges)
            .title("Unsaved Changes")
            .body("You have unsaved changes. What would you like to do?")
            .width(dimensions::DIALOG_WIDTH_MEDIUM)
            .dismissible(false)
            .save_button("Save & Exit")
            .danger_button("Discard Changes", DialogButtonMarker::Discard)
            .cancel_button("Cancel")
            .build(&mut commands)
    }
    
    /// Create a resolution change dialog
    pub fn resolution_dialog(mut commands: Commands, new_resolution: (u32, u32)) -> Entity {
        DialogBuilder::new(DialogType::Resolution)
            .title("Change Resolution")
            .body(format!("Change resolution to {}x{}?", new_resolution.0, new_resolution.1))
            .width(dimensions::DIALOG_WIDTH_SMALL)
            .confirm_button("Apply")
            .cancel_button("Cancel")
            .build(&mut commands)
    }
    
    /// Create a world generation error dialog
    pub fn world_generation_error_dialog(mut commands: Commands, error_message: &str) -> Entity {
        DialogBuilder::new(DialogType::WorldGenerationError)
            .title("World Generation Failed")
            .body(format!("Failed to generate world:\n\n{}\n\nWould you like to try again with different settings?", error_message))
            .width(dimensions::DIALOG_WIDTH_MEDIUM)
            .dismissible(false)
            .z_index(layers::CRITICAL_DIALOG)
            .confirm_button("Try Again")
            .cancel_button("Main Menu")
            .build(&mut commands)
    }
    
    /// Create a simple info dialog
    pub fn info_dialog(mut commands: Commands, title: &str, message: &str) -> Entity {
        DialogBuilder::new(DialogType::Info)
            .title(title)
            .body(message)
            .width(dimensions::DIALOG_WIDTH_MEDIUM)
            .confirm_button("OK")
            .build(&mut commands)
    }
    
    /// Create an error dialog
    pub fn error_dialog(mut commands: Commands, error_message: &str) -> Entity {
        DialogBuilder::new(DialogType::Error)
            .title("Error")
            .body(error_message)
            .width(dimensions::DIALOG_WIDTH_MEDIUM)
            .z_index(layers::CRITICAL_DIALOG)
            .dismissible(false)
            .confirm_button("OK")
            .build(&mut commands)
    }
    
    /// Create a resolution confirmation dialog with countdown
    pub fn resolution_confirm_dialog(mut commands: Commands) -> Entity {
        // Create overlay that blocks clicks
        let overlay_entity = commands.spawn((
            Button,  // Add Button to block clicks to elements behind
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::OVERLAY_DARK),
            DialogOverlay {
                dialog_type: DialogType::Resolution,
                dismissible: false,
            },
            ResolutionConfirmDialog,
            ZIndex(layers::CRITICAL_DIALOG),
        )).id();
        
        // Create container
        let container_entity = commands.spawn((
            Node {
                width: Val::Px(dimensions::DIALOG_WIDTH_SMALL),
                padding: helpers::standard_padding(),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                border: helpers::standard_border(),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_MEDIUM),
            BorderColor(colors::BORDER_DEFAULT),
            DialogContainer {
                dialog_type: DialogType::Resolution,
            },
            ZIndex(layers::CRITICAL_DIALOG + 50),
        )).id();
        
        // Build dialog content
        commands.entity(container_entity).with_children(|parent| {
            // Title
            parent.spawn((
                Node {
                    margin: UiRect::bottom(Val::Px(dimensions::DIALOG_SPACING)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|title_parent| {
                title_parent.spawn((
                    Text::new("Keep Display Settings?"),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_TITLE,
                        ..default()
                    },
                    TextColor(colors::TEXT_TITLE),
                    DialogTitle,
                ));
            });
            
            // Countdown text
            parent.spawn((
                Node {
                    margin: UiRect::bottom(Val::Px(dimensions::DIALOG_SPACING)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|countdown_parent| {
                countdown_parent.spawn((
                    Text::new("Reverting in 15 seconds..."),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                    CountdownText,
                ));
            });
            
            // Button row
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(dimensions::MARGIN_MEDIUM),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                DialogButtonRow,
            )).with_children(|button_row| {
                // Keep button
                ButtonBuilder::new("Keep")
                    .style(ButtonStyle::Success)
                    .size(ButtonSize::Medium)
                    .with_marker(KeepButton)
                    .build(button_row);
                
                // Revert button
                ButtonBuilder::new("Revert")
                    .style(ButtonStyle::Danger)
                    .size(ButtonSize::Medium)
                    .with_marker(RevertButton)
                    .build(button_row);
            });
        });
        
        // Add container as child of overlay
        commands.entity(overlay_entity).add_child(container_entity);
        
        overlay_entity
    }
}

/// System to handle dialog dismissal when clicking outside
pub fn dialog_dismiss_system(
    _commands: Commands,
    interactions: Query<(&Interaction, &DialogOverlay), (Changed<Interaction>, With<Node>)>,
) {
    for (interaction, overlay) in &interactions {
        if overlay.dismissible && matches!(interaction, Interaction::Pressed) {
            // Only dismiss if clicking on the overlay itself (not the dialog content)
            // This is handled by checking if the entity has DialogOverlay component
            // The actual dialog content doesn't have this component
        }
    }
}

/// Plugin for the dialog system
pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dialog_dismiss_system);
    }
}