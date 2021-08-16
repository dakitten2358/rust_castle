use rltk::{Rltk, GameState};
use specs::prelude::*;

mod input;

#[allow(dead_code)]
struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self, context: &Rltk) {
        let mut player_input_system = input::PlayerInputSystem::new(context);
        player_input_system.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();
        self.run_systems(context);
        context.print(1,1, "hello world");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let terminal_builder = RltkBuilder::simple(40,25)?;
    let context = terminal_builder
        .with_title("Castle Adventure!")
        .build()?;
    
    let mut game_state = State {
        ecs: World::new()
    };

    game_state.ecs.register::<input::PlayerInputComponent>();
    game_state.ecs.register::<input::PlayerInputMappingComponent>();

    rltk::main_loop(context, game_state)
}
