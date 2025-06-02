use crate::utils::GameContext;
use crate::errour_ui::{draw_game_ui, draw_main_menu, draw_settings, GameUIEvent, MainMenuUIEvent, SettingsUIEvent, draw_post_mission_screen, draw_loadout_menu, draw_campaign_hub};

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
        MainMenuUIEvent::PlayClicked => context.app_state = AppState::InGame,
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
    let event = draw_game_ui(context);

    match event {
        GameUIEvent::PauseClicked => context.game_state = GameState::Paused,
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