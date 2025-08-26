use macroquad::prelude::*;

use crate::components::projectile::Projectile;
use crate::components::common::{Velocity, Rotation, RotationalVelocity, Damage, Dead};
use crate::managers::attack_manager::AttackCommand;
use crate::managers::creature_manager::{CreatureManager};
use crate::{components::{collider::CircleCollider}};
use crate::components::animation::{Animation, SpriteSheet};
use crate::assets::animations::projectile_01_animation;
use crate::assets::animations::projectile_01_sprite_sheet;

pub struct ProjectileManager {
    pub projectiles: Vec<Projectile>,
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Velocity>,
    pub rotations: Vec<Rotation>,
    pub rotational_velocities: Vec<RotationalVelocity>,
    pub colliders: Vec<CircleCollider>,
    pub damages: Vec<Damage>,
    pub animations: Vec<Animation>,
    pub sprite_sheets: Vec<SpriteSheet>,
    pub dead_flags: Vec<Dead>,
}

impl ProjectileManager {
    pub fn new() -> Self {
        Self {
            projectiles: Vec::new(),
            positions: Vec::new(),
            velocities: Vec::new(),
            rotations: Vec::new(),
            rotational_velocities: Vec::new(),
            colliders: Vec::new(),
            damages: Vec::new(),
            animations: Vec::new(),
            sprite_sheets: Vec::new(),
            dead_flags: Vec::new(),            
        }
    }

    pub fn spawn(&mut self,
        spawn_pos: Vec2,
        target_index: usize,
        dmg: f32,
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
            radius: 3.0,
        });
        self.animations.push(projectile_01_animation());
        self.sprite_sheets.push(projectile_01_sprite_sheet());
        self.dead_flags.push(Dead(false));

        self.projectiles.push(Projectile {
            position_index,
            collider_index,
            target_index,
            animation_index,
            sprite_sheet_index,
            dead_flag_index,
            dmg,
        });
    }

    pub fn process_attack_commands(
        &mut self,
        attack_commands: &[AttackCommand],
    ) {
        for attack in attack_commands {
            println!("We are spawning a projectile");
            self.spawn(attack.attacker_pos, attack.target_index, 5.0);
        }
    }

    pub fn process_hanging_projectiles(
        &mut self,
        creature_manager: &mut CreatureManager,
    ) {
        let mut to_destory = Vec::new();

        // We look through all of our projectiles 
        for (proj_index, projectile) in self.projectiles.iter().enumerate() {
            // store reference to the target index
            let target_index = projectile.target_index;

            // We make sure the index is not out of bounds according to the creature manager data
            if target_index < creature_manager.creatures.len() {
                // store refernce to the specific targets dead_flag
                let dead_flag_index = creature_manager.creatures[target_index].dead_flag_index;

                if creature_manager.dead_flags[dead_flag_index].0 {
                    to_destory.push(proj_index);
                }
            } else {
                // If target is somehow out of bounds, we might as well destoy the projectile still
                // NOTE::: we may want to somehow flag this as an issue
                to_destory.push(proj_index);
            }
        }

        // Now we can safely destroy the projectiles that we have identified above
        for proj_index in to_destory {
            self.destroy(proj_index);
        }
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