use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

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

pub fn draw_ui(ui_skin: &Skin) {
    root_ui().push_skin(ui_skin);
    root_ui().window(hash!(), vec2(360., 650.), vec2(1200., 300.), |ui| {
            widgets::Button::new("Play")
                .position(vec2(420.0, 15.0))
                .ui(ui);
            widgets::Button::new("Options")
                .position(vec2(400.0, 130.0))
                .ui(ui);

            widgets::Button::new("Quit")
                .position(vec2(420.0, 195.0))
                .ui(ui);
        });
}