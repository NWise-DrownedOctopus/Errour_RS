use macroquad::prelude::*;
use crate::utils::GameContext;
use macroquad::ui::{hash, root_ui, widgets, Skin};

pub enum MainMenuUIEvent {
    None,
    PlayClicked,
    SettingsClicked,
    QuitClicked,
}

pub enum SettingsUIEvent {
    None,
    AudioClicked,
    VideoClicked,
    GameClicked,
    BackClicked,
}

pub enum GameUIEvent {
    None,
    PauseClicked,
}

pub fn init_ui_skin() -> Skin{
    let skin1 = {
        let button_style = root_ui().style_builder()
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../art/ui/window_background_1.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(20.0, 20.0, 20.0, 20.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();

        Skin {
            window_style,
            button_style,
            ..root_ui().default_skin()
        }
    };

    return skin1;
}

pub fn draw_main_menu(context: &mut GameContext) -> MainMenuUIEvent {
    let mut event = MainMenuUIEvent::None;

    root_ui().push_skin(&context.window_skin);

    root_ui().window(hash!(), vec2(360., 650.), vec2(1200., 300.), |ui| {
        if widgets::Button::new("Play")
            .position(vec2(420.0, 15.0))
            .ui(ui)
        {
            event = MainMenuUIEvent::PlayClicked;
        }    

        if widgets::Button::new("Options")
            .position(vec2(400.0, 130.0))
            .ui(ui)
        {
            event = MainMenuUIEvent::SettingsClicked;
        }

        if widgets::Button::new("Quit")
            .position(vec2(420.0, 195.0))
            .ui(ui)
        {
            event = MainMenuUIEvent::QuitClicked;
        }            
    });

    event
}



pub fn draw_settings(context: &mut GameContext) -> SettingsUIEvent{
    let mut event = SettingsUIEvent::None;

    root_ui().push_skin(&context.window_skin);

    root_ui().window(hash!(), vec2(360., 650.), vec2(1200., 300.), |ui| {
        if widgets::Button::new("Video")
            .position(vec2(420.0, 15.0))
            .ui(ui)
        {
            event = SettingsUIEvent::VideoClicked;
        }

        if widgets::Button::new("Audio")
            .position(vec2(400.0, 130.0))
            .ui(ui)
        {
            event = SettingsUIEvent::AudioClicked;
        }

        if widgets::Button::new("Game")
            .position(vec2(400.0, 500.0))
            .ui(ui)
        {
            event = SettingsUIEvent::GameClicked;
        }

        if widgets::Button::new("Back")
            .position(vec2(420.0, 195.0))
            .ui(ui)
        {
            event = SettingsUIEvent::BackClicked;
        }
    });

    event
}

pub fn draw_game_ui(context: &mut GameContext) -> GameUIEvent {
    let mut event = GameUIEvent::None;

    root_ui().push_skin(&context.window_skin);
    // Left Bar
    root_ui().window(hash!(), vec2(0., 50.), vec2(610., 1030.), |_ui| {});
    root_ui().window(hash!(), vec2(610., 50.), vec2(50., 1030.), |_ui| {});

    // Right Bar
    root_ui().window(hash!(), vec2(1710., 50.), vec2(210., 1000.), |_ui| {});    

    // Top Bar
    root_ui().window(hash!(), vec2(0., 0.), vec2(1920., 50.), |ui| {
        if widgets::Button::new("Pause")
            .position(vec2(50.0, 0.0))
            .ui(ui)
        {
            event = GameUIEvent::PauseClicked;
        }
    });

    // Bottom Bar
    root_ui().window(hash!(), vec2(660., 1050.), vec2(1920., 30.), |_ui| {});

    event
}

pub fn draw_post_mission_screen(context: &mut GameContext) {
    root_ui().push_skin(&context.window_skin);
}

pub fn draw_loadout_menu(context: &mut GameContext) {
    root_ui().push_skin(&context.window_skin);
}

pub fn draw_campaign_hub(context: &mut GameContext) {
    root_ui().push_skin(&context.window_skin);
}

