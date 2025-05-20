use errour_ui::{draw_ui, init_ui_skin};
use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

mod utils;
mod errour_ui;

#[macroquad::main("main_menu")]
async fn main() {
    utils::check_screen_size();
    
    utils::scale_screen();

    let window_skin = init_ui_skin().clone();
    
    loop {  
        /*      
        if utils::check_screen_size() == false {
            next_frame().await;
            continue;        
        }    
        */    

        clear_background(BLACK);

        draw_ui(&window_skin); 

        next_frame().await
    }
}
