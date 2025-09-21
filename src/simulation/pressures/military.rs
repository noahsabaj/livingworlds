//! Military pressure calculations
//!
//! Military pressure drives army building, fortification, and defensive alliances.

use super::types::PressureLevel;
use crate::nations::Nation;

/// Military pressures affecting a nation
#[derive(Debug, Clone)]
pub struct MilitaryPressure {
    /// Threat from stronger neighbors
    pub external_threat: PressureLevel,
    /// Internal military weakness
    pub military_weakness: PressureLevel,
    /// Border vulnerability
    pub border_exposure: PressureLevel,
    /// Recent military defeats
    pub defeat_trauma: PressureLevel,
}

/// Calculate military pressures for a nation
pub fn calculate_military_pressure(
    nation: &Nation,
    neighbor_strengths: &[f32],
    province_count: usize,
    recent_defeats: u32,
) -> MilitaryPressure {
    // External threat from stronger neighbors
    let avg_neighbor_strength = if !neighbor_strengths.is_empty() {
        neighbor_strengths.iter().sum::<f32>() / neighbor_strengths.len() as f32
    } else {
        0.0
    };

    let strength_ratio = nation.military_strength / avg_neighbor_strength.max(1.0);
    let external_threat = if strength_ratio < 1.0 {
        PressureLevel::new(1.0 - strength_ratio)
    } else {
        PressureLevel::NONE
    };

    // Military weakness relative to territory size
    let needed_strength = province_count as f32 * 100.0; // 100 strength per province ideal
    let weakness_ratio = nation.military_strength / needed_strength.max(1.0);
    let military_weakness = if weakness_ratio < 1.0 {
        PressureLevel::new(1.0 - weakness_ratio)
    } else {
        PressureLevel::NONE
    };

    // Border exposure (simplified - will expand with actual border calculation)
    let border_ratio = province_count as f32 * 0.3; // Assume 30% are border provinces
    let defender_ratio = nation.military_strength / (border_ratio * 50.0).max(1.0);
    let border_exposure = if defender_ratio < 1.0 {
        PressureLevel::new(1.0 - defender_ratio)
    } else {
        PressureLevel::NONE
    };

    // Recent defeats create lasting pressure
    let defeat_trauma = if recent_defeats > 0 {
        PressureLevel::new((recent_defeats as f32 / 3.0).min(1.0)) // Max trauma at 3 defeats
    } else {
        PressureLevel::NONE
    };

    MilitaryPressure {
        external_threat,
        military_weakness,
        border_exposure,
        defeat_trauma,
    }
}

/// Military actions to address pressures
#[derive(Debug, Clone)]
pub enum MilitaryAction {
    /// Build up military forces
    RecruitArmy { urgency: f32 },
    /// Build fortifications
    Fortify { priority_borders: Vec<u32> },
    /// Seek defensive alliances
    SeekAlliance { desperation: f32 },
    /// Preemptive strike against threat
    PreemptiveStrike { target: u32 },
    /// Military reforms to improve effectiveness
    MilitaryReform { focus: ReformFocus },
}

#[derive(Debug, Clone)]
pub enum ReformFocus {
    Training,
    Equipment,
    Organization,
    Tactics,
}

/// Determine military action based on pressures
pub fn resolve_military_pressure(pressure: &MilitaryPressure) -> Option<MilitaryAction> {
    // Critical external threat - immediate response
    if pressure.external_threat.is_critical() {
        // Desperate times - consider preemptive strike or alliance
        if pressure.military_weakness.is_high() {
            return Some(MilitaryAction::SeekAlliance {
                desperation: pressure.external_threat.value(),
            });
        } else {
            return Some(MilitaryAction::PreemptiveStrike {
                target: 0, // System will determine target
            });
        }
    }

    // High military weakness - build up forces
    if pressure.military_weakness.is_high() {
        return Some(MilitaryAction::RecruitArmy {
            urgency: pressure.military_weakness.value(),
        });
    }

    // Border exposure - fortify
    if pressure.border_exposure.is_high() {
        return Some(MilitaryAction::Fortify {
            priority_borders: Vec::new(), // System will determine
        });
    }

    // Defeat trauma with moderate weakness - reform military
    if pressure.defeat_trauma.is_moderate() && pressure.military_weakness.is_moderate() {
        return Some(MilitaryAction::MilitaryReform {
            focus: ReformFocus::Training,
        });
    }

    // Moderate external threat - seek allies
    if pressure.external_threat.is_moderate() {
        return Some(MilitaryAction::SeekAlliance {
            desperation: pressure.external_threat.value() * 0.5,
        });
    }

    None
}
