use std::io::Write;

// Example game worlds
mod worlds;
use worlds::dungeon::create_world;

// This is just one example of how running the game engine might look!
// A more advanced UI could be designed that displays the current game room,
// points scored, number of moves executed or more...
fn main() -> anyhow::Result<()> {
    // Create and start game
    let world = create_world().build();
    let player = adventure::models::Player::new();
    let mut game = adventure::engine::game::Game::new(player, world)
        .ok_or_else(|| anyhow::anyhow!("Failed to create game"))?;
    let opening_prints = game.start();

    // Display opening text
    for line in opening_prints {
        println!("{}", line);
    }

    let mut input = String::new();

    // Gameplay loop
    loop {
        // Prompt
        print!("> ");
        std::io::stdout().flush()?;

        // Read input
        input.clear();
        std::io::stdin().read_line(&mut input)?;

        // Exit command
        if input.trim() == "exit" || input.trim() == "quit" {
            println!("Thanks for playing!");
            break;
        }

        // Execute command and print response
        let response = game.execute_str(&input);
        for line in response {
            println!("{}", line);
        }

        // Check if game is over
        if game.game_over {
            println!("Game Over!");
            break;
        }
    }

    Ok(())
}
