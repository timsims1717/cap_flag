use amethyst::{
    core::{
        math::{Point3,Vector2},
        Transform,
    },
    renderer::camera::Camera,
    window::ScreenDimensions,
};

pub const TILE_SIZE: f32 = 64.;

pub fn mouse_to_world_iso_simple(mouse_x: f32, mouse_y: f32, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform) -> (f32, f32) {
    let diagonal = Vector2::new(screen_dim.width(), screen_dim.height());
    let world_point = camera.projection().screen_to_world_point(
        Point3::new(mouse_x, mouse_y, 0.),
        diagonal,
        camera_transform,
    );
    (world_point.x, -world_point.y)
}

pub fn mouse_to_map_iso_simple(mouse_x: f32, mouse_y: f32, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform) -> (f32, f32) {
    let diagonal = Vector2::new(screen_dim.width(), screen_dim.height());
    let world_point = camera.projection().screen_to_world_point(
        Point3::new(mouse_x, mouse_y, 0.),
        diagonal,
        camera_transform,
    );
    world_to_map_iso_simple(world_point.x, -world_point.y)
}

pub fn map_to_world_iso_simple(map_x: f32, map_y: f32) -> (f32, f32) {
    let world_x = (map_x - map_y) * TILE_SIZE * 0.5;
    let world_y = (map_x + map_y) * TILE_SIZE * 0.25;
    (world_x, world_y)
}

pub fn map_to_world_iso(map_x: f32, map_y: f32, elevation: f32, height: f32) -> (f32, f32) {
    let world_x = (map_x - map_y) * TILE_SIZE * 0.5;
    let world_y = ((map_x + map_y) * TILE_SIZE * 0.25) - (elevation * TILE_SIZE * 0.5) + height;
    (world_x, world_y)
}

pub fn world_to_map_iso_simple(world_x: f32, world_y: f32) -> (f32, f32) {
    let map_x = (world_x / (TILE_SIZE * 0.5) + world_y / (TILE_SIZE * 0.25)) * 0.5;
    let map_y = (world_y / (TILE_SIZE * 0.25) - world_x / (TILE_SIZE * 0.5)) * 0.5;
    (map_x, map_y)
}

pub fn world_to_map_iso(world_x: f32, world_y: f32) -> (f32, f32) {
    let map_x = (world_x / (TILE_SIZE * 0.5) + world_y / (TILE_SIZE * 0.25)) * 0.5;
    let map_y = (world_y / (TILE_SIZE * 0.25) - world_x / (TILE_SIZE * 0.5)) * 0.5;
    (map_x, map_y)
}

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

pub fn mouse_to_world_iso(mouse_x: f32, mouse_y: f32, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform) -> (f32, f32) {
    let diagonal = Vector2::new(screen_dim.width(), screen_dim.height());
    let world_point = camera.projection().screen_to_world_point(
        Point3::new(mouse_x, mouse_y, 0.),
        diagonal,
        camera_transform,
    );
    (world_point.x, -world_point.y)
}

pub fn mouse_to_map_iso(mouse_x: f32, mouse_y: f32, screen_dim: &ScreenDimensions, camera: &Camera, camera_transform: &Transform) -> (f32, f32) {
    let diagonal = Vector2::new(screen_dim.width(), screen_dim.height());
    let world_point = camera.projection().screen_to_world_point(
        Point3::new(mouse_x, mouse_y, 0.),
        diagonal,
        camera_transform,
    );
    world_to_map_iso(world_point.x, -world_point.y)
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