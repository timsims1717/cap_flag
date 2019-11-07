use amethyst::ecs::Entity;
use serde::{Serialize, Deserialize};

use crate::components::Tile;
use crate::resources::TerrainSet;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MapDimensions {
    pub width: usize,
    pub height: usize,
}

// the resource that holds a list of tile entities
#[derive(Clone)]
pub struct TileMap {
    pub v: Vec<Entity>,
}

impl TileMap {
    pub fn get(&self, x: usize, y: usize, w: usize) -> Entity {
        self.v[y * w + x]
    }
}

// a map from a file (not used during play)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub dimensions: MapDimensions,
    pub terrain_file: String,
    pub data: String,
    #[serde(default)]
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    // converts the map into a set of proper tiles
    pub fn build_tiles(&mut self, terrain: &TerrainSet) {
        assert_ne!(self.dimensions.width, 0);
        assert_ne!(self.dimensions.height, 0);
        let chars: Vec<char> = self.data.chars().collect();
        assert_eq!(self.dimensions.width * self.dimensions.height, chars.len() / 2);
        self.tiles = vec![vec![Tile::new(); self.dimensions.width]; self.dimensions.height];
        for i in 0..(chars.len() / 2) {
            let t = chars[i*2];
            let ec = chars[i*2+1];
            let e = ec.to_digit(10).unwrap() as usize;
            let y = (i)/self.dimensions.width;
            let x = (i)%self.dimensions.width;
            self.tiles[y][x] = terrain.create_tile(t, x, y, e).unwrap();
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