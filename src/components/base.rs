use macroquad::prelude::*;

use crate::{components::{collider::CircleCollider}, utils::GameContext};
use crate::assets::animations::player_base_idel_animation;
use crate::assets::animations::player_base_idel_sprite_sheet;

pub struct PlayerBase {
    pub pos_index: usize,
    pub col_index: usize,
    pub animation_index: usize,
    pub sprite_sheet_index: usize,
}

impl PlayerBase {
    pub fn init(context: &mut GameContext) -> Self {
        let position = vec2(525.0, 500.0);
        let base_collider = CircleCollider {
            center: vec2(525.0, 500.0),
            radius: 25.0,
        };

        let pos_index = context.positions.len();
        let col_index = context.colliders.len();
        context.positions.push(position);
        context.colliders.push(base_collider);

        let animation_index = context.animations.len();
        context.animations.push(player_base_idel_animation());

        let sprite_sheet_index = context.sprite_sheets.len();
        context.sprite_sheets.push(player_base_idel_sprite_sheet());

        Self { 
            pos_index,
            col_index,
            animation_index,
            sprite_sheet_index,
        }
    }
}

// Needs Refactor
/*
pub fn update_player_base_target(context: &mut GameContext) {
    // First if we have no target let us try to find one
    if context.player_base.target.is_none() {
        println!("No target selected");
        
        for enemy in &context.creatures {
            let valid_target_found = context.player_base.fire_range_collider.intersects(&enemy.collider as &dyn Collider);

            if valid_target_found {
                context.player_base.target = Some(enemy);
                println!("Target found. New Target at {}", enemy.pos);
                return;
            }
            println!("Target not valid. Not intersecting")
        }  
    }

    if let Some(creature) = context.player_base.target {
        println!("Current target is at: {:?}", creature.pos);
    }    
}

    ///// DELETE THIS

    let player_base = PlayerBase {
        pos: vec2(525.0, 500.0),
        fire_speed: 3.0,
        rot: 0.0,
        rot_speed: 3.0,
        size: 30.0,
        // target: None,
        base_collider: CircleCollider {
            center: vec2(525.0, 500.0),
            radius: 25.0,
        },
        collided: false,
        animator: Animator {
            texture: &art_assets.player_base_texture,
            frame_width: 48.0,
            frame_height: 48.0,
            columns: 4,
            animation: player_base_idel_animation(),
            shadow_offset: 3.0,
        },
        health: 100.0,
        fire_range_collider: CircleCollider {
            center: vec2(525.0, 500.0),
            radius: 00.0,
        },
    };
*/