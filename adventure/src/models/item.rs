use std::collections::HashMap;
use crate::{errors::ParseError, util::capitalize};
use super::behavior::ItemBehavior;

/// Represents a single item in the game
#[derive(Debug)]
pub struct Item {
    /// Name of the item
    pub name: String,
    /// Rules for printing item
    pub determiner: Determiner,
    /// Descriptive words for item
    pub adjectives: Vec<String>,
    /// Determines additional functionality an item might have
    pub behaviors: Vec<ItemBehavior>,
    /// Whether the item has been moved from its initial starting location
    pub moved: bool,
    /// A key-value pair specifying unique print events an item might have.
    ///
    /// The Key is the Event the print will be fired on.
    /// The Value specifies the text to be printed.
    pub print: HashMap<String, String>
}

/// A determiner explains how to verbalize an item's name
#[derive(Debug)]
pub enum Determiner {
    Singular, // Singular item (a/an)
    ProperNoun, // Proper noun (The)
    Plural, // Plural (some)
    None, // No prefix
}

impl TryFrom<&str> for Determiner {
    type Error = ParseError;

    fn try_from(word: &str) -> Result<Self, Self::Error> {
        Ok(match word {
            "a" | "an" => Determiner::Singular,
            "some" => Determiner::Plural,
            // HACK: "The" can also be used for plurals ("the grapes")
            "the" => Determiner::ProperNoun,
            _ => return Err(ParseError::NotParseable),
        })
    }
}

impl Item {
    /// Gets the canonical name for this item (determiner + adjectives + name)
    /// Example: "a red sword", "the treasure", "some coins"
    pub fn get_canonical_name(&self) -> String {
        let article = match self.determiner {
            Determiner::Singular => "a ",
            Determiner::ProperNoun => "the ",
            Determiner::Plural => "some ",
            Determiner::None => "",
        };

        let adjectives = if self.adjectives.is_empty() {
            String::new()
        } else {
            format!("{} ", self.adjectives.join(" "))
        };

        format!("{}{}{}", article, adjectives, self.name)
    }

    /// Gets the printed description for this item.
    /// Uses "initial" print if unmoved, otherwise constructs from canonical name
    pub fn get_room_description(&self) -> String {
        // If unmoved and has an initial print, use that
        if !self.moved {
            if let Some(initial) = self.print.get("initial") {
                return initial.clone();
            }
        }

        // Otherwise construct: "A red sword is here."
        let canonical = capitalize(self.get_canonical_name().as_str());
        format!("{} is here.", canonical)
    }

    /// Formats the contents of a container for display.
    /// Returns empty Vec if not a container.
    ///
    /// items should be the HashMap of all items in the world, used to determine the names of contained items.
    pub fn format_container_contents(&self, items: &std::collections::HashMap<i32, Item>) -> Vec<String> {
        let mut output = Vec::new();

        if let Some(contents) = self.get_container() {
            if !contents.is_empty() {
                output.push(format!("Inside the {} you see:", self.name));
                for &item_id in contents {
                    if let Some(item) = items.get(&item_id) {
                        output.push(format!("  {}", item.name));
                    }
                }
            }
        }

        output
    }
}

