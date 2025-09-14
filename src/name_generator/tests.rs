//! Unit tests for the name generator system

#[cfg(test)]
mod tests {
    use crate::name_generator::{NameGenerator, NameType, Culture, Gender, PersonRole, Region, CitySize, NameRelation};
    use std::collections::HashSet;

    #[test]
    fn test_world_name_generation() {
        let mut gen = NameGenerator::new();
        for _ in 0..20 {
            let name = gen.generate(NameType::World);
            assert!(!name.is_empty(), "Generated world name should not be empty");
            assert!(name.len() < 100, "Generated world name should be reasonable length");
            println!("World: {}", name);
        }
    }

    #[test]
    fn test_all_cultures_nation_names() {
        let mut gen = NameGenerator::new();
        for culture in Culture::all() {
            let name = gen.generate(NameType::Nation { culture: *culture });
            assert!(!name.is_empty(), "Nation name for {:?} should not be empty", culture);
            println!("{:?} Nation: {}", culture, name);
        }
    }

    #[test]
    fn test_province_generation_all_regions() {
        let mut gen = NameGenerator::new();
        let regions = [
            Region::Coastal, Region::Mountain, Region::Desert, Region::Forest,
            Region::Plains, Region::River, Region::Arctic, Region::Tropical,
            Region::Valley, Region::Island,
        ];

        for region in regions {
            for culture in Culture::all() {
                let name = gen.generate(NameType::Province {
                    region,
                    culture: *culture
                });
                assert!(!name.is_empty());
                println!("{:?}/{:?} Province: {}", region, culture, name);
            }
        }
    }

    #[test]
    fn test_city_generation_all_sizes() {
        let mut gen = NameGenerator::new();
        let sizes = [
            CitySize::Hamlet, CitySize::Village, CitySize::Town,
            CitySize::City, CitySize::Metropolis,
        ];

        for size in sizes {
            let name = gen.generate(NameType::City {
                size,
                culture: Culture::Western,
            });
            assert!(!name.is_empty());
            println!("{:?}: {}", size, name);
        }
    }

    #[test]
    fn test_person_generation_all_combinations() {
        let mut gen = NameGenerator::new();
        let genders = [Gender::Male, Gender::Female, Gender::Neutral];
        let roles = [
            PersonRole::Ruler, PersonRole::General, PersonRole::Diplomat,
            PersonRole::Merchant, PersonRole::Scholar, PersonRole::Priest,
            PersonRole::Explorer, PersonRole::Commoner,
        ];

        for culture in Culture::all() {
            for gender in genders {
                for role in roles {
                    let name = gen.generate(NameType::Person {
                        gender,
                        culture: *culture,
                        role,
                    });
                    assert!(!name.is_empty());
                    println!("{:?}/{:?}/{:?}: {}", culture, gender, role, name);
                }
            }
        }
    }

    #[test]
    fn test_geographic_features() {
        let mut gen = NameGenerator::new();

        let river = gen.generate(NameType::River);
        assert!(!river.is_empty());
        println!("River: {}", river);

        let mountain = gen.generate(NameType::Mountain);
        assert!(!mountain.is_empty());
        println!("Mountain: {}", mountain);

        let ocean = gen.generate(NameType::Ocean);
        assert!(!ocean.is_empty());
        println!("Ocean: {}", ocean);

        let desert = gen.generate(NameType::Desert);
        assert!(!desert.is_empty());
        println!("Desert: {}", desert);

        let forest = gen.generate(NameType::Forest);
        assert!(!forest.is_empty());
        println!("Forest: {}", forest);
    }

    #[test]
    fn test_uniqueness_enforcement() {
        let mut gen = NameGenerator::new();
        let mut names = HashSet::new();

        // Generate many world names and check for uniqueness
        for i in 0..200 {
            let name = gen.generate(NameType::World);
            assert!(
                !names.contains(&name),
                "Duplicate name generated at iteration {}: {}",
                i, name
            );
            names.insert(name);
        }

        assert_eq!(names.len(), 200);
        assert_eq!(gen.names_generated(), 200);
    }

    #[test]
    fn test_deterministic_generation() {
        let mut gen1 = NameGenerator::with_seed(42);
        let mut gen2 = NameGenerator::with_seed(42);

        for _ in 0..20 {
            let name1 = gen1.generate(NameType::World);
            let name2 = gen2.generate(NameType::World);
            assert_eq!(name1, name2, "Deterministic generation should produce same names");
        }
    }

    #[test]
    fn test_related_names() {
        let mut gen = NameGenerator::new();
        let parent = "Alexandria";

        let new_settlement = gen.generate_related_name(parent, NameRelation::NewSettlement);
        assert!(new_settlement.starts_with("New"));
        println!("New Settlement: {}", new_settlement);

        let old_settlement = gen.generate_related_name(parent, NameRelation::OldSettlement);
        assert!(old_settlement.starts_with("Old"));
        println!("Old Settlement: {}", old_settlement);

        let child_city = gen.generate_related_name(parent, NameRelation::ChildCity);
        assert!(child_city.contains(parent));
        println!("Child City: {}", child_city);

        let twin_city = gen.generate_related_name(parent, NameRelation::TwinCity);
        assert!(twin_city.contains(parent));
        println!("Twin City: {}", twin_city);

        let rival_city = gen.generate_related_name(parent, NameRelation::RivalCity);
        assert!(rival_city.contains(parent));
        println!("Rival City: {}", rival_city);
    }

    #[test]
    fn test_cache_clearing() {
        let mut gen = NameGenerator::new();

        // Generate some names
        for _ in 0..10 {
            gen.generate(NameType::World);
        }
        assert_eq!(gen.names_generated(), 10);

        // Clear cache
        gen.clear_cache();
        assert_eq!(gen.names_generated(), 0);

        // Should be able to generate same names again
        for _ in 0..10 {
            gen.generate(NameType::World);
        }
        assert_eq!(gen.names_generated(), 10);
    }

    #[test]
    fn test_roman_numeral_fallback() {
        use crate::name_generator::to_roman_numeral;

        assert_eq!(to_roman_numeral(1), "I");
        assert_eq!(to_roman_numeral(5), "V");
        assert_eq!(to_roman_numeral(10), "X");
        assert_eq!(to_roman_numeral(50), "L");
        assert_eq!(to_roman_numeral(51), "");  // Beyond range
    }

    #[test]
    fn test_name_length_reasonable() {
        let mut gen = NameGenerator::new();

        // Test various name types to ensure reasonable lengths
        for _ in 0..100 {
            let world = gen.generate(NameType::World);
            assert!(world.len() < 50, "World name too long: {}", world);

            let nation = gen.generate(NameType::Nation { culture: Culture::Western });
            assert!(nation.len() < 60, "Nation name too long: {}", nation);

            let person = gen.generate(NameType::Person {
                gender: Gender::Male,
                culture: Culture::Eastern,
                role: PersonRole::Ruler,
            });
            assert!(person.len() < 70, "Person name too long: {}", person);
        }
    }
}