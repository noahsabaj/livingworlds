//! Tree layout algorithm for family tree visualization

use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::nations::{
    Character, CharacterId, CharacterRole, HasRelationship,
    RelationshipMetadata, RelationshipType,
};
use super::types::*;

/// Build a family tree layout for a house
pub fn build_tree_layout(
    house_entity: Entity,
    root_character: Option<Entity>,
    characters: &Query<(Entity, &Character)>,
    relationships: &Query<(&HasRelationship, &RelationshipMetadata)>,
    config: &TreeLayoutConfig,
) -> FamilyTreeLayout {
    let mut layout = FamilyTreeLayout {
        house_entity: Some(house_entity),
        ..default()
    };

    // Find all characters in this house
    let house_characters: Vec<(Entity, &Character)> = characters
        .iter()
        .filter(|(_, c)| c.house_id == house_entity)
        .collect();

    if house_characters.is_empty() {
        return layout;
    }

    // Determine root character (founder or specified character)
    let root_entity = root_character.unwrap_or_else(|| {
        // Find oldest character or ruler
        house_characters
            .iter()
            .max_by_key(|(_, c)| c.age)
            .map(|(e, _)| *e)
            .unwrap()
    });

    let root_char = house_characters
        .iter()
        .find(|(e, _)| *e == root_entity)
        .map(|(_, c)| *c);

    if let Some(root) = root_char {
        layout.root_character = Some(root.id);

        // Build relationship graph
        let rel_graph = build_relationship_graph(&house_characters, relationships);

        // Assign generations using BFS from root
        let generations = assign_generations(root.id, &rel_graph);

        // Create tree nodes
        for (entity, character) in &house_characters {
            let generation = generations.get(&character.id).copied().unwrap_or(0);
            layout.nodes.insert(
                character.id,
                TreeNode {
                    character_entity: *entity,
                    character_id: character.id,
                    name: character.name.clone(),
                    age: character.age,
                    role: character.role.clone(),
                    title: character.title.clone(),
                    is_alive: character.health > 0.0,
                    generation,
                    position: Vec2::ZERO, // Will be calculated
                },
            );
        }

        // Create tree edges
        for (from_char, relationships) in &rel_graph {
            for (to_char, rel_type) in relationships {
                // Only add parent-child and spouse edges for now
                match rel_type {
                    RelationshipType::Parent
                    | RelationshipType::Child
                    | RelationshipType::Spouse => {
                        layout.edges.push(TreeEdge {
                            from_id: *from_char,
                            to_id: *to_char,
                            relationship: rel_type.clone(),
                            visible: true,
                        });
                    }
                    _ => {}
                }
            }
        }

        // Calculate positions
        calculate_positions(&mut layout, config);
    }

    layout
}

/// Build a graph of relationships between characters
fn build_relationship_graph(
    characters: &[(Entity, &Character)],
    relationships: &Query<(&HasRelationship, &RelationshipMetadata)>,
) -> HashMap<CharacterId, Vec<(CharacterId, RelationshipType)>> {
    let mut graph: HashMap<CharacterId, Vec<(CharacterId, RelationshipType)>> = HashMap::new();

    for (entity, character) in characters {
        let char_id = character.id;
        graph.entry(char_id).or_default();

        // Get all relationships for this character
        if let Ok((has_rel, metadata)) = relationships.get(*entity) {
            // Find the target character
            if let Some((_, target_char)) = characters.iter().find(|(e, _)| *e == has_rel.0) {
                graph
                    .entry(char_id)
                    .or_default()
                    .push((target_char.id, metadata.relationship_type.clone()));
            }
        }
    }

    graph
}

/// Assign generation numbers using BFS from root
fn assign_generations(
    root: CharacterId,
    graph: &HashMap<CharacterId, Vec<(CharacterId, RelationshipType)>>,
) -> HashMap<CharacterId, u32> {
    let mut generations = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((root, 0u32));
    visited.insert(root);

    while let Some((char_id, generation)) = queue.pop_front() {
        generations.insert(char_id, generation);

        // Get children (relationships where type is Child)
        if let Some(relationships) = graph.get(&char_id) {
            for (target, rel_type) in relationships {
                if matches!(rel_type, RelationshipType::Child) && !visited.contains(target) {
                    visited.insert(*target);
                    queue.push_back((*target, generation + 1));
                }
            }
        }
    }

    generations
}

/// Calculate positions for all nodes in the tree
fn calculate_positions(layout: &mut FamilyTreeLayout, config: &TreeLayoutConfig) {
    let generation_count = layout.generation_count();

    // For each generation, arrange characters horizontally
    for generation_num in 0..generation_count {
        let mut nodes_in_gen: Vec<_> = layout
            .nodes
            .values_mut()
            .filter(|n| n.generation == generation_num)
            .collect();

        let node_count = nodes_in_gen.len();
        if node_count == 0 {
            continue;
        }

        // Calculate Y position (generation spacing)
        let y = (generation_num as f32) * (config.node_height + config.vertical_spacing);

        // Calculate X positions (spread horizontally, centered)
        let total_width = (node_count as f32) * config.node_width
            + ((node_count - 1) as f32) * config.horizontal_spacing;
        let start_x = -total_width / 2.0;

        for (i, node) in nodes_in_gen.iter_mut().enumerate() {
            let x = start_x + (i as f32) * (config.node_width + config.horizontal_spacing);
            node.position = Vec2::new(x, y);
        }
    }

    // Calculate bounds
    layout.bounds = calculate_bounds(layout);
}

/// Calculate the bounding box of all nodes
fn calculate_bounds(layout: &FamilyTreeLayout) -> TreeBounds {
    let mut bounds = TreeBounds::default();

    if layout.nodes.is_empty() {
        return bounds;
    }

    let positions: Vec<Vec2> = layout.nodes.values().map(|n| n.position).collect();

    bounds.min_x = positions.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
    bounds.max_x = positions.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
    bounds.min_y = positions.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
    bounds.max_y = positions.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);

    bounds
}

/// Trace bloodline ancestors and descendants
pub fn trace_bloodline(
    character_id: CharacterId,
    layout: &FamilyTreeLayout,
) -> (Vec<CharacterId>, Vec<CharacterId>) {
    let mut ancestors = Vec::new();
    let mut descendants = Vec::new();

    // Trace ancestors (parent relationships going backward)
    let mut queue = VecDeque::new();
    queue.push_back(character_id);
    let mut visited = HashSet::new();

    while let Some(id) = queue.pop_front() {
        if visited.contains(&id) {
            continue;
        }
        visited.insert(id);

        for edge in layout.edges_to(id) {
            if matches!(edge.relationship, RelationshipType::Parent) {
                ancestors.push(edge.from_id);
                queue.push_back(edge.from_id);
            }
        }
    }

    // Trace descendants (child relationships going forward)
    let mut queue = VecDeque::new();
    queue.push_back(character_id);
    let mut visited = HashSet::new();

    while let Some(id) = queue.pop_front() {
        if visited.contains(&id) {
            continue;
        }
        visited.insert(id);

        for edge in layout.edges_from(id) {
            if matches!(edge.relationship, RelationshipType::Child) {
                descendants.push(edge.to_id);
                queue.push_back(edge.to_id);
            }
        }
    }

    (ancestors, descendants)
}
