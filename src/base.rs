use macroquad::prelude::*;

use crate::{animation::Animator, collision::CircleCollider};

pub struct PlayerBase<'a> {
    pub pos: Vec2,
    pub fire_speed: f32,
    pub rot: f32,
    pub rot_speed: f32,
    pub size: f32,
    pub target: Option<Vec2>,
    pub collider: CircleCollider,
    pub collided: bool,
    pub animator: Animator<'a>,
    pub health: f32,
}

pub fn draw_base(base: &mut PlayerBase) {
    base.animator.update();
    base.animator.draw(base.pos);
}