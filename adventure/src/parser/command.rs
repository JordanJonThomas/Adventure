use crate::parser::grammar::{NounPhrase, Preposition, Verb};

/// A command is an action intent stated by the player.
///
/// A command does not have context to the game world, meaning
/// invalid actions can be defined.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    /// The intended action.
    pub verb: Verb,
    /// The target(s) to execute the action against.
    pub direct: Vec<NounPhrase>,
    /// The relation between the direct and indirect targets.
    pub preposition: Option<Preposition>,
    /// The secondary target for an action.
    pub indirect: Option<NounPhrase>,
}

