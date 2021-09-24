use crate::components::*;
use crate::game::DynamicMarker;
use crate::items::ItemFlags;
use crate::room::BelongsToRoom;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::saveload::*;
use std::fs::File;

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
pub struct DynamicRoomData {
    pub room: i32,
    pub items: Vec<DynamicItemData>,
    pub descriptions: Vec<DynamicDescriptionData>,
}

impl DynamicRoomData {
    pub fn empty(room: i32) -> Self {
        Self {
            room: room,
            items: Vec::new(),
            descriptions: Vec::new(),
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
        let item_flag = crate::items::name_to_item(item_name);
        crate::room::create_item_at(world, room, item_flag, item.position.x, item.position.y);
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

fn setup_dynamic_data_example() {
    let mut rooms = Vec::new();

    let mut room = DynamicRoomData {
        room: 12,
        items: Vec::new(),
        descriptions: Vec::new(),
    };

    let item1 = DynamicItemData {
        item: "lamp".to_string(),
        position: DynamicPosition { x: 4, y: 16 },
    };
    let item2 = DynamicItemData {
        item: "hourglass".to_string(),
        position: DynamicPosition { x: 5, y: 17 },
    };

    room.items.push(item1);
    room.items.push(item2);
    rooms.push(room);

    let writer = std::fs::File::create("./data/dynrooms.json").unwrap();
    let mut serializer = serde_json::Serializer::pretty(writer);

    (&rooms).serialize(&mut serializer);
    //let j = serde_json::to_string_pretty(&rooms);
}
