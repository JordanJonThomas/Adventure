use std::collections::HashMap;
use crate::{
    engine::Game, 
    models::{Direction, condition::{ConditionPredicate, ConditionTrigger}}
};

/// Conditional text that appears in room descriptions based on game state
#[derive(Debug)]
pub struct ConditionalDescription {
    /// Predicate that must be true for text to display
    pub predicate: ConditionPredicate,
    /// Text to append to room description
    pub text: String,
    /// If true, show with both long and short descriptions
    /// If false, show only when long_desc is displayed
    pub show_in_short: bool,
}

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
    /// Conditions that can trigger in this room.
    pub conditions: Vec<ConditionTrigger>,
    /// Conditional descriptions that appear based on game state
    pub conditional_descriptions: Vec<ConditionalDescription>,
}

impl Room {
    /// Gets the full room description including conditional text
    /// 
    /// # Arguments
    /// * `game` - Current game state for evaluating predicates
    /// * `use_long_desc` - If true, use long_desc (first visit/look). If false, use desc (re-entry)
    pub fn get_full_description(&self, game: &Game, use_long_desc: bool) -> Vec<String> {
        let mut output = Vec::new();

        // Add room name
        output.push(self.name.clone());

        // Add base description
        if use_long_desc {
            if let Some(long_desc) = &self.long_desc {
                output.push(long_desc.clone());
            } else if let Some(desc) = &self.desc {
                output.push(desc.clone());
            }
        } else {
            if let Some(desc) = &self.desc {
                output.push(desc.clone());
            }
        }

        // Add conditional descriptions (in order)
        for cond_desc in &self.conditional_descriptions {
            // Skip if this is short desc and condition isn't marked for short
            if !use_long_desc && !cond_desc.show_in_short {
                continue;
            }

            // Evaluate predicate
            if game.evaluate_predicate(&cond_desc.predicate) {
                output.push(cond_desc.text.clone());
            }
        }

        output
    }
}
