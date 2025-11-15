//! Family tree viewer UI rendering

use bevy::prelude::*;
use crate::ui::*;
use crate::nations::CharacterRole;
use super::types::*;

/// Spawn the family tree viewer panel
pub fn spawn_family_tree_panel(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)), // Dark overlay
            FamilyTreePanel,
            Visibility::Hidden, // Start hidden
            ZIndex(1000), // On top of everything
        ))
        .with_children(|parent| {
            // Header
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("FAMILY TREE"),
                        TextFont {
                            font_size: TEXT_SIZE_TITLE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_HEADER),
                    ));

                    // Close button
                    ButtonBuilder::new("Close")
                        .size(ButtonSize::Small)
                        .with_marker(CloseTreeButton)
                        .build(parent);
                });

            // Filter controls
            spawn_tree_filters(parent);

            // Tree content area (scrollable)
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        overflow: Overflow::scroll(),
                        ..default()
                    },
                    TreeContentArea,
                ))
                .with_children(|parent| {
                    // Tree visualization container
                    parent.spawn((
                        Node {
                            width: Val::Px(2000.0), // Large enough for tree
                            height: Val::Px(2000.0),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        TreeVisualizationContainer,
                    ));
                });
        });
}

/// Spawn filter controls
fn spawn_tree_filters(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(12.0)),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Show:"),
                TextFont {
                    font_size: TEXT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
            ));

            // Filter checkboxes (simplified for now)
            for label in ["Blood", "Marriage", "Romance", "Political"] {
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(4.0),
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|parent| {
                        // Checkbox
                        parent.spawn((
                            Node {
                                width: Val::Px(16.0),
                                height: Val::Px(16.0),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(UI_BACKGROUND_COLOR),
                            BorderColor::all(UI_BORDER_COLOR),
                        ));

                        parent.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: TEXT_SIZE_NORMAL,
                                ..default()
                            },
                            TextColor(TEXT_COLOR_PRIMARY),
                        ));
                    });
            }
        });
}

/// Update tree visualization when layout changes
pub fn update_tree_visualization(
    mut commands: Commands,
    layout: Res<FamilyTreeLayout>,
    container: Query<Entity, With<TreeVisualizationContainer>>,
    existing_nodes: Query<Entity, With<TreeNodeUI>>,
    existing_lines: Query<Entity, With<RelationshipLine>>,
    highlight: Res<BloodlineHighlight>,
) {
    if !layout.is_changed() && !highlight.is_changed() {
        return;
    }

    let Ok(container_entity) = container.single() else {
        return;
    };

    // Despawn existing visualization
    for entity in existing_nodes.iter().chain(existing_lines.iter()) {
        commands.entity(entity).despawn();
    }

    // Draw relationship lines first (so they appear below nodes)
    commands.entity(container_entity).with_children(|parent| {
        for edge in &layout.edges {
            if !edge.visible {
                continue;
            }

            let from_node = layout.get_node(edge.from_id);
            let to_node = layout.get_node(edge.to_id);

            if let (Some(from), Some(to)) = (from_node, to_node) {
                draw_relationship_line(parent, from, to, &edge.relationship);
            }
        }

        // Draw nodes
        for node in layout.nodes.values() {
            let is_highlighted = highlight.is_highlighted(node.character_id);
            draw_character_node(parent, node, is_highlighted);
        }
    });
}

/// Draw a character node
fn draw_character_node(parent: &mut ChildBuilder, node: &TreeNode, is_highlighted: bool) {
    let base_color = if is_highlighted {
        Color::srgb(1.0, 0.84, 0.0) // Gold highlight
    } else if !node.is_alive {
        Color::srgb(0.4, 0.4, 0.4) // Gray for dead
    } else {
        UI_BACKGROUND_COLOR
    };

    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(node.position.x),
                top: Val::Px(node.position.y),
                width: Val::Px(80.0),
                height: Val::Px(100.0),
                padding: UiRect::all(Val::Px(6.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(base_color),
            BorderColor::all(if is_highlighted {
                Color::srgb(1.0, 0.84, 0.0)
            } else {
                UI_BORDER_COLOR
            }),
            TreeNodeUI {
                character_id: node.character_id,
            },
            Interaction::default(),
        ))
        .with_children(|parent| {
            // Role icon
            let icon = get_role_icon(&node.role);
            parent.spawn((
                Text::new(icon),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
            ));

            // Name
            parent.spawn((
                Text::new(truncate_name(&node.name, 10)),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(TEXT_COLOR_PRIMARY),
            ));

            // Age
            let age_text = if node.is_alive {
                format!("{}y", node.age)
            } else {
                format!("†{}", node.age)
            };
            parent.spawn((
                Text::new(age_text),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(TEXT_COLOR_SECONDARY),
            ));

            // Title
            if let Some(title) = &node.title {
                parent.spawn((
                    Text::new(truncate_name(title, 8)),
                    TextFont {
                        font_size: 10.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR_SECONDARY),
                ));
            }
        });
}

/// Draw a relationship line between two nodes
fn draw_relationship_line(
    parent: &mut ChildBuilder,
    from: &TreeNode,
    to: &TreeNode,
    relationship: &crate::nations::RelationshipType,
) {
    use crate::nations::RelationshipType;

    let color = match relationship {
        RelationshipType::Parent | RelationshipType::Child => Color::srgb(0.8, 0.8, 0.8),
        RelationshipType::Spouse => Color::srgb(1.0, 0.5, 0.5), // Pink
        _ => Color::srgb(0.5, 0.5, 0.5),
    };

    // Simple vertical line for parent-child
    if matches!(relationship, RelationshipType::Parent | RelationshipType::Child) {
        let from_center = from.position + Vec2::new(40.0, 100.0);
        let to_center = to.position + Vec2::new(40.0, 0.0);

        // Vertical line
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(from_center.x - 1.0),
                top: Val::Px(from_center.y),
                width: Val::Px(2.0),
                height: Val::Px(to_center.y - from_center.y),
                ..default()
            },
            BackgroundColor(color),
            RelationshipLine {
                from_id: from.character_id,
                to_id: to.character_id,
                relationship: relationship.clone(),
            },
        ));
    }
    // Horizontal line for spouses (same generation)
    else if matches!(relationship, RelationshipType::Spouse) {
        let from_center = from.position + Vec2::new(80.0, 50.0);
        let to_center = to.position + Vec2::new(0.0, 50.0);

        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(from_center.x),
                top: Val::Px(from_center.y - 1.0),
                width: Val::Px((to_center.x - from_center.x).abs()),
                height: Val::Px(2.0),
                ..default()
            },
            BackgroundColor(color),
            RelationshipLine {
                from_id: from.character_id,
                to_id: to.character_id,
                relationship: relationship.clone(),
            },
        ));
    }
}

/// Get icon for character role
fn get_role_icon(role: &CharacterRole) -> &'static str {
    match role {
        CharacterRole::Ruler => "♔", // Crown
        CharacterRole::Heir => "♕", // Crown outline
        CharacterRole::Spouse => "♡", // Heart
        CharacterRole::Child => "◆", // Diamond
        CharacterRole::Sibling => "◇", // Diamond outline
        CharacterRole::Cousin => "○", // Circle
        CharacterRole::Advisor => "☆", // Star
        CharacterRole::General => "⚔", // Swords
        CharacterRole::Courtier => "◌", // Small circle
        CharacterRole::Bastard => "✦", // Star burst (special)
    }
}

/// Truncate name to max length
fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[..max_len - 3])
    }
}

/// Marker for close button
#[derive(Component)]
pub struct CloseTreeButton;

/// Marker for tree content area
#[derive(Component)]
pub struct TreeContentArea;

/// Marker for tree visualization container
#[derive(Component)]
pub struct TreeVisualizationContainer;
