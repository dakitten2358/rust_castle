use std::fs::File;
use std::io::Read;
use specs::prelude::*;
use std::str;
use regex::Regex;
use specs_derive::Component;

#[derive(Debug, Copy, Clone)]
pub enum ExitDirection {
    Invalid,

    North,
    South,
    East,
    West,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
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
    direction: ExitDirection,
    to_room: i32,
}

#[derive(Debug)]
pub struct RoomData {
    tiles: Vec<TileData>,
    description: Vec<String>,
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

pub fn change_room(world: &mut World, new_room: i32, old_room: i32) {
    // remove the old room
    let room_data = get_room_data(world, new_room);
    create_room_entities(world, &room_data);
}

pub fn get_room_data(world: &World, room: i32) -> RoomData {
    let room_datas = world.fetch::<Vec<RoomData>>();
    room_datas[room as usize].clone()
}

pub fn create_room_entities(world: &mut World, room_data: &RoomData) {
    for tile in &room_data.tiles {
        let mut entity_builder = world.create_entity();
        entity_builder = entity_builder.with(crate::game::Position{ x: tile.x, y: tile.y});
        entity_builder = entity_builder.with(crate::render::Renderable::new(tile.glyph, rltk::GREY));
        entity_builder.build();
    }
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

            let exit_data = ExitData { direction: direction, to_room: room_index};
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
        85 => Collision::Enabled,
        68 => Collision::Enabled,
        _ => Collision::Disabled,
    }
}