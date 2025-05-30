use macroquad::prelude::*;
use errour_ui::{draw_main_menu, draw_game_ui, draw_settings, init_ui_skin};
use utils::{configure_camera, draw_grid_test};
use errour_ui::UIState;

mod utils;
mod errour_ui;

#[macroquad::main("main_menu")]
async fn main() {
    utils::check_screen_size();    
    utils::scale_screen();

    let window_skin = init_ui_skin().clone();
    let mut ui_state = UIState::MainMenu;

    let camera: Camera2D = configure_camera();
    set_camera(&camera);

    let mut debug_mode: bool = false;
    
    loop {    
        // Here I need to figure out how to render to the web
        // Mostly this involves rendering when the window size can't be fixed like I want
        /*    
        if utils::check_screen_size() == false {
            next_frame().await;
            continue;        
        }      
        */       

        if is_key_pressed(KeyCode::G) {
            debug_mode = !debug_mode;
        }  

        // We clear the background and set it to a default state of black
        clear_background(BLACK);

        // We call the appropriate draw function based on the current state of the UI
        match ui_state {
            UIState::MainMenu => draw_main_menu(&window_skin, &mut ui_state),
            UIState::Settings => draw_settings(&window_skin, &mut ui_state),
            UIState::GameUI => draw_game_ui(&window_skin),
        };

        // We draw game objects to the screen
        if debug_mode {
            draw_grid_test(50.0, 21);
            draw_rectangle(0.0, 0.0, 50.0, 50.0, GREEN);
        }        

        // We wait until the next frame before we continue our game loop, ensuring our code only runs
        // once per frame
        next_frame().await
    }
}
