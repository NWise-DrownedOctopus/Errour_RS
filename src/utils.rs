use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

const VIRTUAL_WIDTH: i32 = 1920;
const VIRTUAL_HEIGHT: i32 = 1080;

use macroquad::window::{request_new_screen_size};

use crate::events::GameEvent;

use crate::assets::art_assets::GameArtAssets;
use crate::components::base::PlayerBase;

// Needs Refactor
use crate::game_manager::AppState;
use crate::game_manager::GameState;
use crate::managers::attack_manager::AttackManager;
use crate::managers::creature_manager::CreatureManager;
use crate::managers::projectile_manager::ProjectileManager;
use macroquad::ui::{Skin};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InGamePhase {
    Awake,
    Start,
    Update,
}

pub struct GameContext {
    pub window_skin: Skin,
    pub debug_mode: bool,
    pub app_state: AppState,
    pub in_game_phase: Option<InGamePhase>,
    pub level_config: Option<LevelConfig>,
    pub game_state: GameState,
    pub game_camera: Camera2D,
    pub game_camera_move_speed: f32,

    // Managers
    pub creature_manager: CreatureManager,
    pub projectile_manager: ProjectileManager,
    pub attack_manager: AttackManager,

    // Componenet Storage
    pub art_assets: GameArtAssets,

    // Events
    pub events: Vec<GameEvent>,

    // Indices of entities
    pub player_base: Option<PlayerBase>,
}

#[derive(Copy, Clone)]
pub struct Timer {
    pub elapsed: f32,
    pub duration: f32,
    pub done: bool,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Self {
            elapsed: 0.0,
            duration,
            done: false,
        }
    }

    pub fn update(&mut self) {
        if !self.done {
            self.elapsed += get_frame_time();
            if self.elapsed >= self.duration {
                self.done = true;
            }
        }
    }

    pub fn is_ready(&self) -> bool {
        self.done
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.done = false;
    }
}

pub fn scale_screen() -> (f32, f32, f32) {
    let sw = screen_width();
    let sh = screen_height();

    // Here we find the minium of either our actual screen width / virtual screen width or hight
    // This will be the scale we use to resize our window to match the device resolution
    let scale = (sw / VIRTUAL_WIDTH as f32).min(sh / VIRTUAL_HEIGHT as f32);

    // Offesets are used to create letterboxing if needed
    // We devide by two to make it even on either side of the window
    let offset_x = (sw - VIRTUAL_WIDTH as f32 * scale) / 2.0;
    let offset_y = (sh - VIRTUAL_HEIGHT as f32 * scale) / 2.0;

    // OLD MAY NEED TO BE DELETED
    // Here we need to determine if our virtual screen size fits on the current screen, and how to scale it it
    // First lets check if the virtual screen is too large
    request_new_screen_size(VIRTUAL_WIDTH as f32, VIRTUAL_HEIGHT as f32);

    (scale, offset_x, offset_y)    
}

pub fn to_screen_cords (x: f32, y: f32) -> (f32, f32) {
    let (scale, offset_x, offset_y) = scale_screen();
    (offset_x + x * scale, offset_y + y * scale)
}

pub fn to_virtual_cords (sx: f32, sy: f32) -> (f32, f32) {
    let (scale, offset_x, offset_y) = scale_screen();
    ((sx - offset_x) / scale, (sy - offset_y) / scale)
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

pub fn update_camera_pos(context: &mut GameContext, x_pos_change: f32, y_pos_change: f32) {
    context.game_camera.target = context.game_camera.target + vec2(x_pos_change, y_pos_change)

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerResources {
    pub metal: u32,
    pub xp: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFile {
    pub player_id: String,
    pub current_level: u32,
    pub open_levels: Vec<u32>,
    pub player_resources: PlayerResources,
    pub last_save_time: String, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelConfig {
    pub level_id: u32,
    pub name: String,
    pub starting_resources: PlayerResources,
    pub enemy_waves: Vec<EnemyWave>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyWave {
    pub enemy_type: String,
    pub count: u32,
}

fn load_save_file(path: &str) -> SaveFile {
    let json = fs::read_to_string(path).expect("Failed to read save file");
    serde_json::from_str(&json).expect("Failed to parse save file")
}

pub fn load_level_config(path: &str) -> LevelConfig {
    let json = fs::read_to_string(path).expect("Failed to read level config");
    serde_json::from_str(&json).expect("Failed to parse level config")
}