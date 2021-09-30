use crate::components::*;
use crate::game::DynamicMarker;
use crate::room::BelongsToRoom;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::saveload::*;
use std::fs::File;
use crate::items::{get_item_name};
use crate::render::Renderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicItemData {
    pub item: String,
    pub position: DynamicPosition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicDescriptionData {
    pub keyword: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicCombatStats {
    pub health: i32, 
    pub max_health: i32,
    pub damage: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicEnemy {
    pub position: DynamicPosition,
    pub stats: DynamicCombatStats,
    pub glyph: char,
    pub description: DynamicDescriptionData,    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicRoomData {
    pub room: i32,
    pub items: Vec<DynamicItemData>,
    pub descriptions: Vec<DynamicDescriptionData>,
    //pub enemies: Vec<DynamicEnemy>,
}

impl DynamicRoomData {
    pub fn empty(room: i32) -> Self {
        Self {
            room: room,
            items: Vec::new(),
            descriptions: Vec::new(),
            //enemies: Vec::new(),
        }
    }
}

pub fn load_dynamic_rooms(world: &mut World) {
    //setup_dynamic_data_example();

    let f = File::open("data/dynrooms.json").expect("data not found");
    let loaded_rooms: Vec<DynamicRoomData> =
        serde_json::from_reader(f).expect("failed to deserializer!");

    let mut rooms = Vec::new();
    for room in 0..83 {
        match find_room(room, &loaded_rooms) {
            Some(dyn_room_data) => rooms.push(dyn_room_data.clone()),
            None => rooms.push(DynamicRoomData::empty(room)),
        }
    }

    world.insert(rooms);
}

pub fn update_dynamic_room(world: &mut World, room: i32) {
    let mut room_data = DynamicRoomData::empty(room);

    let room_ownership = world.read_storage::<BelongsToRoom>();
    let positions = world.read_storage::<Position>();
    let marked = world.read_storage::<SimpleMarker<DynamicMarker>>();
    let pickups = world.read_storage::<PickupTrigger>();

    for (pickup, pos, _mark, _room) in (&pickups, &positions, &marked, &room_ownership).join() {
        let i = DynamicItemData {
            item: get_item_name(pickup.item_to_pickup, world),
            position: DynamicPosition { x: pos.x, y: pos.y },
        };
        room_data.items.push(i);
    }

    let descriptions = world.read_storage::<Description>();
    for (desc, _mark, _room) in (&descriptions, &marked, &room_ownership).join() {
        let d = DynamicDescriptionData {
            keyword: desc.input_name.clone(),
            text: desc.description.clone(),
        };
        room_data.descriptions.push(d);
    }

    let combat_stats = world.read_storage::<CombatStats>();
    let applies_damages = world.read_storage::<AppliesDamage>();
    let renderables = world.read_storage::<Renderable>();

    /*
    for (combat_stat, applies_damage, renderable, description, position, _room) in (&combat_stats, &applies_damages, &renderables, &descriptions, &positions, &room_ownership).join() {
        let e = DynamicEnemy {
            stats: DynamicCombatStats { health: combat_stat.health, max_health: combat_stat.max_health, damage: applies_damage.damage },
            glyph: renderable.glyph;
            position: DynamicPosition { x: position.x, y: position.y },
            description: DynamicDescriptionData { keyword: description.text }
        }
    }
    */

    // save it
    let mut room_datas = world.fetch_mut::<Vec<DynamicRoomData>>();
    room_datas[room as usize] = room_data;
}

fn find_room<'a>(room_index: i32, rooms: &'a Vec<DynamicRoomData>) -> Option<&'a DynamicRoomData> {
    for room in rooms {
        if room.room == room_index {
            return Some(&room);
        }
    }
    return None;
}

fn get_dynamic_room_data(world: &World, room: i32) -> DynamicRoomData {
    let room_datas = world.fetch::<Vec<DynamicRoomData>>();
    room_datas[room as usize].clone()
}

pub fn create_dynamic_room_entities(world: &mut World, room: i32) {
    let room_data = &get_dynamic_room_data(world, room);

    for item in &room_data.items {
        let item_name = item.item.as_str();
        crate::items::create_item_by_name(world, room, item_name, item.position.x, item.position.y);
    }

    for desc in &room_data.descriptions {
        let keyword = desc.keyword.as_str();
        let description = desc.text.as_str();
        create_description(world, room, keyword, description)
    }
}

fn create_description(world: &mut World, room: i32, word: &str, description: &str) {
    world
        .create_entity()
        .with(BelongsToRoom { room: room })
        .with(Description::new(word, description))
        .marked::<SimpleMarker<DynamicMarker>>()
        .build();
}
