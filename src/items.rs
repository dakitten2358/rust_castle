use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::saveload::*;
use std::{fmt, fs::File};

use crate::components::{Description, PickupTrigger, Position};
use crate::game::DynamicMarker;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct ItemFlags: u32 {
        const EMPTY         = 0;
        const LAMP          = 1 << 0;       // 2660
        const SCEPTER       = 1 << 1;       // 00DF
        const BOOK          = 1 << 2;       // 2584
        const MAGICWAND     = 1 << 3;       // 2500
        const SWORD         = 1 << 4;       // 253C
        const KEY           = 1 << 5;       // 03C4
        const EYEGLASSES    = 1 << 6;       // 221E
        const HELMET        = 1 << 7;       // 00A2
        const WINEFLASK     = 1 << 8;       // 0021 or 00A1
        const CRYSTALBALL   = 1 << 9;       // 00B0
        const NECKLACE      = 1 << 10;      // 00A7
        const HOLYCROSS     = 1 << 11;      // 0074
        const DIAMOND       = 1 << 12;      // 2666
        const SILVERBARS    = 1 << 13;      // 2261
        const RUBIES        = 1 << 14;      // 003A
        const JADEFIGURINE  = 1 << 15;      // 00A5
        const HARP          = 1 << 16;      // 266B
        const HOURGLASS     = 1 << 17;      // 03A6
        const LARGEGEM      = 1 << 18;      // 0398
        const GOLDBAR       = 1 << 19;      // 25A0
        const FANCYGOBLET   = 1 << 20;      // 00B5
        const CROWN         = 1 << 21;      // 2302
        const ANYTHING      = 0b1111111111111111111111111111111;
    }
}
/*
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
*/
fn item_to_glyph(item_type: ItemFlags) -> char {
    match item_type {
        ItemFlags::LAMP => '\u{2660}',
        ItemFlags::SCEPTER => '\u{00DF}',
        ItemFlags::BOOK => '\u{2584}',
        ItemFlags::MAGICWAND => '\u{2500}',
        ItemFlags::SWORD => '\u{253C}',
        ItemFlags::KEY => '\u{03C4}',
        ItemFlags::EYEGLASSES => '\u{221E}',
        ItemFlags::HELMET => '\u{00A2}',
        ItemFlags::WINEFLASK => '\u{00A1}',
        ItemFlags::CRYSTALBALL => '\u{00B0}',
        ItemFlags::NECKLACE => '\u{00A7}',
        ItemFlags::HOLYCROSS => '\u{0074}',
        ItemFlags::DIAMOND => '\u{2666}',
        ItemFlags::SILVERBARS => '\u{2261}',
        ItemFlags::RUBIES => '\u{003A}',
        ItemFlags::JADEFIGURINE => '\u{00A5}',
        ItemFlags::HARP => '\u{266B}',
        ItemFlags::HOURGLASS => '\u{03A6}',
        ItemFlags::LARGEGEM => '\u{0398}',
        ItemFlags::GOLDBAR => '\u{25A0}',
        ItemFlags::FANCYGOBLET => '\u{00B5}',
        ItemFlags::CROWN => '\u{2302}',
        _ => ' ',
    }
}

pub fn create_item(world: &mut World, room: i32, item_type: ItemFlags, x: i32, y: i32) {
    let item = find_item(item_type, &world.fetch::<Vec<ItemData>>()).expect("failed to find item").clone();
    spawn_item(world, room, &item, x, y);
}

pub fn create_item_by_name(world: &mut World, room: i32, item_name: &str, x: i32, y: i32) {
    let item = find_item_by_name(item_name, &world.fetch::<Vec<ItemData>>()).expect("failed to find item").clone();
    spawn_item(world, room, &item, x, y);
}

fn spawn_item(world: &mut World, room: i32, item: &ItemData, x: i32, y: i32) {
    match item.flag {
        _ => {
            world.create_entity()
                .with(Position{ x: x, y: y})
                .with(crate::render::Renderable::new_with_z(item_to_glyph(item.flag), rltk::WHITE, 1))
                .with(PickupTrigger{item_to_pickup: item.flag})
                .with(crate::room::BelongsToRoom { room: room })
                .with(if let Some(explicit_name) = &item.input_name { Description::new_explicit(&explicit_name, &item.name, &item.description)} else { Description::new(&item.name, &item.description) })
                .marked::<SimpleMarker<DynamicMarker>>()
                .build();
        }
    }
}

pub fn get_item_name(item_to_find: ItemFlags, world: &World) -> String {
    let items = world.fetch::<Vec<ItemData>>();
    let item = find_item(item_to_find, &items).expect("failed to find item");
    if let Some(input_name) = &item.input_name {
        return input_name.clone();
    }
    return item.name.clone();
}

fn find_item<'a>(item_to_find: ItemFlags, items: &'a Vec<ItemData>) -> Option<&'a ItemData> {
    for item in items {
        if item.flag == item_to_find {
            return Some(item);
        }
    }
    return None;
}

fn find_item_by_name<'a>(item_to_find: &str, items: &'a Vec<ItemData>) -> Option<&'a ItemData> {
    for item in items {
        if let Some(input_name) = &item.input_name {
            if input_name == item_to_find {
                return Some(item);
            }
        }
        if item.name.to_ascii_lowercase() == item_to_find {
            return Some(item);
        }
    }
    return None;
}

#[derive(Serialize, Deserialize, Clone)]
struct ItemData {
    pub flag: ItemFlags,
    pub name: String, 
    pub input_name: Option<String>,
    pub description: String,
}

pub fn load_items(world: &mut World)
{
    //save_items();

    let f = File::open("data/items.json").expect("item data not found");
    let items: Vec<ItemData> =
        serde_json::from_reader(f).expect("failed to deserializer!");

    world.insert(items);
}

fn save_items()
{
    let mut items = Vec::new();

    let item = ItemData { flag: ItemFlags::LAMP, name: "Lamp".to_string(), input_name: Some("lamp".to_string()), description: "It's bright!".to_string()};
    items.push(item);

    let writer = std::fs::File::create("./data/items.json").unwrap();
    let mut serializer = serde_json::Serializer::pretty(writer);

    (&items).serialize(&mut serializer);
}