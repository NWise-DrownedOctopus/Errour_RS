use macroquad::prelude::*;
use crate::utils::window_conf;

mod utils;

#[macroquad::main(window_conf)]
async fn main() {
    utils::scale_screen();
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("CAN I GET WEB TO WORK!?", 20.0, 300.0, 30.0, DARKGRAY);

        next_frame().await
    }
}