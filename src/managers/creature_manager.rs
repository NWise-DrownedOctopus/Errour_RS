use macroquad::prelude::*;

use crate::components::creature::Creature;
use crate::systems::render::draw_animated_entity;
use crate::{components::{collider::CircleCollider}};
use crate::components::animation::{Animation, SpriteSheet};
use crate::utils::{GameContext};
use crate::assets::animations::enemy1_idel_animation;
use crate::assets::animations::enemy1_idel_sprite_sheet;
use crate::components::base::PlayerBase;

pub struct CreatureManager {
    pub creatures: Vec<Creature>,
}

impl CreatureManager {
    pub fn new() -> Self {
        Self {
            creatures: Vec::new(),
        }
    }

    pub fn spawn(&mut self,
        positions: &mut Vec<Vec2>,
        colliders: &mut Vec<CircleCollider>,
        animations: &mut Vec<Animation>,
        sprite_sheets: &mut Vec<SpriteSheet>,
        spawn_pos: Vec2
    ) {
        // Grab index at end of vector
        let position_index = positions.len();
        let collider_index = colliders.len();
        let animation_index = animations.len();
        let sprite_sheet_index = sprite_sheets.len();

        //push new index onto data vectors
        positions.push(spawn_pos);
        colliders.push(CircleCollider {
            center: spawn_pos,
            radius: 5.0,
        });
        animations.push(enemy1_idel_animation());
        sprite_sheets.push(enemy1_idel_sprite_sheet());

        self.creatures.push(Creature {
            position_index,
            collider_index,
            animation_index,
            sprite_sheet_index,
        });
    }

    pub fn update(&mut self,
    positions: &mut Vec<Vec2>,
    player_base: Option<&PlayerBase>,
    ) {
    for i in 0..self.creatures.len() {
        // Access creature
        let creature = &mut self.creatures[i];

        // Get base position
        if let Some(base) = player_base {
            let creature_pos = positions[creature.position_index];
            let base_pos = positions[base.pos_index];

            let dir = (base_pos - creature_pos).normalize_or_zero();
            positions[creature.position_index] += dir * 1.0; // move by 1 unit or scale as needed
        }
    }
}

    pub fn remove(&mut self, context: &mut GameContext, index: usize) {
        if let Some(creature) = self.creatures.get(index) {
            // Optional: set some components to dummy/default to mark them as "deleted"
            context.positions[creature.position_index] = Vec2::ZERO;
            context.colliders[creature.collider_index].radius = 0.0;
            // Or remove them from context entirely, if you're prepared to remap indices.
        }
        self.creatures.remove(index);
    }
}