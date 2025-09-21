//! Performance Dashboard UI Setup

use super::super::ChildBuilder;
use super::types::*;
use crate::ui::styles::{colors, dimensions};
use bevy::prelude::*;

/// Setup the performance dashboard UI
pub fn setup_performance_dashboard(mut commands: Commands) {
    // Main dashboard container (top-right corner)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(60.0), // Below speed control
                right: Val::Px(10.0),
                width: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
                row_gap: Val::Px(dimensions::PADDING_SMALL),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARK.with_alpha(0.95)),
            BorderRadius::all(Val::Px(4.0)),
            Visibility::Hidden, // Start hidden
            PerformancePanel,
        ))
        .with_children(|parent| {
            // Title bar
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(Val::Px(dimensions::PADDING_SMALL)),
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_MEDIUM),
                    BorderRadius::top(Val::Px(4.0)),
                ))
                .with_children(|title_bar| {
                    // Title text
                    title_bar.spawn((
                        Text::new("Rayon Performance"),
                        TextColor(colors::TEXT_PRIMARY),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_MEDIUM,
                            ..default()
                        },
                    ));

                    // Close button (X)
                    title_bar
                        .spawn((
                            Node {
                                width: Val::Px(20.0),
                                height: Val::Px(20.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            Button,
                            BackgroundColor(colors::SECONDARY),
                            BorderRadius::all(Val::Px(2.0)),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("×"),
                                TextColor(colors::TEXT_SECONDARY),
                                TextFont {
                                    font_size: dimensions::FONT_SIZE_LARGE,
                                    ..default()
                                },
                            ));
                        });
                });

            // Thread utilization section
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    BackgroundColor(colors::SURFACE),
                    ThreadUtilizationDisplay,
                ))
                .with_children(|section| {
                    // Section title
                    section.spawn((
                        Text::new("Thread Utilization"),
                        TextColor(colors::TEXT_SECONDARY),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                    ));

                    // Utilization bar container
                    section
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(20.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(colors::BACKGROUND_DARK),
                            BorderRadius::all(Val::Px(2.0)),
                        ))
                        .with_children(|bar| {
                            // Fill bar (will be updated dynamically)
                            bar.spawn((
                                Node {
                                    width: Val::Percent(0.0), // Updated by system
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(colors::SUCCESS),
                                BorderRadius::all(Val::Px(2.0)),
                            ));
                        });

                    // Thread count text
                    section.spawn((
                        Text::new("0/0 threads (0%)"),
                        TextColor(colors::TEXT_PRIMARY),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                    ));
                });

            // Metrics summary section
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    BackgroundColor(colors::SURFACE),
                    MetricsSummaryDisplay,
                ))
                .with_children(|section| {
                    // Section title
                    section.spawn((
                        Text::new("Performance Summary"),
                        TextColor(colors::TEXT_SECONDARY),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                    ));

                    // Metrics grid
                    section
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                            column_gap: Val::Px(dimensions::PADDING_MEDIUM),
                            row_gap: Val::Px(dimensions::PADDING_SMALL),
                            ..default()
                        },))
                        .with_children(|grid| {
                            // Total operations
                            spawn_metric_item(grid, "Total Ops:", "0");
                            // Average duration
                            spawn_metric_item(grid, "Avg Duration:", "0.0ms");
                            // Total time
                            spawn_metric_item(grid, "Total Time:", "0.0s");
                            // Throughput
                            spawn_metric_item(grid, "Throughput:", "0/sec");
                        });
                });

            // Recent operations section
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        max_height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(dimensions::PADDING_SMALL)),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(colors::SURFACE),
                    OperationsListDisplay,
                ))
                .with_children(|section| {
                    // Section title
                    section.spawn((
                        Text::new("Recent Operations"),
                        TextColor(colors::TEXT_SECONDARY),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                    ));

                    // Operations will be added dynamically
                });
        });
}

/// Helper to spawn a metric item in the grid
fn spawn_metric_item(parent: &mut ChildBuilder, label: &str, value: &str) {
    // Label
    parent.spawn((
        Text::new(label),
        TextColor(colors::TEXT_SECONDARY),
        TextFont {
            font_size: dimensions::FONT_SIZE_SMALL,
            ..default()
        },
    ));

    // Value
    parent.spawn((
        Text::new(value),
        TextColor(colors::TEXT_PRIMARY),
        TextFont {
            font_size: dimensions::FONT_SIZE_SMALL,
            ..default()
        },
    ));
}
