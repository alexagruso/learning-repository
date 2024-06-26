use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Add<&Vec2> for Position {
    type Output = Position;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<&Vec2> for Position {
    fn add_assign(&mut self, rhs: &Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
