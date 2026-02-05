use adventure::{
    builders::{ItemBuilder, RoomBuilder, WorldBuilder},
    models::{ItemBehavior, Determiner, Direction},
};

pub fn create_world() -> WorldBuilder {
    let torch = ItemBuilder::new("torch") // 0
        .with_determiner(Determiner::Singular)
        .with_behavior(ItemBehavior::Holdable)
        .with_behavior(ItemBehavior::Size { value: 1 })
        .with_print("initial", "A flickering torch rests in a wall sconce.")
        .build();

    let key = ItemBuilder::new("key") // 1
        .with_determiner(Determiner::Singular)
        .with_adjective("rusty".to_string())
        .with_behavior(ItemBehavior::Holdable)
        .with_behavior(ItemBehavior::Size { value: 0 })
        .with_print("initial", "A rusty iron key lies forgotten in the corner.")
        .build();

    let letter = ItemBuilder::new("letter") // 2
        .with_determiner(Determiner::Singular)
        .with_behavior(ItemBehavior::Holdable)
        .with_behavior(ItemBehavior::Readable {
            text: "Dear Brother,\nThe treasure is hidden in the locked chest. The key?\nI dropped it somewhere in the cell. Good luck!\n- Marcus".to_string()
        })
        .with_behavior(ItemBehavior::Size { value: 0 })
        .build();

    let dagger = ItemBuilder::new("dagger") // 3
        .with_determiner(Determiner::Singular)
        .with_behavior(ItemBehavior::Holdable)
        .with_behavior(ItemBehavior::Weapon { damage: 5 })
        .with_behavior(ItemBehavior::Size { value: 1 })
        .build();

    let coins = ItemBuilder::new("coins") // 4
        .with_determiner(Determiner::Plural)
        .with_adjective("gold".to_string())
        .with_behavior(ItemBehavior::Holdable)
        .with_behavior(ItemBehavior::Size { value: 0 })
        .with_print("initial", "A pile of gleaming gold coins catches your eye.")
        .build();

    let ruby = ItemBuilder::new("ruby") // 5
        .with_determiner(Determiner::Singular)
        .with_adjective("red".to_string())
        .with_behavior(ItemBehavior::Holdable)
        .with_behavior(ItemBehavior::Size { value: 0 })
        .build();

    let chest = ItemBuilder::new("chest") // 6
        .with_determiner(Determiner::Singular)
        .with_adjective("wooden".to_string())
        .with_behaviors(vec![
            ItemBehavior::Container {
                contents: vec![4, 5],
                capacity: Some(10), // coins and ruby
            },
            ItemBehavior::Openable { is_open: false },
            ItemBehavior::Locked {
                is_locked: true,
                key_id: Some(1),
            },
            ItemBehavior::Size { value: 3 },
        ])
        .with_print("initial", "A heavy wooden chest bound with iron sits against the wall, secured with a rusty lock.")
        .build();

    let rack = ItemBuilder::new("rack") // 7
        .with_determiner(Determiner::Singular)
        .with_adjective("weapon".to_string())
        .with_behaviors(vec![
            ItemBehavior::Container {
                contents: vec![3], // dagger
                capacity: Some(5),
            },
            ItemBehavior::Openable { is_open: true },
            ItemBehavior::Size { value: 4 },
        ])
        .with_print("initial", "A wooden weapon rack stands against the east wall.")
        .build();

    let skeleton = ItemBuilder::new("skeleton") // 8
        .with_determiner(Determiner::Singular)
        .with_behavior(ItemBehavior::Size { value: 2 })
        .with_print("initial", "The skeletal remains of a prisoner lie crumpled in the corner, still wearing tattered rags.")
        .with_print("look", "The skeleton has been here for years. Nothing remains but bones and rotting cloth.")
        .build();

    WorldBuilder::new()
        // Items
        .with_item(0, torch)
        .with_item(1, key)
        .with_item(2, letter)
        .with_item(3, dagger)
        .with_item(4, coins)
        .with_item(5, ruby)
        .with_item(6, chest)
        .with_item(7, rack)
        .with_item(8, skeleton)
        // Rooms
        .with_room(0, RoomBuilder::new("Dungeon Entrance")
            .with_desc("A dark stone corridor stretches before you.")
            .with_long_desc("You stand at the entrance to an ancient dungeon. The air is damp and cold, and the flickering torchlight barely penetrates the darkness ahead.\nStone walls, slick with moisture, stretch into the gloom. Two passages lead deeper: one north into what sounds like a guardroom, and another east toward the prison cells.")
            .with_item(0) // torch
            .with_exit(Direction::North, 2)
            .with_exit(Direction::East, 1)
            .build())
        .with_room(1, RoomBuilder::new("Prison Cell")
            .with_desc("A cramped stone cell.")
            .with_long_desc("This cramped prison cell reeks of despair and decay. The walls are covered in scratch marks and illegible writings.\nA single barred window high above lets in a shaft of pale light. The cell door stands open to the west.")
            .with_item(1) // key
            .with_item(6) // chest
            .with_item(8) // skeleton
            .with_exit(Direction::West, 0)
            .build())
        .with_room(2, RoomBuilder::new("Guard Room")
            .with_desc("An abandoned guard room.")
            .with_long_desc("This was once a guard post, but it's long since been abandoned. A simple wooden table sits in the center, covered with dust and old parchment.\nA moldy straw mattress lies in one corner. The exit leads south back to the entrance.")
            .with_item(2) // letter
            .with_item(7) // rack
            .with_exit(Direction::South, 0)
            .build())
        .with_initial_prints(vec!["You descend the stone stairs into the darkness below...\n".to_string()])
}
