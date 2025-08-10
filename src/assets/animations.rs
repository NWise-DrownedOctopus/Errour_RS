use crate::assets::art_assets::SpriteID;
use crate::components::animation::Animation;
use crate::components::animation::SpriteSheet;


pub fn enemy1_idel_animation() -> Animation {
    Animation {
        start_frame: 9,
        frame_count: 1,
        frame_time: 1.0,
        current_frame: 0,
        timer: 0.0
    }
}

pub fn enemy1_idel_sprite_sheet() -> SpriteSheet {
    SpriteSheet {
            texture_id: SpriteID::Enemy,
            frame_width: 48.0,
            frame_height: 48.0,
            columns: 4,
            shadow_offset: 3.0,
        }
}

pub fn player_base_idel_animation() -> Animation {
    Animation {
        start_frame: 0,
        frame_count: 4,
        frame_time: 0.5,
        current_frame: 0,
        timer: 0.0
    }
}

pub fn player_base_idel_sprite_sheet() -> SpriteSheet {
    SpriteSheet {
            texture_id: SpriteID::PlayerBase,
            frame_width: 48.0,
            frame_height: 48.0,
            columns: 4,
            shadow_offset: 3.0,
        }
}

pub fn projectile_01_animation() -> Animation {
    Animation {
        start_frame: 1,
        frame_count: 4,
        frame_time: 1.0,
        current_frame: 0,
        timer: 0.3
    }
}

pub fn projectile_01_sprite_sheet() -> SpriteSheet {
    SpriteSheet {
            texture_id: SpriteID::Projectile,
            frame_width: 8.0,
            frame_height: 8.0,
            columns: 4,
            shadow_offset: 3.0,
        }
}
