//! Command resolution converts a parsed [Command] into an executable [Action]
//!
//! During this step, specified nouns are resolved from strings of text provided by the user
//! and converted into ID's for lookup in the game object.

use crate::{
    engine::{action::Action, game::Game},
    errors::ActionError,
    models::{Direction, Item},
    parser::{Command, NounPhrase, Verb},
};

impl Game {
    /// Converts a parsed Command into an executable Action
    pub fn resolve_command(&self, cmd: Command) -> Result<Action, ActionError> {
        match cmd.verb {
            Verb::Move { .. } => resolve_move(self, cmd),
            Verb::Get => resolve_get(self, cmd),
            Verb::Drop => resolve_drop(self, cmd),
            Verb::Put => resolve_put(self, cmd),
            Verb::Look => resolve_look(self, cmd),
            Verb::Open => resolve_open(self, cmd),
            Verb::Close => resolve_close(self, cmd),
            Verb::Attack => resolve_attack(self, cmd),
            Verb::Unlock => resolve_unlock(self, cmd),
            Verb::Read => resolve_read(self, cmd),
        }
    }
}

fn resolve_move(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    // Extract direction from the verb or from direct object
    let direction = match cmd.verb {
        // Direction already in verb (e.g., player typed "north")
        Verb::Move { dir: Some(d) } => d,
        // Need to parse from direct object (e.g., "go north")
        Verb::Move { dir: None } => {
            if cmd.direct.is_empty() {
                return Err(ActionError::DirectionNotFound("".to_string()));
            }
            let phrase = &cmd.direct[0];
            Direction::try_from(phrase.noun.as_str())
                .map_err(|_| ActionError::DirectionNotFound(phrase.noun.clone()))?
        }
        _ => panic!("resolve_move called with non-Move verb"),
    };

    // Verify the current room has an exit in this direction
    let current_room = game.get_current_room();
    if !current_room.exits.contains_key(&direction) {
        return Err(ActionError::NoExit(direction));
    }

    Ok(Action::Move { dir: direction })
}

fn resolve_get(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    let mut ids = cmd.direct.iter()
        .map(|noun| find_visible_item(game, noun))
        .collect::<Result<Vec<_>, _>>()?;

    // keep only unique item IDs
    ids.sort_unstable();
    ids.dedup();

    Ok(Action::Get { items: ids })
}

fn resolve_drop(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    let mut ids = cmd.direct.iter()
        .map(|noun| find_item_in_inventory(game, noun))
        .collect::<Result<Vec<_>, _>>()?;

    // keep only unique item IDs
    ids.sort_unstable();
    ids.dedup();

    Ok(Action::Drop { items: ids })
}

fn resolve_put(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    // Put requires exactly one direct object and one indirect object
    if cmd.direct.len() != 1 {
        return Err(ActionError::InvalidCommand("Put requires exactly one item".to_string()));
    }

    let indirect = cmd.indirect.as_ref()
        .ok_or_else(|| ActionError::InvalidCommand("Put requires a container (use 'put X in Y')".to_string()))?;

    // Resolve the item from inventory
    let item_id = find_item_in_inventory(game, &cmd.direct[0])?;

    // Try to find container in inventory first, then in current room
    let container_id = find_item_in_inventory(game, indirect)
        .or_else(|_| find_visible_item(game, indirect))?;

    Ok(Action::Put {
        item: item_id,
        container: container_id
    })
}

fn resolve_look(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    if cmd.direct.len() > 1 {
        return Err(ActionError::InvalidCommand("Cannot Look at more than 1 object".to_string()));
    }
    // Determine if item is in room or inventory
    let item = cmd.direct.first()
        .map(|noun| {
            find_visible_item(game, noun)
                .or(find_item_in_inventory(game, noun))
        })
        .transpose()?;
    Ok(Action::Look { target: item }) // target: None says to look at the room
}

fn resolve_read(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    if cmd.direct.len() > 1 {
        return Err(ActionError::InvalidCommand("Cannot Read more than 1 object".to_string()));
    }

    let target = cmd.direct.first()
        .map(|noun| find_visible_item(game, noun)
            .or(find_item_in_inventory(game, noun)))
        .transpose()?
        .ok_or(ActionError::InvalidCommand("What do you want to read?".to_string()))?;

    Ok(Action::Read { target })
}

fn resolve_open(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    if cmd.direct.len() != 1 {
        return Err(ActionError::InvalidCommand("Open requires exactly one item".to_string()));
    }

    // Find the item in room or inventory
    let target_id = find_visible_item(game, &cmd.direct[0])
        .or_else(|_| find_item_in_inventory(game, &cmd.direct[0]))?;

    Ok(Action::Open { target: target_id })
}

fn resolve_close(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    if cmd.direct.len() != 1 {
        return Err(ActionError::InvalidCommand("Close requires exactly one item".to_string()));
    }

    // Find the item in room or inventory
    let target_id = find_visible_item(game, &cmd.direct[0])
        .or_else(|_| find_item_in_inventory(game, &cmd.direct[0]))?;

    Ok(Action::Close { target: target_id })
}

fn resolve_attack(_game: &Game, _cmd: Command) -> Result<Action, ActionError> {
    // TODO: not implementable until Enemies, ItemBehavior are completed, err currently handled at attack execution
    Ok(Action::Attack { target: -1, weapon: None })
}

fn resolve_unlock(game: &Game, cmd: Command) -> Result<Action, ActionError> {
    if cmd.direct.is_empty() {
        return Err(ActionError::InvalidCommand("What do you want to unlock?".to_string()));
    }

    let target_id = find_visible_item(game, &cmd.direct[0])?;

    let key_id = if let Some(ref key_noun) = cmd.indirect {
        find_item_in_inventory(game, key_noun)?
    } else {
        return Err(ActionError::InvalidCommand("What do you want to unlock it with?".to_string()));
    };

    Ok(Action::Unlock { target: target_id, key: key_id })
}

/// Finds a visible item in the current room by noun phrase
/// Searches room items first, then inside open containers
/// Returns the item ID if found, or an error if not found or ambiguous
fn find_visible_item(game: &Game, noun: &NounPhrase) -> Result<i32, ActionError> {
    let current_room = game.get_current_room();

    // Search items in the current room
    let mut matches: Vec<i32> = current_room
        .items
        .iter()
        .filter(|&&item_id| {
            let item = &game.world.items[&item_id];
            // Filter out hidden items
            if item.is_hidden() {
                return false;
            }
            item_matches(item, noun)
        })
        .copied()
        .collect();

    // If no direct match, search inside open containers in the room
    if matches.is_empty() {
        for &container_id in &current_room.items {
            let container = &game.world.items[&container_id];

            // Skip if not a container or if closed
            if !container.is_open() {
                continue;
            }

            // Search inside this container
            if let Some(contents) = container.get_container() {
                for &item_id in contents {
                    let item = &game.world.items[&item_id];
                    if !item.is_hidden() && item_matches(item, noun) {
                        matches.push(item_id);
                    }
                }
            }
        }
    }

    // Check for matches
    match matches.len() {
        0 => Err(ActionError::ItemNotFound(noun.noun.clone())),
        1 => Ok(matches[0]),
        _ => Err(ActionError::AmbiguousItem(noun.noun.clone())),
    }
}

/// Finds an item in the player inventory
/// Returns the item ID if found, or an error if not found or ambiguous
fn find_item_in_inventory(game: &Game, noun: &NounPhrase) -> Result<i32, ActionError> {
    // Search items in the player inventory
    let matches: Vec<i32> = game.player.inventory
        .iter()
        .filter(|&&item_id| {
            let item = &game.world.items[&item_id];
            item_matches(item, noun)
        })
        .copied()
        .collect();

    // Check for matches
    match matches.len() {
        0 => Err(ActionError::ItemNotFoundInventory(noun.noun.clone())),
        1 => Ok(matches[0]),
        _ => Err(ActionError::AmbiguousItem(noun.noun.clone())),
    }
}

/// Checks if an item matches a noun phrase
/// Returns true if the item's name matches the noun AND all adjectives match
fn item_matches(item: &Item, noun: &NounPhrase) -> bool {
    // Check if the noun matches the item name (case-insensitive)
    if !item.name.eq_ignore_ascii_case(&noun.noun) {
        return false;
    }

    // If there are adjectives in the noun phrase, ALL must match
    // the item's adjectives (case-insensitive)
    for adj in &noun.adjectives {
        let matches = item.adjectives.iter()
            .any(|item_adj| item_adj.eq_ignore_ascii_case(adj));

        if !matches {
            return false;
        }
    }

    true
}
