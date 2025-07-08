// Refactor Approved
mod managers;
mod components;
mod systems;
mod assets;

use crate::components::collider::CircleCollider;
use crate::components::animation::Animator;

use crate::assets::animations::{enemy1_idel_animation, player_base_idel_animation};
// May Need Refactoring
mod utils;
mod errour_ui;
mod game_manager;

use macroquad::prelude::*;

use errour_ui::{init_ui_skin};
use game_manager::{update_main_menu, update_campaign_hub, update_loadout_menu, update_gameplay, update_post_mission_screen, update_settings};
use crate::game_manager::AppState;
use crate::game_manager::GameState;
use utils::GameContext;

use crate::assets::art_assets::GameArtAssets;
use crate::components::base::PlayerBase;


#[macroquad::main("main_menu")]
async fn main() {
    utils::check_screen_size();    
    utils::scale_screen();    

    let rt_size = vec2(1050.0, 1000.0);
    let camera_view_rt = render_target(1050, 1000);
    camera_view_rt.texture.set_filter(FilterMode::Nearest);

    let art_assets = GameArtAssets::load().await;

    let player_base = PlayerBase {
        pos: vec2(525.0, 500.0),
        fire_speed: 3.0,
        rot: 0.0,
        rot_speed: 3.0,
        size: 30.0,
        // target: None,
        base_collider: CircleCollider {
            center: vec2(525.0, 500.0),
            radius: 25.0,
        },
        collided: false,
        animator: Animator {
            texture: &art_assets.player_base_texture,
            frame_width: 48.0,
            frame_height: 48.0,
            columns: 4,
            animation: player_base_idel_animation(),
            shadow_offset: 3.0,
        },
        health: 100.0,
        fire_range_collider: CircleCollider {
            center: vec2(525.0, 500.0),
            radius: 00.0,
        },
    };
    
    let mut context = GameContext {
        window_skin: init_ui_skin().clone(),
        debug_mode: false,
        app_state: AppState::MainMenu,
        game_state: GameState::None,
        game_camera: Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(rt_size.x / 2.0, rt_size.y / 2.0),
            render_target: Some(camera_view_rt),
            zoom: vec2(2.0 / rt_size.x, -2.0 / rt_size.y),
            ..Default::default()
        },
        game_camera_move_speed: 5.0,
        // creatures: Vec::new(),
        // player_base,
    };

    // Here we are spawning in 10 creatures at random locations
    /* NEEDS REFACTOR
    for _ in 0..10 {
        let pos = Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
        .normalize() * screen_width().min(screen_height()) / 2.;

        context.creatures.push(Creature {
            pos,
            speed: 0.5,
            rot: 0.,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.,
            target: vec2(525.0, 500.0),
            collided: false,
            animator: Animator {
                texture: &art_assets.enemy_texture,
                animation: enemy1_idel_animation(),
                frame_width: 48.0,
                frame_height: 48.0,
                columns: 3,
                shadow_offset: 25.0,
            },
            collider: CircleCollider {
                center: Vec2::new(pos.x, pos.y),
                radius: 12.0,
            },
            damage: 10.0,
            health: 10.0,
            dead: false,
        })
    }

    */
    
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
