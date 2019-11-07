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
    util::mouse_to_map_iso,
};

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
        if let Some((xf, yf)) = input_handler.mouse_position() {
            // todo: ignore if inside editor panel or if in menu
            // translate the mouse coordinates to map coordinates
            let camera_transform = transforms.get(camera_handle.camera).unwrap();
            let camera = cameras.get(camera_handle.camera).unwrap();
            if let Some((map_x, map_y)) = mouse_to_map_iso(xf, yf, &map_dimensions,&screen_dimensions, camera, camera_transform, &tile_map.clone(), &tiles) {
                // todo: this is where more tiles can be highlighted
                let tile_ui_need: Vec<(usize, usize)> = vec![(map_x, map_y)];
                // go through all ui_tiles, if they aren't needed, remove them, if they are, add them to "found"
                let mut tile_ui_found: Vec<(usize, usize)> = vec![];
                for (entity, tile_ui) in (&*entities, &mut tiles_ui).join() {
                    if tile_ui.tile_x != map_x || tile_ui.tile_y != map_y {
                        entities.delete(entity);
                    } else {
                        tile_ui_found.push((tile_ui.tile_x, tile_ui.tile_y));
                    }
                }
                for (x, y) in tile_ui_need.iter() {
                    // unless the ui_tile already exists in found ...
                    if !(tile_ui_found.iter().any(|(ix, iy)| ix == x && iy == y)) {
                        let parent = tile_map.get(*x, *y, map_dimensions.width);
                        if let Some(tile) = tiles.get(parent) {
                            create_tile_ui(&entities, ui_sprites.set[0].clone(), tile.height, *x, *y, tile.elevation, TileUIElementType::EditorMouseOver, &lazy_update);
                        }
                    }
                }
            } else {
                // if mouse is outside the map
                for (entity, _) in (&*entities, &mut tiles_ui).join() {
                    entities.delete(entity);
                }
            }
        };
    }
}