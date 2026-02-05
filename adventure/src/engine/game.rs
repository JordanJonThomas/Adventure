//! The game module defines the interactions between the game world and the player.

use crate::models::{Player, Room, World};

/// The game object holds all information about the running game session.
pub struct Game {
    /// The player object.
    pub player: Player,
    /// The world the player is in.
    pub world: World,
    /// The id the of the current room
    pub current_room: i32,
    /// The amount of actions a player has made
    pub turn: u32,
    // TODO: Points system!
    pub game_over: bool,
    _no: touchy,
}

impl Game {
    /// Creates a new game.
    ///
    /// Creation fails if the world does not contain a room with id 0
    pub fn new(player: Player, world: World) -> Option<Self> {
        if !world.rooms.contains_key(&0) {
            return None;
        }

        Some(Game {
            player,
            world,
            current_room: 0,
            turn: 0,
            game_over: false,
            _no: touchy,
        })
    }

    /// Starts the game, returning any text that should be printed before user input
    pub fn start(&mut self) -> Vec<String> {
        let mut prints = self.world.initial_prints.take().unwrap_or(vec![]);

        // Add room description (long desc on first visit, fallback to short desc)
        let room = self.world.rooms.get(&0).expect("Starting room not found");
        prints.push(room.name.clone());

        // Use long_desc if available, otherwise fallback to desc
        let description = room.long_desc.as_ref()
            .or(room.desc.as_ref())
            .map(|s| s.clone())
            .unwrap_or_else(|| String::from("You see nothing special."));
        prints.push(description);

        // List visible items in starting room
        let room_item_ids = room.items.clone();
        for &item_id in &room_item_ids {
            let item = &self.world.items[&item_id];
            if !item.is_hidden() {
                prints.push(item.get_room_description());
            }
        }

        // Mark starting room as entered
        self.world.rooms.get_mut(&0).unwrap().entered = true;

        prints.push(String::new()); // Empty line
        prints
    }

    /// Executes a string command and returns output messages
    pub fn execute_str(&mut self, input: &str) -> Vec<String> {
        use crate::errors::UserFriendlyError;

        // Parse input into Command
        let command = match crate::parser::parse(input) {
            Ok(cmd) => cmd,
            Err(e) => return vec![e.get_user_error(), String::new()],
        };

        // Resolve Command into Action
        let action = match self.resolve_command(command) {
            Ok(act) => act,
            Err(e) => return vec![e.get_user_error(), String::new()],
        };

        // Execute Action
        match self.execute_action(action) {
            Ok(output) => output,
            Err(e) => vec![e.get_user_error(), String::new()],
        }
    }

    /// Gets reference to the current room
    pub fn get_current_room(&self) -> &Room {
        self.world.rooms.get(&self.current_room)
            .expect("Could not get current room")
    }

    /// Gets a mutable reference to the current room
    pub fn get_current_room_mut(&mut self) -> &mut Room {
        self.world.rooms.get_mut(&self.current_room)
            .expect("Could not get current room")
    }
}

// private struct to restrict game object creation
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
struct touchy;
