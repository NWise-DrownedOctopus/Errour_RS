use macroquad::prelude::*;
use errour_ui::{draw_main_menu, draw_game_ui, draw_settings, init_ui_skin};

mod utils;
mod errour_ui;

use errour_ui::UIState;

#[macroquad::main("main_menu")]
async fn main() {
    utils::check_screen_size();
    
    utils::scale_screen();

    let window_skin = init_ui_skin().clone();
    let mut ui_state = UIState::MainMenu;
    
    loop {  
        /*      
        if utils::check_screen_size() == false {
            next_frame().await;
            continue;        
        }    
        */    
        
        let (mouse_x, mouse_y) = mouse_position();

        clear_background(BLACK);

        match ui_state {
            UIState::MainMenu => draw_main_menu(&window_skin, &mut ui_state),
            UIState::Settings => draw_settings(&window_skin, &mut ui_state),
            UIState::GameUI => draw_game_ui(&window_skin),
        };        

        next_frame().await
    }
}
