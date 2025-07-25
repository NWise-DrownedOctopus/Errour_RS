use crate::events::GameEvent;
use crate::managers::creature_manager::CreatureManager;
use crate::components::base::PlayerBase;

pub fn update_damage_system(
    events: &mut Vec<GameEvent>,
    creature_manager: &mut CreatureManager,
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
        }
    }
}