//! Errors that can occur during the game.

// NOTE: This file was pretty much 1 to 1 ripped from an older version of this project.
// Most other files have had extensive refactoring done, however I haven't gotten around to doing
// the same here. This doesn't really cause any issues, but there might be some errors that overlap
// in usecase, or some errors that aren't used at all.

// Do not display error message directly to user.
// Instead call associated trait function, UserFriendlyError::get_user_error(), giving "in-game" error explaination
use rand::{rng, seq::IndexedRandom};
use thiserror::Error;
use crate::models::Direction;

/// Creates an in-game error message to display to users when something goes wrong.
pub trait UserFriendlyError {
    fn get_user_error(&self) -> String;
}

/// All errors that can occur in the game
#[derive(Debug, PartialEq, Error)]
pub enum GameError {
    #[error("{0}")]
    Parse(#[from] ParseError),

    #[error("{0}")]
    Action(#[from] ActionError),

    #[error("{0}")]
    World(#[from] WorldError),

    #[error("Unknown error.")]
    Unknown,
}

/// Errors when attempting to create an [`Action`](crate::engine::action::Action)
#[derive(Debug, Error, PartialEq)]
pub enum ActionError {
    #[error("I don't see a '{0}' here")]
    ItemNotFound(String),

    #[error("You don't have a '{0}'")]
    ItemNotFoundInventory(String),

    #[error("I don't understand the direction '{0}'")]
    DirectionNotFound(String),

    #[error("Which {0}?")]
    AmbiguousItem(String),

    #[error("You can't go that way")]
    NoExit(Direction),

    #[error("You can't see that")]
    ItemNotVisible(i32),

    #[error("You can't take that")]
    ItemNotTakeable(i32),

    #[error("{0}")]
    InvalidCommand(String),
}

/// Errors when parsing input from the user
#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    /// Player did not provide a direction
    #[error("No direction given")]
    NoDirection,

    /// Player attempted to move somewhere that is not a direction
    #[error("No direction given")]
    NotADirection(String),

    /// Player provided no input
    #[error("User did not enter an input")]
    EmptyInput,

    /// Player tried to use action without an invalid target.
    /// Field 0 specifies the text of the target that was given from the player.
    #[error("Invalid target {0}")]
    InvalidTarget(String),

    /// Player tried to use action without a target.
    /// Field 0 specifies which action was used as a string
    #[error("No such target to {0}")]
    NoTarget(String),

    /// Player attempted to reference multiple nouns incorrectly
    #[error("Seperator given without following noun.")]
    InvalidList,

    /// Player entered a phrase that is not understood by the game engine.
    #[error("User entered invalid phrase {0}")]
    UnknownPhrase(String),

    #[error("An article was found in an invalid position of the parsed phrase.")]
    UnexpectedArticlePosition,

    /// Player entered a phrase that is not understood by the game engine as the first word of a phrase.
    #[error("User entered invalid verb {0}")]
    UnknownVerb(String),

    /// Player entetered nonsense into input
    #[error("Could not parse string")]
    NotParseable // HACK: Do not use this variant when possible as data is not stored
}

impl ParseError {
    /// If [`self`] is [`ParseError::NoTarget`], replaces the contents of the inner field with the given string.
    pub fn specify_target_action(&mut self, act: String) {
        if let ParseError::NoTarget(s) = self {
            *s = act;
        }
    }

}

/// Errors when interacting with the game world
#[derive(Debug, Error, PartialEq)]
pub enum WorldError {
    /// Failed to find an entity by id.
    /// Intended to be used when the id does not exist in the system.
    #[error("The requested entity does not exist.")]
    NoSuchEntity(i32),

    /// Player attempted to grab an item they are holding
    #[error("Cannot pick up an item that is already held.")]
    AlreadyHeld,

    /// Failed to find an entity by name
    #[error("The requested entity does not exist.")]
    NoSuchNamedEntity(String),

    /// An invalid direction is a direction that a user cannot traverse to
    #[error("Player provided a direction leading to nowhere: {0}")]
    InvalidDirection(Direction),

    /// An error when interacting with an item
    #[error("{0}")]
    ItemError(#[from] ItemError),
}

/// Errors that can occur when interacting with items
#[derive(Debug, Error,PartialEq)]
pub enum ItemError {
    /// Attempted to use an item as a container
    #[error("Entity is not a container.")]
    NotAContainer,

    /// Player attempted open
    #[error("Entity is not openable.")]
    NotOpenable,

    /// Attempted to pick up a non-holdable item
    #[error("Entity is not holdable.")]
    NotAHoldable,

    /// Occurs if an item is attempted to be removed from a location and the location is the game world.
    #[error("Cannot remove an item from the game!")]
    RemoveFromGame,
}

impl From<ItemError> for GameError {
    fn from(err: ItemError) -> Self {GameError::World(WorldError::ItemError(err))}
}

// User friendly messages
impl UserFriendlyError for GameError {
    fn get_user_error(&self) -> String {
        match self {
            GameError::Parse(e) => e.get_user_error(),
            GameError::Action(e) => e.get_user_error(),
            GameError::World(e) => e.get_user_error(),
            GameError::Unknown => "That wasn't supposed to happen...".to_string()
        }
    }
}

impl UserFriendlyError for ActionError {
    fn get_user_error(&self) -> String {
        match self {
            ActionError::ItemNotFound(name) => format!("I don't see a '{}' here", name),
            ActionError::ItemNotFoundInventory(name) => format!("You don't have a '{}'", name),
            ActionError::DirectionNotFound(dir) => format!("I don't understand the direction '{}'", dir),
            ActionError::AmbiguousItem(name) => format!("Which {}?", name),
            ActionError::NoExit(dir) => format!("You can't go {:?}", dir).to_lowercase(),
            ActionError::ItemNotVisible(_) => "You can't see that".to_string(),
            ActionError::ItemNotTakeable(_) => "You can't take that".to_string(),
            ActionError::InvalidCommand(msg) => msg.clone(),
        }
    }
}

impl UserFriendlyError for ParseError {
    fn get_user_error(&self) -> String {
        match self {
            ParseError::UnknownPhrase(phrase) => format!("I understood what you meant until {phrase}"),
            ParseError::NoTarget(act) => format!("What do you want to {act}?"),
            ParseError::InvalidTarget(_) => "You can't see any such thing".to_string(),
            ParseError::NoDirection => format!("You'll have to say which compass direction to go in..."),
            ParseError::NotADirection(word) => format!("You cannot go '{word}'..."),
            ParseError::EmptyInput => "I beg your pardon?".to_string(),
            ParseError::NotParseable => "I dont understand what you mean...".to_string(),
            ParseError::InvalidList => format!("..."), // TODO:
            ParseError::UnknownVerb(_v) => "That is not a verb I recognise.".to_string(),
            ParseError::UnexpectedArticlePosition => "I dont think that's how you refer to that.".to_string(),
        }
    }
}

impl UserFriendlyError for WorldError {
    fn get_user_error(&self) -> String {
        match self {
            WorldError::NoSuchEntity(_) => format!("You cant see any such thing."),
            WorldError::NoSuchNamedEntity(_) => format!("You cant see any such thing."),
            WorldError::InvalidDirection(dir) => format!("You cannot go {dir}"),
            WorldError::ItemError(err) => err.get_user_error(),
            WorldError::AlreadyHeld => "You already have that.".to_string(),
        }
    }
}

impl UserFriendlyError for ItemError {
    fn get_user_error(&self) -> String {
        match self {
            ItemError::NotOpenable => "That is not something you can open".to_string(),
            ItemError::NotAContainer => format!("You cant put that in there..."),
            ItemError::NotAHoldable => cant_do_that(),
            ItemError::RemoveFromGame => format!("..."), // TODO:
        }
    }
}

/// Returns a random message telling the user they cannot do that action
pub fn cant_do_that() -> String {
    let phrases = [
        "Not a prayer.",
        "An interesting idea...",
        "You can't be serious.",
        "Not likely.",
        "A valiant attmept.",
        "What a concept!",
    ];

    // Pick a random phrase
    phrases.choose(&mut rng())
        .expect("Could not choose from phrases array...")
        .to_string()
}
