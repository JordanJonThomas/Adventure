use crate::engine::{Game, action::Action};

/// Specifies which role an item plays in an action (direct target vs indirect/tool).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemRole {
    /// The primary target (e.g., item being taken, door being opened)
    Direct,
    /// The secondary target (e.g., container in "put X in Y", key in "unlock X with Y")
    Indirect,
}

/// A ConditionPredicate is a check against the game state.
#[derive(Debug, Clone)]
pub enum ConditionPredicate {
    // Inventory checks
    HasItem { item_id: i32 },
    LacksItem { item_id: i32 },
    InventoryCount { min: Option<usize>, max: Option<usize> },

    // Room/location checks
    InRoom { room_id: i32 },
    NotInRoom { room_id: i32 },
    RoomVisited { room_id: i32 },
    /// True only when entering this room (Move action targeting current room)
    EnteringRoom,

    // Item state checks
    ItemState { item_id: i32, is_open: Option<bool>, is_locked: Option<bool>, moved: Option<bool> },

    // Action/verb checks
    UsingVerb { verb: crate::parser::Verb },
    TargetingItem { item_id: i32, role: Option<ItemRole> },

    // World state checks
    TurnCount { min: Option<u32>, max: Option<u32> },
    GameFlag { flag_name: String, value: bool },

    // Composable logic
    And { predicates: Vec<ConditionPredicate> },
    Or { predicates: Vec<ConditionPredicate> },
    Not { predicate: Box<ConditionPredicate> },

    // Custom logic escape hatch
    Custom { evaluator: fn(&Game) -> bool },

    // Always true/false (useful for testing)
    Always,
    Never,
}

impl Game {
    /// Evaluates a [`ConditionPredicate`] against the current game state.
    ///
    /// Returns `true` if the condition passes, `false` otherwise.
    ///
    /// Composable predicates (And, Or, Not) support short-circuit evaluation for performance.
    ///
    /// Note: Action-based predicates (UsingVerb, TargetingItem) always return false.
    /// Use [`Self::evaluate_predicate_with_action`] for those.
    pub fn evaluate_predicate(&self, pred: &ConditionPredicate) -> bool {
        match pred {
            // Inventory checks
            ConditionPredicate::HasItem { item_id } => {
                self.player.inventory.contains(item_id)
            }
            ConditionPredicate::LacksItem { item_id } => {
                !self.player.inventory.contains(item_id)
            }
            ConditionPredicate::InventoryCount { min, max } => {
                let count = self.player.inventory.len();
                let min_ok = min.map_or(true, |m| count >= m);
                let max_ok = max.map_or(true, |m| count <= m);
                min_ok && max_ok
            }

            // Room/location checks
            ConditionPredicate::InRoom { room_id } => {
                self.current_room == *room_id
            }
            ConditionPredicate::NotInRoom { room_id } => {
                self.current_room != *room_id
            }
            ConditionPredicate::RoomVisited { room_id } => {
                self.world.rooms.get(room_id)
                    .map_or(false, |room| room.entered)
            }
            ConditionPredicate::EnteringRoom => {
                // This is only meaningful with action and game context.
                // Always false without.
                false
            }

            // Item state checks
            ConditionPredicate::ItemState { item_id, is_open, is_locked, moved } => {
                let Some(item) = self.world.items.get(item_id) else {
                    return false;
                };

                // Check each optional state condition
                let open_ok = is_open.map_or(true, |expected| item.is_open() == expected);
                let locked_ok = is_locked.map_or(true, |expected| item.is_locked() == expected);
                let moved_ok = moved.map_or(true, |expected| item.moved == expected);

                open_ok && locked_ok && moved_ok
            }

            // Action/verb checks - Require Action context
            ConditionPredicate::UsingVerb { .. } => false,
            ConditionPredicate::TargetingItem { .. } => false,

            // World state checks
            ConditionPredicate::TurnCount { min, max } => {
                let min_ok = min.map_or(true, |m| self.turn >= m);
                let max_ok = max.map_or(true, |m| self.turn <= m);
                min_ok && max_ok
            }
            ConditionPredicate::GameFlag { flag_name, value } => {
                self.world.flags.get(flag_name).map_or(false, |v| v == value)
            }

            // Composable logic (with short-circuit evaluation)
            ConditionPredicate::And { predicates } => {
                predicates.iter().all(|p| self.evaluate_predicate(p))
            }
            ConditionPredicate::Or { predicates } => {
                predicates.iter().any(|p| self.evaluate_predicate(p))
            }
            ConditionPredicate::Not { predicate } => {
                !self.evaluate_predicate(predicate)
            }

            // Custom logic escape hatch
            ConditionPredicate::Custom { evaluator } => {
                evaluator(self)
            }

            // Testing helpers
            ConditionPredicate::Always => true,
            ConditionPredicate::Never => false,
        }
    }

    /// Evaluates a [`ConditionPredicate`] with action context for verb/targeting checks.
    ///
    /// Use this in the resolver after creating an Action but before execution.
    /// For predicates that don't need action context, delegates to [`Self::evaluate_predicate`].
    pub fn evaluate_predicate_with_action(
        &self,
        pred: &ConditionPredicate,
        action: &Action,
    ) -> bool {
        match pred {
            ConditionPredicate::UsingVerb { verb } => {
                action_matches_verb(action, verb)
            }
            ConditionPredicate::TargetingItem { item_id, role } => {
                action_targets_item(action, *item_id, *role)
            }
            ConditionPredicate::EnteringRoom => {
                // Only true for Move actions (entering a room)
                matches!(action, Action::Move { .. })
            }
            // Composable predicates recurse with action context
            ConditionPredicate::And { predicates } => {
                predicates.iter().all(|p| self.evaluate_predicate_with_action(p, action))
            }
            ConditionPredicate::Or { predicates } => {
                predicates.iter().any(|p| self.evaluate_predicate_with_action(p, action))
            }
            ConditionPredicate::Not { predicate } => {
                !self.evaluate_predicate_with_action(predicate, action)
            }
            // All other predicates delegate to regular evaluation
            _ => self.evaluate_predicate(pred),
        }
    }
}

/// Utility function to check if an action corresponds to a specific verb.
fn action_matches_verb(action: &Action, verb: &crate::parser::Verb) -> bool {
    use Action;
    use crate::parser::Verb;

    match (action, verb) {
        // If the blocked verb is None, block all movement
        (Action::Move { .. }, Verb::Move { dir: None }) => true,
        // Otherwise compare dirs
        (Action::Move { dir }, Verb::Move { dir: dir2 }) => dir == dir2.as_ref().unwrap(),
        (Action::Get { .. }, Verb::Get) => true,
        (Action::Drop { .. }, Verb::Drop) => true,
        (Action::Look { .. }, Verb::Look) => true,
        (Action::Open { .. }, Verb::Open) => true,
        (Action::Close { .. }, Verb::Close) => true,
        (Action::Attack { .. }, Verb::Attack) => true,
        (Action::Unlock { .. }, Verb::Unlock) => true,
        (Action::Put { .. }, Verb::Put) => true,
        (Action::Read { .. }, Verb::Read) => true,
        _ => false,
    }
}

/// Utility function to check if an action targets a specific item in a specific role.
///
/// If `role` is `None`, checks if the item appears anywhere in the action (direct or indirect).
/// If `role` is `Some(ItemRole::Direct)`, checks only direct targets.
/// If `role` is `Some(ItemRole::Indirect)`, checks only indirect targets.
fn action_targets_item(
    action: &Action,
    item_id: i32,
    role: Option<ItemRole>,
) -> bool {
    use Action;

    match action {
        Action::Get { items } | Action::Drop { items } => {
            // These only have direct targets
            match role {
                None | Some(ItemRole::Direct) => items.contains(&item_id),
                Some(ItemRole::Indirect) => false,
            }
        }
        Action::Put { item, container } => {
            match role {
                None => item == &item_id || container == &item_id,
                Some(ItemRole::Direct) => item == &item_id,
                Some(ItemRole::Indirect) => container == &item_id,
            }
        }
        Action::Look { target } => {
            // These only have direct targets
            match role {
                None | Some(ItemRole::Direct) => target.as_ref() == Some(&item_id),
                Some(ItemRole::Indirect) => false,
            }
        }
        Action::Open { target } | Action::Close { target } | Action::Read { target} => {
            // These only have direct targets
            match role {
                None | Some(ItemRole::Direct) => target == &item_id,
                Some(ItemRole::Indirect) => false,
            }
        }
        Action::Attack { target, weapon } => {
            match role {
                None => target == &item_id || weapon.as_ref() == Some(&item_id),
                Some(ItemRole::Direct) => target == &item_id,
                Some(ItemRole::Indirect) => weapon.as_ref() == Some(&item_id),
            }
        }
        Action::Unlock { target, key } => {
            match role {
                None => target == &item_id || key == &item_id,
                Some(ItemRole::Direct) => target == &item_id,
                Some(ItemRole::Indirect) => key == &item_id,
            }
        }
        Action::Move { .. } => false,
    }
}
