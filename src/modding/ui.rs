//! Mod Browser UI for Living Worlds
//! 
//! This module provides a comprehensive mod browser interface with:
//! - Steam Workshop integration for live browsing
//! - Local mod management
//! - Active modset configuration
//! - Soft-reset functionality

use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInputPlugin, TextInput, TextInputSettings, TextInputSubmitEvent, 
    TextInputValue, TextInputTextFont, TextInputTextColor
};
use crate::states::{GameState, RequestStateTransition};
use crate::ui::styles::{colors, dimensions, layers};
use crate::ui::buttons::{ButtonBuilder, ButtonStyle, ButtonSize, StyledButton};
use crate::loading_screen::{LoadingOperation, LoadingState, start_mod_application_loading};
use super::manager::ModManager;
use super::{ModEnabledEvent, ModDisabledEvent};

// ============================================================================
// EVENTS
// ============================================================================

/// Event to open the mod browser
#[derive(Event)]
pub struct OpenModBrowserEvent;

/// Event to close the mod browser
#[derive(Event)]
pub struct CloseModBrowserEvent;

/// Event to apply mod changes and soft-reset
#[derive(Event)]
pub struct ApplyModChangesEvent;

/// Event to switch tabs in the mod browser
#[derive(Event)]
pub struct SwitchModTabEvent {
    pub tab: ModBrowserTab,
}

// ============================================================================
// COMPONENTS
// ============================================================================

/// Root component for the mod browser UI
#[derive(Component)]
pub struct ModBrowserRoot;

/// Component for mod browser tabs
#[derive(Component)]
pub struct ModBrowserTabButton {
    pub tab: ModBrowserTab,
}

/// Current active tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModBrowserTab {
    Installed,
    Workshop,
    ActiveModset,
}

/// Component for a mod card in the grid
#[derive(Component)]
pub struct ModCard {
    pub mod_id: String,
    pub workshop_id: Option<u64>,
}

/// Component for mod toggle checkbox
#[derive(Component)]
pub struct ModToggle {
    pub mod_id: String,
}

/// Component for the confirm modset button
#[derive(Component)]
pub struct ConfirmModsetButton;

/// Component for the close mod browser button
#[derive(Component)]
pub struct CloseModBrowserButton;

/// Component for mod list items in Active Modset
#[derive(Component)]
pub struct ModListItem {
    pub mod_id: String,
    pub load_order: usize,
}

/// Component for the main content area
#[derive(Component)]
pub struct ContentArea;

/// Component for the search input field marker
#[derive(Component)]
pub struct SearchInputMarker;

/// Component for filter checkboxes
#[derive(Component)]
pub struct FilterCheckbox {
    pub filter_type: FilterType,
    pub checked: bool,
}

/// Types of filters available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    // Type filters
    Gameplay,
    Graphics,
    Audio,
    UI,
    Content,
    // Status filters
    Enabled,
    Disabled,
    UpdateAvailable,
}

// ============================================================================
// RESOURCES
// ============================================================================

/// Current state of the mod browser
#[derive(Resource)]
pub struct ModBrowserState {
    pub current_tab: ModBrowserTab,
    pub selected_mod: Option<String>,
    pub pending_changes: Vec<ModChange>,
    pub workshop_cache: WorkshopCache,
    pub search_query: String,
    pub active_filters: Vec<FilterType>,
}

impl Default for ModBrowserState {
    fn default() -> Self {
        Self {
            current_tab: ModBrowserTab::Installed,
            selected_mod: None,
            pending_changes: Vec::new(),
            workshop_cache: WorkshopCache::default(),
            search_query: String::new(),
            active_filters: Vec::new(),
        }
    }
}

/// Pending mod changes to apply
#[derive(Clone, Debug)]
pub enum ModChange {
    Enable(String),
    Disable(String),
    SetLoadOrder(String, usize),
    Subscribe(u64),
    Unsubscribe(u64),
}

/// Cache for Steam Workshop data
#[derive(Default)]
pub struct WorkshopCache {
    pub items: Vec<WorkshopItem>,
    pub last_update: f64,
}

/// Steam Workshop item data
#[derive(Clone)]
pub struct WorkshopItem {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub description: String,
    pub thumbnail_url: String,
    pub images: Vec<String>,
    pub rating: f32,
    pub rating_count: u32,
    pub subscribers: u32,
    pub size_mb: f32,
    pub updated: String,
    pub tags: Vec<String>,
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct ModBrowserUIPlugin;

impl Plugin for ModBrowserUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
            .init_resource::<ModBrowserState>()
            .add_event::<OpenModBrowserEvent>()
            .add_event::<CloseModBrowserEvent>()
            .add_event::<ApplyModChangesEvent>()
            .add_event::<SwitchModTabEvent>()
            .add_systems(Update, (
                handle_open_mod_browser,
                handle_close_mod_browser,
                handle_close_button_clicks,
                handle_tab_button_clicks,
                handle_confirm_modset_clicks,
                handle_tab_switching,
                handle_mod_toggles,
                handle_apply_changes,
                handle_search_input_changes,
                handle_search_submit,
                update_mod_browser_ui,
            ).chain());
    }
}

// ============================================================================
// UI SPAWNING
// ============================================================================

/// Spawn the mod browser UI
pub fn spawn_mod_browser(
    commands: &mut Commands,
    mod_manager: &ModManager,
    browser_state: &ModBrowserState,
) {
    // Root container - full screen overlay
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(colors::OVERLAY_DARK),
        ZIndex(layers::MODAL_OVERLAY),
        ModBrowserRoot,
    )).with_children(|parent| {
        // Header with tabs
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARK),
        )).with_children(|header| {
            // Tab buttons container
            header.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    ..default()
                },
            )).with_children(|tabs| {
                // Installed tab
                ButtonBuilder::new("Installed")
                    .style(if browser_state.current_tab == ModBrowserTab::Installed { 
                        ButtonStyle::Primary 
                    } else { 
                        ButtonStyle::Secondary 
                    })
                    .size(ButtonSize::Medium)
                    .with_marker(ModBrowserTabButton { tab: ModBrowserTab::Installed })
                    .build(tabs);
                
                // Workshop tab  
                ButtonBuilder::new("Workshop")
                    .style(if browser_state.current_tab == ModBrowserTab::Workshop { 
                        ButtonStyle::Primary 
                    } else { 
                        ButtonStyle::Secondary 
                    })
                    .size(ButtonSize::Medium)
                    .with_marker(ModBrowserTabButton { tab: ModBrowserTab::Workshop })
                    .build(tabs);
                
                // Active Modset tab
                ButtonBuilder::new("Active Modset")
                    .style(if browser_state.current_tab == ModBrowserTab::ActiveModset { 
                        ButtonStyle::Primary 
                    } else { 
                        ButtonStyle::Secondary 
                    })
                    .size(ButtonSize::Medium)
                    .with_marker(ModBrowserTabButton { tab: ModBrowserTab::ActiveModset })
                    .build(tabs);
            });
            
            // Search bar with bevy_simple_text_input
            header.spawn((
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(40.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_LIGHT),
                BorderColor(colors::BORDER_DEFAULT),
                TextInput,
                TextInputValue(browser_state.search_query.clone()),
                TextInputTextFont(TextFont {
                    font_size: 16.0,
                    ..default()
                }),
                TextInputTextColor(TextColor(colors::TEXT_PRIMARY)),
                TextInputSettings {
                    retain_on_submit: true,
                    ..default()
                },
                SearchInputMarker,
            ));
            
            // User info
            header.spawn((
                Node {
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
            )).with_children(|info| {
                info.spawn((
                    Text::new("Steam User"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));
            });
        });
        
        // Main content area
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_LIGHT),
        )).with_children(|content| {
            // Left sidebar filters
            content.spawn((
                Node {
                    width: Val::Px(250.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_MEDIUM),
            )).with_children(|sidebar| {
                // Filter header
                sidebar.spawn((
                    Text::new("FILTER MODS"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));
                
                // Type filters
                sidebar.spawn((
                    Text::new("Type"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
                
                // Type filter options
                for (filter_type, filter_enum) in [
                    ("Gameplay", FilterType::Gameplay),
                    ("Graphics", FilterType::Graphics),
                    ("Audio", FilterType::Audio),
                    ("UI", FilterType::UI),
                    ("Content", FilterType::Content),
                ] {
                    let is_checked = browser_state.active_filters.contains(&filter_enum);
                    sidebar.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(10.0),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                    )).with_children(|filter_row| {
                        // Checkbox
                        ButtonBuilder::new(if is_checked { "[X]" } else { "[ ]" })
                            .style(ButtonStyle::Ghost)
                            .size(ButtonSize::Small)
                            .with_marker(FilterCheckbox { 
                                filter_type: filter_enum,
                                checked: is_checked,
                            })
                            .build(filter_row);
                        
                        // Label
                        filter_row.spawn((
                            Text::new(filter_type),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                        ));
                    });
                }
                
                // Divider
                sidebar.spawn((
                    Node {
                        height: Val::Px(1.0),
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(colors::BORDER_DEFAULT),
                ));
                
                // Status filters
                sidebar.spawn((
                    Text::new("Status"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
                
                // Status filter options
                for filter_status in ["Enabled", "Disabled", "Update Available"] {
                    sidebar.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(10.0),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                    )).with_children(|filter_row| {
                        // Checkbox
                        ButtonBuilder::new("[ ]")
                            .style(ButtonStyle::Ghost)
                            .size(ButtonSize::Small)
                            .build(filter_row);
                        
                        // Label
                        filter_row.spawn((
                            Text::new(filter_status),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                        ));
                    });
                }
                
                // Divider
                sidebar.spawn((
                    Node {
                        height: Val::Px(1.0),
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(colors::BORDER_DEFAULT),
                ));
                
                // Sort options
                sidebar.spawn((
                    Text::new("Sort By"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
                
                // Sort dropdown placeholder
                sidebar.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(35.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_LIGHT),
                    BorderColor(colors::BORDER_DEFAULT),
                )).with_children(|dropdown| {
                    dropdown.spawn((
                        Text::new("Name (A-Z)"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_SECONDARY),
                    ));
                });
            });
            
            // Main content (changes based on tab)
            content.spawn((
                Node {
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ContentArea,
            )).with_children(|main| {
                // Show content based on current tab
                spawn_tab_content(main, browser_state.current_tab, mod_manager, &browser_state.search_query);
            });
        });
        
        // Bottom action bar
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                padding: UiRect::all(Val::Px(15.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARK),
        )).with_children(|bar| {
            // Left buttons
            bar.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
            )).with_children(|left| {
                ButtonBuilder::new("Back to Main Menu")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Medium)
                    .with_marker(CloseModBrowserButton)
                    .build(left);
                    
                ButtonBuilder::new("Refresh")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Medium)
                    .build(left);
            });
            
            // Right button - Confirm Modset
            ButtonBuilder::new("CONFIRM MODSET")
                .style(ButtonStyle::Primary)
                .size(ButtonSize::Large)
                .with_marker(ConfirmModsetButton)
                .build(bar);
        });
    });
}

/// Helper function to spawn tab content
fn spawn_tab_content(
    main: &mut ChildSpawnerCommands,
    tab: ModBrowserTab,
    mod_manager: &ModManager,
    search_query: &str,
) {
    match tab {
        ModBrowserTab::Installed => {
            // Grid for mod cards
            main.spawn((
                Node {
                    display: Display::Grid,
                    width: Val::Percent(100.0),
                    grid_template_columns: vec![GridTrack::fr(1.0); 3],
                    row_gap: Val::Px(20.0),
                    column_gap: Val::Px(20.0),
                    ..default()
                },
            )).with_children(|grid| {
                // Filter mods based on search query
                let filtered_mods: Vec<_> = mod_manager.available_mods.iter()
                    .filter(|m| {
                        if search_query.is_empty() {
                            return true;
                        }
                        let query_lower = search_query.to_lowercase();
                        m.manifest.name.to_lowercase().contains(&query_lower) ||
                        m.manifest.description.to_lowercase().contains(&query_lower) ||
                        m.manifest.author.to_lowercase().contains(&query_lower) ||
                        m.manifest.id.to_lowercase().contains(&query_lower)
                    })
                    .collect();
                
                // Generate mod cards for each filtered mod
                for loaded_mod in filtered_mods {
                    // Mod card container
                    grid.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(15.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(colors::BACKGROUND_MEDIUM),
                        BorderColor(colors::BORDER_DEFAULT),
                        ModCard {
                            mod_id: loaded_mod.manifest.id.clone(),
                            workshop_id: None,
                        },
                    )).with_children(|card| {
                        // Mod thumbnail placeholder
                        card.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(120.0),
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(colors::BACKGROUND_DARK),
                        )).with_children(|thumb| {
                            thumb.spawn((
                                Text::new("MOD ICON"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT_TERTIARY),
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(50.0),
                                    top: Val::Percent(50.0),
                                    ..default()
                                },
                            ));
                        });
                        
                        // Mod name
                        card.spawn((
                            Text::new(&loaded_mod.manifest.name),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                            Node {
                                margin: UiRect::bottom(Val::Px(5.0)),
                                ..default()
                            },
                        ));
                        
                        // Author
                        card.spawn((
                            Text::new(format!("by {}", loaded_mod.manifest.author)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                            Node {
                                margin: UiRect::bottom(Val::Px(5.0)),
                                ..default()
                            },
                        ));
                        
                        // Version and compatibility
                        card.spawn((
                            Text::new(format!("v{} | Game v{}", 
                                loaded_mod.manifest.version,
                                loaded_mod.manifest.compatible_game_version
                            )),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_TERTIARY),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                        
                        // Description
                        card.spawn((
                            Text::new(&loaded_mod.manifest.description),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                        
                        // Enable/disable toggle
                        card.spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                        )).with_children(|toggle_row| {
                            // Checkbox button
                            ButtonBuilder::new(if loaded_mod.enabled { "[X]" } else { "[ ]" })
                                .style(ButtonStyle::Secondary)
                                .size(ButtonSize::Small)
                                .with_marker(ModToggle { 
                                    mod_id: loaded_mod.manifest.id.clone() 
                                })
                                .build(toggle_row);
                            
                            // Label
                            toggle_row.spawn((
                                Text::new("Enabled"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT_PRIMARY),
                            ));
                        });
                    });
                }
            });
        }
        ModBrowserTab::Workshop => {
            // Workshop content placeholder
            main.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            )).with_children(|workshop| {
                workshop.spawn((
                    Text::new("Steam Workshop"),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                ));
                
                workshop.spawn((
                    Text::new("Browse and subscribe to community mods"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
                
                workshop.spawn((
                    Text::new("(Steam Workshop integration coming soon)"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_TERTIARY),
                ));
            });
        }
        ModBrowserTab::ActiveModset => {
            // Active modset list
            main.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    ..default()
                },
            )).with_children(|modset| {
                // Header
                modset.spawn((
                    Text::new("Active Modset"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                ));
                
                // List of active mods
                for (index, loaded_mod) in mod_manager.available_mods.iter()
                    .filter(|m| m.enabled)
                    .enumerate() 
                {
                    modset.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            column_gap: Val::Px(15.0),
                            ..default()
                        },
                        BackgroundColor(colors::BACKGROUND_MEDIUM),
                        BorderColor(colors::BORDER_DEFAULT),
                        ModListItem {
                            mod_id: loaded_mod.manifest.id.clone(),
                            load_order: index,
                        },
                    )).with_children(|item| {
                        // Drag handle placeholder
                        item.spawn((
                            Text::new("≡"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_TERTIARY),
                        ));
                        
                        // Load order number
                        item.spawn((
                            Text::new(format!("#{}", index + 1)),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                        ));
                        
                        // Mod name
                        item.spawn((
                            Text::new(&loaded_mod.manifest.name),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                            Node {
                                flex_grow: 1.0,
                                ..default()
                            },
                        ));
                        
                        // Version
                        item.spawn((
                            Text::new(format!("v{}", loaded_mod.manifest.version)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_TERTIARY),
                        ));
                        
                        // Toggle button
                        ButtonBuilder::new("Disable")
                            .style(ButtonStyle::Danger)
                            .size(ButtonSize::Small)
                            .with_marker(ModToggle { 
                                mod_id: loaded_mod.manifest.id.clone() 
                            })
                            .build(item);
                    });
                }
                
                // Show message if no mods are active
                if !mod_manager.available_mods.iter().any(|m| m.enabled) {
                    modset.spawn((
                        Text::new("No mods currently active"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_TERTIARY),
                        Node {
                            margin: UiRect::top(Val::Px(50.0)),
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                    ));
                }
            });
        }
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Handle opening the mod browser
fn handle_open_mod_browser(
    mut commands: Commands,
    mut events: EventReader<OpenModBrowserEvent>,
    query: Query<Entity, With<ModBrowserRoot>>,
    mod_manager: Res<ModManager>,
    state: Res<ModBrowserState>,
) {
    for _ in events.read() {
        // Remove existing browser if any
        for entity in &query {
            commands.entity(entity).despawn_recursive();
        }
        
        // Spawn new browser
        spawn_mod_browser(&mut commands, &mod_manager, &state);
    }
}

/// Handle closing the mod browser
fn handle_close_mod_browser(
    mut commands: Commands,
    mut events: EventReader<CloseModBrowserEvent>,
    query: Query<Entity, With<ModBrowserRoot>>,
) {
    for _ in events.read() {
        for entity in &query {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Handle close button clicks
fn handle_close_button_clicks(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseModBrowserButton>)>,
    mut close_events: EventWriter<CloseModBrowserEvent>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            close_events.send(CloseModBrowserEvent);
        }
    }
}

/// Handle tab button clicks
fn handle_tab_button_clicks(
    mut interaction_query: Query<(&Interaction, &ModBrowserTabButton), Changed<Interaction>>,
    mut switch_events: EventWriter<SwitchModTabEvent>,
) {
    for (interaction, tab_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            switch_events.send(SwitchModTabEvent { tab: tab_button.tab });
        }
    }
}

/// Handle confirm modset button clicks
fn handle_confirm_modset_clicks(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ConfirmModsetButton>)>,
    mut apply_events: EventWriter<ApplyModChangesEvent>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            apply_events.send(ApplyModChangesEvent);
        }
    }
}

/// Handle tab switching
fn handle_tab_switching(
    mut commands: Commands,
    mut events: EventReader<SwitchModTabEvent>,
    mut state: ResMut<ModBrowserState>,
    content_query: Query<(Entity, Option<&Children>), With<ContentArea>>,
    mod_manager: Res<ModManager>,
) {
    for event in events.read() {
        state.current_tab = event.tab;
        
        // Rebuild content area with new tab content
        for (entity, children) in content_query.iter() {
            // Despawn all existing children if any
            if let Some(children) = children {
                for child in children.iter() {
                    commands.entity(child).despawn();
                }
            }
            
            // Rebuild content based on new tab
            commands.entity(entity).with_children(|main| {
                spawn_tab_content(main, event.tab, &mod_manager, &state.search_query);
            });
        }
    }
}

/// Handle mod toggle checkboxes
fn handle_mod_toggles(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &ModToggle), Changed<Interaction>>,
    mut state: ResMut<ModBrowserState>,
    mut mod_manager: ResMut<ModManager>,
    browser_query: Query<Entity, With<ModBrowserRoot>>,
) {
    for (interaction, toggle) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Find the mod and toggle its enabled state
            if let Some(mod_ref) = mod_manager.available_mods.iter_mut()
                .find(|m| m.manifest.id == toggle.mod_id) 
            {
                mod_ref.enabled = !mod_ref.enabled;
                
                // Add to pending changes for later application
                if mod_ref.enabled {
                    state.pending_changes.push(ModChange::Enable(toggle.mod_id.clone()));
                } else {
                    state.pending_changes.push(ModChange::Disable(toggle.mod_id.clone()));
                }
                
                // Rebuild the UI to reflect the change
                for entity in &browser_query {
                    commands.entity(entity).despawn();
                }
                spawn_mod_browser(&mut commands, &mod_manager, &state);
            }
        }
    }
}

/// Handle applying mod changes and soft-reset
fn handle_apply_changes(
    mut events: EventReader<ApplyModChangesEvent>,
    mut loading_state: ResMut<LoadingState>,
    mut state_transition: EventWriter<RequestStateTransition>,
) {
    for _ in events.read() {
        // Start the mod application loading
        start_mod_application_loading(&mut loading_state);
        
        // Transition to loading state
        state_transition.send(RequestStateTransition {
            from: GameState::InGame,
            to: GameState::LoadingWorld,
        });
    }
}

/// Handle text input changes for search
fn handle_search_input_changes(
    mut browser_state: ResMut<ModBrowserState>,
    mut text_inputs: Query<&TextInputValue, (Changed<TextInputValue>, With<SearchInputMarker>)>,
    content_query: Query<(Entity, Option<&Children>), With<ContentArea>>,
    mut commands: Commands,
    mod_manager: Res<ModManager>,
) {
    for text_value in &mut text_inputs {
        // Update the browser state with new search query
        browser_state.search_query = text_value.0.clone();
        
        // Rebuild the content area to show filtered results
        for (entity, children) in content_query.iter() {
            // Despawn all existing children if any
            if let Some(children) = children {
                for child in children.iter() {
                    commands.entity(child).despawn();
                }
            }
            
            // Rebuild content with filtered mods
            commands.entity(entity).with_children(|main| {
                spawn_tab_content(main, browser_state.current_tab, &mod_manager, &browser_state.search_query);
            });
        }
    }
}

/// Handle search submit events (Enter key)
fn handle_search_submit(
    mut submit_events: EventReader<TextInputSubmitEvent>,
    text_inputs: Query<&TextInputValue>,
    browser_state: Res<ModBrowserState>,
) {
    for event in submit_events.read() {
        if let Ok(text_value) = text_inputs.get(event.entity) {
            // For now, just log the search submission
            println!("Search submitted: {}", text_value.0);
            
            // In the future, this could trigger more advanced search operations
            // like searching the Steam Workshop API
        }
    }
}

/// Update the mod browser UI based on current state
fn update_mod_browser_ui(
    state: Res<ModBrowserState>,
    mut tab_query: Query<(&ModBrowserTabButton, &mut StyledButton, &mut BackgroundColor)>,
) {
    if !state.is_changed() {
        return;
    }
    
    // Update tab button styles
    for (tab_button, mut styled_button, mut bg_color) in &mut tab_query {
        if tab_button.tab == state.current_tab {
            styled_button.style = ButtonStyle::Primary;
            *bg_color = BackgroundColor(colors::PRIMARY);
        } else {
            styled_button.style = ButtonStyle::Secondary;
            *bg_color = BackgroundColor(colors::SECONDARY);
        }
    }
}
