use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::{Component, ConvertSaveload};
use std::cmp::*;

#[derive(Component, Clone, PartialEq, ConvertSaveload)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance_sq(a: &Position, b: &Position) -> i32 {
        (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y)
    }
    pub fn delta(a: &Position, b: &Position) -> (i32, i32) {
        (b.x - a.x, b.y - a.y)
    }
    pub fn equals_xy(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }
}

#[derive(Component)]
pub struct ColliderComponent {}

#[derive(Component, ConvertSaveload)]
pub struct DebugName {
    pub text: String,
}
