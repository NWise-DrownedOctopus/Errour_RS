use macroquad::prelude::*;

pub struct Animation {
    pub start_frame: usize,
    pub frame_count: usize,
    pub frame_time: f32,
    pub current_frame: usize,
    pub timer: f32,
}

pub struct Animator<'a> {
    pub texture: &'a Texture2D,
    pub frame_width: f32,
    pub frame_height: f32,
    pub columns: usize,
    pub animation: Animation,
    pub shadow_offset: f32,
}

impl<'a> Animator<'a> {
    // This function will check how much time as passed since the last frame, and then update to the next frame
    // if the frame time is larger than the frame time.
    pub fn update(&mut self) {
        self.animation.timer += get_frame_time();
        if self.animation.timer > self.animation.frame_time {
            self.animation.timer = 0.0;
            // Make sure we loop if "current_frame" would be larger than frame_count
            self.animation.current_frame = (self.animation.current_frame + 1) % self.animation.frame_count;
        }
    }

    pub fn draw(&self, position: Vec2) {
        let frame_index = self.animation.start_frame + self.animation.current_frame;
        let x = (frame_index % self.columns) as f32 * self.frame_width;
        let y = (frame_index / self.columns) as f32 * self.frame_height;
        let source = Rect::new(x,y, self.frame_width, self.frame_height);
        let shadow_color = Color::new(0.0, 0.0, 0.0, 0.1);

        // First We draw the shadow for the creature flying
        draw_texture_ex(
            // added & here to borrow, but not sure why I needed to borrow
            &self.texture,
            position.x - (self.frame_height/2.0) - self.shadow_offset,
            position.y - (self.frame_width/2.0) - self.shadow_offset,
            shadow_color,
            DrawTextureParams {
                source: Some(source),
                flip_y: true,
                ..Default::default()
            },
        );

        // After Shadow draw creature sprite on top
        draw_texture_ex(
            // added & here to borrow, but not sure why I needed to borrow
            &self.texture,
            position.x - (self.frame_height/2.0),
            position.y - (self.frame_width/2.0),
            WHITE,
            DrawTextureParams {
                source: Some(source),
                flip_y: true,
                ..Default::default()
            },
        );
        
    }
}