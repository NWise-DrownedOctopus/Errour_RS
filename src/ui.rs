use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

//lets make a working button
// It should detect mouse hover
// It should detect mouse press
// It should have the ability to call a any function when pressed, but do nothing by default
/* 
struct Button {
    x_pos: i32,
    y_pos: i32,
    width: i32,
    hieght: i32,
    bg_color: color,
    hover: bool,
    pressed: bool
}

impl Button {
    fn draw_button() {
        draw_rectangle(x_pos, y_pos, width, height, bg_color);
    }

    fn pressed(optional_callback: Option<impl Fn()>) {
        println!("Button was pressed");

        if let Some(callback) = optional_callback {
            callback();
        }
    }

    fn change_color(new_color: color) {
        bg_color = new_color;
    }
}*/

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