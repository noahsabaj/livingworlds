#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::name_generator::{NameGenerator, Culture};
    use crate::nations::governance::types::GovernmentType;

    #[test]
    fn test_strip_government_structures() {
        // Test compound patterns
        assert_eq!(
            utils::strip_government_structures("Royal Kingdom of Britannia"),
            "Britannia"
        );
        assert_eq!(
            utils::strip_government_structures("Imperial Empire of Roma"),
            "Roma"
        );
        assert_eq!(
            utils::strip_government_structures("United Republic of States"),
            "States"
        );

        // Test standard patterns with "The"
        assert_eq!(
            utils::strip_government_structures("The Kingdom of Avalon"),
            "Avalon"
        );
        assert_eq!(
            utils::strip_government_structures("The Republic of Venice"),
            "Venice"
        );

        // Test suffix patterns
        assert_eq!(
            utils::strip_government_structures("Britannia Empire"),
            "Britannia"
        );
        assert_eq!(
            utils::strip_government_structures("Roma Federation"),
            "Roma"
        );

        // Test multiple adjectives
        assert_eq!(
            utils::strip_government_structures("Royal Grand Britannia"),
            "Britannia"
        );
        assert_eq!(
            utils::strip_government_structures("Free Democratic States"),
            "States"
        );
    }

    #[test]
    fn test_clean_nation_name() {
        assert_eq!(
            validation::clean_nation_name("Royal Kingdom of Britannia"),
            "Britannia"
        );
        assert_eq!(
            validation::clean_nation_name("Democratic Socialist Republic"),
            "" // All words are government-related
        );
        assert_eq!(
            validation::clean_nation_name("The Grand Imperial Empire of Roma"),
            "The of Roma" // Only non-government words remain
        );
    }

    #[test]
    fn test_name_consistency_validation() {
        // Anarchist governments shouldn't have royal terms
        assert!(validation::validate_name_consistency(
            "Free Territory of Britannia",
            GovernmentType::AnarchoSyndicalism
        ).is_ok());
        
        assert!(validation::validate_name_consistency(
            "Royal Kingdom of Britannia",
            GovernmentType::AnarchoSyndicalism
        ).is_err());

        // Democratic governments shouldn't have imperial terms
        assert!(validation::validate_name_consistency(
            "Republic of Venice",
            GovernmentType::ParliamentaryDemocracy
        ).is_ok());
        
        assert!(validation::validate_name_consistency(
            "Imperial Empire of Venice",
            GovernmentType::ParliamentaryDemocracy
        ).is_err());
    }

    #[test]
    fn test_governance_aware_name_generation() {
        let mut generator = NameGenerator::with_seed(12345);

        // Test anarchist government gets appropriate name
        let (name, title) = generate_governance_aware_name(
            &mut generator,
            Culture::Western,
            &GovernmentType::AnarchoSyndicalism,
        );
        assert!(name.contains("Free Territory"));
        assert_eq!(title, "Speaker");

        // Test monarchy gets appropriate name
        let (name, title) = generate_governance_aware_name(
            &mut generator,
            Culture::Western,
            &GovernmentType::AbsoluteMonarchy,
        );
        assert!(name.contains("Kingdom"));
        assert_eq!(title, "King");

        // Test democracy gets appropriate name
        let (name, title) = generate_governance_aware_name(
            &mut generator,
            Culture::Western,
            &GovernmentType::ParliamentaryDemocracy,
        );
        assert!(name.contains("Parliamentary Republic"));
        assert_eq!(title, "Prime Minister");
    }

    #[test]
    fn test_no_contradictory_names() {
        let mut generator = NameGenerator::with_seed(42);

        // Generate 100 names for anarchist government
        // and ensure none contain hierarchical terms
        for _ in 0..100 {
            let (name, _) = generate_governance_aware_name(
                &mut generator,
                Culture::Western,
                &GovernmentType::AnarchoCommunism,
            );

            // Check that anarchist names don't contain royal/imperial terms
            assert!(!name.to_lowercase().contains("royal"));
            assert!(!name.to_lowercase().contains("imperial"));
            assert!(!name.to_lowercase().contains("kingdom"));
            assert!(!name.to_lowercase().contains("empire"));
            assert!(!name.to_lowercase().contains("crown"));
        }
    }

    #[test]
    fn test_ruler_titles_match_government() {
        use super::super::get_ruler_title;
        use crate::nations::governance::types::Gender;

        // Anarchist governments have speaker titles
        assert_eq!(
            get_ruler_title(&GovernmentType::AnarchoSyndicalism, Gender::Neutral),
            "Speaker"
        );

        // Democratic governments have president/PM titles
        assert_eq!(
            get_ruler_title(&GovernmentType::PresidentialRepublic, Gender::Neutral),
            "President"
        );

        // Monarchies have king titles
        assert_eq!(
            get_ruler_title(&GovernmentType::AbsoluteMonarchy, Gender::Male),
            "King"
        );

        // Additional coverage for other government categories
        assert_eq!(
            get_ruler_title(&GovernmentType::ParliamentaryDemocracy, Gender::Neutral),
            "Prime Minister"
        );

        assert_eq!(
            get_ruler_title(&GovernmentType::Theocracy, Gender::Neutral),
            "High Priest"
        );

        assert_eq!(
            get_ruler_title(&GovernmentType::Empire, Gender::Neutral),
            "Emperor"
        );
    }
}