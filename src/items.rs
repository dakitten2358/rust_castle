use bitflags::bitflags;
use std::fmt;
use specs::prelude::*;

use crate::inventory::PickupTrigger;
use crate::game::{Position, Description};

bitflags! {
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

pub fn item_to_description(item_type: ItemFlags) -> (&'static str, &'static str) {
    match item_type {
        ItemFlags::LAMP => ("Lamp", "It's bright!"),
        _ => ("", ""),
    }
}

pub fn create_item_at(world: &mut World, room: i32, item_type: ItemFlags, x: i32, y:i32)
{
    match item_type {
        _ => {
            let (name, description) = item_to_description(item_type);
            world.create_entity()
                .with(Position{ x: x, y: y})
                .with(crate::render::Renderable::new_with_z(item_to_glyph(item_type), rltk::WHITE, 1))
                .with(PickupTrigger{item_to_pickup: item_type})
                .with(crate::room::BelongsToRoom { room: room })
                .with(Description::new(name, description))
                .build();
        }
    }
}
