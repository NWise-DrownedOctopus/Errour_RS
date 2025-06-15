use macroquad::prelude::*;

pub struct GameArtAssets {
    pub enemy_texture: Texture2D,
    pub player_base_texture: Texture2D,
}

impl GameArtAssets {
    pub async fn load() -> Self {
        let enemy_texture = load_texture("art/vindex/enemies_01.png").await.unwrap();
        enemy_texture.set_filter(FilterMode::Nearest);

        let player_base_texture = load_texture("art/errour/player_base_01.png").await.unwrap();
        player_base_texture.set_filter(FilterMode::Nearest); 

        let ground_texture = load_texture("art/environment/spots.png").await.unwrap();
        ground_texture.set_filter(FilterMode::Nearest);

        Self {
            enemy_texture,
            player_base_texture,
        }
    }
}