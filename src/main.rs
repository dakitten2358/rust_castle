use rltk::{Rltk, GameState};
use specs::prelude::*;

mod input;
mod render;
mod game;
mod hud;
mod room;

pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self, context: &mut Rltk) {
        // immutable context
        let mut player_input_system = input::PlayerInputSystem::new(context);
        player_input_system.run_now(&self.ecs);

        let mut apply_player_movement_input = game::ApplyPlayerMovementInputSystem{};
        apply_player_movement_input.run_now(&self.ecs);

        let mut movement_system = game::MovementSystem::new();
        movement_system.run_now(&self.ecs);

        // requires mutable context
        self.draw_entities(context);
        self.draw_hud(context);

        self.ecs.maintain();
    }

    fn draw_entities(&mut self, context: &mut Rltk) {
        let mut render_system = render::RenderSystem::new(context);
        render_system.run_now(&self.ecs);
    }

    fn draw_hud(&mut self, context: &mut Rltk) {
        let mut hud_system = hud::HudSystem::new(&self, context);
        hud_system.run_now(&self.ecs);
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();
        self.run_systems(context);
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

    register_components(&mut game_state.ecs);

    room::load_rooms(&mut game_state.ecs);
    room::change_room(&mut game_state.ecs, 13, -1);

    game_state.ecs.create_entity()
        .with(input::PlayerInputMappingComponent{})
        .with(input::PlayerInputComponent::new())
        .with(game::Position{ x: 12, y: 9})
        .with(render::Renderable::new('\u{2663}', rltk::WHITE))
        .with(game::Player{})
        .with(game::Movement::new())
        .with(game::ColliderComponent{})
        .build();

    rltk::main_loop(context, game_state)
}

fn register_components(world: &mut World)
{
    world.register::<input::PlayerInputComponent>();
    world.register::<input::PlayerInputMappingComponent>();
    world.register::<render::Renderable>();
    world.register::<game::Position>();
    world.register::<game::Player>();
    world.register::<game::Movement>();
    world.register::<game::ColliderComponent>();
}
