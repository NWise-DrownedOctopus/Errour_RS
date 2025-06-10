use macroquad::prelude::*;

use crate::animation::Animator;

pub struct Creature<'a> {
    pub pos: Vec2,
    pub speed: f32,
    pub rot: f32,
    pub rot_speed: f32,
    pub size: f32,
    pub target: Vec2,
    pub collided: bool,
    pub animator: Animator<'a>,
}

pub fn draw_creature(creature: &mut Creature) {
    // draw_circle(creature.pos.x, creature.pos.y, 10.0, PINK);
    creature.animator.update();
    creature.animator.draw(creature.pos);
}