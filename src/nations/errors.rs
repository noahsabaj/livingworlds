//! Error types for nation systems
//!
//! Provides comprehensive error handling with graceful recovery strategies,
//! replacing panics and unwraps with proper error propagation.

use thiserror::Error;
use bevy::prelude::Entity;

// NationId deleted - now using Entity directly
use super::governance::{GovernmentType, GovernmentCategory};
use super::laws::{LawId, LawPrerequisite};

/// Main error type for nation operations
#[derive(Error, Debug)]
pub enum NationError {
    /// Government transition errors
    #[error("Government transition error: {0}")]
    Transition(#[from] TransitionError),

    /// Law system errors
    #[error("Law system error: {0}")]
    Law(#[from] LawError),

    /// Territory management errors
    #[error("Territory error: {0}")]
    Territory(#[from] TerritoryError),

    /// Diplomatic errors
    #[error("Diplomatic error: {0}")]
    Diplomatic(#[from] DiplomaticError),

    /// Economic errors
    #[error("Economic error: {0}")]
    Economic(#[from] EconomicError),

    /// Military errors
    #[error("Military error: {0}")]
    Military(#[from] MilitaryError),

    /// Data integrity errors
    #[error("Data integrity error: {0}")]
    Integrity(#[from] IntegrityError),

    /// Entity not found
    #[error("Nation entity {0:?} not found")]
    EntityNotFound(Entity),

    /// Nation ID not found
    #[error("Nation with ID {0:?} not found")]
    NationNotFound(NationId),

    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// Government transition specific errors
#[derive(Error, Debug)]
pub enum TransitionError {
    /// Invalid government type transition
    #[error("Invalid transition from {from:?} to {to:?}: {reason}")]
    InvalidTransition {
        from: GovernmentType,
        to: GovernmentType,
        reason: String,
    },

    /// Insufficient legitimacy for transition
    #[error("Insufficient legitimacy ({current:.2}) for transition (required: {required:.2})")]
    InsufficientLegitimacy { current: f32, required: f32 },

    /// Transition on cooldown
    #[error("Cannot transition: cooldown active for {remaining:.1} more days")]
    TransitionCooldown { remaining: f32 },

    /// Stability too low
    #[error("Stability too low ({current:.2}) for peaceful transition (minimum: {required:.2})")]
    StabilityTooLow { current: f32, required: f32 },

    /// Revolutionary conditions not met
    #[error("Revolutionary conditions not met: fervor={fervor:.2}, required={required:.2}")]
    RevolutionConditionsNotMet { fervor: f32, required: f32 },

    /// Coup failed
    #[error("Coup attempt failed: military strength={strength:.2}, threshold={threshold:.2}")]
    CoupFailed { strength: f32, threshold: f32 },
}

/// Law system specific errors
#[derive(Error, Debug)]
pub enum LawError {
    /// Law not found
    #[error("Law {0:?} not found in registry")]
    LawNotFound(LawId),

    /// Prerequisites not met
    #[error("Prerequisites not met for law {law:?}: {missing:?}")]
    PrerequisitesNotMet {
        law: LawId,
        missing: Vec<LawPrerequisite>,
    },

    /// Conflicting laws active
    #[error("Law {proposed:?} conflicts with active laws: {conflicts:?}")]
    ConflictingLaws {
        proposed: LawId,
        conflicts: Vec<LawId>,
    },

    /// Law already active
    #[error("Law {0:?} is already active")]
    AlreadyActive(LawId),

    /// Cannot repeal constitutional law
    #[error("Cannot repeal constitutional law {0:?} without special procedures")]
    ConstitutionalRepeal(LawId),

    /// Insufficient support for passage
    #[error("Insufficient support ({support:.2}) for law {law:?} (required: {required:.2})")]
    InsufficientSupport {
        law: LawId,
        support: f32,
        required: f32,
    },

    /// Law on cooldown
    #[error("Law {0:?} cannot be proposed again for {1:.1} days")]
    LawCooldown(LawId, f32),

    /// Government incompatible with law
    #[error("Government type {government:?} incompatible with law {law:?}")]
    IncompatibleGovernment {
        government: GovernmentCategory,
        law: LawId,
    },
}

/// Territory management errors
#[derive(Error, Debug)]
pub enum TerritoryError {
    /// Province not owned
    #[error("Province {province:?} not owned by nation {nation:?}")]
    ProvinceNotOwned { province: u32, nation: NationId },

    /// Cannot set capital
    #[error("Cannot set capital to province {0}: {1}")]
    InvalidCapital(u32, String),

    /// Territory disconnected
    #[error("Territory would become disconnected by losing province {0}")]
    TerritoryDisconnected(u32),

    /// Insufficient provinces
    #[error("Nation requires at least {required} provinces (has {current})")]
    InsufficientProvinces { required: usize, current: usize },

    /// Province already owned
    #[error("Province {0} already owned by another nation")]
    ProvinceAlreadyOwned(u32),
}

/// Diplomatic errors
#[derive(Error, Debug)]
pub enum DiplomaticError {
    /// Already allied
    #[error("Already allied with nation {0:?}")]
    AlreadyAllied(NationId),

    /// Already at war
    #[error("Already at war with nation {0:?}")]
    AlreadyAtWar(NationId),

    /// Cannot declare war on ally
    #[error("Cannot declare war on allied nation {0:?}")]
    AllyProtection(NationId),

    /// Peace treaty still active
    #[error("Peace treaty with {nation:?} active for {remaining:.1} more days")]
    PeaceTreatyActive { nation: NationId, remaining: f32 },

    /// Insufficient reputation
    #[error("Insufficient diplomatic reputation ({current:.2}) for action (required: {required:.2})")]
    InsufficientReputation { current: f32, required: f32 },

    /// Cannot self-target
    #[error("Cannot perform diplomatic action on self")]
    SelfTarget,
}

/// Economic errors
#[derive(Error, Debug)]
pub enum EconomicError {
    /// Insufficient treasury
    #[error("Insufficient treasury ({available:.0}) for cost ({required:.0})")]
    InsufficientFunds { available: f32, required: f32 },

    /// Trade route blocked
    #[error("Trade route to {0:?} is blocked")]
    TradeRouteBlocked(NationId),

    /// Economic collapse imminent
    #[error("Economic collapse imminent: debt={debt:.0}, income={income:.0}")]
    EconomicCollapse { debt: f32, income: f32 },

    /// Resource depleted
    #[error("Resource {resource} depleted in province {province}")]
    ResourceDepleted { resource: String, province: u32 },
}

/// Military errors
#[derive(Error, Debug)]
pub enum MilitaryError {
    /// Insufficient military strength
    #[error("Insufficient military strength ({available:.0}) for operation ({required:.0})")]
    InsufficientStrength { available: f32, required: f32 },

    /// Army not found
    #[error("Army {0:?} not found")]
    ArmyNotFound(Entity),

    /// Cannot move army
    #[error("Cannot move army to province {0}: {1}")]
    InvalidMovement(u32, String),

    /// Conscription failed
    #[error("Conscription failed: available={available}, required={required}")]
    ConscriptionFailed { available: u32, required: u32 },
}

/// Data integrity errors
#[derive(Error, Debug)]
pub enum IntegrityError {
    /// Orphaned entity
    #[error("Orphaned entity {0:?} with no valid references")]
    OrphanedEntity(Entity),

    /// Duplicate nation ID
    #[error("Duplicate nation ID {0:?} detected")]
    DuplicateNationId(NationId),

    /// Invalid state
    #[error("Invalid state detected: {0}")]
    InvalidState(String),

    /// Cache desync
    #[error("Cache desynchronized: {cache_type} version mismatch")]
    CacheDesync { cache_type: String },

    /// Relationship inconsistency
    #[error("Relationship inconsistency: {0}")]
    RelationshipInconsistency(String),
}

/// Result type for nation operations
pub type NationResult<T> = Result<T, NationError>;

/// Recovery strategies for different error types
pub enum RecoveryStrategy {
    /// Retry the operation after a delay
    RetryAfter(f32),

    /// Fall back to a default value
    UseDefault,

    /// Cancel the operation and log
    CancelAndLog,

    /// Attempt automatic repair
    AttemptRepair,

    /// Escalate to user intervention
    RequireUserIntervention,

    /// Fail gracefully with partial results
    PartialSuccess,
}

impl NationError {
    /// Suggest a recovery strategy for this error
    pub fn recovery_strategy(&self) -> RecoveryStrategy {
        match self {
            // Transitions can be retried after cooldown
            NationError::Transition(TransitionError::TransitionCooldown { remaining }) => {
                RecoveryStrategy::RetryAfter(*remaining)
            }

            // Law conflicts need user intervention
            NationError::Law(LawError::ConflictingLaws { .. }) => {
                RecoveryStrategy::RequireUserIntervention
            }

            // Economic issues might auto-resolve
            NationError::Economic(EconomicError::InsufficientFunds { .. }) => {
                RecoveryStrategy::RetryAfter(5.0)
            }

            // Integrity errors need repair
            NationError::Integrity(_) => RecoveryStrategy::AttemptRepair,

            // Most others cancel and log
            _ => RecoveryStrategy::CancelAndLog,
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        !matches!(
            self,
            NationError::Integrity(_) | NationError::EntityNotFound(_)
        )
    }

    /// Get severity level (for logging/alerting)
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            NationError::Integrity(_) => ErrorSeverity::Critical,
            NationError::EntityNotFound(_) | NationError::NationNotFound(_) => {
                ErrorSeverity::Error
            }
            NationError::Economic(EconomicError::EconomicCollapse { .. }) => {
                ErrorSeverity::Warning
            }
            _ => ErrorSeverity::Info,
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational - expected gameplay events
    Info,

    /// Warning - concerning but not critical
    Warning,

    /// Error - operation failed but game continues
    Error,

    /// Critical - data integrity or crash risk
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_recovery_strategies() {
        let cooldown_error = NationError::Transition(
            TransitionError::TransitionCooldown { remaining: 10.0 }
        );

        match cooldown_error.recovery_strategy() {
            RecoveryStrategy::RetryAfter(delay) => assert_eq!(delay, 10.0),
            _ => panic!("Expected RetryAfter strategy"),
        }
    }

    #[test]
    fn test_error_severity() {
        let integrity_error = NationError::Integrity(
            IntegrityError::InvalidState("test".to_string())
        );
        assert_eq!(integrity_error.severity(), ErrorSeverity::Critical);

        let funds_error = NationError::Economic(
            EconomicError::InsufficientFunds {
                available: 100.0,
                required: 200.0,
            }
        );
        assert_eq!(funds_error.severity(), ErrorSeverity::Info);
    }

    #[test]
    fn test_error_recoverability() {
        let recoverable = NationError::Law(LawError::LawCooldown(LawId::new(1), 5.0));
        assert!(recoverable.is_recoverable());

        let non_recoverable = NationError::EntityNotFound(Entity::PLACEHOLDER);
        assert!(!non_recoverable.is_recoverable());
    }
}