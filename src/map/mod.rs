mod map;
mod tile;
mod terrain;

use std::fs;

pub use self::terrain::TerrainSet;
pub use self::map::Map;
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

pub fn map_to_world_iso(map_x: f32, map_y: f32, tile_size: f32) -> (f32, f32) {
    let world_x = (map_x - map_y) * tile_size * 0.5;
    let world_y = (map_x + map_y) * tile_size * 0.25;
    (world_x, world_y)
}

pub fn world_to_map_iso(world_x: f32, world_y: f32, tile_size: f32) -> (f32, f32) {
    let map_x = (world_x / (tile_size * 0.5) + world_y / (tile_size * 0.25)) / 2.;
    let map_y = (world_y / (tile_size * 0.25) - world_x / (tile_size * 0.5)) / 2.;
    (map_x, map_y)
}