use crate::animation::Animation;

pub fn enemy1_idel_animation() -> Animation {
    Animation {
        start_frame: 9,
        frame_count: 1,
        frame_time: 1.0,
        current_frame: 0,
        timer: 0.0
    }
}