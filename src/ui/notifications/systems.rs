//! Systems for handling notification events and updates.

use bevy::prelude::*;

use super::components::*;
use super::spawning::*;
use super::types::*;

/// Handle ShowNotification events and spawn appropriate UI
pub fn handle_notification_events(
    mut commands: Commands,
    mut events: MessageReader<ShowNotification>,
    container_query: Query<Entity, With<NotificationContainer>>,
    time: Res<Time>,
) {
    // Get or create notification container
    let container = match container_query.iter().next() {
        Some(entity) => entity,
        None => {
            // Container doesn't exist yet, create it
            spawn_notification_container(&mut commands)
        }
    };

    // Process each notification event
    for notification in events.read() {
        match notification.position {
            NotificationPosition::TopCenter | NotificationPosition::BottomRight => {
                spawn_toast(&mut commands, container, notification, &time);
            }
            NotificationPosition::Banner => {
                spawn_banner(&mut commands, container, notification);
            }
        }
    }
}

/// Update timers and auto-dismiss expired toasts
pub fn update_toast_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ToastTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            // Timer expired, despawn the toast
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
    }
}

/// Clean up notification container when there are no children
pub fn cleanup_empty_container(
    mut commands: Commands,
    query: Query<(Entity, &Children), With<NotificationContainer>>,
) {
    for (entity, children) in &query {
        if children.is_empty() {
            // All notifications dismissed, but keep the container for future use
            // No cleanup needed - container persists
        }
    }
}

/// System to initialize the notification container on app startup
pub fn setup_notification_container(mut commands: Commands) {
    spawn_notification_container(&mut commands);
}
