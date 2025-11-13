//! UI component markers for world configuration
//!
//! This module contains all the marker components used to identify
//! UI elements in the world configuration screen.

use super::types::*;
use crate::resources::WorldSize;
use bevy::prelude::*;

// Root markers
#[derive(Component)]
pub struct AdvancedPanel;

#[derive(Component)]
pub struct AdvancedToggle;

#[derive(Component)]
pub struct AdvancedToggleText;

#[derive(Component)]
pub struct AdvancedToggleChevron;

// Input markers
#[derive(Component)]
pub struct WorldNameInput;

#[derive(Component)]
pub struct WorldNameText;

#[derive(Component)]
pub struct SeedInput;

#[derive(Component)]
pub struct SeedText;

// Button markers
#[derive(Component)]
pub struct GenerateButton;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct RandomNameButton;

#[derive(Component)]
pub struct RandomSeedButton;

// Selection button markers
#[derive(Component)]
pub struct PresetButton(pub WorldPreset);

#[derive(Component)]
pub struct SizeButton(pub WorldSize);

#[derive(Component)]
pub struct ClimateButton(pub ClimateType);

#[derive(Component)]
pub struct IslandButton(pub IslandFrequency);

#[derive(Component)]
pub struct AggressionButton(pub AggressionLevel);

#[derive(Component)]
pub struct ResourceButton(pub ResourceAbundance);

// Display text markers
#[derive(Component)]
pub struct WorldPreviewText;

#[derive(Component)]
pub struct PresetDescriptionText;

#[derive(Component)]
pub struct PresetDescription(pub String);

#[derive(Component)]
pub struct GenerationTimeEstimate;

// Slider markers
#[derive(Component)]
pub struct ContinentSlider;

#[derive(Component)]
pub struct ContinentValueText;

#[derive(Component)]
pub struct OceanSlider;

#[derive(Component)]
pub struct OceanValueText;

#[derive(Component)]
pub struct RiverSlider;

#[derive(Component)]
pub struct RiverValueText;

#[derive(Component)]
pub struct StartingNationsSlider;

#[derive(Component)]
pub struct StartingNationsValueText;

#[derive(Component)]
pub struct TechSpeedSlider;

#[derive(Component)]
pub struct TechSpeedValueText;

// Trait for selection components
pub trait SelectionComponent {
    type Value: Clone + PartialEq + Send + Sync + 'static;
    fn value(&self) -> Self::Value;
}

impl SelectionComponent for ClimateButton {
    type Value = ClimateType;
    fn value(&self) -> Self::Value {
        self.0
    }
}

impl SelectionComponent for IslandButton {
    type Value = IslandFrequency;
    fn value(&self) -> Self::Value {
        self.0
    }
}

impl SelectionComponent for AggressionButton {
    type Value = AggressionLevel;
    fn value(&self) -> Self::Value {
        self.0
    }
}

impl SelectionComponent for ResourceButton {
    type Value = ResourceAbundance;
    fn value(&self) -> Self::Value {
        self.0
    }
}

impl SelectionComponent for PresetButton {
    type Value = WorldPreset;
    fn value(&self) -> Self::Value {
        self.0
    }
}

impl SelectionComponent for SizeButton {
    type Value = WorldSize;
    fn value(&self) -> Self::Value {
        self.0
    }
}
