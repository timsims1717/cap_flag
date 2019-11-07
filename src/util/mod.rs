use amethyst::{
    core::{
        math::{Point3,Vector2},
        Transform,
    },
    ecs::{
        Join,
        prelude::WriteStorage
    },
    renderer::camera::Camera,
    window::ScreenDimensions,
};
use crate::components::Tile;
use crate::resources::{MapDimensions, TileMap};

pub const TILE_SIZE: f32 = 64.;
pub const HALF_TILE: f32 = 32.;
pub const QUARTER_TILE: f32 = 16.;
pub const ELEVATION: f32 = 24.;

// takes mouse coordinates and converts them to world coordinates
pub fn mouse_to_world_iso(mouse_x: f32, mouse_y: f32, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform) -> (f32, f32) {
    let diagonal = Vector2::new(screen_dim.width(), screen_dim.height());
    let world_point = camera.projection().screen_to_world_point(
        Point3::new(mouse_x, mouse_y, 0.),
        diagonal,
        camera_transform,
    );
    (world_point.x, -world_point.y)
}

// takes mouse coordinates and converts them to base map coordinates
pub fn mouse_to_map_iso_simple(mouse_x: f32, mouse_y: f32, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform) -> (f32, f32) {
    let diagonal = Vector2::new(screen_dim.width(), screen_dim.height());
    let world_point = camera.projection().screen_to_world_point(
        Point3::new(mouse_x, mouse_y, 0.),
        diagonal,
        camera_transform,
    );
    world_to_map_iso_simple(world_point.x, -world_point.y)
}

// takes mouse coordinates and converts them to map coordinates, taking elevation into account
pub fn mouse_to_map_iso(mouse_x: f32, mouse_y: f32, map_dim: &MapDimensions, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform, tile_map: &TileMap, tiles: &WriteStorage<Tile>) -> Option<(usize, usize)> {
    let diagonal = Vector2::new(screen_dim.width(), screen_dim.height());
    let world_point = camera.projection().screen_to_world_point(
        Point3::new(mouse_x, mouse_y, 0.),
        diagonal,
        camera_transform,
    );
    world_to_map_iso(world_point.x, -world_point.y, map_dim.width, map_dim.height, tile_map, tiles)
}

// takes base map coordinates and converts them to world coordinates
pub fn map_to_world_iso_simple(map_x: f32, map_y: f32) -> (f32, f32) {
    let world_x = (map_x - map_y) * HALF_TILE;
    let world_y = (map_x + map_y) * QUARTER_TILE;
    (world_x, world_y)
}

// takes map coordinates and converts them to world coordinates, taking elevation into account
pub fn map_to_world_iso(map_x: f32, map_y: f32, elevation: f32, height: f32) -> (f32, f32) {
    let world_x = (map_x - map_y) * HALF_TILE;
    let world_y = ((map_x + map_y) * QUARTER_TILE) - (elevation * ELEVATION) + height;
    (world_x, world_y)
}

// takes world coordinates and converts them into base map coordinates
pub fn world_to_map_iso_simple(world_x: f32, world_y: f32) -> (f32, f32) {
    let map_x = (world_x / (HALF_TILE) + world_y / (QUARTER_TILE)) * 0.5;
    let map_y = (world_y / (QUARTER_TILE) - world_x / (HALF_TILE)) * 0.5;
    (map_x, map_y)
}

// takes world coordinates and converts them into map coordinates, taking elevation into account
pub fn world_to_map_iso(world_x: f32, world_y: f32, width: usize, height: usize, tile_map: &TileMap, tiles: &WriteStorage<Tile>) -> Option<(usize, usize)> {
    // get map coords of base tile
    let (base_x_f, base_y_f) = world_to_map_iso_simple(world_x, world_y - HALF_TILE);
    let (mut base_x, mut base_y) = (base_x_f.floor() as isize, base_y_f.floor() as isize);
    // determine if the mouse click is on the left or right of the tile using the following truth table:
    /*
    map_add  world_x | result
      even    even   |  right
      even    odd    |  left
      odd     even   |  left
      odd     odd    |  right
    */
    let map_even = (base_x + base_y) % 2 == 0;
    let world_even = (world_x / 32.).floor() as i32 % 2 == 0;
    let mut is_left = map_even ^ world_even;

    // get map coords of every possible tile down from the mouse click
    let wi = width as isize;
    let hi = height as isize;
    let mut tiles_in_column: Vec<(usize, usize, bool)> = vec![];
    while base_x < wi && base_y < hi {
        if tile_exists(base_x, base_y, wi, hi) {
            tiles_in_column.push((base_x as usize, base_y as usize, is_left));
        }
        if is_left {
            base_y += 1;
        } else {
            base_x += 1;
        }
        is_left = !is_left;
    }
    // get actual tiles and check if mouse is inside the tile
    for (x, y, is_left) in tiles_in_column.into_iter() {
        let entity = tile_map.get(x, y, width);
        if let Some(tile) = tiles.get(entity) {
            let (tile_world_x, tile_world_y) = map_to_world_iso(tile.x as f32, tile.y as f32, tile.elevation as f32, tile.height as f32 / 2.);
            let (above, below) = match is_left {
                true => (0.5, -0.5),
                false => (-0.5, 0.5),
            };
            let b = HALF_TILE;
            let test_x = world_x - tile_world_x;
            let test_y = world_y - tile_world_y;if test_y > below * test_x && test_y <= above * test_x + b {
                return Some((tile.x, tile.y));
            }
        }
    }
    None
}

// checks if a tile exists
pub fn tile_exists(x: isize, y: isize, width: isize, height: isize) -> bool {
    return x >= 0 && y >= 0 && x < width && y < height;
}

// finds the closest map coordinates when outside the map
pub fn closest_point_in_map_iso(map_x: f32, map_y: f32, w: f32, h: f32, e: f32) -> (f32, f32) {
    let origin = e * -1.;
    let (x, y) = if map_x < origin && map_y < origin {
        (origin, origin)
    } else if map_x < origin && map_y > h {
        (origin, h)
    } else if map_x > w && map_y < origin {
        (w, origin)
    } else if map_x > w && map_y > h {
        (w, h)
    } else if map_x < origin {
        (origin, map_y)
    } else if map_x > w {
        (w, map_y)
    } else if map_y < origin {
        (map_x, origin)
    } else if map_y > h {
        (map_x, h)
    } else {
        (map_x, map_y)
    };
    map_to_world_iso_simple(x, y)
}

#[allow(dead_code)]
pub enum TileLayer{
    Base, BaseUI,
    ObjectUL, ObjectUR, ObjectCU,
    ObjectCLU, ObjectCRU,
    ObjectLU, ObjectRU,
    ObjectC,
    ObjectLD, ObjectRD,
    ObjectCLD, ObjectCRD,
    ObjectDL, ObjectDR, ObjectCD,
    ObjectUI,
}

// generates a z value based on (x,y) and layer
pub fn z_value_iso(map_x: f32, map_y: f32, map_z: f32, layer: TileLayer) -> f32 {
    (map_x + map_y + map_z + tile_layer_value(layer)) * 0.001
}

pub fn tile_layer_value(layer: TileLayer) -> f32 {
    match layer {
        TileLayer::Base => 0.0,
        TileLayer::BaseUI => 0.05,
        TileLayer::ObjectUL | TileLayer::ObjectUR | TileLayer::ObjectCU => 0.1,
        TileLayer::ObjectCLU | TileLayer::ObjectCRU => 0.15,
        TileLayer::ObjectLU | TileLayer::ObjectRU => 0.2,
        TileLayer::ObjectC => 0.25,
        TileLayer::ObjectLD | TileLayer::ObjectRD => 0.3,
        TileLayer::ObjectCLD | TileLayer::ObjectCRD => 0.35,
        TileLayer::ObjectDL | TileLayer::ObjectDR | TileLayer::ObjectCD => 0.4,
        TileLayer::ObjectUI => 0.45,
    }
}