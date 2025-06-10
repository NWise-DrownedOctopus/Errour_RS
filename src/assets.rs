use macroquad::prelude::*;

pub struct GameArtAssets {
    pub enemy_texture: Texture2D,
}

impl GameArtAssets {
    pub async fn load() -> Self {
        let enemy_texture = load_texture("art/vindex/enemies_01.png").await.unwrap();
        enemy_texture.set_filter(FilterMode::Nearest); 

        Self {
            enemy_texture,
        }
    }
}