use macroquad::prelude::*;

use crate::components::collider::{CircleCollider};
use crate::events::GameEvent;
use crate::managers::creature_manager::CreatureManager;
use crate::components::base::PlayerBase;

pub fn update_collision(
    creature_manager: &mut CreatureManager,
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
}

fn intersects (
    creature_pos: Vec2,
    creature_collider: &CircleCollider,
    base_pos: Vec2,
    base_collider: &CircleCollider,
) -> bool {
    let dist = base_pos.distance(creature_pos);
    dist <= (base_collider.radius + creature_collider.radius)
}

/* 
impl CircleCollider {
    fn contains_point(&self, point: Vec2) -> bool {
        self.center.distance(point) <= self.radius
    }

    fn intersects(&self, other: &CircleCollider) -> bool {
        let dist = self.center.distance(other.center);
        dist <= (self.radius + other.radius)     
    }

    fn debug_draw(&self) {
        let debug_color = Color::new(255.0, 0.0, 255.0, 0.5);
        draw_circle(self.center.x,self.center.y, self.radius, debug_color);
    }
}
*/

/* 
impl RectCollider {
    
    fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.x &&
        point.x <= self.x + self.width &&
        point.y >= self.y &&
        point.y <= self.y + self.height
    }

    fn intersects(&self, other: &RectCollider) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y

    }

    fn debug_draw(&self) {
        let debug_color = Color::new(255.0, 0.0, 255.0, 0.5);
        draw_rectangle(self.x, self.y, self.width, self.height, debug_color);
    }
}

*/