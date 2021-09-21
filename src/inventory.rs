use specs::prelude::*;
use specs_derive::Component;

use crate::{StateAction};
use crate::game::{Player, Position, Movement};
use crate::items::{ItemFlags};

#[derive(Component, Debug)]
pub struct PickupTrigger {
    pub item_to_pickup: ItemFlags,
}

#[derive(Component)]
pub struct InventoryComponent {
    items: ItemFlags,
}

impl InventoryComponent {
    pub fn new() -> Self {
        Self {
            items: ItemFlags::EMPTY,
        }
    }
    pub fn add(&mut self, item: ItemFlags) {
        self.items |= item;
    }
}

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