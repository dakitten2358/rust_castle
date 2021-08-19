use rltk::{Rltk, GameState};
use specs::prelude::*;

mod input;
mod render;
mod game;
mod hud;
mod room;

pub struct State {
    world: World,
    room: i32,
}

impl State {
    fn run_systems(&mut self, context: &mut Rltk) {
        // requires mutable context
        self.draw_entities(context);
        self.draw_hud(context);

        // immutable context
        let mut player_input_system = input::PlayerInputSystem::new(context);
        player_input_system.run_now(&self.world);

        let mut apply_player_movement_input = game::ApplyPlayerMovementInputSystem{};
        apply_player_movement_input.run_now(&self.world);

        let mut movement_system = game::MovementSystem::new();
        movement_system.run_now(&self.world);

        let mut exit_trigger_system = game::ExitTriggerSystem::new();
        exit_trigger_system.run_now(&self.world);
        match exit_trigger_system.exit_data {
            Some(exit_data) => {
                let old_room = self.room;
                self.room = exit_data.to_room;
                room::change_room(&mut self.world, self.room, old_room);
                
                // adjust player position if needed
                for (_player, position) in (&self.world.read_storage::<game::Player>(), &mut self.world.write_storage::<game::Position>()).join() {
                    match exit_data.direction {
                        room::ExitDirection::North => { position.y = 17; },
                        room::ExitDirection::South => { position.y = 0;}, 
                        room::ExitDirection::East => { position.x = 0;},
                        room::ExitDirection::West => { position.x = 23; },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
        self.world.maintain();
    }

    fn draw_entities(&mut self, context: &mut Rltk) {
        let mut render_system = render::RenderSystem::new(context);
        render_system.run_now(&self.world);
    }

    fn draw_hud(&mut self, context: &mut Rltk) {
        let mut hud_system = hud::HudSystem::new(&self, context, self.room);
        hud_system.run_now(&self.world);
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
        world: World::new(),
        room: 0
    };

    register_components(&mut game_state.world);

    game::create_player_entity(&mut game_state.world);

    room::load_rooms(&mut game_state.world);
    room::change_room(&mut game_state.world, 0, -1);

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
    world.register::<room::BelongsToRoom>();
    world.register::<room::ExitTrigger>();
}
