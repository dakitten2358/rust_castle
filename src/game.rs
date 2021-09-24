use specs::prelude::*;
use specs_derive::Component;
use std::cmp::*;
use std::str::FromStr;

use itertools::Itertools;

use crate::input::{PlayerInputMappingComponent, PlayerInputComponent, PlayerTextInputComponent};
use crate::render::{Renderable};
use crate::StateAction;
use crate::inventory::{InventoryComponent};
use crate::combat::{CombatStats, CombatLog};

#[allow(unused_imports)]
use crate::hud::{DebugHudComponent};

#[derive(Component, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance_sq(a: &Position, b: &Position) -> i32 { (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y) }
    pub fn delta(a: &Position, b: &Position) -> (i32, i32) { (b.x - a.x, b.y - a.y) }
    pub fn equals_xy(&self, x: i32, y: i32) -> bool { self.x == x && self.y == y }
}

#[derive(Component)]
pub struct DebugName {
    pub text: String,
}

#[derive(Component)]
pub struct Description {
    pub input_name: String,
    pub name: String,
    pub description: String,
}

impl Description {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            input_name: name.to_ascii_lowercase(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }

    pub fn new_explicit(input_name: &str, name: &str, description: &str) -> Self {
        Self {
            input_name: input_name.to_ascii_lowercase(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Component)]
pub struct Player {}

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

#[derive(Component)]
pub struct Movement {
    cumulative_x_movement: i32,
    cumulative_y_movement: i32,

    moved_this_frame: bool,
    attempted_x_movement: i32,
    attempted_y_movement: i32,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            cumulative_x_movement: 0,
            cumulative_y_movement: 0,
            moved_this_frame: false,

            attempted_x_movement: 0,
            attempted_y_movement: 0,
        }
    }

    pub fn add_movement_input(&mut self, x: i32, y: i32) {
        self.cumulative_x_movement += x;
        self.cumulative_y_movement += y;
    }

    pub fn clear_movement(&mut self) {
        let (attempt_x, attempt_y) = self.get_movement_input();
        self.attempted_x_movement = attempt_x;
        self.attempted_y_movement = attempt_y;

        self.cumulative_x_movement = 0;
        self.cumulative_y_movement = 0;
        self.moved_this_frame = false;
    }

    fn get_movement_input(&self) -> (i32, i32) {
        (min(1, max(-1, self.cumulative_x_movement)), min(1, max(-1, self.cumulative_y_movement)))       
    }

    pub fn moved(&mut self) {
        self.moved_this_frame = true;
    }

    pub fn did_move(&self) -> bool {
        self.moved_this_frame
    }

    fn attempted_to_move(&self) -> bool {
        self.attempted_x_movement != 0 || self.attempted_y_movement != 0
    }

    pub fn was_move_blocked(&self) -> bool {
        !self.did_move() && self.attempted_to_move()
    }

    pub fn get_attempted_move(&self) -> (i32, i32) {
        (self.attempted_x_movement, self.attempted_y_movement)
    }
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

#[derive(Component)]
pub struct ColliderComponent {}

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

#[derive(Component)]
pub struct ActiveDescriptionComponent {
    pub description: String,
}

impl ActiveDescriptionComponent {
    pub fn new() -> Self {
        Self {
            description: String::new(),
        }
    }

    pub fn set(&mut self, new_description: &str) {
        self.description = String::from_str(new_description).unwrap();
    }
}

pub struct PlayerTextCommandSystem {}

impl PlayerTextCommandSystem {
    fn process_text_input<'a>(&self, text_command: String, descriptions: &ReadStorage<'a, Description>) -> Option<String> {
        let mut tokens = text_command.split_whitespace();
        let first_token = tokens.next();
        let args_string = tokens.format(" ").to_string();
        let args = args_string.as_str();
        match first_token
        {
            Some(token) => {
                match token {
                    "look" => {
                        self.process_look(args, descriptions)
                    },
                    "use" => self.process_use(args),
                    _ => None
                }
            },
            None => None
        }
    }

    fn process_look<'a>(&self, look_at_target_name: &str, descriptions: &ReadStorage<'a, Description>) -> Option<String> {
        match look_at_target_name {
            target_name if target_name.len() > 0 => {
                self.process_look_target(target_name, descriptions)
            },
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

    fn process_use(&self, _use_target_name: &str) -> Option<String> {
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