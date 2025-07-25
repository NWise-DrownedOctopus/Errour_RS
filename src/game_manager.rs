use macroquad::prelude::*;

use crate::errour_ui::{draw_game_ui, draw_main_menu, draw_settings, GameUIEvent, MainMenuUIEvent, SettingsUIEvent, draw_post_mission_screen, draw_loadout_menu, draw_campaign_hub, CampaignHubUIEvent};

use crate::systems::collision::update_collision;
use crate::systems::damage::{update_damage_system};
use crate::utils::{update_camera_pos, GameContext, InGamePhase, load_level_config};
use crate::utils::{draw_grid_test};

use crate::systems::render::{draw_animated_entity, animation_system};
use crate::systems::movement::{movement_update};

use crate::components::base::PlayerBase;

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
            context.app_state = AppState::CampaignHub;
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
    let event = draw_campaign_hub(context);
    
    match event {
        CampaignHubUIEvent::Level1 => {
            // Load Level 1           
            context.app_state = AppState::InGame;  
        }
        CampaignHubUIEvent::Level2 => {
            // Load Level 2
            context.app_state = AppState::InGame;
        },
        CampaignHubUIEvent::Level3 => {
            // Load Level 3
            context.app_state = AppState::InGame;
        },
        CampaignHubUIEvent::None => {} // Do Nothing
    }
}

pub fn update_loadout_menu(context: &mut GameContext) {
    draw_loadout_menu(context);
}

pub fn update_gameplay(context: &mut GameContext) {
    match context.in_game_phase.unwrap_or(crate::utils::InGamePhase::Awake) {
        InGamePhase::Awake => {
            context.level_config = Some(load_level_config("data/leveldata/level1.json"));
            context.in_game_phase = Some(InGamePhase::Start);
        }
        InGamePhase::Start => {
            if let Some(_config) = &context.level_config {
                // Use config to spawn player, set resources, etc.
                context.player_base = Some(PlayerBase::new());

                let screen_radius = screen_width().min(screen_height()) / 2.;
                // Here we are spawning in 10 creatures at random locations
                let creature_manager = &mut context.creature_manager;

                for _ in 0..10 {
                    let pos = Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                    .normalize() * screen_radius;

                    creature_manager.spawn(pos);
                }
            }
            context.in_game_phase = Some(InGamePhase::Update);
        }
        InGamePhase::Update => {
            update(context);
            late_update(context);
            context.in_game_phase = Some(InGamePhase::Update);
        }
    }       
}

pub fn update(context: &mut GameContext) {
     //////////////////////////////////////////////////////////////// UPDATE LOGIC  
    update_input(context);

    movement_update(
        &mut context.creature_manager,
        context.player_base.as_ref(),
    );    

    update_collision(
        &mut context.creature_manager, 
        context.player_base.as_ref(),
        &mut context.events,
    );

    update_damage_system(
        &mut context.events,
        &mut context.creature_manager,
        &mut context.player_base
    );

    // Here we update our player base targeting
    // update_player_base_target(context);

    // For each creatrure that collded with the base, remove them from vec
    // context.creatures.retain(|creature| !creature.dead);
    

    //////////////////////////////////////////////////////////////// DRAW
    // Update Camera
    set_camera(&context.game_camera);

    // We clear the background and set it to a default state
    clear_background(BEIGE);      

    for (_i, creature) in context.creature_manager.creatures.iter().enumerate() {
        // Skip dead creatures
        if context.creature_manager.dead_flags[creature.dead_flag_index].0 {
            continue;
        }

        let position = context.creature_manager.positions[creature.position_index];
        let animation = &context.creature_manager.animations[creature.animation_index];
        let sprite_sheet = &context.creature_manager.sprite_sheets[creature.sprite_sheet_index];

        draw_animated_entity(context, position, animation, sprite_sheet);
    }

    // Here we handel our animation system
    animation_system(context);
    if let Some(base) = &context.player_base {
        draw_animated_entity(context, base.pos, &base.animation, &base.sprite_sheet);
    }    

    if context.debug_mode {
        draw_grid_test(50.0, 21);
        draw_rectangle(0.0, 0.0, 50.0, 50.0, GREEN);
        // For each creature, we want to draw their collider here
        /*
        for creature in context.creatures.iter_mut() {
            creature.collider.debug_draw();
        }
        context.player_base.base_collider.debug_draw();
        context.player_base.fire_range_collider.debug_draw();
        */ 
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
}

pub fn late_update(context: &mut GameContext) {
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

pub fn update_input(context: &mut GameContext) {
    if is_key_pressed(KeyCode::G) {
        context.debug_mode = !context.debug_mode;
    }

    if is_key_pressed(KeyCode::Space) {
        if matches!(context.game_state, GameState::Playing) {context.game_state = GameState::Paused;}        
        else if matches!(context.game_state, GameState::Paused) {context.game_state = GameState::Playing;}
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