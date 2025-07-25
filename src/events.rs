#[derive(Debug, Clone)]
pub enum GameEvent {
    CreatureHitBase {
        creature_index: usize,
        damage: u32,
    },
}