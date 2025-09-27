//! Shortcut registry for managing keyboard bindings

use bevy::prelude::*;
use std::collections::HashMap;
use super::types::*;

/// Central registry for all keyboard shortcuts
#[derive(Resource, Default)]
pub struct ShortcutRegistry {
    /// Map of shortcut IDs to their definitions
    shortcuts: HashMap<ShortcutId, ShortcutDefinition>,
    /// Map of key bindings to shortcut IDs (for conflict detection)
    bindings: HashMap<(KeyBinding, ShortcutContext), ShortcutId>,
    /// Grouped shortcuts for UI display
    groups: HashMap<String, ShortcutGroup>,
    /// Currently active context
    active_context: ShortcutContext,
}

impl ShortcutRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new shortcut
    pub fn register(
        &mut self,
        id: ShortcutId,
        binding: KeyBinding,
        description: impl Into<String>,
        context: ShortcutContext,
    ) -> Result<(), ShortcutConflict> {
        // Check for conflicts
        if let Some(existing_id) = self.bindings.get(&(binding, context)) {
            if existing_id != &id {
                return Err(ShortcutConflict {
                    new_id: id.clone(),
                    existing_id: existing_id.clone(),
                    binding,
                    context,
                });
            }
        }

        // Create definition
        let definition = ShortcutDefinition {
            id: id.clone(),
            binding,
            description: description.into(),
            context,
            enabled: true,
        };

        // Store in registry
        self.shortcuts.insert(id.clone(), definition.clone());
        self.bindings.insert((binding, context), id);

        Ok(())
    }

    /// Register multiple shortcuts at once
    pub fn register_many(&mut self, shortcuts: Vec<(ShortcutId, KeyBinding, &str, ShortcutContext)>) {
        for (id, binding, desc, context) in shortcuts {
            let _ = self.register(id, binding, desc, context);
        }
    }

    /// Check if a shortcut was just pressed
    pub fn just_pressed(
        &self,
        id: ShortcutId,
        keyboard: &ButtonInput<KeyCode>,
    ) -> bool {
        if let Some(definition) = self.shortcuts.get(&id) {
            if !definition.enabled {
                return false;
            }

            if !definition.context.matches(&self.active_context) {
                return false;
            }

            definition.binding.just_pressed(keyboard)
        } else {
            false
        }
    }

    /// Check if any shortcut was triggered and return its ID
    pub fn check_shortcuts(&self, keyboard: &ButtonInput<KeyCode>) -> Option<ShortcutId> {
        for definition in self.shortcuts.values() {
            if !definition.enabled {
                continue;
            }

            if !definition.context.matches(&self.active_context) {
                continue;
            }

            if definition.binding.just_pressed(keyboard) {
                return Some(definition.id.clone());
            }
        }
        None
    }

    /// Set the active context
    pub fn set_context(&mut self, context: ShortcutContext) {
        self.active_context = context;
    }

    /// Get current context
    pub fn context(&self) -> ShortcutContext {
        self.active_context
    }

    /// Enable/disable a shortcut
    pub fn set_enabled(&mut self, id: ShortcutId, enabled: bool) {
        if let Some(definition) = self.shortcuts.get_mut(&id) {
            definition.enabled = enabled;
        }
    }

    /// Get shortcut definition
    pub fn get(&self, id: &ShortcutId) -> Option<&ShortcutDefinition> {
        self.shortcuts.get(id)
    }

    /// Change binding for a shortcut
    pub fn rebind(
        &mut self,
        id: ShortcutId,
        new_binding: KeyBinding,
    ) -> Result<(), ShortcutConflict> {
        if let Some(definition) = self.shortcuts.get(&id) {
            let context = definition.context;

            // Check for conflicts with new binding
            if let Some(existing_id) = self.bindings.get(&(new_binding, context)) {
                if existing_id != &id {
                    return Err(ShortcutConflict {
                        new_id: id.clone(),
                        existing_id: existing_id.clone(),
                        binding: new_binding,
                        context,
                    });
                }
            }

            // Remove old binding
            self.bindings.remove(&(definition.binding, context));

            // Update definition
            if let Some(definition) = self.shortcuts.get_mut(&id) {
                definition.binding = new_binding;
            }

            // Add new binding
            self.bindings.insert((new_binding, context), id);
        }

        Ok(())
    }

    /// Get all shortcuts in a specific context
    pub fn shortcuts_in_context(&self, context: ShortcutContext) -> Vec<&ShortcutDefinition> {
        self.shortcuts
            .values()
            .filter(|def| def.context.matches(&context))
            .collect()
    }

    /// Add shortcut to a group
    pub fn add_to_group(&mut self, id: ShortcutId, group_name: impl Into<String>) {
        let group_name = group_name.into();
        let group = self.groups.entry(group_name.clone()).or_insert(ShortcutGroup {
            name: group_name,
            shortcuts: Vec::new(),
        });

        if !group.shortcuts.contains(&id) {
            group.shortcuts.push(id);
        }
    }

    /// Get all groups
    pub fn groups(&self) -> &HashMap<String, ShortcutGroup> {
        &self.groups
    }

    /// Clear all shortcuts
    pub fn clear(&mut self) {
        self.shortcuts.clear();
        self.bindings.clear();
        self.groups.clear();
    }

    /// Get display string for a shortcut
    pub fn display_string(&self, id: ShortcutId) -> Option<String> {
        self.shortcuts.get(&id).map(|def| def.display_string())
    }

    /// Initialize with default game shortcuts
    pub fn init_defaults(&mut self) {
        use ShortcutId::*;

        // Time controls
        self.register_many(vec![
            (Pause, KeyBinding::single(KeyCode::Space), "Pause/Resume", ShortcutContext::InGame),
            (SpeedUp, KeyBinding::single(KeyCode::Equal), "Speed Up", ShortcutContext::InGame),
            (SlowDown, KeyBinding::single(KeyCode::Minus), "Slow Down", ShortcutContext::InGame),
            (Speed1, KeyBinding::single(KeyCode::Digit1), "Speed 1x", ShortcutContext::InGame),
            (Speed2, KeyBinding::single(KeyCode::Digit2), "Speed 2x", ShortcutContext::InGame),
            (Speed3, KeyBinding::single(KeyCode::Digit3), "Speed 3x", ShortcutContext::InGame),
            (Speed4, KeyBinding::single(KeyCode::Digit4), "Speed 4x", ShortcutContext::InGame),
            (Speed5, KeyBinding::single(KeyCode::Digit5), "Speed 5x", ShortcutContext::InGame),
        ]);

        // Camera controls
        self.register_many(vec![
            (CameraUp, KeyBinding::single(KeyCode::KeyW), "Move Camera Up", ShortcutContext::InGame),
            (CameraDown, KeyBinding::single(KeyCode::KeyS), "Move Camera Down", ShortcutContext::InGame),
            (CameraLeft, KeyBinding::single(KeyCode::KeyA), "Move Camera Left", ShortcutContext::InGame),
            (CameraRight, KeyBinding::single(KeyCode::KeyD), "Move Camera Right", ShortcutContext::InGame),
            (CameraZoomIn, KeyBinding::single(KeyCode::KeyQ), "Zoom In", ShortcutContext::InGame),
            (CameraZoomOut, KeyBinding::single(KeyCode::KeyE), "Zoom Out", ShortcutContext::InGame),
        ]);

        // File operations
        self.register_many(vec![
            (SaveGame, KeyBinding::single(KeyCode::KeyS).with_ctrl(), "Save Game", ShortcutContext::InGame),
            (LoadGame, KeyBinding::single(KeyCode::KeyL).with_ctrl(), "Load Game", ShortcutContext::Global),
            (QuickSave, KeyBinding::single(KeyCode::F5), "Quick Save", ShortcutContext::InGame),
            (QuickLoad, KeyBinding::single(KeyCode::F9), "Quick Load", ShortcutContext::Global),
        ]);

        // UI toggles
        self.register_many(vec![
            (ToggleBorders, KeyBinding::single(KeyCode::KeyB), "Toggle Borders", ShortcutContext::InGame),
            (ToggleGrid, KeyBinding::single(KeyCode::KeyG), "Toggle Grid", ShortcutContext::InGame),
            (ToggleHud, KeyBinding::single(KeyCode::KeyH), "Toggle HUD", ShortcutContext::InGame),
            (ToggleFps, KeyBinding::single(KeyCode::F3), "Toggle FPS", ShortcutContext::Global),
            (ToggleFullscreen, KeyBinding::single(KeyCode::F11), "Toggle Fullscreen", ShortcutContext::Global),
        ]);

        // Map modes
        self.register_many(vec![
            (MapModeToggle, KeyBinding::single(KeyCode::Tab), "Toggle Map Mode", ShortcutContext::InGame),
        ]);

        // Menus
        self.register_many(vec![
            (OpenMainMenu, KeyBinding::single(KeyCode::Escape), "Main Menu", ShortcutContext::Global),
            (OpenSettings, KeyBinding::single(KeyCode::KeyO).with_ctrl(), "Settings", ShortcutContext::Global),
            (OpenHelp, KeyBinding::single(KeyCode::F1), "Help", ShortcutContext::Global),
        ]);

        // Settings navigation (only active in settings menu)
        self.register_many(vec![
            (SettingsNavigateNext, KeyBinding::single(KeyCode::Tab), "Navigate Next", ShortcutContext::Settings),
            (SettingsNavigatePrevious, KeyBinding::single(KeyCode::Tab).with_shift(), "Navigate Previous", ShortcutContext::Settings),
            (SettingsActivate, KeyBinding::single(KeyCode::Enter), "Activate Element", ShortcutContext::Settings),
            (SettingsActivateSpace, KeyBinding::single(KeyCode::Space), "Activate Element", ShortcutContext::Settings),
        ]);

        // Dropdown navigation (available globally when dropdowns are open)
        self.register_many(vec![
            (DropdownUp, KeyBinding::single(KeyCode::ArrowUp), "Dropdown Previous", ShortcutContext::Global),
            (DropdownDown, KeyBinding::single(KeyCode::ArrowDown), "Dropdown Next", ShortcutContext::Global),
            (DropdownSelect, KeyBinding::single(KeyCode::Enter), "Dropdown Select", ShortcutContext::Global),
        ]);

        // Screenshot
        self.register_many(vec![
            (TakeScreenshot, KeyBinding::single(KeyCode::F12), "Take Screenshot", ShortcutContext::Global),
        ]);

        // Debug shortcuts (only in debug builds)
        #[cfg(debug_assertions)]
        self.register_many(vec![
            (DebugForceEnactLaw, KeyBinding::single(KeyCode::KeyE).with_shift(), "Debug: Force Enact Law", ShortcutContext::InGame),
            (DebugForceRepealLaw, KeyBinding::single(KeyCode::KeyR).with_shift(), "Debug: Force Repeal Law", ShortcutContext::InGame),
            (DebugTriggerProposal, KeyBinding::single(KeyCode::KeyP).with_shift(), "Debug: Trigger Law Proposal", ShortcutContext::InGame),
        ]);

        // Create groups
        self.add_to_group(Pause, "Time Controls");
        self.add_to_group(SpeedUp, "Time Controls");
        self.add_to_group(SlowDown, "Time Controls");

        self.add_to_group(CameraUp, "Camera");
        self.add_to_group(CameraDown, "Camera");
        self.add_to_group(CameraLeft, "Camera");
        self.add_to_group(CameraRight, "Camera");
    }
}

/// Definition of a single shortcut
#[derive(Debug, Clone)]
pub struct ShortcutDefinition {
    pub id: ShortcutId,
    pub binding: KeyBinding,
    pub description: String,
    pub context: ShortcutContext,
    pub enabled: bool,
}

impl ShortcutDefinition {
    /// Get display string for this shortcut
    pub fn display_string(&self) -> String {
        format!("{}: {}", self.binding.display_string(), self.description)
    }
}

/// Conflict when registering shortcuts
#[derive(Debug, Clone)]
pub struct ShortcutConflict {
    pub new_id: ShortcutId,
    pub existing_id: ShortcutId,
    pub binding: KeyBinding,
    pub context: ShortcutContext,
}

impl std::fmt::Display for ShortcutConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Shortcut conflict: {} already bound to {:?} in context {:?}",
            self.binding.display_string(),
            self.existing_id,
            self.context
        )
    }
}

impl std::error::Error for ShortcutConflict {}

/// Group of related shortcuts
#[derive(Debug, Clone)]
pub struct ShortcutGroup {
    pub name: String,
    pub shortcuts: Vec<ShortcutId>,
}