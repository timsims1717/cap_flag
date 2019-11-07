use amethyst::ecs::prelude::{Component, DenseVecStorage};
use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use crate::util::TILE_SIZE;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Slope {
    None, NW, NE, SE, SW,
}

// serves as a tile "generator", from the Terrain resource pack
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainTile {
    pub name: String,
    pub char_code: char,
    pub tiles: Vec<usize>,
    #[serde(default)]
    pub height: usize,
    #[serde(default = "slope_none")]
    pub slope: Slope,
}

impl TerrainTile {
    // generates a Tile from the parameters and its own data
    // randomly assigns a sprite to the tile if there are multiple sprites for this TerrainTile
    // possibly add option for tiling by index instead of randomly
    pub fn create_tile(self, x: usize, y: usize, e: usize) -> Tile {
        return Tile{
            sprite_index: self.tiles[thread_rng().gen_range(0, self.tiles.len())],
            x,
            y,
            height: self.height,
            elevation: e,
            slope: self.slope
        }
    }
}

// a tile on the map
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    pub sprite_index: usize,
    pub x: usize,
    pub y: usize,
    #[serde(default)]
    pub height: usize,
    #[serde(default)]
    pub elevation: usize,
    #[serde(default = "slope_none")]
    pub slope: Slope,
}

impl Tile {
    pub fn new() -> Tile {
        Tile{
            sprite_index: 0,
            x: 0,
            y: 0,
            height: 0,
            elevation: 0,
            slope: Slope::None,
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

fn slope_none() -> Slope {
    Slope::None
}

// a ui element for a tile
pub struct TileUIElement {
    pub tile_x: usize,
    pub tile_y: usize,
    pub el_type: TileUIElementType,
}

pub enum TileUIElementType {
    EditorMouseOver,
}

impl Component for TileUIElement {
    type Storage = DenseVecStorage<Self>;
}