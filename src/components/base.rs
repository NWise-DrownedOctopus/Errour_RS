use macroquad::prelude::*;

use crate::{components::{animation::Animation, animation::SpriteSheet, collider::CircleCollider}};
use crate::assets::animations::player_base_idel_animation;
use crate::assets::animations::player_base_idel_sprite_sheet;

pub struct PlayerBase {
    pub pos: Vec2,
    pub health: u32,
    pub base_collider: CircleCollider,
    pub animation: Animation,
    pub sprite_sheet: SpriteSheet,
}

impl PlayerBase {
    pub fn new() -> Self {
        Self {
            pos: vec2(525.0, 500.0),
            health: 100,
            base_collider: CircleCollider {radius: 25.0},
            animation: player_base_idel_animation(),
            sprite_sheet: player_base_idel_sprite_sheet(),
        }

    }

    pub fn take_damage(&mut self, damage: u32){
        self.health -= damage;

        if self.health <= 0 {
            self.health = 0;
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
*/