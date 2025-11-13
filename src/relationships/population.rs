//! Population Relationships - Demographics and residence
//!
//! This module defines relationships for population groups, their residence in provinces,
//! and demographic tracking for migration and population dynamics.

use bevy::prelude::*;

// ================================================================================================
// POPULATION RESIDENCE RELATIONSHIPS
// ================================================================================================

/// A population group resides in a province
/// Demographic tracking and migration mechanics
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = HostsPopulations)]
pub struct ResidesIn(pub Entity);

/// Reverse relationship: A province hosts population groups
/// Automatically maintained by Bevy when `ResidesIn` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = ResidesIn, linked_spawn)]
pub struct HostsPopulations(Vec<Entity>); // Private for safety - Bevy handles internal access

impl HostsPopulations {
    /// Get read-only access to population groups in this province
    pub fn populations(&self) -> &[Entity] {
        &self.0
    }

    /// Get the number of population groups
    pub fn population_group_count(&self) -> usize {
        self.0.len()
    }

    /// Check if a specific population group resides here
    pub fn hosts_population(&self, pop_group: Entity) -> bool {
        self.0.contains(&pop_group)
    }

    /// Check if province has any population
    pub fn has_population(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// POPULATION ENTITIES
// ================================================================================================

/// Marker component for population group entities
#[derive(Component, Debug, Clone)]
pub struct PopulationGroup {
    pub name: String,
    pub size: u32, // Number of people in this group
    pub culture: crate::name_generator::Culture,
    pub social_class: SocialClass,
    pub primary_occupation: Occupation,
    pub growth_rate: f32,        // Annual population growth rate
    pub migration_tendency: f32, // Likelihood to migrate (0.0 to 1.0)
}

/// Demographics data for a province
#[derive(Component, Debug, Clone)]
pub struct Demographics {
    pub total_population: u32,
    pub population_density: f32, // People per unit area
    pub growth_rate: f32,        // Overall population growth
    pub cultural_composition: Vec<CulturalGroup>,
    pub social_stratification: SocialStratification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocialClass {
    Nobility,  // Aristocrats and rulers
    Clergy,    // Religious leaders
    Merchants, // Traders and craftsmen
    Farmers,   // Agricultural workers
    Laborers,  // Manual workers
    Slaves,    // Enslaved population
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Occupation {
    Agriculture,    // Farming and herding
    Craftsmanship,  // Artisans and craftsmen
    Trade,          // Merchants and traders
    Military,       // Soldiers and guards
    Clergy,         // Religious workers
    Administration, // Government officials
    Labor,          // General laborers
}

#[derive(Debug, Clone)]
pub struct CulturalGroup {
    pub culture: crate::name_generator::Culture,
    pub population: u32,
    pub percentage: f32,
}

#[derive(Debug, Clone)]
pub struct SocialStratification {
    pub nobility_percentage: f32,
    pub middle_class_percentage: f32,
    pub working_class_percentage: f32,
    pub gini_coefficient: f32, // Wealth inequality measure
}

// ================================================================================================
// MIGRATION DATA
// ================================================================================================

/// Migration flow data between provinces
#[derive(Component, Debug, Clone)]
pub struct MigrationFlow {
    pub origin: Entity,       // Source province
    pub destination: Entity,  // Destination province
    pub population_size: u32, // Number of migrants
    pub migration_type: MigrationType,
    pub push_factors: Vec<PushFactor>,
    pub pull_factors: Vec<PullFactor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationType {
    Economic,      // Seeking better opportunities
    Political,     // Fleeing persecution or conflict
    Environmental, // Escaping natural disasters
    Religious,     // Religious migration
    Seasonal,      // Temporary seasonal migration
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushFactor {
    Poverty,     // Economic hardship
    War,         // Military conflict
    Persecution, // Political or religious persecution
    Famine,      // Food shortage
    Disease,     // Epidemic or plague
    Taxation,    // Excessive taxation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PullFactor {
    Opportunity, // Economic opportunities
    Safety,      // Security and peace
    Resources,   // Abundant resources
    Freedom,     // Political or religious freedom
    Family,      // Family connections
    Climate,     // Better climate
}

// ================================================================================================
// QUERY SYSTEMS - Demographic analysis
// ================================================================================================

/// System for querying population groups and their locations
pub fn query_population_distribution_system(
    populations_query: Query<(Entity, &PopulationGroup, Option<&ResidesIn>)>,
) {
    for (pop_entity, population, resides_in) in populations_query.iter() {
        let location = resides_in.map(|ri| ri.0);
        debug!(
            "Population group {:?} ({}) resides in province {:?}",
            pop_entity, population.name, location
        );
    }
}

/// System for querying demographic composition by province
pub fn query_provincial_demographics_system(
    provinces_query: Query<(Entity, &Demographics, &HostsPopulations)>,
) {
    for (province_entity, demographics, hosts_pops) in provinces_query.iter() {
        debug!(
            "Province {:?} has {} population, {} groups",
            province_entity,
            demographics.total_population,
            hosts_pops.0.len()
        );
    }
}

/// Find all population groups in a province
pub fn find_province_populations(
    province_entity: Entity,
    provinces_query: &Query<&HostsPopulations>,
) -> Option<Vec<Entity>> {
    provinces_query
        .get(province_entity)
        .ok()
        .map(|hosts| hosts.0.clone())
}

/// Calculate total population in a province
pub fn calculate_provincial_population(
    province_entity: Entity,
    provinces_query: &Query<&HostsPopulations>,
    populations_query: &Query<&PopulationGroup>,
) -> u32 {
    if let Some(population_entities) = find_province_populations(province_entity, provinces_query) {
        population_entities
            .iter()
            .filter_map(|&pop_entity| populations_query.get(pop_entity).ok())
            .map(|pop_group| pop_group.size)
            .sum()
    } else {
        0
    }
}

// ================================================================================================
// DEMOGRAPHIC SYSTEMS
// ================================================================================================

/// Updates demographic data for all provinces
pub fn update_provincial_demographics(
    mut provinces_query: Query<(Entity, &mut Demographics, &HostsPopulations)>,
    populations_query: Query<&PopulationGroup>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (province_entity, mut demographics, hosts_populations) in &mut provinces_query {
        // Calculate total population
        let population_groups: Vec<&PopulationGroup> = hosts_populations
            .populations()
            .iter()
            .filter_map(|&pop_entity| populations_query.get(pop_entity).ok())
            .collect();

        demographics.total_population = population_groups.iter().map(|pop| pop.size).sum();

        // Calculate average growth rate
        if !population_groups.is_empty() {
            demographics.growth_rate = population_groups
                .iter()
                .map(|pop| pop.growth_rate * (pop.size as f32))
                .sum::<f32>()
                / demographics.total_population as f32;
        } else {
            demographics.growth_rate = 0.0;
        }

        // Update cultural composition
        demographics.cultural_composition = calculate_cultural_composition(&population_groups);

        // Update social stratification
        demographics.social_stratification =
            calculate_social_stratification(&population_groups);
    }
}

/// Calculate cultural composition of a province
fn calculate_cultural_composition(population_groups: &[&PopulationGroup]) -> Vec<CulturalGroup> {
    use std::collections::HashMap;

    let total_population: u32 = population_groups.iter().map(|pop| pop.size).sum();
    if total_population == 0 {
        return Vec::new();
    }

    // Pre-allocate HashMap with capacity based on population group count
    // Most provinces have 1-5 different cultures, so use group count as upper bound
    let mut culture_counts: HashMap<crate::name_generator::Culture, u32> =
        HashMap::with_capacity(population_groups.len().min(8)); // Cap at 8 for reasonable memory usage
    for pop_group in population_groups {
        *culture_counts.entry(pop_group.culture).or_insert(0) += pop_group.size;
    }

    culture_counts
        .into_iter()
        .map(|(culture, population)| CulturalGroup {
            culture,
            population,
            percentage: (population as f32 / total_population as f32) * 100.0,
        })
        .collect()
}

/// Calculate social stratification metrics
fn calculate_social_stratification(population_groups: &[&PopulationGroup]) -> SocialStratification {
    let total_population: u32 = population_groups.iter().map(|pop| pop.size).sum();
    if total_population == 0 {
        return SocialStratification {
            nobility_percentage: 0.0,
            middle_class_percentage: 0.0,
            working_class_percentage: 0.0,
            gini_coefficient: 0.0,
        };
    }

    let nobility_pop = population_groups
        .iter()
        .filter(|pop| {
            matches!(
                pop.social_class,
                SocialClass::Nobility | SocialClass::Clergy
            )
        })
        .map(|pop| pop.size)
        .sum::<u32>();

    let middle_class_pop = population_groups
        .iter()
        .filter(|pop| matches!(pop.social_class, SocialClass::Merchants))
        .map(|pop| pop.size)
        .sum::<u32>();

    let working_class_pop = population_groups
        .iter()
        .filter(|pop| {
            matches!(
                pop.social_class,
                SocialClass::Farmers | SocialClass::Laborers | SocialClass::Slaves
            )
        })
        .map(|pop| pop.size)
        .sum::<u32>();

    SocialStratification {
        nobility_percentage: (nobility_pop as f32 / total_population as f32) * 100.0,
        middle_class_percentage: (middle_class_pop as f32 / total_population as f32) * 100.0,
        working_class_percentage: (working_class_pop as f32 / total_population as f32) * 100.0,
        gini_coefficient: calculate_gini_coefficient(population_groups),
    }
}

/// Calculate Gini coefficient for wealth inequality
fn calculate_gini_coefficient(population_groups: &[&PopulationGroup]) -> f32 {
    // Simplified Gini calculation based on social class wealth assumptions
    let wealth_distribution: Vec<f32> = population_groups
        .iter()
        .map(|pop| {
            let wealth_per_person = match pop.social_class {
                SocialClass::Nobility => 100.0,
                SocialClass::Clergy => 50.0,
                SocialClass::Merchants => 30.0,
                SocialClass::Farmers => 10.0,
                SocialClass::Laborers => 5.0,
                SocialClass::Slaves => 1.0,
            };
            wealth_per_person * pop.size as f32
        })
        .collect();

    // Simplified Gini calculation (actual Gini is more complex)
    if wealth_distribution.is_empty() {
        return 0.0;
    }

    let total_wealth: f32 = wealth_distribution.iter().sum();
    let mean_wealth = total_wealth / wealth_distribution.len() as f32;

    let mut sum_abs_diff = 0.0;
    for i in 0..wealth_distribution.len() {
        for j in 0..wealth_distribution.len() {
            sum_abs_diff += (wealth_distribution[i] - wealth_distribution[j]).abs();
        }
    }

    if mean_wealth == 0.0 {
        0.0
    } else {
        sum_abs_diff
            / (2.0
                * wealth_distribution.len() as f32
                * wealth_distribution.len() as f32
                * mean_wealth)
    }
}

// ================================================================================================
// DEMOGRAPHIC EVENTS
// ================================================================================================

/// Event fired when significant migration occurs
#[derive(Message, Debug, Clone)]
pub struct MigrationEvent {
    pub population_group: Entity,
    pub from_province: Entity,
    pub to_province: Entity,
    pub migration_size: u32,
    pub migration_type: MigrationType,
}

/// Event fired when population growth/decline is significant
#[derive(Message, Debug, Clone)]
pub struct PopulationChangeEvent {
    pub province: Entity,
    pub old_population: u32,
    pub new_population: u32,
    pub change_type: PopulationChangeType,
}

/// Event fired when demographic composition changes significantly
#[derive(Message, Debug, Clone)]
pub struct DemographicShiftEvent {
    pub province: Entity,
    pub shift_type: DemographicShiftType,
    pub magnitude: f32, // How significant the shift is
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopulationChangeType {
    NaturalGrowth,
    Immigration,
    Emigration,
    War,
    Disease,
    Famine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemographicShiftType {
    CulturalChange, // Cultural composition changed
    SocialMobility, // Social class distribution changed
    Urbanization,   // Rural to urban migration
    Ruralization,   // Urban to rural migration
}

// ================================================================================================
// VALIDATION SYSTEMS
// ================================================================================================

/// Validates population residence relationships
pub fn validate_population_residence(
    populations_query: Query<(Entity, &PopulationGroup, Option<&ResidesIn>)>,
    provinces_query: Query<Entity>,
) {
    let valid_provinces: std::collections::HashSet<Entity> = provinces_query.iter().collect();

    for (pop_entity, population, resides_in) in populations_query.iter() {
        if let Some(resides_in) = resides_in {
            if !valid_provinces.contains(&resides_in.0) {
                warn!(
                    "Population group {:?} ({}) resides in invalid province {:?}",
                    pop_entity, population.name, resides_in.0
                );
            }
        }
    }
}

/// Validates demographic data consistency
pub fn validate_demographic_consistency(demographics_query: Query<(Entity, &Demographics)>) {
    for (province_entity, demographics) in demographics_query.iter() {
        // Check that cultural composition percentages add up to ~100%
        let total_percentage: f32 = demographics
            .cultural_composition
            .iter()
            .map(|group| group.percentage)
            .sum();

        if (total_percentage - 100.0).abs() > 5.0 && demographics.total_population > 0 {
            warn!(
                "Province {:?} cultural composition adds up to {}% instead of 100%",
                province_entity, total_percentage
            );
        }

        // Check social stratification
        let social_total = demographics.social_stratification.nobility_percentage
            + demographics.social_stratification.middle_class_percentage
            + demographics.social_stratification.working_class_percentage;

        if (social_total - 100.0).abs() > 5.0 && demographics.total_population > 0 {
            warn!(
                "Province {:?} social stratification adds up to {}% instead of 100%",
                province_entity, social_total
            );
        }
    }
}
