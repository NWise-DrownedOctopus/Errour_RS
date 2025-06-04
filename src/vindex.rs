use macroquad::prelude::*;

pub struct Creature {
    pub pos: Vec2,
    pub speed: f32,
    pub rot: f32,
    pub rot_speed: f32,
    pub size: f32,
    pub target: Vec2,
    pub collided: bool,
}

pub fn move_towards(creature: &mut Creature, dt: &f32) -> Vec2 {
    // Calculate interpolation factor based on speed and delta time
    // You can tweak this to get the speed you want    
    let t = creature.speed * dt;
    creature.pos.lerp(creature.target, t.min(1.0)) // clamp t so it doesn't exceed 1.0
}

pub fn draw_creature(creature: &mut Creature) {
    draw_circle(creature.pos.x, creature.pos.y, 10.0, PINK);
}