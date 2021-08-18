use specs::prelude::*;
use specs_derive::Component;
use std::cmp::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Movement {
    cumulative_x_movement: i32,
    cumulative_y_movement: i32,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            cumulative_x_movement: 0,
            cumulative_y_movement: 0,
        }
    }

    pub fn add_movement_input(&mut self, x: i32, y: i32) {
        self.cumulative_x_movement += x;
        self.cumulative_y_movement += y;
    }

    pub fn clear_movement(&mut self) {
        self.cumulative_x_movement = 0;
        self.cumulative_y_movement = 0;
    }

    pub fn get_movement_input(&self) -> (i32, i32) {
        (min(1, max(-1, self.cumulative_x_movement)), min(1, max(-1, self.cumulative_y_movement)))       
    }

    pub fn wants_to_move(&self) -> bool {
        self.cumulative_x_movement != 0 || self.cumulative_y_movement != 0
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
            if !movement.wants_to_move() { continue; }

            let (delta_x, delta_y) = movement.get_movement_input();
            let (tentative_x, tentative_y) = (position.x + delta_x, position.y + delta_y);

            if (blockers.contains(&(tentative_x, tentative_y)) == false || (blockers.contains(&(tentative_x, tentative_y)) && free_blockers.contains(&(tentative_x, tentative_y)))) && new_blockers.contains(&(tentative_x, tentative_y)) == false {
                new_blockers.push((tentative_x, tentative_y));
                free_blockers.push((position.x, position.y));

                position.x = tentative_x;
                position.y = tentative_y;
            }            
            movement.clear_movement();
        }
    }
}

impl MovementSystem {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct ApplyPlayerMovementInputSystem {}

impl<'a> System<'a> for ApplyPlayerMovementInputSystem {
    type SystemData = (ReadStorage<'a, crate::input::PlayerInputComponent>, WriteStorage<'a, Movement>);

    fn run(&mut self, (player_inputs, mut movements): Self::SystemData) {
        for (player_input, movement) in (&player_inputs, &mut movements).join() {
            if player_input.move_left { movement.add_movement_input(-1, 0); }
            if player_input.move_right { movement.add_movement_input(1, 0); }
            if player_input.move_up { movement.add_movement_input(0, -1); }
            if player_input.move_down { movement.add_movement_input(0, 1); }
        }
    }
}

#[derive(Component)]
pub struct ColliderComponent {}