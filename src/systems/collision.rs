use macroquad::prelude::*;

use crate::components::collider::{CircleCollider};
use crate::events::GameEvent;
use crate::managers::creature_manager::CreatureManager;
use crate::managers::projectile_manager::ProjectileManager;
use crate::components::base::PlayerBase;


pub fn update_collision(
    creature_manager: &mut CreatureManager,
    projectile_manager: &mut ProjectileManager,
    player_base: Option<&PlayerBase>,
    events: &mut Vec<GameEvent>,
) {
    // Check if creature is colliding with base collider
    for (creature_index, creature) in creature_manager.creatures.iter().enumerate() {
        // Skip dead creatures
        if creature_manager.dead_flags[creature.dead_flag_index].0 {
            continue;
        }

        let creature_pos = creature_manager.positions[creature.position_index];
        let creature_collider = &creature_manager.colliders[creature.collider_index];

        if let Some(base) = player_base.as_ref() {
            let base_pos = base.pos;
            let base_collider = &base.base_collider;

            if intersects(creature_pos, creature_collider, base_pos, base_collider) {
                println!("Creature Hit Base");
                events.push(GameEvent::CreatureHitBase {
                    creature_index,
                    damage: 10,
                });
            }
        }
    }

    // Check if projectile is colliding with creature
    for (projectile_index, projectile) in projectile_manager.projectiles.iter().enumerate() {
        if projectile_manager.dead_flags[projectile.dead_flag_index].0 {
            continue;
        }

        let creature = &creature_manager.creatures[projectile.target_index];
        let creature_pos = creature_manager.positions[creature.position_index];
        let creature_collider = &creature_manager.colliders[creature.collider_index];
        
        let projectile_pos = projectile_manager.positions[projectile.position_index];
        let projectile_collider = &projectile_manager.colliders[projectile.collider_index];

        if intersects(projectile_pos, projectile_collider, creature_pos, creature_collider) {
            println!("Projectile Hit Creature");
            events.push(GameEvent::ProjectileHitCreature {
                creature_index: projectile.target_index,
                projectile_index,
            });
        }
    }
}

pub fn intersects (
    other_pos: Vec2,
    other_collider: &CircleCollider,
    self_pos: Vec2,
    self_collider: &CircleCollider,
) -> bool {
    let dist = self_pos.distance(other_pos);
    dist <= (self_collider.radius + other_collider.radius)
}