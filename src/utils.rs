use macroquad::prelude::*;
const VIRTUAL_WIDTH: i32 = 1920;
const VIRTUAL_HEIGHT: i32 = 1080;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Errour".to_string(),
        window_width: VIRTUAL_WIDTH,
        window_height: VIRTUAL_HEIGHT,
        ..Default::default()
    }
}

pub fn scale_screen() {
    // Here we need to determine if our virtual screen size fits on the current screen, and how to scale it it
    // First lets check if the virtual screen is too large
    if (VIRTUAL_HEIGHT as f32) > screen_height() {
        println!("virtual too tall");
    }
    else if (VIRTUAL_HEIGHT as f32) < screen_height() {
        println!("virtual too short")
    }
    else {
        println!("virtual has same height")
    }

    if (VIRTUAL_WIDTH as f32) > screen_width() {
        println!("virtual too wide");
    }
    else if (VIRTUAL_WIDTH as f32) < screen_width() {
        println!("virtual too narrow")
    }
    else {
        println!("virtual has same width")
    }

    println!("The current width of the screen is: {}", screen_width());
    println!("The current height of the screen is: {}", screen_height());

}
   