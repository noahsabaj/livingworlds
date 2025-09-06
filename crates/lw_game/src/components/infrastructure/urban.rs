//! Urban infrastructure components
//!
//! Cities, towns, and urban development infrastructure.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// City component - major urban center
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub population: u32,
    pub area_km2: Fixed32,
    pub density: Fixed32,
    pub development_level: UrbanDevelopmentLevel,
    pub districts: Vec<Entity>,  // References to district entities
}

/// Town component - medium urban center
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Town {
    pub population: u32,
    pub market_importance: Fixed32,
    pub administrative_role: AdministrativeRole,
    pub growth_rate: Fixed32,
}

/// Village component - small settlement
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Village {
    pub population: u32,
    pub agricultural_focus: bool,
    pub crafts_specialization: Option<String>,
    pub market_day: Option<u8>,  // Day of week/month
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UrbanDevelopmentLevel {
    Primitive,
    Medieval,
    Renaissance,
    Industrial,
    Modern,
    PostModern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdministrativeRole {
    None,
    Local,
    Regional,
    Provincial,
    National,
}

/// Urban development component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct UrbanDevelopment {
    pub building_density: Fixed32,
    pub infrastructure_quality: Fixed32,
    pub public_services: PublicServices,
    pub zoning: UrbanZoning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicServices {
    pub education_coverage: Fixed32,
    pub healthcare_coverage: Fixed32,
    pub police_coverage: Fixed32,
    pub fire_coverage: Fixed32,
    pub public_transport: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanZoning {
    pub residential_percentage: Fixed32,
    pub commercial_percentage: Fixed32,
    pub industrial_percentage: Fixed32,
    pub public_percentage: Fixed32,
    pub green_space_percentage: Fixed32,
}

/// Housing infrastructure
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Housing {
    pub housing_type: HousingType,
    pub units: u32,
    pub average_quality: Fixed32,
    pub occupancy_rate: Fixed32,
    pub affordability_index: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HousingType {
    Slums,
    Tenements,
    Apartments,
    Townhouses,
    SingleFamily,
    Luxury,
}

/// Commercial district component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CommercialDistrict {
    pub shop_count: u32,
    pub market_squares: u32,
    pub warehouse_capacity: Fixed32,
    pub foot_traffic: Fixed32,
    pub prosperity_level: Fixed32,
}

/// Industrial zone component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IndustrialZone {
    pub factory_count: u32,
    pub production_capacity: Fixed32,
    pub pollution_level: Fixed32,
    pub worker_housing_proximity: Fixed32,
    pub transport_connectivity: Fixed32,
}

/// Public building component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PublicBuilding {
    pub building_type: PublicBuildingType,
    pub capacity: u32,
    pub maintenance_state: Fixed32,
    pub utilization_rate: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicBuildingType {
    TownHall,
    Court,
    School,
    Hospital,
    Library,
    Museum,
    Theater,
    Stadium,
    Market,
    PostOffice,
}

/// Park and green space component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct GreenSpace {
    pub space_type: GreenSpaceType,
    pub area_hectares: Fixed32,
    pub maintenance_quality: Fixed32,
    pub biodiversity: Fixed32,
    pub recreational_value: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GreenSpaceType {
    Park,
    Garden,
    Plaza,
    Cemetery,
    NatureReserve,
    Waterfront,
}

/// Slum component - informal settlements
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Slum {
    pub population_density: Fixed32,
    pub sanitation_access: Fixed32,
    pub crime_rate: Fixed32,
    pub informal_economy_size: Fixed32,
    pub upgrade_potential: Fixed32,
}

/// Urban sprawl component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct UrbanSprawl {
    pub growth_direction: Vec2,
    pub expansion_rate: Fixed32,
    pub infrastructure_lag: Fixed32,
    pub agricultural_land_loss: Fixed32,
}