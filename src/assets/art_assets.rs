use macroquad::prelude::*;
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpriteID {
    Enemy,
    PlayerBase,
    Ground,
}

pub struct GameArtAssets {
    textures: [Texture2D; 3],
}

impl GameArtAssets {
    pub fn get(&self, id: SpriteID) -> &Texture2D {
        &self.textures[id as usize]
    }

    pub async fn load() -> Self {
        let enemy = load_texture("art/vindex/enemies_01.png").await.unwrap();
        let player_base = load_texture("art/errour/player_base_01.png").await.unwrap();  
        let ground = load_texture("art/environment/spots.png").await.unwrap();
        
        enemy.set_filter(FilterMode::Nearest);
        player_base.set_filter(FilterMode::Nearest); 
        ground.set_filter(FilterMode::Nearest);

        Self {
            textures: [enemy, player_base, ground],
        }
    }
}