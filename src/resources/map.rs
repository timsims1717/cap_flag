use serde::{Serialize, Deserialize};

use crate::resources::{TerrainSet, Tile};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MapDimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub dimensions: MapDimensions,
    pub terrain_file: String,
    pub data: String,
    #[serde(default)]
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn build_tiles(&mut self, terrain: &TerrainSet) {
        assert_ne!(self.dimensions.width, 0);
        assert_ne!(self.dimensions.height, 0);
        assert_eq!(self.dimensions.width * self.dimensions.height, self.data.chars().count());
        self.tiles = vec![vec![Tile::new(); self.dimensions.width]; self.dimensions.height];
        for (i, t) in self.data.chars().enumerate() {
            self.tiles[i/self.dimensions.width][i%self.dimensions.width] = terrain.create_tile(t, 0).unwrap();
        }
    }
}

impl Default for Map {
    fn default() -> Map {
        Map{
            dimensions: MapDimensions{ width: 0, height: 0},
            terrain_file: "".to_owned(),
            data: "".to_owned(),
            tiles: vec![vec![]]
        }
    }
}