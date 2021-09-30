use rltk::{GameState, Rltk};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

mod ai;
mod combat;
mod components;
mod game;
mod hud;
mod input;
mod inventory;
mod items;
mod render;
mod room;

use crate::components::*;

pub struct State {
    world: World,
    room: i32,
}

pub enum StateAction {
    None,
    DeleteEntities {
        entities: Vec<Entity>,
    },
    ChangeRoom {
        direction: room::ExitDirection,
        to_room: i32,
    },
    Quit,
    DebugSave,
    DebugLoad,
}

impl State {
    fn run_systems(&mut self, context: &mut Rltk) {
        // requires mutable context
        self.draw_entities(context);
        self.draw_hud(context);
        self.draw_debug(context);

        // immutable context
        let mut player_input_system = input::PlayerInputSystem::new(context);
        player_input_system.run_now(&self.world);

        let mut apply_player_movement_input = game::ApplyPlayerMovementInputSystem::new();
        apply_player_movement_input.run_now(&self.world);
        if apply_player_movement_input.player_moved {
            let mut ai = ai::AiMoveToPlayerSystem {};
            ai.run_now(&self.world);
        }

        let mut player_commands = game::PlayerTextCommandSystem::new();
        player_commands.run_now(&self.world);
        self.handle_state_action(player_commands.state_action);

        let mut movement_system = game::MovementSystem::new();
        movement_system.run_now(&self.world);

        let mut pickups = inventory::PickupTriggerSystem::new();
        pickups.run_now(&self.world);
        self.handle_state_action(pickups.state_action);

        if apply_player_movement_input.player_moved {
            let mut melee_system = combat::MeleeCombatSystem {};
            melee_system.run_now(&self.world);

            let mut damage_system = combat::DamageSystem {};
            damage_system.run_now(&self.world);

            let mut dead_system = combat::ClearDeadSystem::new();
            dead_system.run_now(&self.world);
            self.handle_state_action(dead_system.state_action);

            let mut exit_trigger_system = game::ExitTriggerSystem::new();
            exit_trigger_system.run_now(&self.world);
            self.handle_state_action(exit_trigger_system.state_action);
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

    fn draw_debug(&mut self, context: &mut Rltk) {
        let mut debug_hud = hud::DebugHudSystem::new(&self, context, self.room);
        debug_hud.run_now(&self.world);
    }

    fn change_room(&mut self, to_room: i32, direction: room::ExitDirection) {
        let old_room = self.room;
        self.room = to_room;
        room::change_room(&mut self.world, self.room, old_room);

        // adjust player position if needed
        for (_player, position) in (
            &self.world.read_storage::<Player>(),
            &mut self.world.write_storage::<Position>(),
        )
            .join()
        {
            match direction {
                room::ExitDirection::North => {
                    position.y = 17;
                }
                room::ExitDirection::South => {
                    position.y = 0;
                }
                room::ExitDirection::East => {
                    position.x = 0;
                }
                room::ExitDirection::West => {
                    position.x = 23;
                }
                _ => {}
            }
        }
    }

    fn handle_state_action(&mut self, action: StateAction) {
        match action {
            StateAction::DeleteEntities { entities } => {
                for entity in entities {
                    self.world
                        .delete_entity(entity)
                        .expect("Failed to delete an entity!");
                }
            }
            StateAction::ChangeRoom { direction, to_room } => {
                self.change_room(to_room, direction);
            }
            StateAction::Quit => {
                std::process::exit(0);
            }
            StateAction::DebugSave => {
                self.debug_save();
            }
            StateAction::DebugLoad => {
                self.debug_load();
            }
            StateAction::None => {}
        }
    }

    fn debug_save(&mut self) {}

    fn debug_load(&mut self) {}
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();
        self.run_systems(context);
    }
}

fn main() -> rltk::BError {
    let context = terminal_builder(2).build()?;

    let mut game_state = State {
        world: World::new(),
        room: 0,
    };

    // register types
    register_markers(&mut game_state.world);
    register_components(&mut game_state.world);

    // load raw data
    items::load_items(&mut game_state.world);
    room::load_rooms(&mut game_state.world);
    room::load_dynamic_rooms(&mut game_state.world);

    // start game
    game::create_player_entity(&mut game_state.world);
    room::change_room(&mut game_state.world, 0, -1);
    ai::create_test_ai(&mut game_state.world);
    rltk::main_loop(context, game_state)
}

fn register_markers(world: &mut World) {
    world.register::<SimpleMarker<game::DynamicMarker>>();
    world.insert(SimpleMarkerAllocator::<game::DynamicMarker>::default());
}

fn register_components(world: &mut World) {
    world.register::<PlayerInputComponent>();
    world.register::<PlayerTextInputComponent>();
    world.register::<PlayerInputMappingComponent>();
    world.register::<render::Renderable>();
    world.register::<Position>();
    world.register::<Player>();
    world.register::<Movement>();
    world.register::<ColliderComponent>();
    world.register::<room::BelongsToRoom>();
    world.register::<room::ExitTrigger>();
    world.register::<DebugHudComponent>();
    world.register::<ActiveDescriptionComponent>();
    world.register::<ai::AiMoveToPlayer>();
    world.register::<InventoryComponent>();
    world.register::<PickupTrigger>();
    world.register::<CombatStats>();
    world.register::<ApplyDamageComponent>();
    world.register::<AppliesDamage>();
    world.register::<WantsToAttack>();
    world.register::<DeadTag>();
    world.register::<DebugName>();
    world.register::<CombatLog>();
    world.register::<Description>();
}

fn terminal_builder(scale: i32) -> rltk::RltkBuilder {
    use rltk::RltkBuilder;
    let terminal_builder = RltkBuilder::new();
    let context = terminal_builder
        .with_dimensions(40, 25)
        .with_tile_dimensions(10 * scale, 10 * scale)
        .with_font("castle10x10.png", 10, 10)
        .with_simple_console(40, 25, "castle10x10.png")
        .with_title("Castle Adventure!");
    context
}
