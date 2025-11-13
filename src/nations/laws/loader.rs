//! Data-driven law loading system
//!
//! This module provides hot-reloadable law definitions from RON files,
//! eliminating the need for hardcoded law data and enabling rapid iteration
//! and modding support.

use bevy::prelude::*;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

use super::types::{Law, LawId, LawCategory};

/// Asset type for law definition files
/// Wraps a Vec<Law> for proper asset handling
#[derive(Asset, TypePath, Debug)]
pub struct LawDefinitionAsset {
    /// The laws defined in this asset file
    pub laws: Vec<Law>,
}

/// Errors that can occur when loading law definitions
#[derive(Error, Debug)]
pub enum LawLoadError {
    #[error("Failed to parse RON: {0}")]
    RonError(#[from] ron::error::SpannedError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Duplicate law ID: {0:?}")]
    DuplicateId(LawId),

    #[error("Invalid law reference: {0:?}")]
    InvalidReference(LawId),
}

/// Asset loader for RON law definition files
#[derive(Default)]
pub struct LawDefinitionLoader;

impl AssetLoader for LawDefinitionLoader {
    type Asset = LawDefinitionAsset;
    type Settings = ();
    type Error = LawLoadError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let laws: Vec<Law> = ron::de::from_bytes(&bytes)?;

        // Validate law data
        validate_laws(&laws)?;

        Ok(LawDefinitionAsset { laws })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

/// Validates law definitions for internal consistency
fn validate_laws(laws: &[Law]) -> Result<(), LawLoadError> {
    let mut seen_ids = HashMap::new();

    // Check for duplicate IDs
    for law in laws {
        if let Some(prev_name) = seen_ids.insert(law.id, &law.name) {
            error!("Duplicate law ID {:?}: '{}' and '{}'", law.id, prev_name, law.name);
            return Err(LawLoadError::DuplicateId(law.id));
        }
    }

    // Validate law references (conflicts_with and prerequisites)
    let valid_ids: HashMap<LawId, ()> = laws.iter().map(|l| (l.id, ())).collect();

    for law in laws {
        // Check conflicts_with references
        for conflict_id in &law.conflicts_with {
            if !valid_ids.contains_key(conflict_id) && !is_known_external_law(*conflict_id) {
                warn!("Law '{}' references unknown conflict law {:?}", law.name, conflict_id);
            }
        }

        // Check prerequisite law references
        use super::types::LawPrerequisite;
        for prereq in &law.prerequisites {
            if let LawPrerequisite::RequiresLaw(required_id) = prereq {
                if !valid_ids.contains_key(required_id) && !is_known_external_law(*required_id) {
                    warn!("Law '{}' references unknown prerequisite law {:?}", law.name, required_id);
                }
            }
        }
    }

    Ok(())
}

/// Checks if a law ID is from another category (external to this file)
fn is_known_external_law(id: LawId) -> bool {
    // Laws are organized by ID ranges:
    // 1000-1999: Economic
    // 2000-2999: Military
    // 3000-3999: Social
    // etc.
    // This allows cross-category references
    id.0 < 10000 // All valid law IDs are below 10000
}

/// Resource that manages all loaded law definitions
#[derive(Resource, Default)]
pub struct LoadedLaws {
    /// All loaded laws indexed by ID
    pub by_id: HashMap<LawId, Law>,

    /// Laws organized by category
    pub by_category: HashMap<LawCategory, Vec<Law>>,

    /// Handles to loaded law assets for hot-reload tracking
    asset_handles: Vec<Handle<LawDefinitionAsset>>,
}

impl LoadedLaws {
    /// Get a law by its ID
    pub fn get(&self, id: LawId) -> Option<&Law> {
        self.by_id.get(&id)
    }

    /// Get all laws in a category
    pub fn get_category(&self, category: LawCategory) -> &[Law] {
        self.by_category.get(&category).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get all loaded laws
    pub fn all(&self) -> Vec<&Law> {
        self.by_id.values().collect()
    }

    /// Add laws from a loaded asset
    pub fn add_from_asset(&mut self, asset: &LawDefinitionAsset) {
        for law in &asset.laws {
            // Add to ID index
            self.by_id.insert(law.id, law.clone());

            // Add to category index
            self.by_category
                .entry(law.category)
                .or_insert_with(Vec::new)
                .push(law.clone());
        }

        info!("Loaded {} laws from asset", asset.laws.len());
    }

    /// Clear all loaded laws (used during hot-reload)
    pub fn clear(&mut self) {
        self.by_id.clear();
        self.by_category.clear();
        info!("Cleared all loaded laws for hot-reload");
    }
}

/// System that loads law assets at startup
pub fn load_law_assets(
    asset_server: Res<AssetServer>,
    mut loaded_laws: ResMut<LoadedLaws>,
) {
    // Load all law definition files
    let categories = [
        ("economic", "taxation"),
        ("economic", "trade"),
        ("economic", "labor"),
        ("military", "conscription"),
        ("military", "organization"),
        ("social", "education"),
        ("social", "healthcare"),
    ];

    for (category, subcategory) in categories {
        let path = format!("laws/{}/{}.ron", category, subcategory);
        let handle: Handle<LawDefinitionAsset> = asset_server.load(path.clone());
        loaded_laws.asset_handles.push(handle);
        info!("Queued law asset for loading: {}", path);
    }
}

/// System that processes loaded law assets
pub fn process_loaded_laws(
    mut loaded_laws: ResMut<LoadedLaws>,
    mut asset_events: MessageReader<AssetEvent<LawDefinitionAsset>>,
    law_assets: Res<Assets<LawDefinitionAsset>>,
) {
    for event in asset_events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => {
                if let Some(asset) = law_assets.get(*id) {
                    // Clear and reload on modification (hot-reload)
                    if matches!(event, AssetEvent::Modified { .. }) {
                        loaded_laws.clear();
                    }

                    loaded_laws.add_from_asset(asset);
                    info!("Processed law asset with {} laws", asset.laws.len());
                }
            }
            AssetEvent::Removed { .. } => {
                // Handle asset removal if needed
            }
            _ => {}
        }
    }
}

use bevy_plugin_builder::define_plugin;

/// Plugin that adds data-driven law loading
define_plugin!(LawLoaderPlugin {
    resources: [LoadedLaws],

    startup: [load_law_assets],

    update: [process_loaded_laws],

    custom_init: |app: &mut App| {
        app.init_asset::<LawDefinitionAsset>()
            .init_asset_loader::<LawDefinitionLoader>();
    }
});