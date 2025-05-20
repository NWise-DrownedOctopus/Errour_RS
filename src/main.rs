use macroquad::prelude::*;
use crate::utils::window_conf;
use crate::errour_ui::draw_ui;

use macroquad::ui::{hash, root_ui, widgets, Skin};

mod utils;
mod errour_ui;

#[macroquad::main("main_menu")]
async fn main() {
    utils::check_screen_size();
    let window_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();

    utils::scale_screen();
    
    loop {        
        if utils::check_screen_size() == false {
            next_frame().await;
            continue;        
        }        
        clear_background(RED);

        draw_ui(); 

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("CAN I GET WEB TO WORK!??", 20.0, 300.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

