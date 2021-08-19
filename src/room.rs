use std::fs::File;
use std::io::Read;
use specs::prelude::*;
use std::str;
use regex::Regex;
use specs_derive::Component;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExitDirection {
    Invalid,

    North,
    South,
    East,
    West,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Collision
{
    Enabled,
    Disabled,
}

#[derive(Debug, Copy, Clone)]
pub struct TileData {
    pub glyph: char,
    pub collision: Collision,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct ExitData {
    pub direction: ExitDirection,
    pub to_room: i32,
}

#[derive(Debug)]
pub struct RoomData {
    tiles: Vec<TileData>,
    pub description: Vec<String>,
    exits: Vec<ExitData>,
}

impl Clone for RoomData {
    fn clone(&self) -> Self {
        let mut new = Self::new();
        for tile_data in &self.tiles {
            new.tiles.push(*tile_data);
        }
        for description in &self.description {
            new.description.push(description.clone())
        }
        for exit_data in &self.exits {
            new.exits.push(*exit_data);
        }
        return new;
    }
}

impl RoomData {
    fn new() -> Self {
        Self { 
            tiles: Vec::new(),
            description: Vec::new(),
            exits: Vec::new()
        }
    }
}

#[derive(Component)]
pub struct BelongsToRoom { 
    pub room: i32,
}

#[derive(Component, Debug)]
pub struct ExitTrigger {
    pub from_direction: ExitDirection,
    pub to_room: i32,
}

impl ExitTrigger {
    fn new(from_direction: ExitDirection, to_room: i32) -> Self {
        Self {
            from_direction: from_direction,
            to_room: to_room,
        }
    }
}

pub fn change_room(world: &mut World, new_room: i32, old_room: i32) {
    // remove the old room
    let old_entities = find_entities_to_remove(world, old_room);
    for old_entity in old_entities {
        world.delete_entity(old_entity).expect("Unable to delete entity");
    }
    // set up the new room
    let room_data = get_room_data(world, new_room);
    create_room_entities(world, new_room, &room_data);
}

fn find_entities_to_remove(world: &mut World, old_room: i32) -> Vec<Entity> {
    let entities = world.entities();
    let room_ownership = world.read_storage::<BelongsToRoom>();

    let mut entities_to_delete = Vec::new();
    for entity in entities.join() {
        let belongs_to_room = room_ownership.get(entity);
        match belongs_to_room {
            Some(b) if b.room == old_room => { entities_to_delete.push(entity); }
            _ => {}
        }        
    }
    entities_to_delete
}

pub fn get_room_data(world: &World, room: i32) -> RoomData {
    let room_datas = world.fetch::<Vec<RoomData>>();
    room_datas[room as usize].clone()
}

pub fn create_room_entities(world: &mut World, room: i32, room_data: &RoomData) {
    // create entities for hte tiles
    for tile in &room_data.tiles {
        let mut entity_builder = world.create_entity();
        entity_builder = entity_builder.with(crate::game::Position{ x: tile.x, y: tile.y});
        entity_builder = entity_builder.with(crate::render::Renderable::new(tile.glyph, rltk::GREY));

        if tile.collision == Collision::Enabled {
            entity_builder = entity_builder.with(crate::game::ColliderComponent{});
        }

        match exit_data_for_tile(room_data, tile) {
            Some(exit_data) => { entity_builder = entity_builder.with(ExitTrigger::new(exit_data.direction, exit_data.to_room))},
            _ => {}
        }

        entity_builder = entity_builder.with(BelongsToRoom{room: room});
        entity_builder.build();
    }

    // find any open edges, and create some entities with just the trigger on them
    for row in 0..18 {
        if !is_tile_at(0, row, &room_data.tiles) {
            match find_exit_data(ExitDirection::West, &room_data.exits) {
                Some(exit_data) => { create_edge_exit_entity(world, room, -1, row, exit_data.direction, exit_data.to_room); },
                _ => {}
            }

        }
        if !is_tile_at(23, row, &room_data.tiles) {
            match find_exit_data(ExitDirection::East, &room_data.exits) {
                Some(exit_data) => { create_edge_exit_entity(world, room, 24, row, exit_data.direction, exit_data.to_room); },
                _ => {}
            }
        }
    }

    for col in 1..23 {
        if !is_tile_at(col, 0, &room_data.tiles) {
            match find_exit_data(ExitDirection::North, &room_data.exits) {
                Some(exit_data) => { create_edge_exit_entity(world, room, col, -1, exit_data.direction, exit_data.to_room); },
                _ => {}
            }
        }
        if !is_tile_at(col, 17, &room_data.tiles) {
            match find_exit_data(ExitDirection::South, &room_data.exits) {
                Some(exit_data) => { create_edge_exit_entity(world, room, col, 18, exit_data.direction, exit_data.to_room); },
                _ => {}
            }
        }
    }
}

fn create_edge_exit_entity(world: &mut World, room: i32, x: i32, y: i32, direction: ExitDirection, to_room: i32) {
    world.create_entity()
        .with(crate::game::Position{x: x, y: y})
        .with(ExitTrigger::new(direction, to_room))
        .with(BelongsToRoom { room: room })
        //.with(crate::render::Renderable::new('x', rltk::YELLOW))
        .build();
}

fn is_tile_at(x: i32, y: i32, tiles: &Vec<TileData>) -> bool {
    for &tile_data in tiles {
        if tile_data.x == x && tile_data.y == y { return true; }
    }
    false
}

fn exit_data_for_tile(room_data: &RoomData, tile: &TileData) -> Option<ExitData> {
    match tile.glyph {
        'U' => find_exit_data(ExitDirection::Up, &room_data.exits),
        'D' => find_exit_data(ExitDirection::Down, &room_data.exits),
        _ => None
    }
}

fn find_exit_data(direction: ExitDirection, exit_datas: &Vec<ExitData>) -> Option<ExitData> {
    for exit_data in exit_datas {
        if exit_data.direction == direction { return Some(*exit_data); }
    }
    return None;
}

pub fn load_rooms(world: &mut World) {
    let mut f = File::open("data/castle.ran").expect("data not found");
    let mut rooms = Vec::new();

    for room in 0..83 {

        let mut room_data_buffer: [u8; 24*18] = [0; 24*18];
        f.read(&mut room_data_buffer).expect("failed to read room data");

        let mut room_data = RoomData::new();
        for row in 0..18 {
            for col in 0..24 {
                let t = room_data_buffer[(row * 24) + col];
                
                // blank tile, skip it
                if t == 32 { continue };

                let mut tile_data = get_tile_data_from_ascii_char(t);
                tile_data.x = col as i32;
                tile_data.y = row as i32;
                room_data.tiles.push(tile_data);                
            }
        }

        let mut room_description_buffer: [u8; 25*5] = [0; 25*5];
        f.read(&mut room_description_buffer).expect("failed ot read room description");
    
        for row in 0..5 {
            let line_bytes = &room_description_buffer[row * 25 .. (row * 25) + 25];
            let line_text = str::from_utf8(line_bytes).expect("failed to convert description text");
            room_data.description.push(String::from(line_text));
        }

        let mut room_exits_bytes: [u8; 18] = [0; 18];
        f.read(&mut room_exits_bytes).expect("failed to read exit string");

        let exit_text = str::from_utf8(&room_exits_bytes).unwrap();
        println!("exits for room {} are {}", room, exit_text);

        let exit_regex = Regex::new(r"(?P<direction>[A-Z])(?P<room>\d+)").unwrap();
        for captures in exit_regex.captures_iter(exit_text) {
            let direction_text = &captures["direction"];
            let room_text = &captures["room"];

            let direction = match direction_text {
                "N" => ExitDirection::North,
                "E" => ExitDirection::East,
                "S" => ExitDirection::South,
                "W" => ExitDirection::West,
                "U" => ExitDirection::Up,
                "D" => ExitDirection::Down,
                _ => ExitDirection::Invalid,
            };

            let room_index = room_text.parse::<i32>().unwrap();

            let exit_data = ExitData { direction: direction, to_room: room_index - 1};
            room_data.exits.push(exit_data);
        }

        rooms.push(room_data);
    }

    world.insert(rooms);
}

fn get_tile_data_from_ascii_char(ascii_char: u8) -> TileData {
    TileData { 
        glyph: map_ascii_to_char(ascii_char),
        collision: map_ascii_to_collision(ascii_char),
        x: 0,
        y: 0,
    }
}

fn map_ascii_to_char(ascii_char: u8) -> char {
    match ascii_char {
        178 => '\u{2593}',  // wall
        205 => '\u{2550}',  // horiz double pipe
        219 => '\u{2588}',  // solid wall
        218 => '\u{250C}',  // upper left single pipe
        196 => '\u{2500}',  // horiz single pipe
        191 => '\u{2510}',  // upper right single pipe
        192 => '\u{2514}',  // lower left single pipe
        217 => '\u{2518}',  // lower right single pipe
        177 => '\u{2592}',  // bush??
        176 => '\u{2591}',  // bush??
        179 => '\u{2502}',  // vertical single pipe
        226 => '\u{0393}',  // r shape?
        224 => '\u{03B1}',  // a (alpha shape)
        195 => '\u{251C}',  // left t pipe
        107 => '\u{006B}',  // k shape (kobold?)
        98 => 'b',          // b shape (bat?)
        247 => '\u{2248}',  // approx equals
        _ => ascii_char.into(),
    }
}

fn map_ascii_to_collision(ascii_char: u8) -> Collision {
    match ascii_char {
        0 => Collision::Disabled,
        85 => Collision::Disabled,
        68 => Collision::Disabled,
        _ => Collision::Enabled,
    }
}