use macroquad::prelude::*;

pub enum InputEvent {
    MouseMove { x: f32, y: f32 },
    MouseDown {  button: MouseButton },
    KeyPress { key: KeyCode },
}

pub struct InputManager {
    pub events: Vec<InputEvent>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),                  
        }
    }

    pub fn push_event (&mut self, event: InputEvent) {
        self.events.push(event);
    }

    pub fn drain_events (&mut self) -> Vec<InputEvent> {
        // Here is where we handle the actual input
        std::mem::take(&mut self.events)
    }
}