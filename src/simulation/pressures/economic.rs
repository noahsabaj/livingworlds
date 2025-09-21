//! Economic pressure calculations
//!
//! Economic pressure drives trade, resource exploitation, and taxation policies.

use super::types::PressureLevel;
use crate::nations::Nation;
use crate::world::Province;

/// Economic pressures affecting a nation
#[derive(Debug, Clone)]
pub struct EconomicPressure {
    /// Treasury running low (drives taxation or raiding)
    pub treasury_shortage: PressureLevel,
    /// Trade imbalance (drives trade route development)
    pub trade_deficit: PressureLevel,
    /// Resource scarcity (drives expansion to resource-rich areas)
    pub resource_scarcity: PressureLevel,
    /// Infrastructure costs exceeding income
    pub maintenance_burden: PressureLevel,
}

/// Calculate economic pressures for a nation
pub fn calculate_economic_pressure(
    nation: &Nation,
    controlled_provinces: &[&Province],
) -> EconomicPressure {
    // Treasury pressure based on current funds vs needs
    let treasury_minimum = controlled_provinces.len() as f32 * 100.0; // 100 gold per province minimum
    let treasury_ratio = nation.treasury / treasury_minimum.max(1.0);
    let treasury_shortage = if treasury_ratio < 1.0 {
        PressureLevel::new(1.0 - treasury_ratio)
    } else {
        PressureLevel::NONE
    };

    // Trade deficit (simplified - will expand with trade routes)
    let trade_balance = nation.treasury * 0.1; // Placeholder: 10% of treasury as trade income
    let trade_needs = controlled_provinces.len() as f32 * 50.0;
    let trade_ratio = trade_balance / trade_needs.max(1.0);
    let trade_deficit = if trade_ratio < 1.0 {
        PressureLevel::new(1.0 - trade_ratio)
    } else {
        PressureLevel::NONE
    };

    // Resource scarcity - check for strategic resources
    let has_iron = controlled_provinces.iter().any(|p| p.iron.has_any());
    let has_gold = controlled_provinces.iter().any(|p| p.gold.has_any());
    let resource_score = (has_iron as u8 + has_gold as u8) as f32 / 2.0;
    let resource_scarcity = PressureLevel::new(1.0 - resource_score);

    // Infrastructure maintenance burden
    let infrastructure_cost = controlled_provinces.len() as f32 * 20.0; // 20 gold per province
    let income = calculate_nation_income(controlled_provinces);
    let maintenance_ratio = infrastructure_cost / income.max(1.0);
    let maintenance_burden = if maintenance_ratio > 0.5 {
        PressureLevel::new((maintenance_ratio - 0.5) * 2.0) // Scale 0.5-1.0 to 0.0-1.0
    } else {
        PressureLevel::NONE
    };

    EconomicPressure {
        treasury_shortage,
        trade_deficit,
        resource_scarcity,
        maintenance_burden,
    }
}

/// Calculate basic income for a nation from its provinces
fn calculate_nation_income(provinces: &[&Province]) -> f32 {
    provinces
        .iter()
        .map(|p| {
            let base_tax = p.population as f32 * 0.01; // 0.01 gold per person
            let trade_income = p.agriculture.value() * 10.0; // Agricultural surplus trade
            let resource_income = calculate_resource_income(p);
            base_tax + trade_income + resource_income
        })
        .sum()
}

/// Calculate income from province resources
fn calculate_resource_income(province: &Province) -> f32 {
    let mut income = 0.0;
    income += province.iron.value() as f32 * 0.5;
    income += province.copper.value() as f32 * 0.3;
    income += province.gold.value() as f32 * 2.0;
    income += province.gems.value() as f32 * 5.0;
    income += province.stone.value() as f32 * 0.1;
    income
}

/// Economic actions to address pressures
#[derive(Debug, Clone)]
pub enum EconomicAction {
    /// Increase taxation
    RaiseTaxes { rate_increase: f32 },
    /// Seek resource-rich territories
    SeekResources { target_resource: ResourceTarget },
    /// Establish trade routes
    EstablishTrade { priority_partners: Vec<u32> },
    /// Reduce infrastructure spending
    CutSpending { reduction_percentage: f32 },
    /// Raid neighbors for wealth
    RaidForWealth { desperation_level: f32 },
}

#[derive(Debug, Clone)]
pub enum ResourceTarget {
    Gold,
    Iron,
    Agricultural,
    Any,
}

/// Determine economic action based on pressures
pub fn resolve_economic_pressure(pressure: &EconomicPressure) -> Option<EconomicAction> {
    // Critical treasury shortage - immediate action needed
    if pressure.treasury_shortage.is_critical() {
        return Some(EconomicAction::RaidForWealth {
            desperation_level: pressure.treasury_shortage.value(),
        });
    }

    // High resource scarcity - seek expansion
    if pressure.resource_scarcity.is_high() {
        return Some(EconomicAction::SeekResources {
            target_resource: ResourceTarget::Any,
        });
    }

    // Moderate treasury issues - raise taxes
    if pressure.treasury_shortage.is_moderate() {
        return Some(EconomicAction::RaiseTaxes {
            rate_increase: pressure.treasury_shortage.value() * 0.2,
        });
    }

    // Trade deficit - establish routes
    if pressure.trade_deficit.is_moderate() {
        return Some(EconomicAction::EstablishTrade {
            priority_partners: Vec::new(), // System will find partners
        });
    }

    // Maintenance burden - cut spending
    if pressure.maintenance_burden.is_high() {
        return Some(EconomicAction::CutSpending {
            reduction_percentage: pressure.maintenance_burden.value() * 0.3,
        });
    }

    None
}
