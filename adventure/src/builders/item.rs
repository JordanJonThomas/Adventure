use std::collections::HashMap;
use crate::models::{Determiner, Item, ItemBehavior};


/// An item builder object, used to create items
pub struct ItemBuilder {
    name: String,
    determiner: Option<Determiner>,
    adjectives: Vec<String>,
    behaviors: Vec<ItemBehavior>,
    print: HashMap<String, String>
}

impl ItemBuilder {
    /// Creates a new item builder
    pub fn new(name: &str) -> Self {
        ItemBuilder {
            name: name.to_string(),
            determiner: None,
            adjectives: Vec::new(),
            behaviors: Vec::new(),
            print: HashMap::new(),
        }
    }

    /// Creates a new "regular" item builder.
    /// Regular items are [Holdable](crate::models::ItemBehavior::Holdable) and have a default size of 1.
    pub fn new_regular(name: &str) -> Self {
        // Behaviors expected for most items
        let default_behaviors = vec![
            ItemBehavior::Holdable,
            ItemBehavior::Size { value: 1 }, // Small by default
        ];

        ItemBuilder {
            name: name.to_string(),
            determiner: None,
            behaviors: default_behaviors,
            adjectives: Vec::new(),
            print: HashMap::new(),
        }
    }

    /// Adds the given determiner to the item
    pub fn with_determiner(mut self, d: Determiner) -> Self {
        self.determiner = Some(d);
        self
    }

    /// Adds the given adjective to the item
    pub fn with_adjective(mut self, a: String) -> Self {
        self.adjectives.push(a);
        self
    }

    /// Adds the given item behavior to the item
    pub fn with_behavior(mut self, b: ItemBehavior) -> Self {
        self.behaviors.push(b);
        self
    }

    /// Adds multiple behaviors at once
    pub fn with_behaviors(mut self, behaviors: Vec<ItemBehavior>) -> Self {
        self.behaviors.extend(behaviors);
        self
    }

    /// Adds an initial print to the item, printing that text if the item hasnt been moved
    pub fn with_print(mut self, event: &str, print: &str) -> Self {
        self.print.insert(event.to_string(), print.to_string());
        self
    }

    /// Constructs the item object
    pub fn build(self) -> Item {
        let determiner = self.determiner.unwrap_or(Determiner::None);

        Item {
            name: self.name,
            determiner,
            adjectives: self.adjectives,
            behaviors: self.behaviors,
            moved: false,
            print: self.print,
        }
    }
}
