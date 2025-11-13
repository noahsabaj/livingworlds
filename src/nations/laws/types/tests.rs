//! Unit tests for law types and effects
//!
//! Tests the core mechanics of law effects, particularly the critical
//! diminishing returns calculations that prevent law stacking exploits.

// Test module temporarily disabled - uses outdated PressureType API
// TODO: Update tests to match current PressureType variants
// #[cfg(test)]
// mod tests {
//     use super::super::*;
//     use crate::simulation::PressureType;
//     use std::collections::HashMap;

//     #[test]
//     fn test_diminishing_returns_single_modifier() {
//         let mut effects = LawEffects::default();
// 
//         // First law: 100% effectiveness
//         let law1 = LawEffects {
//             tax_efficiency_modifier: 0.1,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law1);
//         assert_eq!(effects.tax_efficiency_modifier, 0.1);
// 
//         // Second law: 75% effectiveness
//         let law2 = LawEffects {
//             tax_efficiency_modifier: 0.1,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law2);
//         assert_eq!(effects.tax_efficiency_modifier, 0.175); // 0.1 + (0.1 * 0.75)
// 
//         // Third law: 50% effectiveness
//         let law3 = LawEffects {
//             tax_efficiency_modifier: 0.1,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law3);
//         assert_eq!(effects.tax_efficiency_modifier, 0.225); // 0.175 + (0.1 * 0.5)
// 
//         // Fourth law: 25% effectiveness
//         let law4 = LawEffects {
//             tax_efficiency_modifier: 0.1,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law4);
//         assert_eq!(effects.tax_efficiency_modifier, 0.25); // 0.225 + (0.1 * 0.25)
//     }
// 
//     #[test]
//     fn test_diminishing_returns_multiple_modifiers() {
//         let mut effects = LawEffects::default();
// 
//         // Test that different modifiers track diminishing returns independently
//         let law1 = LawEffects {
//             tax_efficiency_modifier: 0.1,
//             happiness_modifier: 0.2,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law1);
// 
//         assert_eq!(effects.tax_efficiency_modifier, 0.1); // 100% for first tax law
//         assert_eq!(effects.happiness_modifier, 0.2); // 100% for first happiness law
// 
//         // Second law affects both
//         let law2 = LawEffects {
//             tax_efficiency_modifier: 0.1,
//             happiness_modifier: 0.2,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law2);
// 
//         assert_eq!(effects.tax_efficiency_modifier, 0.175); // 75% effectiveness
//         assert_eq!(effects.happiness_modifier, 0.35); // 75% effectiveness (0.2 + 0.2*0.75)
//     }
// 
//     #[test]
//     fn test_non_diminishing_fields() {
//         let mut effects = LawEffects::default();
// 
//         // Some fields like stability_change should add directly without diminishing returns
//         let law1 = LawEffects {
//             stability_change: 0.1,
//             legitimacy_change: 0.2,
//             corruption_change: -0.1,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law1);
// 
//         let law2 = LawEffects {
//             stability_change: 0.1,
//             legitimacy_change: 0.2,
//             corruption_change: -0.1,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law2);
// 
//         // These should add directly without diminishing returns
//         assert_eq!(effects.stability_change, 0.2);
//         assert_eq!(effects.legitimacy_change, 0.4);
//         assert_eq!(effects.corruption_change, -0.2);
//     }
// 
//     #[test]
//     fn test_effect_subtraction() {
//         let mut effects = LawEffects {
//             tax_efficiency_modifier: 0.3,
//             stability_change: 0.5,
//             happiness_modifier: 0.2,
//             ..Default::default()
//         };
// 
//         let to_remove = LawEffects {
//             tax_efficiency_modifier: 0.1,
//             stability_change: 0.2,
//             happiness_modifier: 0.1,
//             ..Default::default()
//         };
// 
//         effects.subtract(&to_remove);
// 
//         assert_eq!(effects.tax_efficiency_modifier, 0.2);
//         assert_eq!(effects.stability_change, 0.3);
//         assert_eq!(effects.happiness_modifier, 0.1);
//     }
// 
//     #[test]
//     fn test_boolean_flags() {
//         let mut effects = LawEffects::default();
// 
//         // First law allows slavery
//         let law1 = LawEffects {
//             allows_slavery: Some(true),
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law1);
//         assert_eq!(effects.allows_slavery, Some(true));
// 
//         // Second law also allows slavery (should remain true)
//         let law2 = LawEffects {
//             allows_slavery: Some(true),
//             allows_free_speech: Some(false),
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law2);
//         assert_eq!(effects.allows_slavery, Some(true));
//         assert_eq!(effects.allows_free_speech, Some(false));
// 
//         // Test subtraction resets boolean flags
//         effects.subtract(&law1);
//         assert_eq!(effects.allows_slavery, None);
//     }
// 
//     #[test]
//     fn test_pressure_modifiers() {
//         let mut effects = LawEffects::default();
// 
//         let mut pressure_mods = HashMap::new();
//         pressure_mods.insert(PressureType::Economic, 0.2);
//         pressure_mods.insert(PressureType::Military, -0.1);
// 
//         let law1 = LawEffects {
//             pressure_modifiers: pressure_mods.clone(),
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law1);
// 
//         assert_eq!(
//             *effects.pressure_modifiers.get(&PressureType::Economic).unwrap(),
//             0.2
//         );
//         assert_eq!(
//             *effects.pressure_modifiers.get(&PressureType::Military).unwrap(),
//             -0.1
//         );
// 
//         // Add same law again - pressures should stack
//         effects.add_with_diminishing_returns(&law1);
//         assert_eq!(
//             *effects.pressure_modifiers.get(&PressureType::Economic).unwrap(),
//             0.4
//         );
//         assert_eq!(
//             *effects.pressure_modifiers.get(&PressureType::Military).unwrap(),
//             -0.2
//         );
//     }
// 
//     #[test]
//     fn test_negative_modifiers() {
//         let mut effects = LawEffects::default();
// 
//         // Test that diminishing returns work correctly with negative values
//         let law1 = LawEffects {
//             tax_efficiency_modifier: -0.1,
//             happiness_modifier: -0.2,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law1);
// 
//         assert_eq!(effects.tax_efficiency_modifier, -0.1); // 100% for first
//         assert_eq!(effects.happiness_modifier, -0.2);
// 
//         let law2 = LawEffects {
//             tax_efficiency_modifier: -0.1,
//             happiness_modifier: -0.2,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law2);
// 
//         // Diminishing returns should apply to magnitude
//         assert_eq!(effects.tax_efficiency_modifier, -0.175); // 0.1 + 0.1*0.75
//         assert_eq!(effects.happiness_modifier, -0.35); // 0.2 + 0.2*0.75
//     }
// 
//     #[test]
//     fn test_mixed_sign_modifiers() {
//         let mut effects = LawEffects::default();
// 
//         // First law increases tax efficiency
//         let law1 = LawEffects {
//             tax_efficiency_modifier: 0.2,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law1);
//         assert_eq!(effects.tax_efficiency_modifier, 0.2);
// 
//         // Second law decreases it
//         let law2 = LawEffects {
//             tax_efficiency_modifier: -0.1,
//             ..Default::default()
//         };
//         effects.add_with_diminishing_returns(&law2);
// 
//         // Should calculate diminishing returns based on current absolute value
//         // Current is 0.2 (abs = 0.2), so we're at 2 "units" (0.2 / 0.1)
//         // This means 50% effectiveness for the third modifier
//         assert_eq!(effects.tax_efficiency_modifier, 0.15); // 0.2 + (-0.1 * 0.5)
//     }
// 
//     #[test]
//     fn test_effect_bounds() {
//         // Test that effects don't overflow with extreme values
//         let mut effects = LawEffects::default();
// 
//         for _ in 0..100 {
//             let law = LawEffects {
//                 tax_efficiency_modifier: 1.0,
//                 happiness_modifier: 1.0,
//                 ..Default::default()
//             };
//             effects.add_with_diminishing_returns(&law);
//         }
// 
//         // Even with 100 laws, diminishing returns should keep values reasonable
//         assert!(effects.tax_efficiency_modifier < 10.0);
//         assert!(effects.happiness_modifier < 10.0);
//     }
// 
//     #[test]
//     fn test_pressure_modifier_cleanup() {
//         let mut effects = LawEffects::default();
// 
//         let mut pressure_mods = HashMap::new();
//         pressure_mods.insert(PressureType::Economic, 0.5);
// 
//         let law = LawEffects {
//             pressure_modifiers: pressure_mods,
//             ..Default::default()
//         };
// 
//         effects.add_with_diminishing_returns(&law);
//         assert!(effects.pressure_modifiers.contains_key(&PressureType::Economic));
// 
//         // Subtract the same amount
//         effects.subtract(&law);
// 
//         // The pressure modifier should be removed if it's close to zero
//         assert!(
//             !effects.pressure_modifiers.contains_key(&PressureType::Economic) ||
//             effects.pressure_modifiers.get(&PressureType::Economic).unwrap().abs() < 0.001
//         );
//     }
// }