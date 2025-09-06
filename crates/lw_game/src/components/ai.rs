//! AI Decision System - No Cheating, Adaptive Strategy
//! 
//! AI nations use the same information available to players. They genuinely try
//! to make their chosen systems work and adapt based on actual results.

use bevy::prelude::*;
use lw_core::{Fixed32, Vec2fx};
use serde::{Deserialize, Serialize};
use super::individual::*;
use super::economics::*;
use super::governance::*;
use super::governance::policy_incentives::{PolicyType, SubsidyType, TaxType, RegulationType};
use std::collections::HashMap;

/// AI personality that shapes decision-making
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct AIPersonality {
    pub strategy: AIStrategy,
    pub adaptability: Fixed32,          // 0-1, willingness to change course
    pub risk_tolerance: Fixed32,        // 0-1, comfort with uncertainty
    pub time_horizon: Fixed32,          // 0-1, short vs long-term thinking
    pub ideological_rigidity: Fixed32,  // 0-1, willingness to abandon ideology
    pub information_processing: InformationStyle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AIStrategy {
    Pragmatic {
        // Adapts system based on results
        performance_threshold: Fixed32,   // When to consider changes
        change_delay: Fixed32,           // How long to wait before changing
    },
    Ideological {
        // Maintains system regardless of results
        core_belief: Ideology,
        sacrifice_tolerance: Fixed32,    // How much suffering to accept
    },
    Opportunistic {
        // Changes system for advantage
        advantage_threshold: Fixed32,    // Minimum benefit to change
        reputation_concern: Fixed32,     // Care about being seen as flip-flopper
    },
    Conservative {
        // Preserves traditional systems
        tradition_value: Fixed32,        // How much tradition matters
        crisis_threshold: Fixed32,       // How bad before change
    },
    Revolutionary {
        // Seeks to transform everything
        transformation_speed: Fixed32,   // How fast to change
        resistance_expectation: Fixed32, // Expected opposition
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InformationStyle {
    Analytical {
        // Processes lots of data carefully
        data_requirements: Fixed32,      // How much info before decision
        accuracy_weight: Fixed32,        // Preference for reliable info
    },
    Intuitive {
        // Makes quick decisions on limited info
        pattern_recognition: Fixed32,    // Ability to see trends
        confidence_level: Fixed32,       // Trust in gut feelings
    },
    Consultative {
        // Seeks advice from others
        trusted_advisors: Vec<Entity>,   // Who they listen to
        consensus_requirement: Fixed32,  // Need agreement before action
    },
    Populist {
        // Follows popular opinion
        opinion_polling: Fixed32,        // How well they read the people
        media_influence: Fixed32,        // Susceptible to media pressure
    },
}

/// What the AI knows about its situation (same info available to player)
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct AIKnowledge {
    pub own_performance: PerformanceAssessment,
    pub neighbor_intelligence: Vec<NeighborIntel>,
    pub internal_situation: InternalAssessment,
    pub global_knowledge: GlobalKnowledge,  // Renamed from global_trends to match usage
    pub information_age: Fixed32,           // How old is this knowledge?
    pub information_reliability: Fixed32,   // How trustworthy?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceAssessment {
    pub economic_growth: Fixed32,           // Recent GDP growth
    pub population_satisfaction: Fixed32,   // How happy are people?
    pub military_strength: Fixed32,         // Relative power
    pub diplomatic_standing: Fixed32,       // International reputation
    pub internal_stability: Fixed32,        // Risk of revolt/collapse
    pub trend_direction: TrendDirection,    // Getting better/worse?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving { rate: Fixed32 },
    Stable,
    Declining { rate: Fixed32 },
    Volatile { amplitude: Fixed32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeighborIntel {
    pub nation: Entity,
    pub government_type: Option<Government>,  // Might not know details
    pub economic_system: Option<EconomicSystem>,
    pub military_estimate: Fixed32,          // Rough strength assessment
    pub relationship_history: Vec<HistoricalEvent>,
    pub threat_level: Fixed32,               // 0-1, how dangerous
    pub opportunity_level: Fixed32,          // 0-1, potential for cooperation
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoricalEvent {
    pub event_type: EventType,
    pub timestamp: Fixed32,
    pub impact: Fixed32,        // -1 to 1, negative/positive
    pub reliability: Fixed32,   // 0-1, how certain of this info
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventType {
    Trade { volume: Fixed32 },
    War { outcome: WarOutcome },
    Diplomacy { agreement_type: String },
    Cultural { exchange_type: String },
    Migration { population: Fixed32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WarOutcome {
    Victory,
    Defeat,
    Stalemate,
    Ongoing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InternalAssessment {
    pub popular_support: HashMap<SocialClass, Fixed32>, // Support by class
    pub elite_loyalty: Fixed32,                         // Are elites loyal?
    pub military_loyalty: Fixed32,                      // Will army obey?
    pub economic_problems: Vec<EconomicProblem>,        // What's wrong?
    pub social_tensions: Vec<SocialTension>,            // Internal conflicts
    pub reform_pressure: Fixed32,                       // Demand for change
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EconomicProblem {
    pub problem_type: EconomicProblemType,
    pub severity: Fixed32,       // 0-1, how bad
    pub affected_population: Fixed32, // 0-1, who's impacted
    pub duration: Fixed32,       // How long has this been a problem
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EconomicProblemType {
    Unemployment { rate: Fixed32 },
    Inflation { rate: Fixed32 },
    Shortages { goods: Vec<GoodType> },
    Inequality { gini: Fixed32 },
    Corruption { affected_sectors: Vec<IndustryType> },
    Debt { debt_to_gdp: Fixed32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SocialTension {
    pub groups: (SocialGroup, SocialGroup),  // Who's in conflict
    pub issue: String,                       // What they're fighting about
    pub intensity: Fixed32,                  // 0-1, how serious
    pub trend: TrendDirection,               // Getting worse/better?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SocialGroup {
    Class(SocialClass),
    Religion(Entity),
    Culture(Entity),
    Region(Entity),
    Age(AgeGroup),
    Profession(JobType),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgeGroup {
    Youth,
    Adult,
    Elder,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalKnowledge {
    pub technological_trends: Vec<TechTrend>,
    pub successful_policies: Vec<PolicySuccess>,  // What's working elsewhere
    pub failed_experiments: Vec<PolicyFailure>,   // What's not working
    pub trade_opportunities: Vec<TradeOpportunity>,
    pub external_threats: Vec<ExternalThreat>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TechTrend {
    pub technology: String,
    pub adoption_rate: Fixed32,     // How fast it's spreading
    pub impact_estimate: Fixed32,   // How much it changes things
    pub accessibility: Fixed32,     // Can we get it?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicySuccess {
    pub nation: Entity,
    pub policy: PolicyType,
    pub results: Vec<PolicyResult>,
    pub context_similarity: Fixed32, // How similar to our situation
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyFailure {
    pub nation: Entity,
    pub policy: PolicyType,
    pub failure_mode: FailureMode,
    pub lessons: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FailureMode {
    UnintendedConsequences,
    InsufficientResources,
    PopularResistance,
    EliteOpposition,
    ExternalPressure,
    PoorExecution,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyResult {
    pub metric: String,
    pub change: Fixed32,        // How much it improved/worsened
    pub timeframe: Fixed32,     // How long it took
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeOpportunity {
    pub partner: Entity,
    pub goods: Vec<GoodType>,
    pub estimated_benefit: Fixed32,
    pub risks: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalThreat {
    pub source: Entity,
    pub threat_type: ThreatType,
    pub severity: Fixed32,      // 0-1, how dangerous
    pub timeline: Fixed32,      // When it might happen
    pub mitigation_options: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ThreatType {
    Military,
    Economic,
    Cultural,
    Environmental,
    Technological,
}

impl AIPersonality {
    /// Make a major policy decision based on current situation
    pub fn make_policy_decision(&self, knowledge: &AIKnowledge, 
                               current_government: &Government,
                               current_economy: &EconomicSystem) -> PolicyDecision {
        
        match &self.strategy {
            AIStrategy::Pragmatic { performance_threshold, change_delay } => {
                // Check if current performance is acceptable
                let current_score = knowledge.own_performance.overall_score();
                
                if current_score < *performance_threshold {
                    // Performance is poor - consider changes
                    self.evaluate_system_changes(knowledge, current_government, current_economy)
                } else {
                    // Performance is acceptable - minor tweaks only
                    PolicyDecision::MinorReforms(self.suggest_incremental_improvements(knowledge))
                }
            },
            
            AIStrategy::Ideological { core_belief, sacrifice_tolerance } => {
                // Maintain ideological purity regardless of results
                let suffering_level = knowledge.internal_situation.calculate_suffering();
                
                if suffering_level > *sacrifice_tolerance {
                    // Even ideologues have limits
                    PolicyDecision::CrisisResponse(self.ideological_crisis_response(core_belief))
                } else {
                    // Double down on ideology
                    PolicyDecision::IdeologicalReinforcement(self.strengthen_ideology(core_belief))
                }
            },
            
            AIStrategy::Opportunistic { advantage_threshold, reputation_concern } => {
                // Look for opportunities to gain advantage
                let opportunities = self.identify_opportunities(knowledge);
                let best_opportunity = opportunities.iter()
                    .max_by(|a, b| a.expected_benefit.partial_cmp(&b.expected_benefit).unwrap());
                
                if let Some(opp) = best_opportunity {
                    if opp.expected_benefit > *advantage_threshold {
                        let reputation_cost = opp.reputation_damage * *reputation_concern;
                        if opp.expected_benefit > reputation_cost {
                            PolicyDecision::OpportunisticShift(opp.clone())
                        } else {
                            PolicyDecision::StatusQuo
                        }
                    } else {
                        PolicyDecision::StatusQuo
                    }
                } else {
                    PolicyDecision::StatusQuo
                }
            },
            
            AIStrategy::Conservative { tradition_value, crisis_threshold } => {
                // Preserve traditional systems unless crisis forces change
                let crisis_level = knowledge.internal_situation.calculate_crisis_level();
                
                if crisis_level > *crisis_threshold {
                    // TODO: Implement minimal necessary changes logic in AI system
                    PolicyDecision::ReluctantReform(vec![])
                } else {
                    PolicyDecision::TraditionReinforcement
                }
            },
            
            AIStrategy::Revolutionary { transformation_speed, resistance_expectation } => {
                // Push for rapid transformation
                let resistance_level = knowledge.internal_situation.estimate_resistance();
                let safe_transformation_rate = Fixed32::ONE - resistance_level;
                
                let actual_speed = (*transformation_speed).min(safe_transformation_rate * Fixed32::from_float(1.2));
                
                PolicyDecision::Revolution(RevolutionPlan {
                    speed: actual_speed,
                    // TODO: Implement transformation target identification in AI system
                    targets: vec![],
                    // TODO: Implement resistance suppression planning in AI system
                    resistance_management: ResistanceStrategy {
                        suppression_level: Fixed32::ZERO,
                        co_optation_level: Fixed32::from_float(0.8),
                        propaganda_level: Fixed32::from_float(0.2),
                    },
                })
            },
        }
    }
    
    fn evaluate_system_changes(&self, knowledge: &AIKnowledge, 
                              _current_government: &Government,
                              current_economy: &EconomicSystem) -> PolicyDecision {
        
        // Look at what's working elsewhere
        let successful_alternatives = knowledge.global_knowledge.successful_policies.iter()
            .filter(|success| success.context_similarity > Fixed32::from_float(0.6))
            .collect::<Vec<_>>();
        
        if let Some(best_alternative) = successful_alternatives.iter()
            .max_by(|a, b| a.results.iter().map(|r| r.change).sum::<Fixed32>()
                    .partial_cmp(&b.results.iter().map(|r| r.change).sum())
                    .unwrap()) {
            
            PolicyDecision::SystemChange(SystemChangeProposal {
                target_system: best_alternative.policy.clone(),
                expected_benefits: best_alternative.results.clone(),
                transition_plan: self.create_transition_plan(&best_alternative.policy),
                risks: self.assess_transition_risks(knowledge, &best_alternative.policy),
            })
        } else {
            // No good alternatives found - try incremental improvements
            PolicyDecision::MinorReforms(self.suggest_incremental_improvements(knowledge))
        }
    }
    
    fn suggest_incremental_improvements(&self, knowledge: &AIKnowledge) -> Vec<PolicyProposal> {
        let mut proposals = Vec::new();
        
        // Address the worst problems first
        for problem in &knowledge.internal_situation.economic_problems {
            if problem.severity > Fixed32::from_float(0.6) {
                proposals.push(self.create_problem_solution(problem));
            }
        }
        
        // Address social tensions
        for tension in &knowledge.internal_situation.social_tensions {
            if tension.intensity > Fixed32::from_float(0.5) {
                proposals.push(self.create_tension_resolution(&tension));
            }
        }
        
        proposals
    }
    
    fn create_problem_solution(&self, problem: &EconomicProblem) -> PolicyProposal {
        match &problem.problem_type {
            EconomicProblemType::Unemployment { rate } => {
                PolicyProposal {
                    policy_type: PolicyType::Subsidy {
                        subsidy_type: SubsidyType::Infrastructure,
                        amount: *rate * Fixed32::from_float(100.0),
                        conditions: vec![],
                    },
                    popularity: [(SocialClass::Laborers, Fixed32::from_float(0.8))].into(),
                    cost: *rate * Fixed32::from_float(100.0), // Scale with unemployment
                    effectiveness: Fixed32::from_float(0.7),
                    time_horizon: Fixed32::from_float(2.0), // 2 years to see results
                }
            },
            EconomicProblemType::Inflation { rate } => {
                PolicyProposal {
                    policy_type: PolicyType::Tax {
                        tax_type: TaxType::Sales,
                        rate: *rate * Fixed32::from_float(0.5),
                        exemptions: vec![],
                    },
                    popularity: [(SocialClass::Merchants, Fixed32::from_float(-0.3))].into(),
                    cost: Fixed32::ZERO, // Taxes generate revenue
                    effectiveness: Fixed32::from_float(0.6),
                    time_horizon: Fixed32::from_float(1.0),
                }
            },
            // Handle other problem types...
            _ => PolicyProposal {
                policy_type: PolicyType::Regulation { 
                    regulation_type: RegulationType::Competition,
                    strictness: Fixed32::from_float(0.5),
                    penalties: Vec::new(),
                },
                popularity: HashMap::new(),
                cost: Fixed32::from_float(50.0),
                effectiveness: Fixed32::from_float(0.5),
                time_horizon: Fixed32::from_float(1.5),
            }
        }
    }
    
    fn create_tension_resolution(&self, _tension: &SocialTension) -> PolicyProposal {
        // Create policies to address social tensions
        PolicyProposal {
            policy_type: PolicyType::Regulation { 
                regulation_type: RegulationType::Labor,
                strictness: Fixed32::from_float(0.7),
                penalties: Vec::new(),
            },
            popularity: HashMap::new(),
            cost: Fixed32::from_float(30.0),
            effectiveness: Fixed32::from_float(0.4),
            time_horizon: Fixed32::from_float(3.0),
        }
    }
    
    fn create_transition_plan(&self, _target_policy: &PolicyType) -> TransitionPlan {
        TransitionPlan {
            phases: vec![
                TransitionPhase {
                    description: "Preparation".to_string(),
                    duration: Fixed32::from_float(0.5),
                    requirements: Vec::new(),
                },
                TransitionPhase {
                    description: "Implementation".to_string(),
                    duration: Fixed32::from_float(1.0),
                    requirements: Vec::new(),
                },
                TransitionPhase {
                    description: "Stabilization".to_string(),
                    duration: Fixed32::from_float(0.5),
                    requirements: Vec::new(),
                },
            ],
            total_cost: Fixed32::from_float(200.0),
            success_probability: Fixed32::from_float(0.7),
        }
    }
    
    fn assess_transition_risks(&self, _knowledge: &AIKnowledge, _policy: &PolicyType) -> Vec<TransitionRisk> {
        vec![
            TransitionRisk {
                risk_type: "Popular Resistance".to_string(),
                probability: Fixed32::from_float(0.3),
                impact: Fixed32::from_float(0.5),
                mitigation: "Gradual Implementation".to_string(),
            }
        ]
    }
    
    // Placeholder implementations for other strategy methods...
    fn ideological_crisis_response(&self, _belief: &Ideology) -> CrisisResponse {
        CrisisResponse {
            emergency_measures: Vec::new(),
            temporary_compromises: Vec::new(),
            ideological_justification: "Necessary Evil".to_string(),
        }
    }
    
    fn strengthen_ideology(&self, _belief: &Ideology) -> IdeologicalReinforcement {
        IdeologicalReinforcement {
            propaganda_campaign: Fixed32::from_float(0.5),
            purge_opponents: Fixed32::from_float(0.2),
            reward_supporters: Fixed32::from_float(0.8),
        }
    }
    
    fn identify_opportunities(&self, _knowledge: &AIKnowledge) -> Vec<Opportunity> {
        Vec::new() // Placeholder
    }
}

#[derive(Clone, Debug)]
pub enum PolicyDecision {
    StatusQuo,
    MinorReforms(Vec<PolicyProposal>),
    SystemChange(SystemChangeProposal),
    CrisisResponse(CrisisResponse),
    IdeologicalReinforcement(IdeologicalReinforcement),
    OpportunisticShift(Opportunity),
    ReluctantReform(Vec<PolicyProposal>),
    TraditionReinforcement,
    Revolution(RevolutionPlan),
}

#[derive(Clone, Debug)]
pub struct SystemChangeProposal {
    pub target_system: PolicyType,
    pub expected_benefits: Vec<PolicyResult>,
    pub transition_plan: TransitionPlan,
    pub risks: Vec<TransitionRisk>,
}

#[derive(Clone, Debug)]
pub struct TransitionPlan {
    pub phases: Vec<TransitionPhase>,
    pub total_cost: Fixed32,
    pub success_probability: Fixed32,
}

#[derive(Clone, Debug)]
pub struct TransitionPhase {
    pub description: String,
    pub duration: Fixed32,
    pub requirements: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct TransitionRisk {
    pub risk_type: String,
    pub probability: Fixed32,
    pub impact: Fixed32,
    pub mitigation: String,
}

#[derive(Clone, Debug)]
pub struct CrisisResponse {
    pub emergency_measures: Vec<PolicyType>,
    pub temporary_compromises: Vec<String>,
    pub ideological_justification: String,
}

#[derive(Clone, Debug)]
pub struct IdeologicalReinforcement {
    pub propaganda_campaign: Fixed32,
    pub purge_opponents: Fixed32,
    pub reward_supporters: Fixed32,
}

#[derive(Clone, Debug)]
pub struct Opportunity {
    pub description: String,
    pub expected_benefit: Fixed32,
    pub reputation_damage: Fixed32,
    pub implementation_difficulty: Fixed32,
}

#[derive(Clone, Debug)]
pub struct RevolutionPlan {
    pub speed: Fixed32,
    pub targets: Vec<String>,
    pub resistance_management: ResistanceStrategy,
}

#[derive(Clone, Debug)]
pub struct ResistanceStrategy {
    pub suppression_level: Fixed32,
    pub co_optation_level: Fixed32,
    pub propaganda_level: Fixed32,
}

impl PerformanceAssessment {
    pub fn overall_score(&self) -> Fixed32 {
        (self.economic_growth + self.population_satisfaction + 
         self.military_strength + self.diplomatic_standing + self.internal_stability) / Fixed32::from_num(5)
    }
}

impl InternalAssessment {
    pub fn calculate_suffering(&self) -> Fixed32 {
        let economic_suffering: Fixed32 = self.economic_problems.iter()
            .map(|p| p.severity * p.affected_population)
            .sum();
        
        let social_suffering: Fixed32 = self.social_tensions.iter()
            .map(|t| t.intensity)
            .sum::<Fixed32>() / Fixed32::from_float(self.social_tensions.len().max(1) as f32);
        
        (economic_suffering + social_suffering) / Fixed32::from_num(2)
    }
    
    pub fn calculate_crisis_level(&self) -> Fixed32 {
        let instability = Fixed32::ONE - self.military_loyalty.min(self.elite_loyalty);
        let unrest = self.reform_pressure;
        let problems = self.calculate_suffering();
        
        (instability + unrest + problems) / Fixed32::from_num(3)
    }
    
    pub fn estimate_resistance(&self) -> Fixed32 {
        let elite_resistance = Fixed32::ONE - self.elite_loyalty;
        let popular_resistance = self.reform_pressure; // High reform pressure = resistance to change
        let military_risk = Fixed32::ONE - self.military_loyalty;
        
        (elite_resistance + popular_resistance + military_risk) / Fixed32::from_num(3)
    }
}