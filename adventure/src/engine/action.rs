use crate::models::Direction;

/// An action is a resolved player interaction with the game.
///
/// Actions are created from a parsed command, and require proper
/// game context to be constructed.
///
/// An action uses entity ID's to reference what object the player is attempting to interact with.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// The player wants to move somewhere.
    Move { dir: Direction },
    /// The player wants to get items.
    Get { items: Vec<i32> },
    /// The player wants to drop items.
    Drop { items: Vec<i32> },
    /// The player wants to put an item in a container.
    Put { item: i32, container: i32 },
    /// The player is looking at an item or the surrounding area.
    Look { target: Option<i32> },
    /// The player is opening something.
    Open { target: i32 },
    /// The player is closing something.
    Close { target: i32 },
    /// The player is attacking something.
    /// Optionally, an item to attack with may be specified
    Attack { target: i32, weapon: Option<i32> },
    /// The player is unlocking something with a key.
    Unlock { target: i32, key: i32 },
    /// The player is trying to read something.
    Read { target: i32 },
}
