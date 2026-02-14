# Adventure

A text-based adventure game engine inspired by Zork, built in Rust 🦀.
<img width="1703" height="379" alt="image" src="https://github.com/user-attachments/assets/3f24d0aa-5944-41bf-a0d1-c17e22a16dbf" />

## How to Play

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd Adventure
   ```

2. Navigate to the binary crate:
   ```bash
   cd adventure-bin
   ```

3. Run the game:
   ```bash
   cargo run
   ```

4. Enter commands at the `>` prompt. Type `exit` or `quit` to end the game.

## Game Features

### Commands

The game engine supports natural language commands with various verbs and synonyms:

- **Movement**: `go north`, `walk east`, or simply `north`, `south`, `east`, `west`
- **Item interaction**: `get stick`, `take sword`, `drop rock`
- **Inventory management**: `put newspaper in chest`, `place item on table`
- **Examination**: `look`, `examine chest`, `inspect room`, `read newspaper`
- **Containers**: `open chest`, `close door`
- **Locks**: `unlock door with key`

### Item Behaviors

Items can have multiple behaviors that determine their functionality:

- **Readable**: Items containing text that can be read
- **Container**: Items that can hold other items (with optional capacity limits)
- **Openable**: Items that can be opened and closed
- **Key**: Items that unlock specific locks
- **Size**: Determines how much space an item takes in containers

### World Features

- Connected rooms with directional exits (north, south, east, west, up, down)
- Items placed in rooms or inside containers
- Item descriptions that change based on context
- Adjectives for item disambiguation (e.g., "red sword" vs "blue sword")
- Custom print messages for specific item events
- **Condition system** for dynamic gameplay and puzzles:
  - Block actions based on inventory, location, or game state
  - Reveal hidden items or areas when conditions are met
  - Display conditional messages and room descriptions
  - Track game state with flags
  - Compose complex logic with AND/OR/NOT predicates

## Building a Map

Maps are built programmatically using the builder pattern. Create a new world file in `adventure-bin/src/worlds/`:

```rust
use adventure::{
    builders::{WorldBuilder, RoomBuilder, ItemBuilder},
    models::{Direction, ItemBehavior, Player},
    engine::Game,
};

pub fn create_world() -> WorldBuilder {
    WorldBuilder::new()
        // Add items first (referenced by ID)
        .with_item(0, ItemBuilder::new_regular("sword")
            .with_adjective("rusty".to_string())
            .with_print("initial", "A rusty sword lies on the ground.")
            .build())
        
        .with_item(1, ItemBuilder::new("chest")
            .with_behavior(ItemBehavior::Container {
                contents: vec![],
                capacity: Some(10)
            })
            .with_behavior(ItemBehavior::Openable { is_open: false })
            .build())
        
        // Add rooms
        .with_room(0, RoomBuilder::new("Entry Hall")
            .with_desc("A dimly lit entrance hall. An exit can be seen to the north.")
            .with_item(0) // sword
            .with_item(1) // chest
            .with_exit(Direction::North, 1)
            .build())
        
        .with_room(1, RoomBuilder::new("Treasure Room")
            .with_desc("Gold and jewels glitter in the torchlight.")
            .with_exit(Direction::South, 0)
            .build())
        
        // Set opening message and build world
        .with_initial_prints(vec!["Welcome to the Adventure!".into()])
        .build()
}
```

### Builder Components

#### WorldBuilder
- `with_item(id, item)`: Add an item to the world with a unique ID
- `with_room(id, room)`: Add a room with a unique ID
- `with_initial_prints(messages)`: Set the opening text when the game starts
- `build()`: Construct the final world

#### RoomBuilder
- `new(name)`: Create a room with a name
- `with_desc(description)`: Set the room description
- `with_item(item_id)`: Place an item in the room
- `with_exit(direction, room_id)`: Connect to another room
- `build()`: Construct the final room

#### ItemBuilder
- `new(name)` or `new_regular(name)`: Create an item (regular uses singular determiner)
- `with_adjective(adj)`: Add descriptive adjectives
- `with_behavior(behavior)`: Add functionality to the item
- `with_print(event, text)`: Set custom text for events (e.g., "initial", "look")
- `build()`: Construct the final item

## Planned Features

Although pretty neat, this project in its current state is missing a few features that stop it from being fully Zork-complete, and there's a few other neat ideas I have planned as well, including but not limited to:

### NPC's, Enemies and Combat
What good is an adventure game without person to person interaction!? The original zork franchise contains many creatures to combat, making this feature high up on my priority list.

### Data-Based Map Design
Allowing for another data format to represent maps in, such as XML or RON would allow for a farther audience to create stories and adventures! 
I'm currently exploring ways to design a system like this, without giving up too much of the freedom that code allows for when designing worlds.

### And More!
Other planned features include a scoring system, an advanced UI design for the CLI version, and even more! 

## Inspirations & Reflection
I discovered Zork at a very young age, and immediately became fascinated with how my words could be interpreted by a computer to play a video game!
When I began to learn Java, I discovered [This Java Implementation](https://github.com/dtschust/Zork) of a text based adventure game and decided I had to give it a shot myself.
Since then, I have implemented simple versions of this idea in numerous languages. I recently decided to give it a shot again, and I felt the quality of this project was high enough that it was worth posting online!
I know very little about licensing code, so if anyone is interested in selling me on a license to pick, or has anything else they'd like to discuss, feel free to shoot me a message on any of my social medias :)
