use amethyst::{
    core::{math::{Point3, Vector2, Vector3}, Transform, Time},
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::{camera::Camera, SpriteRender},
    window::ScreenDimensions,
};
use std::collections::HashMap;
use crate::{
    components::{Tile, TileUIElement, TileUIElementType},
    enitities::{create_tile_ui},
    resources::{CameraHandle, MapDimensions, TerrainSet, TerrainSprites, TileMap, UISprites},
    util::{map_to_world_iso_simple, mouse_to_map_iso_simple, world_to_map_iso_simple},
};
use crate::util::mouse_to_world_iso_simple;

pub struct EditorTileSystem;

impl<'s> System<'s> for EditorTileSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, TileUIElement>,
        WriteStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, MapDimensions>,
        ReadExpect<'s, CameraHandle>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, TerrainSet>,
        ReadExpect<'s, TerrainSprites>,
        ReadExpect<'s, TileMap>,
        ReadExpect<'s, UISprites>,
    );

    fn run(&mut self, (
        entities,
        mut tiles_ui,
        mut tiles,
        transforms,
        cameras,
        input_handler,
        map_dimensions,
        camera_handle,
        lazy_update,
        screen_dimensions,
        terrain_set,
        terrain_sprites,
        tile_map,
        ui_sprites,
    ): Self::SystemData) {
        if let Some((x, y)) = input_handler.mouse_position() {
            let camera_transform = transforms.get(camera_handle.camera).unwrap();
            let camera = cameras.get(camera_handle.camera).unwrap();
            let (world_x, world_y) = mouse_to_world_iso_simple(x, y, &screen_dimensions, camera, camera_transform);
            let (map_x, map_y) = mouse_to_map_iso_simple(x, y, &screen_dimensions, camera, camera_transform);
            let (map_x_i, map_y_i) = (map_x.floor() as isize, map_y.floor() as isize);
            if map_x_i >= 0 && map_x_i < map_dimensions.width as isize
                && map_y_i >= 0 && map_y_i < map_dimensions.height as isize {
                let (map_x_u, map_y_u) = (map_x_i as usize, map_y_i as usize);
                let tile_ui_need: Vec<(usize, usize)> = vec![(map_x_u, map_y_u)];
                let mut tile_ui_found: Vec<(usize, usize)> = vec![];
                for (entity, tile_ui) in (&*entities, &mut tiles_ui).join() {
                    if tile_ui.tile_x != map_x_u || tile_ui.tile_y != map_y_u {
                        entities.delete(entity);
                    } else {
                        tile_ui_found.push((tile_ui.tile_x, tile_ui.tile_y));
                    }
                }
                for (x, y) in tile_ui_need.iter() {
                    if !(tile_ui_found.iter().any(|(ix, iy)| ix == x && iy == y)) {
                        let parent = tile_map.v[y * map_dimensions.width + x];
                        if let Some(tile) = tiles.get(parent) {
                            create_tile_ui(&entities, ui_sprites.set[0].clone(), tile.height, *x, *y, tile.elevation, TileUIElementType::EditorMouseOver, &lazy_update);
                        }
                    }
                }
            } else {
                for (entity, _) in (&*entities, &mut tiles_ui).join() {
                    entities.delete(entity);
                }
            }
        };
    }
}