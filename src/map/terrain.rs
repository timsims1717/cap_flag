use serde::{Serialize, Deserialize};
use crate::map::{TerrainTile, Tile};
use amethyst::Error;

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

    pub fn create_tile(&self, t: char, e: usize) -> amethyst::Result<Tile> {
        match self.tiles.clone()
            .into_iter()
            .find(|tile| tile.char_code == t) {
            Some(tile) => Ok(tile.create_tile(e)),
            _ => Err(Error::from_string("tile incompatible with terrain set"))
        }
    }
}

fn default_description() -> String {
    "No Description".to_string()
}