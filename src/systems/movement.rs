use crate::managers::creature_manager::CreatureManager;
use crate::components::base::PlayerBase;

pub fn movement_update(
    
    creature_manager: &mut CreatureManager,
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
    }