//! Everything that exists inside the game world.
use std::{collections::HashMap, fmt::Display};
use phf::phf_map;
use crate::{errors::ParseError, models::{Item, Room, condition::ConditionTrigger}};

/// Represents the entire world of the game.
#[derive(Debug)]
pub struct World {
    /// The list of all rooms in the game.
    /// The starting room is the room with the 0 Id.
    /// If no starting room is given, the game does not start.
    pub rooms: HashMap<i32, Room>,
    /// The list of all items in the game.
    pub items: HashMap<i32, Item>,
    /// Any text that should be printed before the game starts.
    pub initial_prints: Option<Vec<String>>,
    /// Global conditions that are checked every turn or at specific events
    pub global_conditions: Vec<ConditionTrigger>,
    /// Named boolean flags for tracking game state
    pub flags: HashMap<String, bool>,
}

impl World {
    /// Creates a new empty world
    pub fn empty() -> World {
        World {
            rooms: HashMap::new(),
            items: HashMap::new(),
            initial_prints: None,
            global_conditions: Vec::new(),
            flags: HashMap::new(),
        }
    }
}

/// Represents a direction in the world
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Up,
    Down,
}

/// Static map with k-v pairs for string -> direction
static DIRECTION_MAP: phf::Map<&'static str, Direction> = phf_map! {
    "n" => Direction::North,
    "north" => Direction::North,
    "e" => Direction::East,
    "east" => Direction::East,
    "s" => Direction::South,
    "south" => Direction::South,
    "w" => Direction::West,
    "west" => Direction::West,
    "ne" => Direction::NorthEast,
    "northeast" => Direction::NorthEast,
    "north-east" => Direction::NorthEast,
    "se" => Direction::SouthEast,
    "southeast" => Direction::SouthEast,
    "south-east" => Direction::SouthEast,
    "nw" => Direction::NorthWest,
    "northwest" => Direction::NorthWest,
    "north-west" => Direction::NorthWest,
    "sw" => Direction::SouthWest,
    "southwest" => Direction::SouthWest,
    "south-west" => Direction::SouthWest,
    "u" => Direction::Up,
    "up" => Direction::Up,
    "d" => Direction::Down,
    "down" => Direction::Down,
};

impl TryFrom<&str> for Direction {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        DIRECTION_MAP.get(s)
            .copied()
            .ok_or_else(|| ParseError::NotADirection(s.to_string()))
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        let dir = match self {
            North => "north",
            NorthEast => "north-east",
            East => "east",
            SouthEast => "south-east",
            South => "south",
            SouthWest => "south-west",
            West => "west",
            NorthWest => "north-west",
            Up => "up",
            Down => "down",
        };

        write!(f, "{}", dir)
    }
}
