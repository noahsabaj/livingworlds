//! Test module for the nation name builder

#[cfg(test)]
mod tests {
    use super::super::builder::*;
    use crate::name_generator::{Culture, NameGenerator};
    use crate::nations::governance::types::GovernmentType;

    #[test]
    fn test_no_grammatical_conflicts() {
        let mut rng_gen = NameGenerator::new();

        // Test cases that were problematic:
        // "Security State of Great Senate of Achaea"
        // "Military State of Republican of Bavaria"

        let test_cases = vec![
            (GovernmentType::PoliceState, Culture::Western),
            (GovernmentType::MilitaryJunta, Culture::Western),
            (GovernmentType::FascistState, Culture::Western),
            (GovernmentType::Theocracy, Culture::Western),
            (GovernmentType::ParliamentaryDemocracy, Culture::Western),
        ];

        for (gov, culture) in test_cases {
            for _ in 0..20 {
                let (name, _ruler) = build_nation_name(&mut rng_gen, culture, gov);

                // Check for double political structures
                assert!(!name.contains("State of") || !name.contains("Senate"));
                assert!(!name.contains("State of") || !name.contains("Republic"));
                assert!(!name.contains("State of") || !name.contains("Parliament"));
                assert!(!name.contains("State of") || !name.contains("Empire"));
                assert!(!name.contains("State of") || !name.contains("Kingdom"));

                // Check for conflicting terms
                if name.contains("Security State") || name.contains("Military State") {
                    assert!(!name.contains("Democratic"));
                    assert!(!name.contains("Free"));
                    assert!(!name.contains("Parliamentary"));
                }

                println!("Generated: {}", name);
            }
        }
    }

    #[test]
    fn test_government_appropriate_descriptors() {
        let mut rng_gen = NameGenerator::new();

        // Test theocracies get religious descriptors
        for _ in 0..10 {
            let (name, _) = build_nation_name(&mut rng_gen, Culture::Western, GovernmentType::Theocracy);
            assert!(name.contains("Holy State"));
        }

        // Test military governments get appropriate names
        for _ in 0..10 {
            let (name, _) = build_nation_name(&mut rng_gen, Culture::Western, GovernmentType::MilitaryJunta);
            assert!(name.contains("Military State"));
        }
    }
}