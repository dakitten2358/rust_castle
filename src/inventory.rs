use specs::prelude::*;
use specs_derive::Component;
use std::cmp::*;
use std::str::FromStr;

#[derive(Component, Debug)]
pub struct PickupTrigger {
    pub item_to_pickup: String,
}

#[derive(Component)]
pub struct InventoryComponent {
    items: Vec<String>
}

impl InventoryComponent {
    pub fn add(&mut self, item: &str) {
        self.items.push(String::from_str(item).unwrap());
    }
}

pub struct PickupTriggerSystem {}

impl<'a> System<'a> for PickupTriggerSystem {
    type SystemData = (ReadStorage<'a, crate::game::Player>, ReadStorage<'a, crate::game::Position>, ReadStorage<'a, crate::game::Movement>, ReadStorage<'a, PickupTrigger>, WriteStorage<'a, InventoryComponent>);

    fn run(&mut self, (players, positions, movements, pickup_triggers, mut inventories): Self::SystemData) {
        for (_player, movement, position, inventory) in (&players, &movements, &positions, &mut inventories).join() {
            if movement.did_move() {
                for (pickup_trigger, pickup_position) in (&pickup_triggers, &positions).join() {
                    if position == pickup_position {
                        inventory.add(pickup_trigger.item_to_pickup.as_str());
                        return;
                    }
                }
            }
        }
    }
}

impl PickupTriggerSystem {
    pub fn new() -> Self {
        Self {}
    }
}