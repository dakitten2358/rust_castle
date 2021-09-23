use specs::prelude::*;

use crate::{StateAction};
use crate::components::{Player, Position, Movement, PickupTrigger, InventoryComponent};

pub struct PickupTriggerSystem {
    pub state_action: StateAction,
}

impl<'a> System<'a> for PickupTriggerSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Movement>,
        ReadStorage<'a, PickupTrigger>,
        WriteStorage<'a, InventoryComponent>
    );

    fn run(&mut self, (entities, players, positions, movements, pickup_triggers, mut inventories): Self::SystemData) {
        let mut picked_up_items : Vec<Entity> = Vec::new();
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

        self.state_action = StateAction::DeleteEntities { entities: picked_up_items.clone() };
    }
}

impl PickupTriggerSystem {
    pub fn new() -> Self {
        Self {
            state_action: StateAction::None,
        }
    }

}