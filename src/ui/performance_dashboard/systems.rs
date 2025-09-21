//! Performance Dashboard Update Systems

use super::super::ChildBuilder;
use super::types::*;
use crate::performance::RayonMetrics;
use crate::ui::styles::colors;
use bevy::prelude::*;

/// Toggle dashboard visibility with F12 key
pub fn toggle_dashboard_visibility(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut visibility: ResMut<DashboardVisibility>,
    mut panel_query: Query<&mut Visibility, With<PerformancePanel>>,
) {
    if keyboard.just_pressed(KeyCode::F12) {
        visibility.toggle();

        // Update panel visibility
        if let Ok(mut panel_vis) = panel_query.single_mut() {
            *panel_vis = if visibility.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }

        info!(
            "Performance dashboard {}",
            if visibility.visible {
                "shown"
            } else {
                "hidden"
            }
        );
    }
}

/// Update thread utilization display
pub fn update_thread_utilization(
    metrics: Res<RayonMetrics>,
    mut query: Query<&Children, With<ThreadUtilizationDisplay>>,
    mut text_query: Query<&mut Text>,
    mut node_query: Query<&mut Node>,
) {
    let Ok(children) = query.single() else { return };

    let thread_count = rayon::current_num_threads();
    let utilization = metrics.avg_thread_utilization;

    // Update the fill bar width
    for child in children.iter() {
        if let Ok(grand_children) = query.get(child) {
            for grand_child in grand_children.iter() {
                if let Ok(mut node) = node_query.get_mut(grand_child) {
                    node.width = Val::Percent(utilization);
                }
            }
        }
    }

    // Update the text
    for child in children.iter() {
        if let Ok(mut text) = text_query.get_mut(child) {
            if text.0.contains("threads") {
                let avg_used = (thread_count as f32 * utilization / 100.0) as usize;
                text.0 = format!(
                    "{}/{} threads ({:.1}%)",
                    avg_used, thread_count, utilization
                );
            }
        }
    }
}

/// Update metrics summary display
pub fn update_metrics_summary(
    metrics: Res<RayonMetrics>,
    query: Query<&Children, With<MetricsSummaryDisplay>>,
    mut text_query: Query<&mut Text>,
) {
    let Ok(children) = query.get_single() else {
        return;
    };

    let summary = metrics.get_summary();

    // Find the grid container and update metric values
    for child in children.iter() {
        if let Ok(grid_children) = query.get(child) {
            let mut metric_index = 0;
            for metric_child in grid_children.iter() {
                if let Ok(mut text) = text_query.get_mut(metric_child) {
                    // Update values based on position in grid
                    match metric_index {
                        1 => text.0 = format!("{}", summary.total_operations),
                        3 => text.0 = format!("{:.2}ms", summary.average_duration_ms),
                        5 => text.0 = format!("{:.1}s", summary.total_parallel_time_ms / 1000.0),
                        7 => {
                            // Calculate average throughput
                            let avg_throughput = if !metrics.recent_operations.is_empty() {
                                metrics
                                    .recent_operations
                                    .iter()
                                    .map(|op| op.throughput_per_sec)
                                    .sum::<f32>()
                                    / metrics.recent_operations.len() as f32
                            } else {
                                0.0
                            };
                            text.0 = format!("{:.0}/sec", avg_throughput);
                        }
                        _ => {}
                    }
                    metric_index += 1;
                }
            }
        }
    }
}

/// Update recent operations list
pub fn refresh_operations_list(
    metrics: Res<RayonMetrics>,
    dashboard_vis: Res<DashboardVisibility>,
    mut commands: Commands,
    query: Query<(Entity, &Children), With<OperationsListDisplay>>,
    existing_ops: Query<Entity, With<OperationListItem>>,
) {
    let Ok((list_entity, children)) = query.get_single() else {
        return;
    };

    // Clear old operation items (skip the title)
    let mut items_to_remove = Vec::new();
    for child in children.iter().skip(1) {
        if existing_ops.contains(child) {
            items_to_remove.push(child);
        }
    }
    for entity in items_to_remove {
        commands.entity(entity).despawn();
    }

    // Add new operation items
    let ops_to_show = dashboard_vis
        .max_operations_shown
        .min(metrics.recent_operations.len());
    let start_idx = metrics.recent_operations.len().saturating_sub(ops_to_show);

    commands.entity(list_entity).with_children(|parent| {
        for operation in metrics.recent_operations.iter().skip(start_idx) {
            spawn_operation_item(parent, operation);
        }
    });
}

/// Marker component for operation list items
#[derive(Component)]
pub struct OperationListItem;

/// Helper to spawn an operation item
fn spawn_operation_item(
    parent: &mut ChildBuilder,
    operation: &crate::performance::ParallelOpMetric,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(4.0)),
                margin: UiRect::top(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARK.with_alpha(0.5)),
            BorderRadius::all(Val::Px(2.0)),
            OperationListItem,
        ))
        .with_children(|item| {
            // Operation name
            item.spawn((
                Text::new(operation.operation_name.clone()),
                TextColor(colors::TEXT_PRIMARY),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
            ));

            // Stats container
            item.spawn((Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                ..default()
            },))
                .with_children(|stats| {
                    // Duration
                    stats.spawn((
                        Text::new(format!("{:.1}ms", operation.duration_ms)),
                        TextColor(get_duration_color(operation.duration_ms)),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                    ));

                    // Item count
                    stats.spawn((
                        Text::new(format!("{}items", format_count(operation.items_processed))),
                        TextColor(colors::TEXT_SECONDARY),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                    ));

                    // Thread count
                    stats.spawn((
                        Text::new(format!("{}t", operation.thread_count)),
                        TextColor(colors::TEXT_SECONDARY),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                    ));
                });
        });
}

/// Get color based on operation duration
fn get_duration_color(duration_ms: f32) -> Color {
    if duration_ms < 10.0 {
        colors::SUCCESS // Green for fast
    } else if duration_ms < 50.0 {
        colors::WARNING // Yellow for medium
    } else {
        colors::DANGER // Red for slow
    }
}

/// Format large counts with K/M suffix
fn format_count(count: usize) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f32 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f32 / 1_000.0)
    } else {
        count.to_string()
    }
}
