use std::collections::HashMap;
use crate::models::Direction;

/// Represents a place the player can enter.
/// A room may contain entities the player can interact with.
/// It may also connect to other rooms.
#[derive(Debug)]
pub struct Room {
    /// Name of the room
    pub name: String,
    /// Text printed on re-entry of a room
    pub desc: Option<String>,
    /// Text printed on inital entry or explicit look
    pub long_desc: Option<String>,
    /// Surrounding rooms
    pub exits: HashMap<Direction, i32>,
    /// Stores if the room has been entered
    pub entered: bool,
    /// All Items in the room
    pub items: Vec<i32>,
    // TODO: Remnant from older version
    // Any conditions in the room
    //pub conditions: Vec<ConditionTrigger>,
}
