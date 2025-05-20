use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

pub fn draw_ui() {
    root_ui().window(hash!(), vec2(200., 250.), vec2(300., 300.), |ui| {
            widgets::Button::new("Play")
                .position(vec2(65.0, 15.0))
                .ui(ui);
            widgets::Button::new("Options")
                .position(vec2(40.0, 75.0))
                .ui(ui);

            widgets::Button::new("Quit")
                .position(vec2(65.0, 195.0))
                .ui(ui);
        });
}