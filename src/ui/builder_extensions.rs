//! Extensions for bevy-ui-builders to maintain Living Worlds compatibility
//!
//! This module provides helper functions to add the with_marker functionality
//! that Living Worlds used extensively but isn't in the published crate.

use bevy::prelude::*;
use bevy_ui_builders::{ButtonBuilder as ExternalButtonBuilder, ButtonStyle, ButtonSize};

/// Extension trait for ButtonBuilder to add marker components
pub trait ButtonBuilderExt {
    fn build_with_marker<M: Component>(self, parent: &mut ChildSpawnerCommands, marker: M) -> Entity;
}

impl ButtonBuilderExt for ExternalButtonBuilder {
    fn build_with_marker<M: Component>(self, parent: &mut ChildSpawnerCommands, marker: M) -> Entity {
        let entity = self.build(parent);
        parent.commands().entity(entity).insert(marker);
        entity
    }
}

/// Wrapper for ButtonBuilder that supports with_marker pattern
pub struct ButtonBuilderWrapper {
    inner: ExternalButtonBuilder,
    marker: Option<Box<dyn FnOnce(&mut EntityCommands)>>,
}

impl ButtonBuilderWrapper {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            inner: ExternalButtonBuilder::new(text),
            marker: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.inner = self.inner.style(style);
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    pub fn with_marker<M: Component + 'static>(mut self, marker: M) -> Self {
        self.marker = Some(Box::new(move |commands| {
            commands.insert(marker);
        }));
        self
    }

    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let entity = self.inner.build(parent);
        if let Some(add_marker) = self.marker {
            let mut commands = parent.commands().entity(entity);
            add_marker(&mut commands);
        }
        entity
    }
}

// Re-export as ButtonBuilder for compatibility
pub type ButtonBuilder = ButtonBuilderWrapper;