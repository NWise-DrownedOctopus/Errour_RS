use macroquad::prelude::*;
// use crate::collision::Collider;
use crate::utils::{update_camera_pos, GameContext, InGamePhase, load_level_config};
use crate::errour_ui::{draw_game_ui, draw_main_menu, draw_settings, GameUIEvent, MainMenuUIEvent, SettingsUIEvent, draw_post_mission_screen, draw_loadout_menu, draw_campaign_hub, CampaignHubUIEvent};
use crate::utils::{draw_grid_test};
// use crate::vindex::{draw_creature};
// use crate::base::{draw_base, update_player_base_target};
use crate::systems::render::{draw_animated_entity, animation_system};
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
            if let Some(config) = &context.level_config {
                // Use config to spawn player, set resources, etc.
                context.player_base = Some(PlayerBase::init(context));

                let screen_radius = screen_width().min(screen_height()) / 2.;
                // Here we are spawning in 10 creatures at random locations
                let creature_manager = &mut context.creature_manager;

                for _ in 0..10 {
                    let pos = Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                    .normalize() * screen_radius;

                    creature_manager.spawn(
                        &mut context.positions,
                        &mut context.colliders,
                        &mut context.animations,
                        &mut context.sprite_sheets,
                        pos,
                    );
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
    
    // Move each enemy to thier targets
    /* 
    if matches!(context.game_state, GameState::Playing) {
        for creature in context.creatures.iter_mut() {
            // Move Each Creature
            creature.pos = creature.pos.move_towards(creature.target, creature.speed);

            // Check for Collsions with target
            // For now it is assumed this is the base
            // For now this is every frame
            if creature.collider.intersects(&context.player_base.base_collider) {
                context.player_base.health -= creature.damage;
                creature.dead = true;
            }
        }
    }
    */
    update_input(context);

    context.creature_manager.update(
        &mut context.positions,
        context.player_base.as_ref(),
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

    for i in 0..context.creature_manager.creatures.len() {
        draw_animated_entity(context, context.creature_manager.creatures[i].position_index, context.creature_manager.creatures[i].animation_index, context.creature_manager.creatures[i].sprite_sheet_index);
    }

    // draw_circle(525.0, 500.0, 25.0, GREEN);
    // draw_base(&mut context.player_base);

    // Here we handel our animation system
    animation_system(context);
    if let Some(base) = &context.player_base {
        draw_animated_entity(context, base.pos_index, base.animation_index, base.sprite_sheet_index);
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