use std::collections::HashMap;
use crate::models::{Room, World, Item};

/// The WorldBuilder is used to construct a [World] object
pub struct WorldBuilder {
    pub rooms: HashMap<i32, Room>,
    pub items: HashMap<i32, Item>,
    pub initial_prints: Option<Vec<String>>,
}

impl WorldBuilder {
    /// Creates a new world build
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            items: HashMap::new(),
            initial_prints: None,
        }
    }

    /// Adds a room to the world
    pub fn with_room(mut self, id: i32, room: Room) -> Self {
        let _ = self.rooms.insert(id, room);
        self
    }

    /// Adds an item to the world
    pub fn with_item(mut self, id: i32, item: Item) -> Self {
        let _ = self.items.insert(id, item);
        self
    }

    /// Sets the given text to the initial prints
    pub fn with_initial_prints(mut self, prints: Vec<String>) -> Self {
        self.initial_prints = Some(prints);
        self
    }

    /// Consumes self and builds the world object
    pub fn build(self) -> World {
        World {
            rooms: self.rooms,
            items: self.items,
            initial_prints: self.initial_prints
        }
    }
}
