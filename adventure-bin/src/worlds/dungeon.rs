use adventure::{
    builders::{ItemBuilder, RoomBuilder, WorldBuilder},
    models::{ConditionEffect, ConditionPredicate, ConditionTrigger, Determiner, Direction, ItemBehavior, condition::predicate::ItemRole}, parser::Verb,
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

    let scroll = ItemBuilder::new("scroll") // 9
        .with_determiner(Determiner::Singular)
        .with_adjective("ancient".to_string())
        .with_behavior(ItemBehavior::Holdable)
        .with_behavior(ItemBehavior::Readable {
            text: "The secret chamber reveals itself only to those who possess the torch's light and have explored every corner of this forsaken place, even beneath their own feet.".to_string()
        })
        .with_behavior(ItemBehavior::Size { value: 0 })
        .with_behavior(ItemBehavior::Hidden { revealed: false })
        .with_print("initial", "A mysteriously glowing scroll lies on the ground.")
        .build();

    let orb = ItemBuilder::new("orb") // 10
        .with_determiner(Determiner::Singular)
        .with_adjective("glowing".to_string())
        .with_behavior(ItemBehavior::Holdable)
        .with_behavior(ItemBehavior::Size { value: 1 })
        .with_print("initial", "A mystical orb pulses with ethereal blue light on a stone pedestal.")
        .with_print("look", "The orb's light swirls hypnotically. You sense ancient magic within.")
        .build();

    let guard_table = ItemBuilder::new("table") // 11
        .with_adjective("wooden".to_string())
        .with_behavior(ItemBehavior::Hidden { revealed: false })
        .build();

    let guard_diary = ItemBuilder::new_regular("diary") // 12
        .with_determiner(Determiner::Singular)
        .build();

    // Reveal scroll after visiting all rooms with torch
    let reveal_scroll_condition = ConditionTrigger {
        predicates: ConditionPredicate::And { predicates: vec![
            ConditionPredicate::HasItem { item_id: 0 }, // has torch
            ConditionPredicate::RoomVisited { room_id: 1 }, // visited cell
            ConditionPredicate::RoomVisited { room_id: 2 }, // visited guard room
            ConditionPredicate::GameFlag { flag_name: "scroll_spawned".into(), value: false }, // Scroll has not been "spawned"
        ]},
        effects: vec![
            ConditionEffect::RevealItem { item_id: 9 },
            ConditionEffect::PrintMessage { 
                text: "The torch's light flickers strangely, revealing something hidden...".into() 
            },
            ConditionEffect::SetFlag { flag_name: "scroll_spawned".into(), value: true }
        ]
    };

    // Block entrance to secret chamber before scroll spawned
    let secret_chamber_block_pre = ConditionTrigger {
        predicates: ConditionPredicate::And { predicates: vec![
            ConditionPredicate::GameFlag { flag_name: "scroll_spawned".into(), value: false },
            ConditionPredicate::UsingVerb { verb: Verb::Move { dir: Some(Direction::Down) } }
        ]},
        effects: vec![
            ConditionEffect::BlockAction { message: Some("You cant go down".into()) }
        ]
    };

    // Block entrance to secret chamber without torch
    let secret_chamber_block_post = ConditionTrigger {
        predicates: ConditionPredicate::And { predicates: vec![
            ConditionPredicate::GameFlag { flag_name: "scroll_spawned".into(), value: true },
            ConditionPredicate::LacksItem { item_id: 0 },
            ConditionPredicate::UsingVerb { verb: Verb::Move { dir: Some(Direction::Down) } }
        ]},
        effects: vec![
            ConditionEffect::BlockAction { 
                message: Some("The passage is too dark to enter without a light source.".into()) 
            },
        ]
    };

    let no_peeping = ConditionTrigger {
        predicates: ConditionPredicate::And { predicates: vec![
            ConditionPredicate::UsingVerb { verb: Verb::Read },
            ConditionPredicate::TargetingItem { item_id: 12, role: Some(ItemRole::Direct) }
        ]},
        effects: vec![
            ConditionEffect::BlockAction { message: Some("That would be rude...".to_string()) }
        ]
    };

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
        .with_item(9, scroll)
        .with_item(10, orb)
        .with_item(11, guard_table)
        .with_item(12, guard_diary)

        // Rooms
        .with_room(0, RoomBuilder::new("Dungeon Entrance")
            .with_desc("A dark stone corridor stretches before you.")
            .with_long_desc("You stand at the entrance to an ancient dungeon. The air is damp and cold, and the flickering torchlight barely penetrates the darkness ahead.\nStone walls, slick with moisture, stretch into the gloom. Two passages lead deeper: one north into what sounds like a guardroom, and another east toward the prison cells.")
            .with_item(0) // torch
            .with_item(9) // hidden scroll (revealed by condition)
            .with_exit(Direction::North, 2)
            .with_exit(Direction::East, 1)
            .with_exit(Direction::Down, 4) // secret chamber
            .with_condition(secret_chamber_block_pre) // Block entry without torch
            .with_condition(secret_chamber_block_post) // Block entry without torch
            .with_conditional_desc(
                ConditionPredicate::And { predicates: vec![
                    ConditionPredicate::GameFlag { flag_name: "scroll_spawned".to_string(), value: true },
                    ConditionPredicate::HasItem { item_id: 0 }
                ]},
                "The torchlght flickers strangely against the floor.",
                false
            )
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
            .with_long_desc("This was once a guard post, but it's long since been abandoned. A simple wooden table sits in the center, covered with dust and old parchment.\nA ladder can be seen in the corner leading up. The south exit leads back to the entrance.\n")
            .with_item(2) // letter
            .with_item(7) // rack
            .with_item(11) // table
            .with_exit(Direction::South, 0)
            .with_exit(Direction::Up, 3)
            .build())

        .with_room(3, RoomBuilder::new("Guard Bedroom")
            .with_desc("A guard bedroom.")
            .with_long_desc("A guard bedroom. A moldy straw mattress lies in one corner. Nearby lies a leather bound booklet, seemingly a diary.\nA ladder leads back down to the guard post.")
            .with_item(12)
            .with_exit(Direction::Down, 2)
            .build())

        .with_room(4, RoomBuilder::new("Secret Chamber")
            .with_desc("A hidden chamber filled with mystical energy.")
            .with_long_desc("You have discovered a secret chamber! The walls are lined with ancient runes that glow faintly in the torchlight.\nThe air hums with magical energy. Stone stairs lead back up to the entrance.")
            .with_item(10) // orb
            .with_exit(Direction::Up, 0)
            .build())

        // Other game elements
        .with_global_condition(reveal_scroll_condition) // Reveal scroll after exploration
        .with_global_condition(no_peeping)
        .with_initial_prints(vec!["You descend the stone stairs into the darkness below...\n".to_string()])
        .with_flag("scroll_spawned", false)
}
