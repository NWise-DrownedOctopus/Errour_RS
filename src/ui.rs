use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

pub fn draw_ui() {
    root_ui().same_line(0.);
    root_ui().group(hash!(), vec2(70.0,100.0), |ui| {
        ui.lable(None, "Window 1");

        if ui.button(None, "Skin 1") {
                window1_skin = default_skin.clone();
            }
    });

    root_ui().window(hash!(), vec2(20., 250.), vec2(300., 300.), |ui| {
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