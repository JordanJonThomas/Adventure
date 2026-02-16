//! Item behaviors define capabilities and state for items

use super::Item;

/// Defines a capability or additional state that an item can have.
/// Items can have multiple behaviors via Vec<ItemBehavior>.
#[derive(Debug, Clone)]
pub enum ItemBehavior {
    /// Can be picked up and placed in inventory
    Holdable,

    /// Can store other items inside it
    Container {
        /// Items contained within this item
        contents: Vec<i32>,
        /// Maximum number of items it can hold (None = unlimited)
        capacity: Option<usize>,
    },

    /// Can be opened and closed (affects access to contents, doorways, etc.)
    Openable {
        /// Whether the item is currently open
        is_open: bool
    },

    /// Hidden from view until revealed by some trigger
    /// These items cannot be interacted with until revealed.
    Hidden {
        /// Whether the item has been revealed to the player
        revealed: bool
    },

    /// Scenery item that doesn't show in automatic room listings
    /// These items are interactable but don't appear in "There is a..." descriptions.
    /// Used for items mentioned in room descriptions (windows, walls, etc.)
    Scenery,

    /// Can be used as a weapon in combat
    Weapon {
        /// Damage dealt when attacking with this weapon
        damage: i32
    },

    /// Has text that can be read
    Readable {
        /// The text displayed when reading this item
        text: String
    },

    /// Has a physical size
    Size {
        // NOTE: this is basically an implementation detail for the map developer
        /// The size value
        value: u8
    },

    /// Locked and requires a key to open
    Locked {
        /// Whether the item is currently locked
        is_locked: bool,
        /// ID of the key item required (None = any key works)
        key_id: Option<i32>,
    },
}

impl Item {
    // Query Methods

    /// Returns true if the item can be picked up
    pub fn is_holdable(&self) -> bool {
        self.behaviors.iter().any(|b| matches!(b, ItemBehavior::Holdable))
    }

    /// Returns true if the item has a container
    pub fn has_container(&self) -> bool {
        self.behaviors.iter().any(|b| matches!(b, ItemBehavior::Container { .. }))
    }

    /// Returns true if the item can be opened/closed
    pub fn has_openable(&self) -> bool {
        self.behaviors.iter().any(|b| matches!(b, ItemBehavior::Openable { .. }))
    }

    /// Returns true if the item can be read
    pub fn has_readable(&self) -> bool {
        self.behaviors.iter().any(|b| matches!(b, ItemBehavior::Readable { .. }))
    }

    /// Returns true if the item is a weapon
    pub fn has_weapon(&self) -> bool {
        self.behaviors.iter().any(|b| matches!(b, ItemBehavior::Weapon { .. }))
    }

    /// Returns true if the item is currently hidden from view
    pub fn is_hidden(&self) -> bool {
        self.behaviors.iter()
            .any(|b| matches!(b, ItemBehavior::Hidden { revealed: false }))
    }

    /// Returns true if the item is scenery (doesn't show in room listings)
    pub fn is_scenery(&self) -> bool {
        self.behaviors.iter()
            .any(|b| matches!(b, ItemBehavior::Scenery))
    }

    /// Returns true if the item should be shown in automatic room listings
    /// (not hidden and not scenery)
    pub fn should_list_in_room(&self) -> bool {
        !self.is_hidden() && !self.is_scenery()
    }

    /// Returns true if the item is currently open (or has no Openable behavior)
    pub fn is_open(&self) -> bool {
        self.behaviors.iter()
            .find_map(|b| match b {
                ItemBehavior::Openable { is_open } => Some(*is_open),
                _ => None
            })
            .unwrap_or(true) // Items without Openable behavior are "always open"
    }

    /// Returns true if the item is currently locked
    pub fn is_locked(&self) -> bool {
        self.behaviors.iter()
            .any(|b| matches!(b, ItemBehavior::Locked { is_locked: true, .. }))
    }

    // Getter Methods

    /// Returns a reference to the container contents if this item is a container
    pub fn get_container(&self) -> Option<&Vec<i32>> {
        self.behaviors.iter()
            .find_map(|b| match b {
                ItemBehavior::Container { contents, .. } => Some(contents),
                _ => None
            })
    }

    /// Returns a mutable reference to the container contents if this item is a container
    pub fn get_container_mut(&mut self) -> Option<&mut Vec<i32>> {
        self.behaviors.iter_mut()
            .find_map(|b| match b {
                ItemBehavior::Container { contents, .. } => Some(contents),
                _ => None
            })
    }

    /// Returns the weapon damage if this item is a weapon
    pub fn get_weapon_damage(&self) -> Option<i32> {
        self.behaviors.iter()
            .find_map(|b| match b {
                ItemBehavior::Weapon { damage } => Some(*damage),
                _ => None
            })
    }

    /// Returns the item's size if it has a Size behavior
    pub fn get_size(&self) -> Option<u8> {
        self.behaviors.iter()
            .find_map(|b| match b {
                ItemBehavior::Size { value } => Some(*value),
                _ => None
            })
    }

    /// Returns the readable text if this item is readable
    pub fn get_readable_text(&self) -> Option<&str> {
        self.behaviors.iter()
            .find_map(|b| match b {
                ItemBehavior::Readable { text } => Some(text.as_str()),
                _ => None
            })
    }

    /// Returns the key_id if this item is locked
    pub fn get_key_id(&self) -> Option<i32> {
        self.behaviors.iter()
            .find_map(|b| match b {
                ItemBehavior::Locked { key_id, .. } => *key_id,
                _ => None
            })
    }

    // Setter Methods

    /// Sets the open state for Openable behavior. Returns true if successful.
    pub fn set_open(&mut self, open: bool) -> bool {
        for behavior in &mut self.behaviors {
            if let ItemBehavior::Openable { is_open } = behavior {
                *is_open = open;
                return true;
            }
        }
        false
    }

    /// Sets the locked state for Locked behavior. Returns true if successful.
    pub fn set_locked(&mut self, locked: bool) -> bool {
        for behavior in &mut self.behaviors {
            if let ItemBehavior::Locked { is_locked, .. } = behavior {
                *is_locked = locked;
                return true;
            }
        }
        false
    }

    /// Reveals a hidden item. Returns true if successful.
    pub fn reveal(&mut self) -> bool {
        for behavior in &mut self.behaviors {
            if let ItemBehavior::Hidden { revealed } = behavior {
                *revealed = true;
                return true;
            }
        }
        false
    }

    /// Hides an item. Returns true if successful.
    pub fn hide(&mut self) -> bool {
        for behavior in &mut self.behaviors {
            if let ItemBehavior::Hidden { revealed } = behavior {
                *revealed = false;
                return true;
            }
        }
        false
    }
}
