use macroquad::prelude::*;
use crate::utils::{update_camera_pos, GameContext};
use crate::errour_ui::{draw_game_ui, draw_main_menu, draw_settings, GameUIEvent, MainMenuUIEvent, SettingsUIEvent, draw_post_mission_screen, draw_loadout_menu, draw_campaign_hub};
use crate::utils::{draw_grid_test};
use crate::vindex::{draw_creature};

pub enum AppState {
    MainMenu,
    CampaignHub,
    LoadoutSelection,
    InGame,
    PostMission,
    Settings
}

pub enum GameState {
    None,
    Playing,
    Paused,
    GameOver,
}

pub fn update_main_menu(context: &mut GameContext) {
    // Here we handle input for changes
    let event = draw_main_menu(context);
    
    match event {
        MainMenuUIEvent::PlayClicked => {
            context.app_state = AppState::InGame;
            context.game_state = GameState::Paused;
        }
        MainMenuUIEvent::SettingsClicked => context.app_state = AppState::Settings,
        MainMenuUIEvent::QuitClicked => {
            // Quit the game
        },
        MainMenuUIEvent::None => {} // Do Nothing
    } 
}

pub fn update_campaign_hub(context: &mut GameContext) {
    draw_campaign_hub(context);
}

pub fn update_loadout_menu(context: &mut GameContext) {
    draw_loadout_menu(context);
}

pub fn update_gameplay(context: &mut GameContext) {
    //////////////////////////////////////////////////////////////// UPDATE INPUT
    // Here we handle input for changes
    if is_key_pressed(KeyCode::G) {
        context.debug_mode = !context.debug_mode;
    }

    if matches!(context.game_state, GameState::Playing) {
        if is_key_down(KeyCode::W) {
        update_camera_pos(context, 0., context.game_camera_move_speed);
        }

        if is_key_down(KeyCode::S) {
            update_camera_pos(context, 0., -context.game_camera_move_speed);
        }

        if is_key_down(KeyCode::D) {
            update_camera_pos(context, context.game_camera_move_speed, 0.);
        }

        if is_key_down(KeyCode::A) {
            update_camera_pos(context, -context.game_camera_move_speed, 0.);
        }
    }

      
    //////////////////////////////////////////////////////////////// UPDATE LOGIC
    
    // Move each enemy to thier targets
    if matches!(context.game_state, GameState::Playing) {
        for creature in context.creatures.iter_mut() {
        creature.pos = creature.pos.move_towards(creature.target, creature.speed);
        }
    }
    

    //////////////////////////////////////////////////////////////// DRAW
    // Update Camera
    set_camera(&context.game_camera);

    // We clear the background and set it to a default state
    clear_background(BEIGE);       

    draw_circle(525.0, 500.0, 25.0, GREEN);

    for creature in context.creatures.iter_mut() {
        draw_creature(creature);
    }

    if context.debug_mode {
        draw_grid_test(50.0, 21);
        draw_rectangle(0.0, 0.0, 50.0, 50.0, GREEN);
    }

    // Here we draw the game to a render texture, then draw that texture to the screen
    let rt = &context.game_camera.render_target.as_ref().unwrap().texture;
    set_default_camera();    
    draw_texture_ex(
        rt,
        660.0,
        50.0,
        WHITE,
        DrawTextureParams {
        dest_size: Some(vec2(1050.0, 1000.0)), // match render target size
        ..Default::default()
    },
    );    

    //////////////////////////////////////////////////////////////// UPDATE UI  
    let event = draw_game_ui(context);

    match event {
        GameUIEvent::PauseClicked => {
            if matches!(context.game_state, GameState::Playing) {
                context.game_state = GameState::Paused
            }
            else if matches!(context.game_state, GameState::Paused) {
                context.game_state = GameState::Playing
            }            
        }
        GameUIEvent::None => {} // Do Nothing
    }    
}

pub fn update_post_mission_screen(context: &mut GameContext) {
    draw_post_mission_screen(context);
}

pub fn update_settings(context: &mut GameContext) {
    let event = draw_settings(context);

    match event {
        SettingsUIEvent::AudioClicked => {
            // pull up audio Settings
        },
        SettingsUIEvent::VideoClicked => {
            // pull up video settings
        },
        SettingsUIEvent::GameClicked => {
            // pull up game settings
        }
        SettingsUIEvent::BackClicked => context.app_state = AppState::MainMenu,
        SettingsUIEvent::None => {} // Do Nothing
    }
}