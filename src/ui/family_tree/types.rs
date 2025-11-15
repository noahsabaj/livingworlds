//! Data types for family tree visualization

use bevy::prelude::*;
use crate::nations::{Character, CharacterId, CharacterRole, RelationshipType};
use std::collections::HashMap;

/// A node in the family tree representing a character
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub character_entity: Entity,
    pub character_id: CharacterId,
    pub name: String,
    pub age: u32,
    pub role: CharacterRole,
    pub title: Option<String>,
    pub is_alive: bool,
    pub generation: u32,
    pub position: Vec2, // Final position in tree layout
}

/// An edge connecting two characters in the tree
#[derive(Debug, Clone)]
pub struct TreeEdge {
    pub from_id: CharacterId,
    pub to_id: CharacterId,
    pub relationship: RelationshipType,
    pub visible: bool, // Can be toggled by filters
}

/// Complete family tree layout for a house
#[derive(Resource, Default)]
pub struct FamilyTreeLayout {
    pub house_entity: Option<Entity>,
    pub nodes: HashMap<CharacterId, TreeNode>,
    pub edges: Vec<TreeEdge>,
    pub root_character: Option<CharacterId>, // Character tree is centered on
    pub bounds: TreeBounds,
}

impl FamilyTreeLayout {
    /// Get a node by character ID
    pub fn get_node(&self, id: CharacterId) -> Option<&TreeNode> {
        self.nodes.get(&id)
    }

    /// Get all nodes in a specific generation
    pub fn nodes_in_generation(&self, generation: u32) -> Vec<&TreeNode> {
        self.nodes
            .values()
            .filter(|n| n.generation == generation)
            .collect()
    }

    /// Get the number of generations
    pub fn generation_count(&self) -> u32 {
        self.nodes
            .values()
            .map(|n| n.generation)
            .max()
            .unwrap_or(0)
            + 1
    }

    /// Get edges from a character
    pub fn edges_from(&self, id: CharacterId) -> Vec<&TreeEdge> {
        self.edges
            .iter()
            .filter(|e| e.from_id == id && e.visible)
            .collect()
    }

    /// Get edges to a character
    pub fn edges_to(&self, id: CharacterId) -> Vec<&TreeEdge> {
        self.edges
            .iter()
            .filter(|e| e.to_id == id && e.visible)
            .collect()
    }
}

/// Bounds of the tree layout
#[derive(Debug, Clone, Default)]
pub struct TreeBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl TreeBounds {
    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }
}

/// Tree layout configuration constants
pub struct TreeLayoutConfig {
    pub node_width: f32,
    pub node_height: f32,
    pub horizontal_spacing: f32,
    pub vertical_spacing: f32,
}

impl Default for TreeLayoutConfig {
    fn default() -> Self {
        Self {
            node_width: 80.0,
            node_height: 100.0,
            horizontal_spacing: 40.0,
            vertical_spacing: 60.0,
        }
    }
}

/// Filter state for tree relationships
#[derive(Resource, Default, Clone)]
pub struct TreeRelationshipFilters {
    pub show_blood: bool,     // Parent/Child/Sibling
    pub show_marriage: bool,  // Spouse/ExSpouse
    pub show_romance: bool,   // Lover/SecretLover
    pub show_political: bool, // Ally/Conspirator etc
}

impl TreeRelationshipFilters {
    pub fn all_visible() -> Self {
        Self {
            show_blood: true,
            show_marriage: true,
            show_romance: true,
            show_political: true,
        }
    }

    pub fn should_show(&self, rel: &RelationshipType) -> bool {
        match rel {
            RelationshipType::Parent
            | RelationshipType::Child
            | RelationshipType::Sibling => self.show_blood,
            RelationshipType::Spouse | RelationshipType::ExSpouse => self.show_marriage,
            RelationshipType::Lover
            | RelationshipType::SecretLover
            | RelationshipType::Betrothed
            | RelationshipType::Crush => self.show_romance,
            RelationshipType::Ally
            | RelationshipType::Conspirator
            | RelationshipType::Blackmailer
            | RelationshipType::Puppet
            | RelationshipType::PuppetMaster => self.show_political,
            _ => true, // BestFriend, Friend, Rival, etc - always show
        }
    }
}

/// Marker for family tree panel root
#[derive(Component)]
pub struct FamilyTreePanel;

/// Marker for tree node UI element
#[derive(Component)]
pub struct TreeNodeUI {
    pub character_id: CharacterId,
}

/// Marker for relationship line UI element
#[derive(Component)]
pub struct RelationshipLine {
    pub from_id: CharacterId,
    pub to_id: CharacterId,
    pub relationship: RelationshipType,
}

/// State for bloodline highlighting
#[derive(Resource, Default)]
pub struct BloodlineHighlight {
    pub focused_character: Option<CharacterId>,
    pub ancestors: Vec<CharacterId>,
    pub descendants: Vec<CharacterId>,
}

impl BloodlineHighlight {
    pub fn is_highlighted(&self, id: CharacterId) -> bool {
        self.focused_character == Some(id)
            || self.ancestors.contains(&id)
            || self.descendants.contains(&id)
    }

    pub fn clear(&mut self) {
        self.focused_character = None;
        self.ancestors.clear();
        self.descendants.clear();
    }
}
