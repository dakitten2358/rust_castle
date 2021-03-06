use crate::components::*;
use crate::game::DynamicMarker;
use crate::items::get_item_name;
use crate::room::BelongsToRoom;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::saveload::*;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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
pub struct DynamicEnemy {
    pub name: String,
    pub position: DynamicPosition,
    pub health: Option<i32>, // Option so that when specifying the initial level layout I don't need to specify health, but I do want it saved as state
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicRoomData {
    pub room: i32,
    pub items: Vec<DynamicItemData>,
    pub descriptions: Vec<DynamicDescriptionData>,
    pub enemies: Vec<DynamicEnemy>,
    pub map: Option<DynamicMapData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicMapData {
    pub map: Vec<u8>,
    pub description: Vec<String>,
    pub exits: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DynamicActionData {
    GiveItem { item: String },
    AddFlag { flag: String },
    UpdateMap { x: i32, y: i32 },
}

impl DynamicRoomData {
    pub fn empty(room: i32) -> Self {
        Self {
            room: room,
            items: Vec::new(),
            descriptions: Vec::new(),
            enemies: Vec::new(),
            map: None,
        }
    }
}

pub fn load_dynamic_rooms(world: &mut World) {
    setup_dynamic_room_data_example();

    let f = File::open("data/dynrooms.json").expect("data not found");
    let loaded_rooms: Vec<DynamicRoomData> = serde_json::from_reader(f).expect("failed to deserializer!");

    let mut rooms = Vec::new();
    // 83 + 2 extra = 85
    for room in 0..85 {
        match find_room(room, &loaded_rooms) {
            Some(dyn_room_data) => {
                rooms.push(dyn_room_data.clone());
                match &(dyn_room_data.map) {
                    Some(map_data) => {
                        let mut existing_rooms = world.fetch_mut::<Vec<crate::room::RoomData>>();
                        let mut dyn_room = crate::room::RoomData::new();

                        for row in 0..18 {
                            for col in 0..24 {
                                let t = map_data.map[(row * 24) + col];

                                // blank tile, skip it
                                if t == 32 {
                                    continue;
                                };

                                let mut tile_data = crate::room::get_tile_data_from_ascii_char(t);
                                tile_data.x = col as i32;
                                tile_data.y = row as i32;
                                dyn_room.tiles.push(tile_data);
                            }
                        }

                        for desc_line in &map_data.description {
                            dyn_room.description.push((*desc_line).clone());
                        }

                        for exit in crate::room::parse_exits(map_data.exits.as_str()) {
                            dyn_room.exits.push(exit);
                        }

                        existing_rooms.push(dyn_room);
                    }
                    None => {}
                }
            }
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
    for (combat_stat, description, position, _room) in (&combat_stats, &descriptions, &positions, &room_ownership).join() {
        let e = DynamicEnemy {
            name: description.input_name.clone(),
            health: Some(combat_stat.health),
            position: DynamicPosition {
                x: position.x,
                y: position.y,
            },
        };
        room_data.enemies.push(e);
    }

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

    for enemy in &room_data.enemies {
        let enemy_name = enemy.name.as_str();
        let health = enemy.health;
        crate::enemies::create_enemy(world, room, enemy_name, enemy.position.x, enemy.position.y, health);
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

fn setup_dynamic_room_data_example() {
    let mut rooms = Vec::new();

    let mut map = DynamicMapData {
        map: Vec::new(),
        description: Vec::new(),
        exits: "S83".to_string(),
    };

    let mut f = File::open("data/castle.ran").expect("data not found");
    let copy_room_index = 66;
    let _pos = f.seek(SeekFrom::Start(575 * copy_room_index));

    let mut room_data_buffer: [u8; 24 * 18] = [0; 24 * 18];
    f.read(&mut room_data_buffer).expect("failed to read");
    for row in 0..18 {
        for col in 0..24 {
            let t = room_data_buffer[(row * 24) + col];
            map.map.push(t);
        }
    }

    let room = DynamicRoomData {
        room: 1,
        items: Vec::new(),
        descriptions: Vec::new(),
        enemies: Vec::new(),
        map: Some(map),
    };
    rooms.push(room);

    let writer = std::fs::File::create("./data/dynrooms_ex.json").unwrap();
    let mut serializer = serde_json::Serializer::pretty(writer);

    (&rooms).serialize(&mut serializer).expect("failed to save example rooms");
}
