use std::{
    fmt::Display,
    ops::{Mul, MulAssign},
};

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Velocity {
    x: f32,
    y: f32,
}

impl Mul<f32> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: f32) -> Self::Output {
        Velocity {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Velocity {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
