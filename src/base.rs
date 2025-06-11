use macroquad::prelude::*;

use crate::animation::Animator;

pub struct PlayerBase {
    pub pos: Vec2,
    pub fire_speed: f32,
    pub rot: f32,
    pub rot_speed: f32,
    pub size: f32,
    pub target: Vec2,
    pub collided: bool,
    pub animator: Animator<'a>,
}