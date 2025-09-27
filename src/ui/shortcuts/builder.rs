//! Builder API for creating and registering shortcuts

use bevy::prelude::*;
use super::registry::*;
use super::types::*;
use super::systems::ShortcutHintFilter;

/// Builder for creating keyboard shortcuts
pub struct ShortcutBuilder {
    id: ShortcutId,
    binding: Option<KeyBinding>,
    description: String,
    context: ShortcutContext,
    group: Option<String>,
}

impl ShortcutBuilder {
    /// Create a new shortcut builder
    pub fn new(id: ShortcutId) -> Self {
        Self {
            id,
            binding: None,
            description: String::new(),
            context: ShortcutContext::Global,
            group: None,
        }
    }

    /// Set the key binding
    pub fn key(mut self, key: KeyCode) -> Self {
        self.binding = Some(KeyBinding::single(key));
        self
    }

    /// Set key with Ctrl modifier
    pub fn ctrl_key(mut self, key: KeyCode) -> Self {
        self.binding = Some(KeyBinding::single(key).with_ctrl());
        self
    }

    /// Set key with Shift modifier
    pub fn shift_key(mut self, key: KeyCode) -> Self {
        self.binding = Some(KeyBinding::single(key).with_shift());
        self
    }

    /// Set key with Alt modifier
    pub fn alt_key(mut self, key: KeyCode) -> Self {
        self.binding = Some(KeyBinding::single(key).with_alt());
        self
    }

    /// Set custom binding
    pub fn binding(mut self, binding: KeyBinding) -> Self {
        self.binding = Some(binding);
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set context
    pub fn context(mut self, context: ShortcutContext) -> Self {
        self.context = context;
        self
    }

    /// Set to global context (active everywhere)
    pub fn global(mut self) -> Self {
        self.context = ShortcutContext::Global;
        self
    }

    /// Set to in-game context only
    pub fn in_game(mut self) -> Self {
        self.context = ShortcutContext::InGame;
        self
    }

    /// Set to main menu context only
    pub fn main_menu(mut self) -> Self {
        self.context = ShortcutContext::MainMenu;
        self
    }

    /// Add to a group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Register the shortcut with the registry
    pub fn register(self, registry: &mut ShortcutRegistry) -> Result<(), ShortcutConflict> {
        let binding = self.binding.expect("Shortcut must have a key binding");

        registry.register(self.id.clone(), binding, self.description, self.context)?;

        if let Some(group) = self.group {
            registry.add_to_group(self.id, group);
        }

        Ok(())
    }
}

/// Helper function to create shortcuts
pub fn shortcuts() -> ShortcutSetBuilder {
    ShortcutSetBuilder::new()
}

/// Builder for creating multiple shortcuts at once
pub struct ShortcutSetBuilder {
    shortcuts: Vec<ShortcutBuilder>,
}

impl ShortcutSetBuilder {
    /// Create a new set builder
    pub fn new() -> Self {
        Self {
            shortcuts: Vec::new(),
        }
    }

    /// Add a shortcut to the set
    pub fn add(mut self, shortcut: ShortcutBuilder) -> Self {
        self.shortcuts.push(shortcut);
        self
    }

    /// Add time control shortcuts
    pub fn with_time_controls(mut self) -> Self {
        self.shortcuts.extend(vec![
            ShortcutBuilder::new(ShortcutId::Pause)
                .key(KeyCode::Space)
                .description("Pause/Resume")
                .in_game()
                .group("Time"),
            ShortcutBuilder::new(ShortcutId::SpeedUp)
                .key(KeyCode::Equal)
                .description("Speed Up")
                .in_game()
                .group("Time"),
            ShortcutBuilder::new(ShortcutId::SlowDown)
                .key(KeyCode::Minus)
                .description("Slow Down")
                .in_game()
                .group("Time"),
        ]);
        self
    }

    /// Add camera control shortcuts
    pub fn with_camera_controls(mut self) -> Self {
        self.shortcuts.extend(vec![
            ShortcutBuilder::new(ShortcutId::CameraUp)
                .key(KeyCode::KeyW)
                .description("Move Up")
                .in_game()
                .group("Camera"),
            ShortcutBuilder::new(ShortcutId::CameraDown)
                .key(KeyCode::KeyS)
                .description("Move Down")
                .in_game()
                .group("Camera"),
            ShortcutBuilder::new(ShortcutId::CameraLeft)
                .key(KeyCode::KeyA)
                .description("Move Left")
                .in_game()
                .group("Camera"),
            ShortcutBuilder::new(ShortcutId::CameraRight)
                .key(KeyCode::KeyD)
                .description("Move Right")
                .in_game()
                .group("Camera"),
            ShortcutBuilder::new(ShortcutId::CameraZoomIn)
                .key(KeyCode::KeyQ)
                .description("Zoom In")
                .in_game()
                .group("Camera"),
            ShortcutBuilder::new(ShortcutId::CameraZoomOut)
                .key(KeyCode::KeyE)
                .description("Zoom Out")
                .in_game()
                .group("Camera"),
        ]);
        self
    }

    /// Add file operation shortcuts
    pub fn with_file_operations(mut self) -> Self {
        self.shortcuts.extend(vec![
            ShortcutBuilder::new(ShortcutId::SaveGame)
                .ctrl_key(KeyCode::KeyS)
                .description("Save Game")
                .in_game()
                .group("File"),
            ShortcutBuilder::new(ShortcutId::LoadGame)
                .ctrl_key(KeyCode::KeyL)
                .description("Load Game")
                .global()
                .group("File"),
            ShortcutBuilder::new(ShortcutId::QuickSave)
                .key(KeyCode::F5)
                .description("Quick Save")
                .in_game()
                .group("File"),
            ShortcutBuilder::new(ShortcutId::QuickLoad)
                .key(KeyCode::F9)
                .description("Quick Load")
                .global()
                .group("File"),
        ]);
        self
    }

    /// Add UI toggle shortcuts
    pub fn with_ui_toggles(mut self) -> Self {
        self.shortcuts.extend(vec![
            ShortcutBuilder::new(ShortcutId::ToggleBorders)
                .key(KeyCode::KeyB)
                .description("Toggle Borders")
                .in_game()
                .group("Display"),
            ShortcutBuilder::new(ShortcutId::ToggleGrid)
                .key(KeyCode::KeyG)
                .description("Toggle Grid")
                .in_game()
                .group("Display"),
            ShortcutBuilder::new(ShortcutId::ToggleHud)
                .key(KeyCode::KeyH)
                .description("Toggle HUD")
                .in_game()
                .group("Display"),
            ShortcutBuilder::new(ShortcutId::ToggleFps)
                .key(KeyCode::F3)
                .description("Toggle FPS")
                .global()
                .group("Display"),
        ]);
        self
    }

    /// Register all shortcuts with the registry
    pub fn register_all(self, registry: &mut ShortcutRegistry) {
        for shortcut in self.shortcuts {
            let _ = shortcut.register(registry);
        }
    }
}

/// Extension trait for Commands to work with shortcuts
pub trait ShortcutCommandsExt {
    /// Spawn an entity that shows shortcut hints
    fn spawn_shortcut_hint(&mut self, filter: ShortcutHintFilter) -> Entity;

    /// Start rebinding a shortcut
    fn start_rebinding(&mut self, entity: Entity, shortcut_id: ShortcutId);
}

impl ShortcutCommandsExt for Commands<'_, '_> {
    fn spawn_shortcut_hint(&mut self, filter: ShortcutHintFilter) -> Entity {
        self.spawn((
            Text::new(""),
            super::systems::ShortcutHint { filter },
        )).id()
    }

    fn start_rebinding(&mut self, entity: Entity, shortcut_id: ShortcutId) {
        self.entity(entity).insert(super::systems::RebindingShortcut {
            shortcut_id,
        });
    }
}