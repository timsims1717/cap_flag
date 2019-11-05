mod map;
mod tile;
mod terrain;

use std::fs;

pub use self::terrain::TerrainSet;
pub use self::map::{Map, MapDimensions};
pub use self::tile::{TerrainTile, Tile};
use ron::de::from_str;

pub fn load_test_map() -> amethyst::Result<Map> {
    load_map("test_map".to_owned())
}

//pub fn load_random_map_text() -> Map {
//
//}

pub fn load_map(filename: String) -> amethyst::Result<Map> {
    use amethyst::utils::application_root_dir;

    // add extension
    let f = if !filename.contains(".cfmap") {
        format!("{}{}", filename, ".cfmap")
    } else {
        filename
    };
    // load file
    let app_root = application_root_dir()?;

    let map_path = app_root.join(format!("maps/{}", f));
    let contents = fs::read_to_string(map_path).unwrap();
    let map: Map = from_str(&*contents).unwrap();
    Ok(map)
}

pub fn load_terrain_pack(filename: String) -> amethyst::Result<TerrainSet> {
    use amethyst::utils::application_root_dir;

    // add extension
    let f = if !filename.contains(".cfres") {
        format!("{}{}", filename, ".cfres")
    } else {
        filename
    };
    // load file
    let app_root = application_root_dir()?;

    let map_path = app_root.join(format!("resources/terrain/{}", f));
    let contents = fs::read_to_string(map_path).unwrap();
    let terrain: TerrainSet = from_str(&*contents).unwrap();
    Ok(terrain)
}

pub fn map_to_world_iso(map_x: f32, map_y: f32) -> (f32, f32) {
    let tile_size = 64.;
    let world_x = (map_x - map_y) * tile_size * 0.5;
    let world_y = (map_x + map_y) * tile_size * 0.25;
    (world_x, world_y)
}

pub fn world_to_map_iso(world_x: f32, world_y: f32) -> (f32, f32) {
    let tile_size = 64.;
    let map_x = (world_x / (tile_size * 0.5) + world_y / (tile_size * 0.25)) * 0.5;
    let map_y = (world_y / (tile_size * 0.25) - world_x / (tile_size * 0.5)) * 0.5;
    (map_x, map_y)
}

pub fn closest_point_in_map_iso(map_x: f32, map_y: f32, w: f32, h: f32) -> (f32, f32) {
    let (x, y) = if map_x < 0. && map_y < 0. {
        (0., 0.)
    } else if map_x < 0. && map_y > h {
        (0., h)
    } else if map_x > w && map_y < 0. {
        (w, 0.)
    } else if map_x > w && map_y > h {
        (w, h)
    } else if map_x < 0. {
        (0., map_y)
    } else if map_x > w {
        (w, map_y)
    } else if map_y < 0. {
        (map_x, 0.)
    } else if map_y > h {
        (map_x, h)
    } else {
        (map_x, map_y)
    };
    map_to_world_iso(x, y)
}