use amethyst::{
    core::Transform,
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::SpriteRender,
};

use crate::components::{TileUIElementType, TileUIElement};
use crate::util::{map_to_world_iso, z_value_iso, TileLayer, TILE_SIZE};

// creates a ui element for a tile
pub fn create_tile_ui(
    entities: &Entities,
    sprite: SpriteRender,
    tile_height: usize,
    tile_x: usize,
    tile_y: usize,
    tile_elevation: usize,
    el_type: TileUIElementType,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let ui_entity = entities.create();
    let (world_x, world_y) = map_to_world_iso(tile_x as f32, tile_y as f32, tile_elevation as f32, TILE_SIZE * 0.25);
    let mut transform = Transform::default();
    transform.set_translation_xyz(world_x, -world_y, z_value_iso(tile_x as f32, tile_y as f32, 0., TileLayer::BaseUI));

    let ui_element = TileUIElement {
        tile_x,
        tile_y,
        el_type,
    };

    lazy_update.insert(ui_entity, ui_element);
    lazy_update.insert(ui_entity, transform);
    lazy_update.insert(ui_entity, sprite);
}