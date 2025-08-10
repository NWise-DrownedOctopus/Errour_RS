use crate::events::GameEvent;
use crate::managers::creature_manager::CreatureManager;
use crate::components::base::PlayerBase;
use crate::managers::projectile_manager::{self, ProjectileManager};

pub fn update_damage_system(
    events: &mut Vec<GameEvent>,
    creature_manager: &mut CreatureManager,
    projectile_manager: &mut ProjectileManager,
    player_base: &mut Option<PlayerBase>,
) {
    // Here we handle damage events
    for event in events.drain(..) {
        match event {
            GameEvent::CreatureHitBase { creature_index, damage } => {
                //Kill Creature
                creature_manager.destroy(creature_index);

                // Do Damage To Base
                if let Some(base) = player_base.as_mut() {
                    base.take_damage(damage);
                } else {
                    eprintln!("Tried to damage base, but it doesn't exist!");
                }
            }

            GameEvent::ProjectileHitCreature { creature_index, projectile_index } => {
                // Damage Creature
                if creature_index < creature_manager.creatures.len() {
                    creature_manager.damage_creature(creature_index, projectile_manager.projectiles[projectile_index].dmg);
                }                

                // Destroy Projectile
                projectile_manager.destroy(projectile_index);
            }

            GameEvent::CreatureDied { creature_index } => {
                // A Creature Has Died, Horray
            }
        }
    }
}