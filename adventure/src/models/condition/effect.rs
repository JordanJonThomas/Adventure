use crate::engine::Game;

/// A ConditionEffect modifies the state of the game.
#[derive(Debug, Clone)]
pub enum ConditionEffect {
    /// The block action effect stops whatever action the player was attempting to execute.
    BlockAction { message: Option<String> },

    // Messaging
    /// Print message after action result text
    PrintMessage { text: String },
    /// Print message before everything action result text
    PrintMessageBefore { text: String },
    /// Append text to the current room description
    //AppendRoomDescription { text: String }, // TODO: determine how to implement

    // State modification
    RevealItem { item_id: i32 },
    HideItem { item_id: i32 },
    UnlockItem { item_id: i32 },
    LockItem { item_id: i32 },
    OpenItem { item_id: i32 },
    CloseItem { item_id: i32 },
    MoveItem { item_id: i32, to_room: i32 },
    MovePlayer { to_room: i32 },

    /// The custom effect allows for rust code to be executed against the game object.
    Custom { applier: fn(&mut Game) -> Vec<String> },

    // Game state
    EndGame { message: Option<String> },
}

/// A ConditionResult stores output of an executed [`super::ConditionTrigger`].
/// It also stores a flag stating if the default action behavior has been blocked by the
/// executed effect.
pub struct ConditionResult {
    pub output: Vec<String>,
    pub output_before: Vec<String>,
    pub blocked: bool,
}

impl Game {
    /// Executes a slice of [`ConditionEffect`]s against the game, modifying state.
    ///
    /// Returns a [`ConditionResult`] containing output text and whether the action the user is
    /// was using should be blocked.
    ///
    /// Effects are processed in order.
    ///
    /// # Note
    /// Effects that move or modify items currently being interacted with may cause the subsequent
    /// action to fail gracefully with existing executor error messages.
    pub fn execute_effect(&mut self, effects: &[ConditionEffect]) -> ConditionResult {
        let mut output = Vec::new();
        let mut output_before = Vec::new();
        let mut blocked = false;

        for effect in effects {

            match effect {
                ConditionEffect::BlockAction { message } => {
                    if let Some(msg) = message {
                        output.push(msg.to_string());
                    }
                    blocked = true;
                },
                ConditionEffect::PrintMessage { text } => output.push(text.to_string()),
                ConditionEffect::PrintMessageBefore { text } => output_before.push(text.to_string()),
                ConditionEffect::RevealItem { item_id } => {
                    if let Some(item) = self.world.items.get_mut(item_id) {
                        item.reveal();
                    }
                }
                ConditionEffect::HideItem { item_id } => {
                    if let Some(item) = self.world.items.get_mut(item_id) {
                        item.hide();
                    }
                }
                ConditionEffect::UnlockItem { item_id } => {
                    if let Some(item) = self.world.items.get_mut(item_id) {
                        item.set_locked(false);
                    }
                }
                ConditionEffect::LockItem { item_id } => {
                    if let Some(item) = self.world.items.get_mut(item_id) {
                        item.set_locked(true);
                    }
                }
                ConditionEffect::OpenItem { item_id } => {
                    if let Some(item) = self.world.items.get_mut(item_id) {
                        item.set_open(true);
                    }
                }
                ConditionEffect::CloseItem { item_id } => {
                    if let Some(item) = self.world.items.get_mut(item_id) {
                        item.set_open(false);
                    }
                }
                ConditionEffect::MoveItem { item_id, to_room } => {
                    // Remove item from current location (room or inventory or container)
                    let mut found = false;

                    // Check player inventory first
                    if let Some(pos) = self.player.inventory.iter().position(|&id| id == *item_id) {
                        self.player.inventory.remove(pos);
                        found = true;
                    }

                    // Check all rooms
                    if !found {
                        for room in self.world.rooms.values_mut() {
                            if let Some(pos) = room.items.iter().position(|&id| id == *item_id) {
                                room.items.remove(pos);
                                found = true;
                                break;
                            }
                        }
                    }

                    // Check all containers
                    if !found {
                        for item in self.world.items.values_mut() {
                            if let Some(contents) = item.get_container_mut() {
                                if let Some(pos) = contents.iter().position(|&id| id == *item_id) {
                                    contents.remove(pos);
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }

                    // Add to target room if it exists
                    if found {
                        if let Some(target_room) = self.world.rooms.get_mut(to_room) {
                            target_room.items.push(*item_id);
                        }
                    }
                }
                ConditionEffect::MovePlayer { to_room } => {
                    // Only move if target room exists
                    if self.world.rooms.contains_key(to_room) {
                        self.current_room = *to_room;
                    }
                }
                ConditionEffect::Custom { applier } => {
                    let result = applier(self);
                    output.extend(result);
                }
                ConditionEffect::EndGame { message } => {
                    if let Some(msg) = message {
                        output.push(msg.to_string());
                    }
                    self.game_over = true;
                }
            }
        }


        ConditionResult { output, output_before, blocked }
    }
}
