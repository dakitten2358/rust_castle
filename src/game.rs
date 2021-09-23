use specs::prelude::*;

use crate::render::{Renderable};
use crate::StateAction;
use crate::components::*;

pub fn create_player_entity(world: &mut World) {
    world.create_entity()
        .with(PlayerInputMappingComponent{})
        .with(PlayerInputComponent::new())
        .with(PlayerTextInputComponent::new())
        .with(Position{ x: 12, y: 9})
        .with(Renderable::new_with_z('\u{2663}', rltk::WHITE, 1))
        .with(Player{})
        .with(Movement::new())
        .with(ColliderComponent{})
        .with(ActiveDescriptionComponent::new())
        .with(InventoryComponent::new())
        .with(DebugHudComponent{})
        .with(CombatStats { max_health: 10, health: 10 })
        .with(DebugName { text: "player".to_string() })
        .with(CombatLog::new())
        //.with(DebugHudComponent{})
        .build();
}

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Movement>, WriteStorage<'a, Position>, ReadStorage<'a, ColliderComponent>);

    fn run(&mut self, (mut movements, mut positions, colliders): Self::SystemData) {
        let mut blockers = Vec::new();
        for (position, _collider) in (&positions, &colliders).join() {
            blockers.push((position.x, position.y));
        }

        let mut new_blockers = Vec::new();
        let mut free_blockers = Vec::new();
        for (movement, position, _collider) in (&mut movements, &mut positions, &colliders).join() {
            let (delta_x, delta_y) = movement.get_movement_input();
            movement.clear_movement();

            let (tentative_x, tentative_y) = (position.x + delta_x, position.y + delta_y);
            if (blockers.contains(&(tentative_x, tentative_y)) == false || (blockers.contains(&(tentative_x, tentative_y)) && free_blockers.contains(&(tentative_x, tentative_y)))) && new_blockers.contains(&(tentative_x, tentative_y)) == false {
                new_blockers.push((tentative_x, tentative_y));
                free_blockers.push((position.x, position.y));

                position.x = tentative_x;
                position.y = tentative_y;

                movement.moved();
            }            
            
        }
    }
}

impl MovementSystem {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct ApplyPlayerMovementInputSystem {
    pub player_moved: bool,
}

impl ApplyPlayerMovementInputSystem {
    pub fn new() -> Self {
        Self { player_moved: false }
    }
}

impl<'a> System<'a> for ApplyPlayerMovementInputSystem {
    type SystemData = (ReadStorage<'a, PlayerInputComponent>, WriteStorage<'a, Movement>);

    fn run(&mut self, (player_inputs, mut movements): Self::SystemData) {
        self.player_moved = false;
        for (player_input, movement) in (&player_inputs, &mut movements).join() {
            if player_input.move_left { movement.add_movement_input(-1, 0); self.player_moved = true; }
            if player_input.move_right { movement.add_movement_input(1, 0);  self.player_moved = true; }
            if player_input.move_up { movement.add_movement_input(0, -1);  self.player_moved = true; }
            if player_input.move_down { movement.add_movement_input(0, 1);  self.player_moved = true; }
        }
    }
}

pub struct ExitTriggerSystem{
    pub state_action: StateAction,
}

impl<'a> System<'a> for ExitTriggerSystem {
    type SystemData = (ReadStorage<'a, Player>, ReadStorage<'a, Position>, ReadStorage<'a, Movement>, ReadStorage<'a, crate::room::ExitTrigger>);

    fn run(&mut self, (players, positions, movements, exit_triggers): Self::SystemData) {
        for (_player, movement, position) in (&players, &movements, &positions).join() {
            if movement.did_move() {
                for (exit_trigger, exit_position) in (&exit_triggers, &positions).join() {
                    if position == exit_position {
                        self.state_action = StateAction::ChangeRoom { direction: exit_trigger.from_direction, to_room: exit_trigger.to_room };
                        return;
                    }
                }
            }
        }
    }
}

impl ExitTriggerSystem {
    pub fn new() -> Self {
        Self {
            state_action: StateAction::None,
        }
    }
}

pub struct PlayerTextCommandSystem {}

impl PlayerTextCommandSystem {
    fn process_text_input<'a>(&self, text_command: String, descriptions: &ReadStorage<'a, Description>) -> Option<String> {
        let mut tokens = text_command.split_whitespace();
        match tokens.next()
        {
            Some(token) => {
                match token {
                    "look" => self.process_look(tokens.next(), descriptions),
                    "use" => self.process_use(tokens.next()),
                    _ => None
                }
            },
            None => None
        }
    }

    fn process_look<'a>(&self, look_at_target_name: Option<&str>, descriptions: &ReadStorage<'a, Description>) -> Option<String> {
        match look_at_target_name {
            Some(target_name) => self.process_look_target(target_name, descriptions),
            None => self.process_look_room(),
        }
    }

    fn process_look_target<'a>(&self, _target_name: &str, descriptions: &ReadStorage<'a, Description>) -> Option<String> {
        for description in (descriptions).join() {
            if description.input_name == _target_name {
                return Some(description.description.clone());
            }
        }
        return Some("look at target".to_string());
    }

    fn process_look_room(&self) -> Option<String> {
        return Some("look at room with long description here".to_string());
    }

    fn process_use(&self, _use_target_name: Option<&str>) -> Option<String> {
        return Some("use item".to_string());
    }
}

impl<'a> System<'a> for PlayerTextCommandSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, PlayerTextInputComponent>,
        WriteStorage<'a, ActiveDescriptionComponent>,
        ReadStorage<'a, Description>,
    );

    fn run(&mut self, (players, mut text_inputs, mut active_descriptions, descriptions) : Self::SystemData) {
        for(_player, text_input, description) in (&players, &mut text_inputs, &mut active_descriptions).join() {
            match text_input.consume() {
                Some(text_command) => {
                    match self.process_text_input(text_command, &descriptions)
                    {
                        Some(result) => description.set(result.as_str()),
                        None => description.set("i don't understand"),
                    }
                },
                None => {}
            }
        }       
    }
}