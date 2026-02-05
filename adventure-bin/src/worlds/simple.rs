use adventure::{
    engine::Game,
    builders::{
        RoomBuilder as RoomB,
        WorldBuilder as WorldB,
        ItemBuilder as ItemB,
    },
    models::{Player, Direction as Dir, ItemBehavior}
};

pub fn create_simple_game() -> Game {
    let player = Player::new();
    let world = WorldB::new()
        .with_item(0, ItemB::new_regular("stick")
            .with_print("initial", "On the ground lies a short pointy stick.")
            .with_behavior(ItemBehavior::Weapon { damage: 5 })
            .build())
        .with_item(2, ItemB::new_regular("newspaper")
            .with_behavior(ItemBehavior::Readable { text: "The newspaper headline reads: \"Local Hero Saves Village!\"\nFull story on page 3...".to_string() })
            .build())
        .with_item(1, ItemB::new("chest")
            .with_behavior(ItemBehavior::Container {
                contents: vec![2], // contains newspaper
                capacity: Some(10)
            })
            .with_behavior(ItemBehavior::Openable { is_open: false })
            .with_behavior(ItemBehavior::Size { value: 3 }) // Large
            .with_adjective("big".to_string())
            .with_print("look", "A sturdy wooden chest with iron bands.")
            .build())
        .with_room(0, RoomB::new("Entry")
            .with_desc("You stand in the entry room. There is a door to the east.")
            .with_item(0) // stick
            .with_item(1) // chest
            .with_exit(Dir::East, 1) // hallway
            .build())
        .with_room(1, RoomB::new("Hallway")
            .with_desc("You stand in a large hallway. There is a door to the west.")
            .with_exit(Dir::West, 0) // entry
            .build())
        .with_initial_prints(vec!["This is a simple adventure game sample!".into()])
        .build();

    Game::new(player, world).expect("Failed to create game")
}
