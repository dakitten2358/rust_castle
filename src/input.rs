use rltk::VirtualKeyCode;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct PlayerInputComponent {
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
}

impl PlayerInputComponent {
    pub fn new() -> Self {
        Self {
            move_left: false,
            move_right: false,
            move_up: false,
            move_down: false,
        }
    }

    pub fn clear(&mut self) { 
        self.move_left = false;
        self.move_right = false;
        self.move_up = false;
        self.move_down = false;
    }
}

#[derive(Component, Debug)]
pub struct PlayerInputMappingComponent {}

impl PlayerInputMappingComponent {
    pub fn process_key(&self, key: VirtualKeyCode, player_input: &mut PlayerInputComponent) {
        match key {
            // Movement
            VirtualKeyCode::Left => player_input.move_left = true,
            VirtualKeyCode::Right => player_input.move_right = true,
            VirtualKeyCode::Up => player_input.move_up = true,
            VirtualKeyCode::Down => player_input.move_down = true,            
            // Default
            _ => {}
        }
    }
}

#[derive(Component, Debug)]
pub struct PlayerTextInputComponent {
    input_text: String,
    submitted: bool,
}

impl PlayerTextInputComponent {
    pub fn new() -> Self {
        Self {
            input_text: String::from(""),
            submitted: false,
        }
    }

    pub fn add_character(&mut self, c: char) {
        if !self.submitted {
            self.input_text.push(c);
        }
    }

    pub fn add_space(&mut self) {
        if !self.submitted {
            self.add_character(' ');
        }
    }

    pub fn backspace(&mut self) {
        if !self.submitted {
            self.input_text.pop();
        }
    }

    pub fn clear_text(&mut self) {
        if !self.submitted {
            self.input_text.clear();
        }
    }

    pub fn get_preview(&self) -> String {
        self.input_text.clone()
    }

    pub fn submit(&mut self) {
        self.submitted = true;
    }

    pub fn consume(&mut self) -> Option<String> {
        if self.submitted {
            let text = self.input_text.clone();
        
            self.submitted = false;
            self.input_text.clear();
            Some(text)
        }
        else
        {
            None
        }
    }
}

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
