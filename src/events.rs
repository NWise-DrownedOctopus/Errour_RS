#[derive(Debug, Clone)]
pub enum GameEvent {
    CreatureHitBase {
        creature_index: usize,
        damage: u32,
    },

    ProjectileHitCreature {
        creature_index: usize,
        projectile_index: usize,
    },

    CreatureDied {
        creature_index: usize,
    },
}