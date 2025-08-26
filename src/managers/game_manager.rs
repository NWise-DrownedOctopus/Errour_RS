use macroquad::prelude::*;

use crate::errour_ui::{draw_game_ui, draw_main_menu, draw_settings, GameUIEvent, MainMenuUIEvent, SettingsUIEvent, draw_post_mission_screen, draw_loadout_menu, draw_campaign_hub, CampaignHubUIEvent};
//Components//
use crate::components::base::PlayerBase;
//Managers//

//Systems//
use crate::systems::collision::update_collision;
use crate::systems::damage::{update_damage_system};
use crate::systems::render::{draw_animated_entity, animation_system, debug_collider_draw};
use crate::systems::movement::{movement_update};
//Utils//
use crate::utils::{load_level_config, update_camera_pos, GameContext, InGamePhase};
use crate::utils::{draw_grid_test};

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
                let player_base = PlayerBase::new();

                context.attack_manager.register_attacker(
                    player_base.fire_cooldown,
                    player_base.range,
                    player_base.attack_type,
                    player_base.pos,
                );

                context.player_base = Some(player_base);

                let screen_radius = screen_width().min(screen_height()) / 2.;
                // Here we are spawning in 10 creatures at random locations
                let creature_manager = &mut context.creature_manager;

                for _ in 0..10 {
                    let pos = Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                    .normalize() * screen_radius;
                
                    let creature_health = 1.0;
                    let drop_table_id = 1; // PLACEHOLDER
                    creature_manager.spawn(pos, creature_health, drop_table_id);
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
        &mut context.projectile_manager,
        context.player_base.as_ref(),
    );    

    update_collision(
        &mut context.creature_manager, 
        &mut context.projectile_manager,
        context.player_base.as_ref(),
        &mut context.events,
    );

    // Update attack solutions for play controlled entities (Right now only Base)
    let target_positions = &context.creature_manager.positions;
    let target_colliders = &context.creature_manager.colliders;
    let valid_target_flags = &context.creature_manager.dead_flags;

    context.attack_manager.update_attack_solutions(
        target_positions,
        target_colliders,
        valid_target_flags
    );

    // Fire at valid targets
    let mut attack_commands = Vec::new();
    context.attack_manager.fire_ready_attackers( &mut attack_commands);
    context.projectile_manager.process_attack_commands(&attack_commands);
    

    //////////////////////////////////////////////////////////////// DRAW
    // Update Camera
    set_camera(&context.game_camera);

    // We clear the background and set it to a default state
    clear_background(BEIGE);      

    // Here we handel our animation system
    animation_system(context);
    if let Some(base) = &context.player_base {
        draw_animated_entity(context, base.pos, &base.animation, &base.sprite_sheet);
    }  

    // Here we draw all of the creatures in the creature manager
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

    // Here we draw all of the projectiles in the projectile maanger
    for (_i, projectile) in context.projectile_manager.projectiles.iter().enumerate() {
        // Skip dead Projectiles
        if context.projectile_manager.dead_flags[projectile.dead_flag_index].0 {
            continue;
        }

        let position = context.projectile_manager.positions[projectile.position_index];
        let animation = &context.projectile_manager.animations[projectile.animation_index];
        let sprite_sheet = &context.projectile_manager.sprite_sheets[projectile.sprite_sheet_index];

        draw_animated_entity(context, position, animation, sprite_sheet);
    }      

    if context.debug_mode {
        draw_grid_test(50.0, 21);
        draw_rectangle(0.0, 0.0, 50.0, 50.0, GREEN);
        debug_collider_draw(context);
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
    
    // Here we calculate all of the damage done this frame, and apply it to targets
    update_damage_system(
        &mut context.events,
        &mut context.creature_manager,
        &mut context.projectile_manager,
        &mut context.player_base
    );

    // We should check if any projectiles no longer have a valid target, and destroy them
    context.projectile_manager.process_hanging_projectiles(&mut context.creature_manager);

    // Here we check for any targets that should be removed because they should have died this frame.
    context.creature_manager.process_dead_creatures(&mut context.events);

    // Here we draw our UI
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