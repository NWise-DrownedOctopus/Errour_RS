use macroquad::prelude::*;

use crate::{components::{collider::CircleCollider}, utils::GameContext};
use crate::assets::animations::enemy1_idel_animation;
use crate::assets::animations::enemy1_idel_sprite_sheet;
pub struct Creature {
    pub position_index: usize,
    pub collider_index: usize,
    pub animation_index: usize,
    pub sprite_sheet_index: usize,
}

impl Creature {
    pub fn init(context: &mut GameContext, spawn_pos: Vec2) -> Self {
        let position = spawn_pos;
        let base_collider = CircleCollider {
            center: spawn_pos,
            radius: 5.,
        };

        let position_index = context.positions.len();
        let collider_index = context.colliders.len();
        context.positions.push(position);
        context.colliders.push(base_collider);

        let animation_index = context.animations.len();
        context.animations.push(enemy1_idel_animation());

        let sprite_sheet_index = context.sprite_sheets.len();
        context.sprite_sheets.push(enemy1_idel_sprite_sheet());

        Self { 
            position_index,
            collider_index,
            animation_index,
            sprite_sheet_index,
        }
    }
}