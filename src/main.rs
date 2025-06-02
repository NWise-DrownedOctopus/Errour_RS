mod utils;
mod errour_ui;
mod game_manager;

use macroquad::prelude::*;
use errour_ui::{init_ui_skin};
use utils::{configure_camera, draw_grid_test};
use game_manager::{update_main_menu, update_campaign_hub, update_loadout_menu, update_gameplay, update_post_mission_screen, update_settings};
use crate::game_manager::AppState;
use crate::game_manager::GameState;
use utils::GameContext;

#[macroquad::main("main_menu")]
async fn main() {
    utils::check_screen_size();    
    utils::scale_screen();    

    let mut context = GameContext {
        window_skin: init_ui_skin().clone(),
        debug_mode: false,
        app_state: AppState::MainMenu,
        game_state: GameState::None,
    };

    let camera: Camera2D = configure_camera();
    set_camera(&camera);
    
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
            context.debug_mode = !context.debug_mode;
        }  

        // We clear the background and set it to a default state of black
        clear_background(BLACK);

        // We draw game objects to the screen

        if context.debug_mode {
            draw_grid_test(50.0, 21);
            draw_rectangle(0.0, 0.0, 50.0, 50.0, GREEN);
        }      

        handle_app_state(&mut context);  

        // We wait until the next frame before we continue our game loop, ensuring our code only runs
        // once per frame
        next_frame().await
    }
}

pub fn handle_app_state(context: &mut GameContext) {

    match context.app_state {
        AppState::MainMenu => update_main_menu(context),
        AppState::CampaignHub => update_campaign_hub(context),
        AppState::LoadoutSelection => update_loadout_menu(context),
        AppState::InGame => update_gameplay(context),
        AppState::PostMission => update_post_mission_screen(context),
        AppState::Settings => update_settings(context),
    }
}
