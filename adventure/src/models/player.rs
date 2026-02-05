/// Represents the player in the game
#[derive(Debug, Clone)]
pub struct Player {
    /// Stores the id's of the items in the players inventory
    pub inventory: Vec<i32>,
}

impl Player {
    /// Creates a new player with empty inventory
    pub fn new() -> Self {
        Player {
            inventory: Vec::new(),
        }
    }
}
