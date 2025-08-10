use crate::managers::creature_manager::CreatureManager;
use crate::components::base::PlayerBase;
use crate::managers::projectile_manager::{ProjectileManager};

pub fn movement_update(    
    creature_manager: &mut CreatureManager,
    projectile_manager: &mut ProjectileManager,
    player_base: Option<&PlayerBase>,
    ) {        
        for i in 0..creature_manager.creatures.len() {
            // Access creature
            let creature = &creature_manager.creatures[i];

            if creature_manager.dead_flags[creature.dead_flag_index].0 {
                continue;
            }

            // Get base position
            // I'd Like to convert this to a variable target at some point
            if let Some(base) = player_base {
                let creature_pos = creature_manager.positions[creature.position_index];

                let dir = (base.pos - creature_pos).normalize_or_zero();
                creature_manager.positions[creature.position_index] += dir * 1.0; // move by 1 unit or scale as needed
            }
        }

        for i in 0..projectile_manager.projectiles.len() {
            //Access Projectile
            let projectile = &projectile_manager.projectiles[i];

            //Skip Dead Projectiles
            if projectile_manager.dead_flags[projectile.dead_flag_index].0 {
                continue;             
            }
            
            //Grab Projectile Pos
            let projectile_pos = projectile_manager.positions[projectile.position_index];

            // Make sure target Index is Valid
            if let Some(creature) = creature_manager.creatures.get(projectile.target_index) {
                //Skip Dead Creatures
                if creature_manager.dead_flags[creature.dead_flag_index].0 {
                    continue;
                }

                //Get target position
                let target_pos = creature_manager.positions[creature.position_index];
                
                // Set Direction and Move Towards
                let dir = (target_pos - projectile_pos).normalize_or_zero();
                projectile_manager.positions[projectile.position_index] += dir * 1.0;
            }            
        }
    }