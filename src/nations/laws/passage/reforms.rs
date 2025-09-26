//! Law reform and emergency power systems
//!
//! Handles law reforms, replacements, and emergency powers during crises.

use super::types::EmergencyPower;
use crate::nations::laws::registry::{LawRegistry, NationLaws};
use crate::nations::laws::types::LawId;
use crate::nations::{Nation, Governance, GovernmentCategory};

/// Process a law reform (replacing one law with another)
pub fn process_law_reform(
    old_law: LawId,
    new_law: LawId,
    nation_laws: &mut NationLaws,
    registry: &LawRegistry,
    current_year: i32,
) -> Result<(), String> {
    // Check that old law is active
    if !nation_laws.is_active(old_law) {
        return Err("Old law is not active".to_string());
    }

    // Get both laws
    let old = registry.get_law(old_law)
        .ok_or_else(|| "Old law not found".to_string())?;
    let new = registry.get_law(new_law)
        .ok_or_else(|| "New law not found".to_string())?;

    // Check they're in the same category
    if old.category != new.category {
        return Err("Laws must be in same category for reform".to_string());
    }

    // Repeal old law (uses stored original effects)
    nation_laws.repeal_law(old_law, current_year);

    // Enact new law
    nation_laws.enact_law(new_law, &new.effects, current_year);

    Ok(())
}

/// Handle emergency law powers during crisis
pub fn emergency_law_powers(
    nation: &Nation,
    governance: &Governance,
    crisis_level: f32,
) -> Vec<EmergencyPower> {
    let mut powers = Vec::new();

    // Only available in severe crisis
    if crisis_level < 0.7 {
        return powers;
    }

    match governance.current_government.category() {
        GovernmentCategory::Democratic => {
            // Democracies can temporarily suspend some rights
            if nation.stability < 0.3 {
                powers.push(EmergencyPower::SuspendElections);
                powers.push(EmergencyPower::MartialLaw);
            }
        }
        GovernmentCategory::Autocratic => {
            // Autocracies can implement extreme measures
            powers.push(EmergencyPower::MartialLaw);
            powers.push(EmergencyPower::Purge);
            powers.push(EmergencyPower::TotalMobilization);
        }
        GovernmentCategory::Anarchist => {
            // Anarchists organize emergency councils
            powers.push(EmergencyPower::EmergencyCouncils);
        }
        _ => {
            // Most governments can declare martial law in crisis
            if crisis_level > 0.8 {
                powers.push(EmergencyPower::MartialLaw);
            }
        }
    }

    powers
}