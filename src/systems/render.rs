use macroquad::prelude::*;

// This function should handle drawing the entierty of the game state. It will need refernce to each manager
pub fn draw_game() {

}

// Refactor 
/*
pub fn draw_creature(creature: &mut Creature) {
    creature.animator.update();
    creature.collider.center = Vec2::new(creature.pos.x, creature.pos.y);
    creature.animator.draw(creature.pos);
}*/