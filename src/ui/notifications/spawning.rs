//! Functions for spawning notification UI elements.

use bevy::prelude::*;
use bevy::ui::{PositionType, Val, UiRect, FlexDirection, AlignItems, JustifyContent};

use super::components::*;
use super::styles;
use super::types::*;

/// Spawn the persistent notification container
///
/// This container holds all active notifications and is always present.
pub fn spawn_notification_container(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            NotificationContainer,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                row_gap: styles::toast::STACK_GAP,
                padding: UiRect::top(Val::Px(16.0)),
                ..default()
            },
            ZIndex(styles::NOTIFICATION_Z_INDEX),
            // Don't despawn when changing states
        ))
        .id()
}

/// Spawn a toast notification
pub fn spawn_toast(
    commands: &mut Commands,
    container: Entity,
    notification: &ShowNotification,
    time: &Time,
) {
    let mut toast_entity = Entity::PLACEHOLDER;

    commands.entity(container).with_children(|parent| {
        toast_entity = parent
            .spawn((
                NotificationToast {
                    spawned_at: time.elapsed_secs_f64(),
                },
                Node {
                    max_width: styles::toast::MAX_WIDTH,
                    min_height: styles::toast::MIN_HEIGHT,
                    padding: styles::toast::PADDING,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: styles::toast::ICON_GAP,
                    ..default()
                },
                BorderRadius::all(Val::Px(8.0)),
                BackgroundColor(notification.notification_type.background_color()),
            ))
            .with_children(|toast| {
                // Icon
                toast.spawn((
                    Text::new(notification.notification_type.icon()),
                    TextFont {
                        font_size: styles::toast::ICON_SIZE,
                        ..default()
                    },
                    TextColor(notification.notification_type.text_color()),
                ));

                // Message
                toast.spawn((
                    Text::new(&notification.message),
                    TextFont {
                        font_size: styles::toast::TEXT_SIZE,
                        ..default()
                    },
                    TextColor(notification.notification_type.text_color()),
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                ));

                // Dismiss button (small X)
                let dismiss_target = toast_entity;
                toast
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(24.0),
                            height: Val::Px(24.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BorderRadius::all(Val::Px(12.0)),
                        BackgroundColor(Color::NONE),
                        NotificationDismissButton {
                            target: dismiss_target,
                        },
                    ))
                    .observe(on_dismiss_clicked)
                    .with_children(|button| {
                        button.spawn((
                            Text::new("✕"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(notification.notification_type.text_color()),
                        ));
                    });
            })
            .id();
    });

    // Add timer if duration specified
    if let Some(duration) = notification.duration {
        commands.entity(toast_entity).insert(ToastTimer::new(duration));
    }
}

/// Observer function for dismiss button clicks
fn on_dismiss_clicked(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    q: Query<&NotificationDismissButton>,
) {
    if let Ok(dismiss) = q.get(trigger.entity) {
        if let Ok(mut entity_commands) = commands.get_entity(dismiss.target) {
            entity_commands.despawn();
        }
    }
}

/// Spawn a persistent banner notification
pub fn spawn_banner(
    commands: &mut Commands,
    container: Entity,
    notification: &ShowNotification,
) {
    let mut banner_entity = Entity::PLACEHOLDER;

    commands.entity(container).with_children(|parent| {
        banner_entity = parent
            .spawn((
                NotificationBanner,
                Node {
                    width: Val::Percent(100.0),
                    height: styles::banner::HEIGHT,
                    padding: styles::banner::PADDING,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: styles::banner::ICON_GAP,
                    ..default()
                },
                BackgroundColor(notification.notification_type.background_color()),
            ))
            .with_children(|banner| {
                // Icon
                banner.spawn((
                    Text::new(notification.notification_type.icon()),
                    TextFont {
                        font_size: styles::banner::ICON_SIZE,
                        ..default()
                    },
                    TextColor(notification.notification_type.text_color()),
                ));

                // Message
                banner.spawn((
                    Text::new(&notification.message),
                    TextFont {
                        font_size: styles::banner::TEXT_SIZE,
                        ..default()
                    },
                    TextColor(notification.notification_type.text_color()),
                ));

                // Dismiss button
                let dismiss_target = banner_entity;
                banner
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BorderRadius::all(Val::Px(10.0)),
                        BackgroundColor(Color::NONE),
                        NotificationDismissButton {
                            target: dismiss_target,
                        },
                    ))
                    .observe(on_dismiss_clicked)
                    .with_children(|button| {
                        button.spawn((
                            Text::new("✕"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(notification.notification_type.text_color()),
                        ));
                    });
            })
            .id();
    });
}
