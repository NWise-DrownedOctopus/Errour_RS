// Refactor Approved
mod managers;
mod components;
mod systems;
mod assets;
mod events;

// May Need Refactoring
mod utils;
mod errour_ui;
mod game_manager;

use macroquad::prelude::*;

use errour_ui::{init_ui_skin};
use game_manager::{update_main_menu, update_campaign_hub, update_loadout_menu, update_gameplay, update_post_mission_screen, update_settings};
use crate::game_manager::AppState;
use crate::game_manager::GameState;
use crate::managers::attack_manager::AttackManager;
use crate::managers::creature_manager::CreatureManager;
use crate::managers::projectile_manager::ProjectileManager;
use utils::GameContext;

// use crate::assets::art_assets::GameArtAssets;
use crate::assets::art_assets::GameArtAssets;

#[macroquad::main("main_menu")]
async fn main() {
    utils::check_screen_size();    
    utils::scale_screen();    

    let rt_size = vec2(1050.0, 1000.0);
    let camera_view_rt = render_target(1050, 1000);
    camera_view_rt.texture.set_filter(FilterMode::Nearest);

    let art_assets = GameArtAssets::load().await;
    
    let mut context = GameContext {
        window_skin: init_ui_skin().clone(),
        debug_mode: false,
        app_state: AppState::MainMenu,
        in_game_phase: None,
        level_config: None,
        game_state: GameState::None,
        game_camera: Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(rt_size.x / 2.0, rt_size.y / 2.0),
            render_target: Some(camera_view_rt),
            zoom: vec2(2.0 / rt_size.x, -2.0 / rt_size.y),
            ..Default::default()
        },
        game_camera_move_speed: 5.0,

        // Managers
        creature_manager: CreatureManager::new(),
        projectile_manager: ProjectileManager::new(),
        attack_manager: AttackManager::new(),

        // Componenet Storage
        art_assets: art_assets,

        // Events
        events: Vec::new(),

        // Indices of entities 
        player_base: None,
    };    
    
    loop {    
        // Here I need to figure out how to render to the web
        // Mostly this involves rendering when the window size can't be fixed like I want
        /*    
        if utils::check_screen_size() == false {
            next_frame().await;
            continue;        
        }      
        */

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
