use macroquad::prelude::*;

use crate::{utils::GameContext};
use crate::components::animation::{Animation, SpriteSheet};

pub fn draw_animated_entity(context: &GameContext, pos: Vec2, anim: &Animation, sprite: &SpriteSheet) {
    let frame_index = anim.start_frame + anim.current_frame;
    let x = (frame_index % sprite.columns) as f32 * sprite.frame_width;
    let y = (frame_index / sprite.columns) as f32 * sprite.frame_height;
    let source = Rect::new(x,y, sprite.frame_width, sprite.frame_height);
    let texture = context.art_assets.get(sprite.texture_id);    

    // First We draw the shadow for the creature flying
    draw_texture_ex(
        // added & here to borrow, but not sure why I needed to borrow
        texture,
        pos.x - (sprite.frame_height/2.0) - sprite.shadow_offset,
        pos.y - (sprite.frame_width/2.0) - sprite.shadow_offset,
         Color::new(0.0, 0.0, 0.0, 0.1),
        DrawTextureParams {
            source: Some(source),
            flip_y: true,
            ..Default::default()
        },
    );

    // After Shadow draw creature sprite on top
    draw_texture_ex(
        // added & here to borrow, but not sure why I needed to borrow
        texture,
        pos.x - (sprite.frame_height/2.0),
        pos.y - (sprite.frame_width/2.0),
        WHITE,
        DrawTextureParams {
            source: Some(source),
            flip_y: true,
            ..Default::default()
        },
    );
}

pub fn animation_system(context: &mut GameContext) {
    for animation in &mut context.creature_manager.animations {
        animation.timer += get_frame_time(); // NOTE::Do I need to worry about some being at different times becasuse they are called sequentially??
        if animation.timer > animation.frame_time {
            animation.timer = 0.0;
            animation.current_frame = (animation.current_frame + 1) % animation.frame_count;
        }
    }
}