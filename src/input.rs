use rltk::VirtualKeyCode;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct PlayerInputComponent {
    pub move_left: bool
}

#[derive(Component, Debug)]
pub struct PlayerInputMappingComponent {}

impl PlayerInputMappingComponent {
    #[allow(unused_mut)]
    pub fn process_key(&self, key: VirtualKeyCode, player_input: &mut PlayerInputComponent) {

        player_input.move_left = false;

        match key {
            VirtualKeyCode::Left => player_input.move_left = true,
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
            match self.context.key {
                None => {}
                Some(key) => input_mapping.process_key(key, player_input)
            }
        }
    }
}
