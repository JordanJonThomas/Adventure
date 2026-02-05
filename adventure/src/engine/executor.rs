//! Action execution makes actions actually modify game state

use crate::{
    engine::{action::Action, game::Game},
    errors::{ActionError, GameError}, util::capitalize,
};

impl Game {
    /// Executes an action, modifying game state and returning output messages
    pub fn execute_action(&mut self, action: Action) -> Result<Vec<String>, GameError> {
        match action {
            Action::Move { dir } => execute_move(self, dir),
            Action::Get { items } => execute_get(self, items),
            Action::Drop { items } => execute_drop(self, items),
            Action::Put { item, container } => execute_put(self, item, container),
            Action::Look { target } => execute_look(self, target),
            Action::Read { target } => execute_read(self, target),
            Action::Open { target } => execute_open(self, target),
            Action::Close { target } => execute_close(self, target),
            Action::Attack { target, weapon } => execute_attack(self, target, weapon),
            Action::Unlock { target, key } => execute_unlock(self, target, key),
        }
    }
}

/// Executes a move action, changing the players current room
fn execute_move(game: &mut Game, dir: crate::models::Direction) -> Result<Vec<String>, GameError> {
    let current_room = game.get_current_room();
    let new_room_id = *current_room.exits.get(&dir)
        .expect("Direction should have been validated during resolution");

    // Change room
    game.current_room = new_room_id;
    game.turn += 1;

    // Get new room description
    let new_room = game.get_current_room();
    let mut output = vec![new_room.name.clone()]; // Show room name on entry

    if !new_room.entered {
        // Show long_desc if first visit, otherwise short desc
        if let Some(desc) = new_room.long_desc.as_ref()
            .or(new_room.desc.as_ref())
            .map(|s| s.clone()) {
            output.push(desc);
        }

        // List visible items on first visit
        let room_item_ids = new_room.items.clone();
        for &item_id in &room_item_ids {
            let item = &game.world.items[&item_id];
            if !item.is_hidden() {
                output.push(item.get_room_description());

                // If it's an open container, show its contents
                if item.has_container() && item.is_open() {
                    output.extend(item.format_container_contents(&game.world.items));
                }
            }
        }

        // Mark room as entered
        game.get_current_room_mut().entered = true;
    } else if let Some(desc) = &new_room.desc {
        // show short desc only on re-entry
        output.push(desc.clone());
    }

    output.push(String::new());
    Ok(output)
}

/// Executes a get action transferring items from room or containers to inventory
fn execute_get(game: &mut Game, item_ids: Vec<i32>) -> Result<Vec<String>, GameError> {
    let mut output = Vec::new();
    let is_single = item_ids.len() == 1; // for formatting

    for item_id in item_ids {
        // Skip unholdable items
        if !&game.world.items[&item_id].is_holdable() {
            let item_name = &game.world.items[&item_id].name;
            if is_single {
                output.push(crate::errors::cant_do_that());
            } else {
                output.push(format!("{item_name}: {}", crate::errors::cant_do_that()));
            }
            continue;
        }

        // Try to remove from room first
        let current_room = game.get_current_room_mut();
        let was_in_room = current_room.items.iter().position(|&id| id == item_id); // get idx of item

        if let Some(pos) = was_in_room {
            current_room.items.remove(pos);
        } else {
            // Not in room, must be in a container
            let mut found = false;
            let room_item_ids: Vec<i32> = game.get_current_room().items.clone();

            for &container_id in &room_item_ids {
                let container = game.world.items.get_mut(&container_id).unwrap();
                if let Some(contents) = container.get_container_mut() {
                    if let Some(pos) = contents.iter().position(|&id| id == item_id) {
                        contents.remove(pos);
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                // Shouldn't happen if resolver worked correctly
                let item_name = &game.world.items[&item_id].name;
                if is_single {
                    output.push(format!("You can't find the {}.", item_name));
                } else {
                    output.push(format!("{item_name}: can't find it."));
                }
                continue;
            }
        }

        // Mark item as moved when taken
        let item = game.world.items.get_mut(&item_id).unwrap();
        item.moved = true;

        // Add to inventory
        game.player.inventory.push(item_id);

        // Get item name for output
        let item_name = &game.world.items[&item_id].name;
        if is_single {
            output.push(format!("You grab the {}.", item_name));
        } else {
            output.push(format!("{item_name}: taken."));
        }
    }

    game.turn += 1;
    output.push(String::new()); // Empty line
    Ok(output)
}

/// Executes a drop action, moving items from inventory to the room
fn execute_drop(game: &mut Game, item_ids: Vec<i32>) -> Result<Vec<String>, GameError> {
    let mut output = Vec::new();

    for item_id in item_ids {
        // Remove from inventory
        game.player.inventory.retain(|&id| id != item_id);

        // Mark item as moved when dropped
        let item = game.world.items.get_mut(&item_id).unwrap();
        item.moved = true;

        // Add to room
        let current_room = game.get_current_room_mut();
        current_room.items.push(item_id);

        // Get item name for output
        let item_name = &game.world.items[&item_id].name;
        output.push(format!("Dropped: {}", item_name));
    }

    game.turn += 1;
    output.push(String::new()); // Empty line
    Ok(output)
}

/// Executes a put action, moving items into a container
fn execute_put(game: &mut Game, item_id: i32, container_id: i32) -> Result<Vec<String>, GameError> {
    // Get item and container names before mutation
    let item_name = game.world.items.get(&item_id)
        .ok_or(GameError::Unknown)?
        .name.clone();

    let container = game.world.items.get(&container_id)
        .ok_or(GameError::Unknown)?;
    let container_name = container.name.clone();

    // Validate container has Container behavior
    if !container.has_container() {
        return Ok(vec![
            format!("You can't put things in the {}.", container_name),
            String::new()
        ]);
    }

    // Validate container is openable and open
    if container.has_openable() && !container.is_open() {
        return Ok(vec![
            format!("The {} is closed.", container_name),
            String::new()
        ]);
    }

    // Remove item from inventory
    game.player.inventory.retain(|&id| id != item_id);

    // Add item to container
    let container = game.world.items.get_mut(&container_id).unwrap();
    if let Some(contents) = container.get_container_mut() {
        contents.push(item_id);
    }

    // Mark item as moved
    game.world.items.get_mut(&item_id).unwrap().moved = true;

    game.turn += 1;
    Ok(vec![
        format!("You put the {} in the {}.", item_name, container_name),
        String::new()
    ])
}

/// Executes a look action, displaying room or item information
///
/// If the item being examined has a "look" print, it will be shown to the player.
/// Otherwise, if the item being examined has the [`Readable`](crate::models::ItemBehavior::Readable) 
/// behavior, its text will be displayed.
fn execute_look(game: &mut Game, target: Option<i32>) -> Result<Vec<String>, GameError> {
    let mut output = Vec::new();

    match target {
        None => {
            // Look at room
            let room = game.get_current_room();
            output.push(room.name.clone());

            if let Some(long_desc) = &room.long_desc {
                output.push(long_desc.clone());
            } else if let Some(desc) = &room.desc {
                output.push(desc.clone());
            }

            // List visible items
            let room_item_ids = room.items.clone();
            for &item_id in &room_item_ids {
                let item = &game.world.items[&item_id];
                if !item.is_hidden() {
                    output.push(item.get_room_description());

                    // If it's an open container, show its contents
                    if item.has_container() && item.is_open() {
                        output.extend(item.format_container_contents(&game.world.items));
                    }
                }
            }
        }
        Some(item_id) => {
            // Look at specific item
            let item = &game.world.items[&item_id];

            // Check for custom "look" print, or readable print
            if let Some(look_text) = item.print.get("look") {
                output.push(look_text.clone());
            } else if let Some(readable_text) = item.get_readable_text() {
                output.push(readable_text.to_string());
            } else {
                // Default: "It's a red sword."
                output.push(format!("It's {}.", item.get_canonical_name()));
            }
        }
    }

    output.push(String::new()); // Empty line
    Ok(output)
}

/// Executes the read action, displaying item text if it is readable
fn execute_read(game: &mut Game, target: i32) -> Result<Vec<String>, GameError> {
    let item = &game.world.items[&target];
    let Some(text) = item.get_readable_text() else {
        return Ok(vec!["You cannot read that...".to_string(), String::new()]);
    };
    Ok(vec![text.to_string(), String::new()])
}

/// Executes the open action, attempting to open the specified object.
///
/// Fails if:
/// - The item cannot be opened
/// - The item is locked
/// - The Item is already open
fn execute_open(game: &mut Game, target: i32) -> Result<Vec<String>, GameError> {
    let mut output = Vec::new();

    let item = game.world.items.get_mut(&target)
        .ok_or_else(|| GameError::Unknown)?;

    let item_name = item.name.clone();

    // Check if item can be opened
    if !item.has_openable() {
        return Ok(vec![format!("You can't open the {}.", item_name), String::new()]);
    }

    // Check if locked
    if item.is_locked() {
        return Ok(vec![format!("The {} is locked.", item_name), String::new()]);
    }

    // Check if already open
    if item.is_open() {
        return Ok(vec![format!("The {} is already open.", item_name), String::new()]);
    }

    // Open it
    item.set_open(true);
    output.push(format!("You open the {}.", item_name));

    // Print contents if the item is a container
    let item = &game.world.items[&target];
    output.extend(item.format_container_contents(&game.world.items));

    game.turn += 1;
    output.push(String::new());
    Ok(output)
}

/// Executes the close action against an item
fn execute_close(game: &mut Game, target: i32) -> Result<Vec<String>, GameError> {
    let item = game.world.items.get_mut(&target)
        .ok_or_else(|| GameError::Unknown)?;

    let item_name = item.name.clone();

    // Check if item can be closed
    if !item.has_openable() {
        return Ok(vec![format!("You can't close the {}.", item_name), String::new()]);
    }

    // Check if already closed
    if !item.is_open() {
        return Ok(vec![format!("The {} is already closed.", item_name), String::new()]);
    }

    // Close it
    item.set_open(false);

    game.turn += 1;
    Ok(vec![format!("You close the {}.", item_name), String::new()])
}

/// Executes the attack action, reminding the user that kindness is always an option :)
fn execute_attack(_game: &mut Game, _target: i32, _weapon: Option<i32>) -> Result<Vec<String>, GameError> {
    // TODO: Requires Enemy and combat system
    return Err(GameError::Action(ActionError::InvalidCommand("What good is violence?".to_string())))
}

/// Executes the unlock action against an item
fn execute_unlock(game: &mut Game, target_id: i32, key_id: i32) -> Result<Vec<String>, GameError> {
    let target_name = game.world.items[&target_id].get_canonical_name();
    let key_name = game.world.items[&key_id].get_canonical_name();

    if !game.world.items[&target_id].is_locked() {
        return Err(GameError::Action(ActionError::InvalidCommand(
            format!("{} is not locked.", capitalize(&target_name))
        )));
    }

    // Validate correct key
    let required_key_id = game.world.items[&target_id].get_key_id();
    if let Some(required) = required_key_id {
        if required != key_id {
            return Err(GameError::Action(ActionError::InvalidCommand(
                format!("{} doesn't seem to work.", capitalize(&key_name))
            )));
        }
    }

    // Unlock
    game.world.items.get_mut(&target_id).unwrap().set_locked(false);

    Ok(vec![
        format!("You unlock {} with {}.", target_name, key_name), 
        String::new(),
    ])
}
