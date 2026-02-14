use std::collections::HashMap;
use crate::models::{Room, Direction, ConditionTrigger, ConditionPredicate, ConditionalDescription};

/// A room builder object, used to create rooms
pub struct RoomBuilder {
    pub name: String,
    pub desc: Option<String>,
    pub long_desc: Option<String>,
    pub exits: HashMap<Direction, i32>,
    pub entered: bool,
    pub items: Vec<i32>,
    pub conditions: Vec<ConditionTrigger>,
    pub conditional_descriptions: Vec<ConditionalDescription>,
}

impl RoomBuilder {
    /// Creates a new room builder
    pub fn new(name: &str) -> Self {
        RoomBuilder {
            name: name.to_string(),
            desc: None,
            long_desc: None,
            exits: HashMap::new(),
            entered: false,
            items: Vec::new(),
            conditions: Vec::new(),
            conditional_descriptions: Vec::new(),
        }
    }

    /// Adds a description to the room
    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc = Some(desc.to_string());
        self
    }

    /// Adds a long description to the room
    pub fn with_long_desc(mut self, long_desc: &str) -> Self {
        self.long_desc = Some(long_desc.to_string());
        self
    }

    /// Adds an exit to the room
    pub fn with_exit(mut self, dir: Direction, room_id: i32) -> Self {
        self.exits.insert(dir, room_id);
        self
    }

    /// Adds an item to the room
    pub fn with_item(mut self, item: i32) -> Self {
        self.items.push(item);
        self
    }

    /// Adds a condition that triggers when entering this room
    pub fn with_condition(mut self, cond: ConditionTrigger) -> Self {
        self.conditions.push(cond);
        self
    }

    /// Adds a conditional description that appears based on game state
    /// 
    /// # Arguments
    /// * `predicate` - Condition that must be true for text to display
    /// * `text` - Text to append to room description
    /// * `show_in_short` - If true, show with both long and short descriptions. If false, only with long_desc.
    pub fn with_conditional_desc(mut self, predicate: ConditionPredicate, text: &str, show_in_short: bool) -> Self {
        self.conditional_descriptions.push(ConditionalDescription {
            predicate,
            text: text.to_string(),
            show_in_short,
        });
        self
    }

    /// Constructs the room object
    pub fn build(self) -> Room {
        Room {
            name: self.name,
            desc: self.desc,
            long_desc: self.long_desc,
            exits: self.exits,
            entered: self.entered,
            items: self.items,
            conditions: self.conditions,
            conditional_descriptions: self.conditional_descriptions,
        }
    }
}
