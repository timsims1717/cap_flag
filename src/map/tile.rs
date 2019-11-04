use serde::{Serialize, Deserialize};
use rand::{Rng, thread_rng};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Slope {
    None, NW, NE, SE, SW,
}

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
    pub fn create_tile(self, e: usize) -> Tile {
        return Tile{
            index: self.tiles[thread_rng().gen_range(0,self.tiles.len())],
            height: self.height,
            elevation: e,
            slope: self.slope
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    pub index: usize,
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
            index: 0,
            height: 0,
            elevation: 0,
            slope: Slope::None,
        }
    }
}

fn slope_none() -> Slope {
    Slope::None
}