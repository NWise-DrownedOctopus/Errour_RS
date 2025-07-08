use macroquad::prelude::*;

pub struct Position(pub Vec2);
pub struct Velocity(pub f32);
pub struct Rotation(pub f32);
pub struct RotationalVelocity(pub f32);
pub struct Size(pub f32);
pub struct Target(pub Vec2);
pub struct Collided(pub bool);

pub struct Health(pub f32);
pub struct Damage(pub f32);
pub struct Dead(pub bool);

 