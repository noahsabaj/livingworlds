//! Unit tests for law types and effects
//!
//! Tests the core mechanics of law effects, particularly the critical
//! diminishing returns calculations that prevent law stacking exploits.

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::simulation::PressureType;
    use std::collections::HashMap;

    /// Helper function for approximate floating point equality
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.0001
    }

    /// Assert that two f32 values are approximately equal
    macro_rules! assert_approx_eq {
        ($left:expr, $right:expr) => {
            assert!(
                approx_eq($left, $right),
                "assertion failed: `(left ≈ right)`\n  left: `{:?}`,\n right: `{:?}`",
                $left,
                $right
            )
        };
    }

    #[test]
    fn test_diminishing_returns_single_modifier() {
        let mut effects = LawEffects::default();

        // First law: 100% effectiveness
        let law1 = LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law1);
        assert_approx_eq!(effects.tax_efficiency_modifier, 0.1);

        // Second law: 75% effectiveness (num_existing=1)
        let law2 = LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law2);
        assert_approx_eq!(effects.tax_efficiency_modifier, 0.175); // 0.1 + (0.1 * 0.75)

        // Third law: still 75% effectiveness (num_existing=1, floor(0.175/0.1)=1)
        let law3 = LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law3);
        assert_approx_eq!(effects.tax_efficiency_modifier, 0.25); // 0.175 + (0.1 * 0.75)

        // Fourth law: 50% effectiveness (num_existing=2, floor(0.25/0.1)=2)
        let law4 = LawEffects {
            tax_efficiency_modifier: 0.1,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law4);
        assert_approx_eq!(effects.tax_efficiency_modifier, 0.3); // 0.25 + (0.1 * 0.5)
    }

    #[test]
    fn test_diminishing_returns_multiple_modifiers() {
        let mut effects = LawEffects::default();

        // Test that different modifiers track diminishing returns independently
        let law1 = LawEffects {
            tax_efficiency_modifier: 0.1,
            happiness_modifier: 0.2,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law1);

        assert_approx_eq!(effects.tax_efficiency_modifier, 0.1); // 100% for first tax law
        assert_approx_eq!(effects.happiness_modifier, 0.2); // 100% for first happiness law

        // Second law affects both
        let law2 = LawEffects {
            tax_efficiency_modifier: 0.1,
            happiness_modifier: 0.2,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law2);

        assert_approx_eq!(effects.tax_efficiency_modifier, 0.175); // 75% effectiveness (floor(0.1/0.1)=1)
        assert_approx_eq!(effects.happiness_modifier, 0.3); // 50% effectiveness (floor(0.2/0.1)=2) → 0.2 + 0.2*0.5
    }

    #[test]
    fn test_non_diminishing_fields() {
        let mut effects = LawEffects::default();

        // Some fields like stability_change should add directly without diminishing returns
        let law1 = LawEffects {
            stability_change: 0.1,
            legitimacy_change: 0.2,
            corruption_change: -0.1,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law1);

        let law2 = LawEffects {
            stability_change: 0.1,
            legitimacy_change: 0.2,
            corruption_change: -0.1,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law2);

        // These should add directly without diminishing returns
        assert_approx_eq!(effects.stability_change, 0.2);
        assert_approx_eq!(effects.legitimacy_change, 0.4);
        assert_approx_eq!(effects.corruption_change, -0.2);
    }

    #[test]
    fn test_effect_subtraction() {
        let mut effects = LawEffects {
            tax_efficiency_modifier: 0.3,
            stability_change: 0.5,
            happiness_modifier: 0.2,
            ..Default::default()
        };

        let to_remove = LawEffects {
            tax_efficiency_modifier: 0.1,
            stability_change: 0.2,
            happiness_modifier: 0.1,
            ..Default::default()
        };

        effects.subtract(&to_remove);

        assert_approx_eq!(effects.tax_efficiency_modifier, 0.2);
        assert_approx_eq!(effects.stability_change, 0.3);
        assert_approx_eq!(effects.happiness_modifier, 0.1);
    }

    #[test]
    fn test_boolean_flags() {
        let mut effects = LawEffects::default();

        // First law allows slavery
        let law1 = LawEffects {
            allows_slavery: Some(true),
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law1);
        assert_eq!(effects.allows_slavery, Some(true));

        // Second law also allows slavery (should remain true)
        let law2 = LawEffects {
            allows_slavery: Some(true),
            allows_free_speech: Some(false),
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law2);
        assert_eq!(effects.allows_slavery, Some(true));
        assert_eq!(effects.allows_free_speech, Some(false));

        // Test subtraction resets boolean flags
        effects.subtract(&law1);
        assert_eq!(effects.allows_slavery, None);
    }

    #[test]
    fn test_pressure_modifiers() {
        let mut effects = LawEffects::default();

        let mut pressure_mods = HashMap::new();
        pressure_mods.insert(PressureType::EconomicStrain, 0.2);
        pressure_mods.insert(PressureType::MilitaryVulnerability, -0.1);

        let law1 = LawEffects {
            pressure_modifiers: pressure_mods.clone(),
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law1);

        assert_eq!(
            *effects.pressure_modifiers.get(&PressureType::EconomicStrain).unwrap(),
            0.2
        );
        assert_eq!(
            *effects.pressure_modifiers.get(&PressureType::MilitaryVulnerability).unwrap(),
            -0.1
        );

        // Add same law again - pressures should stack
        effects.add_with_diminishing_returns(&law1);
        assert_eq!(
            *effects.pressure_modifiers.get(&PressureType::EconomicStrain).unwrap(),
            0.4
        );
        assert_eq!(
            *effects.pressure_modifiers.get(&PressureType::MilitaryVulnerability).unwrap(),
            -0.2
        );
    }

    #[test]
    fn test_negative_modifiers() {
        let mut effects = LawEffects::default();

        // Test that diminishing returns work correctly with negative values
        let law1 = LawEffects {
            tax_efficiency_modifier: -0.1,
            happiness_modifier: -0.2,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law1);

        assert_approx_eq!(effects.tax_efficiency_modifier, -0.1); // 100% for first
        assert_approx_eq!(effects.happiness_modifier, -0.2);

        let law2 = LawEffects {
            tax_efficiency_modifier: -0.1,
            happiness_modifier: -0.2,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law2);

        // Diminishing returns should apply to magnitude
        assert_approx_eq!(effects.tax_efficiency_modifier, -0.175); // floor(|-0.1|/0.1)=1 → -0.1 + (-0.1*0.75)
        assert_approx_eq!(effects.happiness_modifier, -0.3); // floor(|-0.2|/0.1)=2 → -0.2 + (-0.2*0.5)
    }

    #[test]
    fn test_mixed_sign_modifiers() {
        let mut effects = LawEffects::default();

        // First law increases tax efficiency
        let law1 = LawEffects {
            tax_efficiency_modifier: 0.2,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law1);
        assert_approx_eq!(effects.tax_efficiency_modifier, 0.2);

        // Second law decreases it
        let law2 = LawEffects {
            tax_efficiency_modifier: -0.1,
            ..Default::default()
        };
        effects.add_with_diminishing_returns(&law2);

        // Should calculate diminishing returns based on current absolute value
        // Current is 0.2 (abs = 0.2), so we're at 2 "units" (0.2 / 0.1)
        // This means 50% effectiveness for the third modifier
        assert_approx_eq!(effects.tax_efficiency_modifier, 0.15); // 0.2 + (-0.1 * 0.5)
    }

    #[test]
    fn test_effect_bounds() {
        // Test that effects don't overflow with extreme values
        let mut effects = LawEffects::default();

        for _ in 0..100 {
            let law = LawEffects {
                tax_efficiency_modifier: 1.0,
                happiness_modifier: 1.0,
                ..Default::default()
            };
            effects.add_with_diminishing_returns(&law);
        }

        // With diminishing returns (25% effectiveness after first law for 1.0 values),
        // 100 laws would give approximately: 1.0 + (99 * 0.25) ≈ 25.75
        // This is reasonable as it prevents exponential growth while allowing meaningful stacking
        assert!(effects.tax_efficiency_modifier < 30.0);
        assert!(effects.happiness_modifier < 30.0);

        // Verify diminishing returns are actually working (should be much less than 100)
        assert!(effects.tax_efficiency_modifier < 100.0);
    }

    #[test]
    fn test_pressure_modifier_cleanup() {
        let mut effects = LawEffects::default();

        let mut pressure_mods = HashMap::new();
        pressure_mods.insert(PressureType::EconomicStrain, 0.5);

        let law = LawEffects {
            pressure_modifiers: pressure_mods,
            ..Default::default()
        };

        effects.add_with_diminishing_returns(&law);
        assert!(effects.pressure_modifiers.contains_key(&PressureType::EconomicStrain));

        // Subtract the same amount
        effects.subtract(&law);

        // The pressure modifier should be removed if it's close to zero
        assert!(
            !effects.pressure_modifiers.contains_key(&PressureType::EconomicStrain) ||
            effects.pressure_modifiers.get(&PressureType::EconomicStrain).unwrap().abs() < 0.001
        );
    }
}