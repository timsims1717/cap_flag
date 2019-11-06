use serde::{Serialize, Deserialize};
use crate::components::{TerrainTile, Tile};
use amethyst::{
    Error,
    renderer::SpriteRender,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainSet {
    pub texture_file: String,
    pub texture_format: String,
    #[serde(default = "default_description")]
    pub desc: String,
    pub tile_size: usize,
    pub num_tiles: usize,
    pub tiles: Vec<TerrainTile>,
}

impl TerrainSet {
    pub fn find_terrain(&self, t: char) -> amethyst::Result<TerrainTile> {
        match self.tiles.clone()
            .into_iter()
            .find(|tile| tile.char_code == t) {
            Some(tile) => Ok(tile.clone()),
            _ => Err(Error::from_string("tile incompatible with terrain set"))
        }
    }

    pub fn create_tile(&self, t: char, x: usize, y: usize, e: usize) -> amethyst::Result<Tile> {
        match self.tiles.clone()
            .into_iter()
            .find(|tile| tile.char_code == t) {
            Some(tile) => Ok(tile.create_tile(x, y, e)),
            _ => Err(Error::from_string("tile incompatible with terrain set"))
        }
    }
}

fn default_description() -> String {
    "No Description".to_string()
}

pub struct TerrainSprites {
    pub set: Vec<SpriteRender>
}