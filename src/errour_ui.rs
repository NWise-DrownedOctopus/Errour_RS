use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

pub enum UIState {
    MainMenu,
    Settings,
    GameUI,
}

pub fn init_ui_skin() -> Skin{
    let skin1 = {
        let button_style = root_ui().style_builder()
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };

    return skin1;
}

pub fn draw_main_menu(ui_skin: &Skin,  ui_state: &mut UIState) {
    root_ui().push_skin(ui_skin);
    root_ui().window(hash!(), vec2(360., 650.), vec2(1200., 300.), |ui| {
            if widgets::Button::new("Play")
                .position(vec2(420.0, 15.0))
                .ui(ui)
            {
                *ui_state = UIState::GameUI;
            }    

            if widgets::Button::new("Options")
                .position(vec2(400.0, 130.0))
                .ui(ui)
            {
                *ui_state = UIState::Settings;
            }

            widgets::Button::new("Quit")
                .position(vec2(420.0, 195.0))
                .ui(ui);
        });
}

pub fn draw_settings(ui_skin: &Skin,  ui_state: &mut UIState) {
    root_ui().push_skin(ui_skin);
    root_ui().window(hash!(), vec2(360., 650.), vec2(1200., 300.), |ui| {
            widgets::Button::new("Video")
                .position(vec2(420.0, 15.0))
                .ui(ui);
            widgets::Button::new("Audio")
                .position(vec2(400.0, 130.0))
                .ui(ui);

            if widgets::Button::new("Back")
                .position(vec2(420.0, 195.0))
                .ui(ui)
            {
                *ui_state = UIState::MainMenu;
            }
        });
}

pub fn draw_game_ui(ui_skin: &Skin) {
    root_ui().push_skin(ui_skin);
    // Left Bar
    root_ui().window(hash!(), vec2(0., 50.), vec2(610., 1030.), |ui| {});
    root_ui().window(hash!(), vec2(610., 50.), vec2(50., 1030.), |ui| {});

    // Right Bar
    root_ui().window(hash!(), vec2(1710., 50.), vec2(210., 1000.), |ui| {});    

    // Top Bar
    root_ui().window(hash!(), vec2(0., 0.), vec2(1920., 50.), |ui| {});

    // Bottom Bar
    root_ui().window(hash!(), vec2(660., 1050.), vec2(1920., 30.), |ui| {});
}