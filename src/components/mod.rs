use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::{Component, ConvertSaveload};
use std::cmp::*;
use std::str::FromStr;

mod common;
mod game;

pub use crate::components::common::*;
pub use crate::components::game::*;

#[derive(Component, ConvertSaveload, Clone)]
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
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, Serialize, Deserialize)]
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

    pub fn get_movement_input(&self) -> (i32, i32) {
        (
            min(1, max(-1, self.cumulative_x_movement)),
            min(1, max(-1, self.cumulative_y_movement)),
        )
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

#[derive(Component, Serialize, Deserialize)]
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

#[derive(Component, Serialize, Deserialize)]
pub struct DebugHudComponent {}
