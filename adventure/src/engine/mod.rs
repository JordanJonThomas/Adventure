//! The engine module defines how objects are interacted with the player

// Structure
pub mod game; // The game object, holds all information about the running game
pub mod action; // Actions are how the player interacts with the world
mod resolver; // Resolver hanles conversion between commands and actions
mod executor; // Executes valid actions against the game world

// Re-exports
pub use game::Game;
