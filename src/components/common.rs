use macroquad::prelude::*;
use std::ops::Not;

pub struct Velocity(pub f32);
pub struct Rotation(pub f32);
pub struct RotationalVelocity(pub f32);

pub struct Health(pub f32);
pub struct Damage(pub f32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dead(pub bool);

impl Not for Dead {
    type Output = Self;

    fn not(self) -> Self::Output {
        Dead(!self.0)
    }
}

impl Dead {
    pub fn is_alive(&self) -> bool {
        !self.0
    }
}

 