use macroquad::prelude::*;

use crate::components::creature::Creature;
use crate::components::common::{Velocity, Rotation, RotationalVelocity, Health, Damage, Dead};
use crate::{components::{collider::CircleCollider}};
use crate::components::animation::{Animation, SpriteSheet};
use crate::assets::animations::enemy1_idel_animation;
use crate::assets::animations::enemy1_idel_sprite_sheet;

pub struct CreatureManager {
    pub creatures: Vec<Creature>,
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Velocity>,
    pub rotations: Vec<Rotation>,
    pub rotational_velocities: Vec<RotationalVelocity>,
    pub colliders: Vec<CircleCollider>,
    pub healths: Vec<Health>,
    pub damages: Vec<Damage>,
    pub animations: Vec<Animation>,
    pub sprite_sheets: Vec<SpriteSheet>,
    pub dead_flags: Vec<Dead>,
}

impl CreatureManager {
    pub fn new() -> Self {
        Self {
            creatures: Vec::new(),
            positions: Vec::new(),
            velocities: Vec::new(),
            rotations: Vec::new(),
            rotational_velocities: Vec::new(),
            colliders: Vec::new(),
            healths: Vec::new(),
            damages: Vec::new(),
            animations: Vec::new(),
            sprite_sheets: Vec::new(),
            dead_flags: Vec::new(),            
        }
    }

    pub fn spawn(&mut self,
        spawn_pos: Vec2
    ) {
        // Grab index at end of vector
        let position_index = self.positions.len();
        let collider_index = self.colliders.len();
        let animation_index = self.animations.len();
        let sprite_sheet_index = self.sprite_sheets.len();
        let dead_flag_index = self.dead_flags.len();

        //push new index onto data vectors
        self.positions.push(spawn_pos);
        self.colliders.push(CircleCollider {
            radius: 12.0,
        });
        self.animations.push(enemy1_idel_animation());
        self.sprite_sheets.push(enemy1_idel_sprite_sheet());
        self.dead_flags.push(Dead(false));

        self.creatures.push(Creature {
            position_index,
            collider_index,
            animation_index,
            sprite_sheet_index,
            dead_flag_index,
        });
    }

    pub fn destroy(&mut self, index: usize) {
        // Destory Creature
        if index < self.dead_flags.len() {
            self.dead_flags[index].0 = true;
        } else {
            eprintln!("CreatureManager::destroy: Tried to destroy invalid index {}", index);
        }
    }
}
