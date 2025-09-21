//! Core types for pressure-driven emergence system
//!
//! Pressure represents internal and external forces that drive nation behavior.
//! When pressures exceed thresholds, nations take actions to relieve them.

use bevy::prelude::*;

/// Different types of pressure that nations experience
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureType {
    /// Too many people for available resources
    PopulationOvercrowding,
    /// Not enough people to maintain infrastructure
    PopulationUnderpopulation,
    /// Economic strain from various sources
    EconomicStrain,
    /// Military threats or weakness
    MilitaryVulnerability,
    /// Ruler losing support
    LegitimacyCrisis,
    /// Cultural tensions within nation
    CulturalDivision,
    /// Religious conflicts
    ReligiousConflict,
    /// Infrastructure decay
    InfrastructureCollapse,
}

/// Intensity level of a pressure
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PressureLevel(pub f32);

impl PressureLevel {
    pub const NONE: Self = Self(0.0);
    pub const LOW: Self = Self(0.25);
    pub const MODERATE: Self = Self(0.5);
    pub const HIGH: Self = Self(0.75);
    pub const CRITICAL: Self = Self(1.0);

    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn is_critical(&self) -> bool {
        self.0 >= 0.9
    }

    pub fn is_high(&self) -> bool {
        self.0 >= 0.7
    }

    pub fn is_moderate(&self) -> bool {
        self.0 >= 0.4
    }

    pub fn is_low(&self) -> bool {
        self.0 >= 0.2
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// A vector of all pressures affecting a nation
#[derive(Component, Debug, Clone)]
pub struct PressureVector {
    /// Map of pressure types to their current levels
    pub pressures: std::collections::HashMap<PressureType, PressureLevel>,
    /// Time since last pressure resolution
    pub time_since_resolution: f32,
    /// Historical pressure trends for decision-making
    pub trends: std::collections::VecDeque<f32>,
}

impl Default for PressureVector {
    fn default() -> Self {
        Self {
            pressures: std::collections::HashMap::new(),
            time_since_resolution: 0.0,
            trends: std::collections::VecDeque::with_capacity(10),
        }
    }
}

impl PressureVector {
    /// Get the highest pressure currently affecting the nation
    pub fn highest_pressure(&self) -> Option<(PressureType, PressureLevel)> {
        self.pressures
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(&t, &l)| (t, l))
    }

    /// Check if any pressure is at critical levels
    pub fn has_critical_pressure(&self) -> bool {
        self.pressures.values().any(|p| p.is_critical())
    }

    /// Get total pressure magnitude (for AI decision urgency)
    pub fn total_magnitude(&self) -> f32 {
        self.pressures.values().map(|p| p.value()).sum()
    }

    /// Update a specific pressure type
    pub fn set_pressure(&mut self, pressure_type: PressureType, level: PressureLevel) {
        self.pressures.insert(pressure_type, level);
    }

    /// Record current total for trend analysis
    pub fn record_trend(&mut self) {
        let total = self.total_magnitude();
        self.trends.push_back(total);
        if self.trends.len() > 10 {
            self.trends.pop_front();
        }
    }

    /// Check if pressures are increasing over time
    pub fn is_worsening(&self) -> bool {
        if self.trends.len() < 2 {
            return false;
        }
        let recent_avg = self.trends.iter().rev().take(3).sum::<f32>() / 3.0;
        let older_avg = self.trends.iter().take(3).sum::<f32>() / 3.0;
        recent_avg > older_avg * 1.1 // 10% worse
    }
}

/// Threshold at which a nation must act on a pressure
#[derive(Debug, Clone, Copy)]
pub struct PressureThreshold {
    /// Base threshold before modifiers
    pub base: f32,
    /// Modifier based on ruler personality
    pub personality_modifier: f32,
    /// Modifier based on government type
    pub government_modifier: f32,
    /// Modifier based on stability
    pub stability_modifier: f32,
}

impl PressureThreshold {
    pub fn new(base: f32) -> Self {
        Self {
            base,
            personality_modifier: 0.0,
            government_modifier: 0.0,
            stability_modifier: 0.0,
        }
    }

    /// Calculate the effective threshold
    pub fn effective_threshold(&self) -> f32 {
        (self.base + self.personality_modifier + self.government_modifier + self.stability_modifier)
            .clamp(0.1, 0.95)
    }

    /// Check if a pressure level exceeds this threshold
    pub fn is_exceeded_by(&self, level: PressureLevel) -> bool {
        level.value() > self.effective_threshold()
    }
}
