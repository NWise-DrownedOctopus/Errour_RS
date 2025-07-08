use macroquad::prelude::*;

use crate::components::collider::CircleCollider;
use crate::components::animation::Animator;
// use crate::utils::{GameContext};
// use crate::vindex::{Creature};

pub struct PlayerBase<'a> {
    pub pos: Vec2,
    pub fire_speed: f32,
    pub rot: f32,
    pub rot_speed: f32,
    pub size: f32,
    // pub target: Option<&'a Creature<'a>>,
    pub base_collider: CircleCollider,
    pub collided: bool,
    pub animator: Animator<'a>,
    pub health: f32,
    pub fire_range_collider: CircleCollider,
}

pub fn draw_base(base: &mut PlayerBase) {
    base.animator.update();
    base.animator.draw(base.pos);
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