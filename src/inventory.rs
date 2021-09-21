use specs::prelude::*;
use specs_derive::Component;
use bitflags::bitflags;
use std::fmt;

use crate::{StateAction};
use crate::game::{Player, Position, Movement};

pub fn test_item(world: &mut World) {
    world.create_entity()
        .with(Position{ x: 10, y: 7})
        .with(crate::render::Renderable::new_with_z('\u{2660}', rltk::WHITE, 1))
        .with(PickupTrigger{item_to_pickup: ItemFlags::LAMP})
        //.with(DebugHudComponent{})
        .build();
}


bitflags! {
    pub struct ItemFlags: u32 {
        const EMPTY         = 0;
        const LAMP          = 1 << 0;
        const SCEPTER       = 1 << 1;
        const BOOK          = 1 << 2;
        const MAGICWAND     = 1 << 3;
        const SWORD         = 1 << 4;
        const KEY           = 1 << 5;
        const EYEGLASSES    = 1 << 6;
        const HELMET        = 1 << 7;
        const WINEFLASK     = 1 << 8;
        const CRYSTALBALL   = 1 << 9;
        const NECKLACE      = 1 << 10;
        const HOLYCROSS     = 1 << 11;
        const DIAMOND       = 1 << 12;
        const SILVERBARS    = 1 << 13;
        const RUBIES        = 1 << 14;
        const JADEFIGURINE  = 1 << 15;
        const HARP          = 1 << 16;
        const HOURGLASS     = 1 << 17;
        const LARGEGEM      = 1 << 18;
        const GOLDBAR       = 1 << 19;
        const FANCYGOBLET   = 1 << 20;
        const CROWN         = 1 << 21;
    }
}

impl fmt::Display for ItemFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ItemFlags::LAMP => write!(f, "Lamp"),
            ItemFlags::SCEPTER => write!(f, "Scepter"),
            ItemFlags::BOOK => write!(f, "Book"),
            ItemFlags::MAGICWAND => write!(f, "Magic Wand"),
            ItemFlags::SWORD => write!(f, "Sword"),
            ItemFlags::KEY => write!(f, "Key"),
            ItemFlags::EYEGLASSES => write!(f, "Eye Glasses"),
            ItemFlags::HELMET => write!(f, "Helmet"),
            ItemFlags::WINEFLASK => write!(f, "Wine Flask"),
            ItemFlags::CRYSTALBALL => write!(f, "Crystal Ball"),
            ItemFlags::NECKLACE => write!(f, "Necklace"),
            ItemFlags::HOLYCROSS => write!(f, "Holy Cross"),
            ItemFlags::DIAMOND => write!(f, "Diamond"),
            ItemFlags::SILVERBARS => write!(f, "Silver Bars"),
            ItemFlags::RUBIES => write!(f, "Rubies"),
            ItemFlags::JADEFIGURINE => write!(f, "Jade Figurine"),
            ItemFlags::HARP => write!(f, "Harp"),
            ItemFlags::HOURGLASS => write!(f, "Hourglass"),
            ItemFlags::LARGEGEM => write!(f, "Large Gem"),
            ItemFlags::GOLDBAR => write!(f, "Gold Bar"),
            ItemFlags::FANCYGOBLET => write!(f, "Fancy Goblet"),
            ItemFlags::CROWN => write!(f, "Crown"),
            _ => write!(f, "<error>"),
        }
    }
}

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