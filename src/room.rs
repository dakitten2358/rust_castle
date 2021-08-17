use std::fs::File;
use std::io::Read;
use specs::prelude::*;
use std::str;

pub fn test_read_file(world: &mut World) {
    let mut f = File::open("data/castle.ran").expect("data not found");
    for room in 0..83 {

        let mut room_data_buffer: [u8; 24*18] = [0; 24*18];
        f.read(&mut room_data_buffer).expect("failed to read room data");

        for row in 0..18 {
            for col in 0..24 {
                let t = room_data_buffer[(row * 24) + col];
                
                // blank tile, skip it
                if t == 32 { continue };

                let ch = map_ascii_to_char(t);               
                world.create_entity()
                    .with(crate::game::Position{ x: col as i32, y: row as i32})
                    .with(crate::render::Renderable::new(ch, rltk::GREY))
                    .build();
            }
        }

        let mut room_description_buffer: [u8; 25*5] = [0; 25*5];
        f.read(&mut room_description_buffer).expect("asd");
    
        for row in 0..5 {
            for col in 0..25 {
                let t = room_description_buffer[(row * 25) + col];
                let ch = map_ascii_to_char(t);
                world.create_entity()
                    .with(crate::game::Position{ x: col as i32, y: (row + 19) as i32})
                    .with(crate::render::Renderable::new(ch, rltk::GREY))
                    .build();
            }
        }

        let mut room_exits_bytes: [u8; 18] = [0; 18];
        f.read(&mut room_exits_bytes).expect("asdasd");

        let s = str::from_utf8(&room_exits_bytes).unwrap();
        println!("exits for room {} are {}", room, s);
    }
 
    //let s = match str::from_utf8(roomDescriptionBuffer);
    //println!("desc: {}", s);
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