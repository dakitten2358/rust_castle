use specs::prelude::*;

use crate::components::*;
use crate::render::Renderable;
use crate::textinput::*;
use crate::StateAction;

pub struct DynamicMarker;

pub struct CurrentRoom(pub i32);

pub fn create_player_entity(world: &mut World) {
    world
        .create_entity()
        .with(PlayerInputMappingComponent {})
        .with(PlayerInputComponent::new())
        .with(PlayerTextInputComponent::new())
        .with(Position { x: 12, y: 9 })
        .with(Renderable::new_with_z('\u{2663}', rltk::WHITE, 1))
        .with(Player {})
        .with(Movement::new())
        .with(ColliderComponent {})
        .with(ActiveDescriptionComponent::new())
        .with(InventoryComponent::new())
        .with(DebugHudComponent {})
        .with(CombatStats {
            max_health: 10,
            health: 10,
        })
        .with(AppliesDamage { damage: 10 })
        .with(DebugName {
            text: "player".to_string(),
        })
        .with(CombatLog::new())
        .with(DebugHudComponent {})
        .build();
}

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Movement>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, ColliderComponent>,
    );

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
            if (blockers.contains(&(tentative_x, tentative_y)) == false
                || (blockers.contains(&(tentative_x, tentative_y)) && free_blockers.contains(&(tentative_x, tentative_y))))
                && new_blockers.contains(&(tentative_x, tentative_y)) == false
            {
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
            if player_input.move_left {
                movement.add_movement_input(-1, 0);
                self.player_moved = true;
            }
            if player_input.move_right {
                movement.add_movement_input(1, 0);
                self.player_moved = true;
            }
            if player_input.move_up {
                movement.add_movement_input(0, -1);
                self.player_moved = true;
            }
            if player_input.move_down {
                movement.add_movement_input(0, 1);
                self.player_moved = true;
            }
        }
    }
}

pub struct ExitTriggerSystem {}

impl<'a> System<'a> for ExitTriggerSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Movement>,
        ReadStorage<'a, crate::room::ExitTrigger>,
        WriteExpect<'a, Vec<StateAction>>,
    );

    fn run(&mut self, (players, positions, movements, exit_triggers, mut state_actions): Self::SystemData) {
        for (_player, movement, position) in (&players, &movements, &positions).join() {
            if movement.did_move() {
                for (exit_trigger, exit_position) in (&exit_triggers, &positions).join() {
                    if position == exit_position {
                        state_actions.push( StateAction::ChangeRoom {
                            direction: exit_trigger.from_direction,
                            to_room: exit_trigger.to_room,
                        });
                        return;
                    }
                }
            }
        }
    }
}

impl ExitTriggerSystem {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct PlayerTextCommandSystem {}

impl PlayerTextCommandSystem {
    pub fn new() -> Self {
        Self {}
    }

    fn process_text_input<'a>(&mut self, text_command: &String, descriptions: &ReadStorage<'a, Description>, state_actions: &mut Vec<StateAction>) -> Option<String> {
        match parse_input(text_command) {
            TextCommand::Some { command, arg } => match command.as_str() {
                "look" => self.process_look(arg, descriptions),
                "use" => self.process_use(arg),
                "quit" => self.process_quit(state_actions),
                _ => None,
            },
            TextCommand::None => None,
        }
    }

    fn process_look<'a>(&self, look_at_target_name: Option<String>, descriptions: &ReadStorage<'a, Description>) -> Option<String> {
        match look_at_target_name {
            Some(target_name) => self.process_look_target(target_name.as_str(), descriptions),
            _ => self.process_look_room(),
        }
    }

    fn process_look_target<'a>(&self, _target_name: &str, descriptions: &ReadStorage<'a, Description>) -> Option<String> {
        for description in (descriptions).join() {
            if description.input_name == _target_name {
                return Some(description.description.clone());
            }
            if description.name.to_ascii_lowercase() == _target_name {
                return Some(description.description.clone());
            }
        }
        return Some("look at target".to_string());
    }

    fn process_look_room(&self) -> Option<String> {
        return Some("look at room with long description here".to_string());
    }

    fn process_use(&self, _use_target_name: Option<String>) -> Option<String> {
        return Some("use item".to_string());
    }

    fn process_quit(&mut self, state_actions: &mut Vec<StateAction>) -> Option<String> {
        state_actions.push(StateAction::Quit);
        None
    }

    fn process_debug_input(&mut self, text_command: &String, state_actions: &mut Vec<StateAction>) {
        let mut tokens = text_command.split_whitespace();
        match tokens.next() {
            Some(token) => match token {
                "go" => state_actions.push(self.process_debug_go(tokens.next())),
                "dsave" => state_actions.push(StateAction::DebugSave),
                "dload" => state_actions.push(StateAction::DebugLoad),
                "redirect" => state_actions.push(self.process_debug_redirect(tokens.next(), tokens.next())),
                _ => {}
            },
            None => {}
        }
    }

    fn process_debug_go(&mut self, to_room_text: Option<&str>) -> StateAction {
        match to_room_text {
            Some(to_room_str) => {
                if let Ok(to_room) = to_room_str.parse::<i32>() {
                    return StateAction::ChangeRoom {
                        direction: crate::room::ExitDirection::Invalid,
                        to_room: to_room,
                    };
                }
            }
            _ => {}
        }
        return StateAction::None;
    }

    fn process_debug_redirect(&mut self, original_room_text: Option<&str>, new_room_text: Option<&str>) -> StateAction {
        if let Some(original_room) = original_room_text {
            if let Some(new_room) = new_room_text {
                return StateAction::RedirectRoom {
                    original_room: original_room.parse::<i32>().unwrap(),
                    new_room: new_room.parse::<i32>().unwrap(),
                };
            }
        }

        return StateAction::None;
    }
}

impl<'a> System<'a> for PlayerTextCommandSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, PlayerTextInputComponent>,
        WriteStorage<'a, ActiveDescriptionComponent>,
        ReadStorage<'a, Description>,
        ReadStorage<'a, DebugHudComponent>,
        WriteExpect<'a, Vec<StateAction>>,
    );

    fn run(&mut self, (entities, players, mut text_inputs, mut active_descriptions, descriptions, debugs, mut _state_actions): Self::SystemData) {
        for (entity, _player, text_input, description) in (&entities, &players, &mut text_inputs, &mut active_descriptions).join() {
            match text_input.consume() {
                Some(text_command) => {
                    match self.process_text_input(&text_command, &descriptions, &mut _state_actions) {
                        Some(result) => description.set(result.as_str()),
                        None => description.set("i don't understand"),
                    }

                    if let Some(_debug) = debugs.get(entity) {
                        self.process_debug_input(&text_command, &mut _state_actions);
                    }
                }
                None => {}
            }
        }
    }
}
