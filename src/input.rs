use rltk::VirtualKeyCode;
use specs::prelude::*;
use crate::components::*;

pub struct PlayerInputSystem<'a> {
    context: &'a rltk::Rltk
}

impl<'a> PlayerInputSystem<'a> {
    pub fn new(with_context: &'a rltk::Rltk) -> Self {
        Self {
            context: with_context
        }
    }

    fn process_text_input(player_text_input: &mut PlayerTextInputComponent, key: VirtualKeyCode) {
        match key {
            // Text input
            VirtualKeyCode::A => player_text_input.add_character('a'),
            VirtualKeyCode::B => player_text_input.add_character('b'),
            VirtualKeyCode::C => player_text_input.add_character('c'),
            VirtualKeyCode::D => player_text_input.add_character('d'),
            VirtualKeyCode::E => player_text_input.add_character('e'),
            VirtualKeyCode::F => player_text_input.add_character('f'),
            VirtualKeyCode::G => player_text_input.add_character('g'),
            VirtualKeyCode::H => player_text_input.add_character('h'),
            VirtualKeyCode::I => player_text_input.add_character('i'),
            VirtualKeyCode::J => player_text_input.add_character('j'),
            VirtualKeyCode::K => player_text_input.add_character('k'),
            VirtualKeyCode::L => player_text_input.add_character('l'),
            VirtualKeyCode::M => player_text_input.add_character('m'),
            VirtualKeyCode::N => player_text_input.add_character('n'),
            VirtualKeyCode::O => player_text_input.add_character('o'),
            VirtualKeyCode::P => player_text_input.add_character('p'),
            VirtualKeyCode::Q => player_text_input.add_character('q'),
            VirtualKeyCode::R => player_text_input.add_character('r'),
            VirtualKeyCode::S => player_text_input.add_character('s'),
            VirtualKeyCode::T => player_text_input.add_character('t'),
            VirtualKeyCode::U => player_text_input.add_character('u'),
            VirtualKeyCode::V => player_text_input.add_character('v'),
            VirtualKeyCode::X => player_text_input.add_character('x'),
            VirtualKeyCode::Y => player_text_input.add_character('y'),
            VirtualKeyCode::Z => player_text_input.add_character('z'),
            VirtualKeyCode::Space => player_text_input.add_space(),
            VirtualKeyCode::Back => player_text_input.backspace(),
            VirtualKeyCode::Escape => player_text_input.clear_text(),
            VirtualKeyCode::Return => player_text_input.submit(),
            VirtualKeyCode::Key0 => player_text_input.add_character('0'),
            VirtualKeyCode::Key1 => player_text_input.add_character('1'),
            VirtualKeyCode::Key2 => player_text_input.add_character('2'),
            VirtualKeyCode::Key3 => player_text_input.add_character('3'),
            VirtualKeyCode::Key4 => player_text_input.add_character('4'),
            VirtualKeyCode::Key5 => player_text_input.add_character('5'),
            VirtualKeyCode::Key6 => player_text_input.add_character('6'),
            VirtualKeyCode::Key7 => player_text_input.add_character('7'),
            VirtualKeyCode::Key8 => player_text_input.add_character('8'),
            VirtualKeyCode::Key9 => player_text_input.add_character('9'),
            _ => {}
        }
    }
}

impl<'a> System<'a> for PlayerInputSystem<'_> {
    type SystemData = (ReadStorage<'a, PlayerInputMappingComponent>, WriteStorage<'a, PlayerInputComponent>, WriteStorage<'a, PlayerTextInputComponent>);

    fn run(&mut self, (input_mappings, mut player_inputs, mut player_text_inputs): Self::SystemData) {
        for (input_mapping, player_input) in (&input_mappings, &mut player_inputs).join() {
            player_input.clear();
            match self.context.key {
                None => {}
                Some(key) => input_mapping.process_key(key, player_input)
            }
        }
        for player_text_input in (&mut player_text_inputs).join() {
            match self.context.key {
                None => {},
                Some(key) => PlayerInputSystem::process_text_input(player_text_input, key)
            }
        }
    }

    
}
