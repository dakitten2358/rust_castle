use specs::prelude::*;

use crate::components::{InventoryComponent, Movement, PickupTrigger, Player, Position};
use crate::StateAction;

pub struct PickupTriggerSystem {}

impl<'a> System<'a> for PickupTriggerSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Movement>,
        ReadStorage<'a, PickupTrigger>,
        WriteStorage<'a, InventoryComponent>,
        WriteExpect<'a, Vec<StateAction>>,
    );

    fn run(&mut self, (entities, players, positions, movements, pickup_triggers, mut inventories, mut state_actions): Self::SystemData) {
        let mut picked_up_items: Vec<Entity> = Vec::new();
        {
            for (_player, movement, position, inventory) in (&players, &movements, &positions, &mut inventories).join() {
                if movement.did_move() {
                    for (pickup_entity, pickup_trigger, pickup_position) in (&entities, &pickup_triggers, &positions).join() {
                        if position == pickup_position {
                            inventory.add(pickup_trigger.item_to_pickup);
                            picked_up_items.push(pickup_entity);
                            break;
                        }
                    }
                }
            }
        }

        state_actions.push(StateAction::DeleteEntities {
            entities: picked_up_items.clone(),
        });
    }
}

impl PickupTriggerSystem {
    pub fn new() -> Self {
        Self {}
    }
}
