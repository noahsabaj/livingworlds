//! Debug commands for law system testing
//!
//! Provides commands to manipulate laws for testing purposes.

use bevy::prelude::*;
use crate::nations::laws::{LawId, NationLaws, LawRegistry, LawEnactmentEvent, LawRepealEvent};
use crate::nations::{Nation, NationId};
use crate::simulation::PressureType;

/// Debug commands for manipulating laws
pub struct LawDebugCommands;

impl LawDebugCommands {
    /// Force enact a law for testing
    pub fn force_enact(
        nation_entity: Entity,
        law_id: LawId,
        nation_laws: &mut NationLaws,
        registry: &LawRegistry,
        messages: &mut MessageWriter<LawEnactmentEvent>,
    ) {
        if let Some(law) = registry.get_law(law_id) {
            // Skip all checks and directly enact
            nation_laws.enact_law(law_id, &law.effects, 0);

            messages.write(LawEnactmentEvent {
                nation_id: NationId(0), // Debug nation ID
                nation_name: format!("Debug Nation {:?}", nation_entity),
                law_id,
                law_name: law.name.clone(),
                category: law.category,
            });

            info!("DEBUG: Force enacted law {:?} for nation", law_id);
        }
    }

    /// Force repeal a law for testing
    pub fn force_repeal(
        nation_entity: Entity,
        law_id: LawId,
        nation_laws: &mut NationLaws,
        registry: &LawRegistry,
        messages: &mut MessageWriter<LawRepealEvent>,
    ) {
        if nation_laws.is_active(law_id) {
            // For debug, we'll use 0 years active since we don't track this currently
            let years_active = 0;
            nation_laws.repeal_law(law_id, 0);  // year = 0 for debug

            if let Some(law) = registry.get_law(law_id) {
                messages.write(LawRepealEvent {
                    nation_id: NationId(0), // Debug nation ID
                    nation_name: format!("Debug Nation {:?}", nation_entity),
                    law_id,
                    law_name: law.name.clone(),
                    category: law.category,
                    years_active,
                });
            }

            info!("DEBUG: Force repealed law {:?} for nation", law_id);
        }
    }

    /// Trigger a law proposal for testing
    pub fn trigger_proposal(
        nation_laws: &mut NationLaws,
        law_id: LawId,
        pressure_type: PressureType,
        initial_support: f32,
    ) {
        let default_debate_days = 30.0;  // Standard debate period
        nation_laws.propose_law(law_id, initial_support, default_debate_days, Some(pressure_type));
        info!("DEBUG: Triggered proposal for law {:?} with {:.0}% support for {} days debate (pressure: {:?})",
            law_id, initial_support * 100.0, default_debate_days, pressure_type);
    }
}

/// Spawn test laws for all nations
pub fn spawn_test_laws(
    mut commands: Commands,
    nations_query: Query<Entity, With<Nation>>,
    registry: Res<LawRegistry>,
) {
    info!("DEBUG: Spawning test laws for all nations");

    // Give each nation some random laws for testing
    let test_law_ids = vec![
        LawId::new(1000), // Flat Tax
        LawId::new(1002), // Progressive Tax
        LawId::new(2000), // Universal Suffrage
        LawId::new(3000), // Freedom of Speech
        LawId::new(4000), // State Religion
    ];

    for nation_entity in &nations_query {
        let mut nation_laws = NationLaws::default();

        // Enact some test laws
        for &law_id in test_law_ids.iter().take(3) {
            if let Some(law) = registry.get_law(law_id) {
                nation_laws.enact_law(law_id, &law.effects, 0);
            }
        }

        // Add some proposed laws
        nation_laws.propose_law(test_law_ids[3], 0.6, 30.0, Some(PressureType::CulturalDivision));
        nation_laws.propose_law(test_law_ids[4], 0.4, 30.0, Some(PressureType::CulturalDivision));

        // Add the component to the nation
        commands.entity(nation_entity).insert(nation_laws);
    }

    info!("DEBUG: Test laws spawned for {} nations", nations_query.iter().count());
}

/// System to force enact a law via keyboard shortcut (for testing)
pub fn force_enact_law(
    mut shortcut_events: MessageReader<crate::ui::ShortcutEvent>,
    mut nations_query: Query<(Entity, &mut NationLaws)>,
    registry: Res<LawRegistry>,
    mut messages: MessageWriter<LawEnactmentEvent>,
) {
    use crate::ui::ShortcutId;

    for event in shortcut_events.read() {
        if event.shortcut_id == ShortcutId::DebugForceEnactLaw {
            if let Some((entity, mut nation_laws)) = nations_query.iter_mut().next() {
                // Pick a random law that's not active
                let all_laws = crate::nations::laws::definitions::get_all_laws();
                for law in all_laws {
                    if !nation_laws.is_active(law.id) {
                        LawDebugCommands::force_enact(
                            entity,
                            law.id,
                            &mut nation_laws,
                            &registry,
                            &mut messages,
                        );
                        break;
                    }
                }
            }
        }
    }
}

/// System to force repeal a law via keyboard shortcut (for testing)
pub fn force_repeal_law(
    mut shortcut_events: MessageReader<crate::ui::ShortcutEvent>,
    mut nations_query: Query<(Entity, &mut NationLaws)>,
    registry: Res<LawRegistry>,
    mut messages: MessageWriter<LawRepealEvent>,
) {
    use crate::ui::ShortcutId;

    for event in shortcut_events.read() {
        if event.shortcut_id == ShortcutId::DebugForceRepealLaw {
            if let Some((entity, mut nation_laws)) = nations_query.iter_mut().next() {
                // Pick a random active law
                if let Some(&law_id) = nation_laws.active_laws.iter().next() {
                    LawDebugCommands::force_repeal(
                        entity,
                        law_id,
                        &mut nation_laws,
                        &registry,
                        &mut messages,
                    );
                }
            }
        }
    }
}

/// System to trigger a test proposal via keyboard shortcut
pub fn trigger_law_proposal(
    mut shortcut_events: MessageReader<crate::ui::ShortcutEvent>,
    mut nations_query: Query<&mut NationLaws>,
) {
    use crate::ui::ShortcutId;

    for event in shortcut_events.read() {
        if event.shortcut_id == ShortcutId::DebugTriggerProposal {
            if let Some(mut nation_laws) = nations_query.iter_mut().next() {
                // Find a law that's not active or proposed
                let test_law_id = LawId::new(5000); // Some military law
                if !nation_laws.is_active(test_law_id) &&
                   !nation_laws.proposed_laws.iter().any(|p| p.law_id == test_law_id) {
                    LawDebugCommands::trigger_proposal(
                        &mut nation_laws,
                        test_law_id,
                        PressureType::MilitaryVulnerability,
                        0.45,
                    );
                }
            }
        }
    }
}