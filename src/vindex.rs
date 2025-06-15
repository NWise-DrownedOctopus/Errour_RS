use macroquad::prelude::*;

use crate::animation::Animator;
use crate::collision::CircleCollider;

pub struct Creature<'a> {
    pub pos: Vec2,
    pub speed: f32,
    pub rot: f32,
    pub rot_speed: f32,
    pub size: f32,
    pub target: Vec2,
    pub collided: bool,
    pub animator: Animator<'a>,
    pub collider: CircleCollider,
    pub damage: f32,
    pub health: f32,
    pub dead: bool,
}

pub fn draw_creature(creature: &mut Creature) {
    creature.animator.update();
    creature.collider.center = Vec2::new(creature.pos.x, creature.pos.y);
    creature.animator.draw(creature.pos);
}