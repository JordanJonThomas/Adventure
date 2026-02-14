//! The condition module handles any conditional logic a map designer may want to implement. 

pub mod predicate;
pub mod effect;

pub use predicate::ConditionPredicate;
pub use effect::{ConditionEffect, ConditionResult};

/// A ConditionTrigger is a set of predicates and effects that
/// can be attacked to a game object to evaluate and modify game state.
#[derive(Debug)]
pub struct ConditionTrigger {
    /// A condition to be evaluated. If true, the effects will be applied to game state.
    pub predicates: ConditionPredicate,
    /// A list of effects to execute against the game if the ConditionPredicate succeeds
    pub effects: Vec<ConditionEffect>
}
