use serde::{Serialize, Deserialize};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::*;

#[derive(Component, Clone, PartialEq, Serialize, Deserialize)]
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
pub struct ColliderComponent {}

#[derive(Component, Serialize, Deserialize)]
pub struct DebugName {
    pub text: String,
}

