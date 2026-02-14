use std::collections::HashMap;
use crate::models::{Room, World, Item, ConditionTrigger};

/// The WorldBuilder is used to construct a [World] object
pub struct WorldBuilder {
    pub rooms: HashMap<i32, Room>,
    pub items: HashMap<i32, Item>,
    pub initial_prints: Option<Vec<String>>,
    pub global_conditions: Vec<ConditionTrigger>,
    pub flags: HashMap<String, bool>,
}

impl WorldBuilder {
    /// Creates a new world build
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            items: HashMap::new(),
            initial_prints: None,
            global_conditions: Vec::new(),
            flags: HashMap::new(),
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

    /// Adds a global condition that is evaluated every turn
    pub fn with_global_condition(mut self, cond: ConditionTrigger) -> Self {
        self.global_conditions.push(cond);
        self
    }

    /// Sets an initial flag value
    pub fn with_flag(mut self, name: &str, value: bool) -> Self {
        self.flags.insert(name.to_string(), value);
        self
    }

    /// Consumes self and builds the world object
    pub fn build(self) -> World {
        World {
            rooms: self.rooms,
            items: self.items,
            initial_prints: self.initial_prints,
            global_conditions: self.global_conditions,
            flags: self.flags,
        }
    }
}
