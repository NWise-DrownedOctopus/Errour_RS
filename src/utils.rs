use macroquad::prelude::*;
const VIRTUAL_WIDTH: i32 = 1920;
const VIRTUAL_HEIGHT: i32 = 1080;

use macroquad::window::{request_new_screen_size};

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