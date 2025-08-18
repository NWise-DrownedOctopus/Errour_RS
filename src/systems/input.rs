use macroquad::prelude::*;

use crate::managers::input_manager::{InputEvent, InputManager};

use crate::utils::GameContext;

pub fn collect_input(input_manager: &mut InputManager) {
    if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            input_manager.push_event(InputEvent::MouseDown { button: MouseButton::Left });
            input_manager.push_event(InputEvent::MouseMove { x, y });
    }

    for key in [KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D] {
        if is_key_pressed(key) {
            input_manager.push_event(InputEvent::KeyPress { key });
        }
    }
}

pub fn process_input(input_manager: &mut InputManager, context: &mut GameContext) {
    for event in input_manager.drain_events() {
        match event {
            InputEvent::MouseMove { x, y } => {
                context.handle_mouse_move(x, y); 
            }
            InputEvent::MouseDown { button } => {
                context.handle_mouse_down(button);
            }
            InputEvent::KeyPress { key } => {
                context.handle_key_press(key);
            }
        }
    }
}