mod editor;
mod map;
mod terrain;
mod ui;

use ron::de::from_str;
use std::fs;

pub use self::map::{Map, MapDimensions, TileMap};
pub use self::terrain::{TerrainSet, TerrainSprites};
pub use self::ui::{CameraHandle, UISprites};

pub fn load_test_map() -> amethyst::Result<Map> {
    load_map("test_map".to_owned())
}

//pub fn load_random_map_text() -> Map {
//
//}

// loads a map from a string filename
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

// loads a terrain pack from a string filename
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