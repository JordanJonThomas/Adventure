//! The models module holds information about any game objects in the game.

// Structure
mod world; // The world object, stores all entities in the game
mod room; // Game room definition
mod item; // Item objects and functionality
mod behavior; // Defines what an item is capable of doing in game
mod player; // Player object

// Re-exports
pub use world::{World, Direction};
pub use room::Room;
pub use item::{Item, Determiner};
pub use behavior::ItemBehavior;
pub use player::Player;
