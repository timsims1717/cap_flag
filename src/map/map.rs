use serde::{Serialize, Deserialize};

use crate::map::{TerrainSet, Tile};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub w: usize,
    pub h: usize,
    pub terrain_file: String,
    pub data: String,
    #[serde(default)]
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn build_tiles(&mut self, terrain: &TerrainSet) {
        assert_ne!(self.w, 0);
        assert_ne!(self.h, 0);
        assert_eq!(self.w * self.h, self.data.chars().count());
        self.tiles = vec![vec![Tile::new(); self.w]; self.h];
        for (i, t) in self.data.chars().enumerate() {
            self.tiles[i/self.w][i%self.w] = terrain.create_tile(t, 0).unwrap();
        }
    }
}

impl Default for Map {
    fn default() -> Map {
        Map{
            w: 0,
            h: 0,
            terrain_file: "".to_owned(),
            data: "".to_owned(),
            tiles: vec![vec![]]
        }
    }
}