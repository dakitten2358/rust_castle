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
            VirtualKeyCode::Left => player_input.move_left = true,
            VirtualKeyCode::Right => player_input.move_right = true,
            VirtualKeyCode::Up => player_input.move_up = true,
            VirtualKeyCode::Down => player_input.move_down = true,
            _ => {}
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
}

impl<'a> System<'a> for PlayerInputSystem<'_> {
    type SystemData = (ReadStorage<'a, PlayerInputMappingComponent>, WriteStorage<'a, PlayerInputComponent>);

    fn run(&mut self, (input_mappings, mut player_inputs): Self::SystemData) {
        for (input_mapping, player_input) in (&input_mappings, &mut player_inputs).join() {
            player_input.clear();
            match self.context.key {
                None => {}
                Some(key) => input_mapping.process_key(key, player_input)
            }
        }
    }
}
