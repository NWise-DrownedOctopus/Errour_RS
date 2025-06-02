use macroquad::prelude::*;
const VIRTUAL_WIDTH: i32 = 1920;
const VIRTUAL_HEIGHT: i32 = 1080;

use macroquad::window::{request_new_screen_size};
use crate::game_manager::AppState;
use crate::game_manager::GameState;
use macroquad::ui::{Skin};

pub struct GameContext {
    pub window_skin: Skin,
    pub debug_mode: bool,
    pub app_state: AppState,
    pub game_state: GameState,
}

pub fn scale_screen() {
    // Here we need to determine if our virtual screen size fits on the current screen, and how to scale it it
    // First lets check if the virtual screen is too large
    request_new_screen_size(VIRTUAL_WIDTH as f32, VIRTUAL_HEIGHT as f32);
}
   
pub fn check_screen_size() -> bool {
    let correct_height: bool;
    let correct_width: bool;
    if (VIRTUAL_HEIGHT as f32) > screen_height() {
        println!("virtual too tall");
        correct_height = false;
    }
    else if (VIRTUAL_HEIGHT as f32) < screen_height() {
        println!("virtual too short");
        correct_height = false;
    }
    else {
        println!("virtual has same height");
        correct_height = true;
    }

    if (VIRTUAL_WIDTH as f32) > screen_width() {
        println!("virtual too wide");
        correct_width = false;
    }
    else if (VIRTUAL_WIDTH as f32) < screen_width() {
        println!("virtual too narrow");
        correct_width = false;
    }
    else {
        println!("virtual has same width");
        correct_width = true;
    }

    println!("The current width of the screen is: {}", screen_width());
    println!("The current height of the screen is: {}", screen_height());

    if correct_width && correct_height {
        return true
    }
    else {
        return false
    }

}

pub fn configure_camera() -> Camera2D {
    Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(300.0, 510.0),
            zoom: vec2(2.0 / 1920.0, -2.0 / 1080.0),
            ..Default::default()
        }
}

pub fn draw_grid_test(spacing: f32, range: i32) {
    let color = GRAY;

    for i in 1..=range {
        let i_f = i as f32 * spacing;
        // Vertical lines
        draw_line(i_f, 0 as f32, i_f, 1000.0, 1.0, color);
    }

    for i in 1..range {
        let i_f = i as f32 * spacing;
        // Horizontal lines
        draw_line(0.0, i_f, spacing * range as f32, i_f, 1.0, color);
    }

    // Draw origin axes
    draw_line(0.0, 0.0, 1050.0, 0.0, 2.0, RED); // X-axis
    draw_line(0.0, 0.0, 0.0, 1000.0, 2.0, BLUE); // Y-axis
}