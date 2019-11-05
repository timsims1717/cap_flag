use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::base::Vector3, transform::Transform},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use crate::resources::{load_test_map, load_terrain_pack, Map, map_to_world_iso, TerrainSet, MapDimensions};
use log::info;

pub struct MapEditorState;

impl SimpleState for MapEditorState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();


        let mut map = load_test_map().unwrap();
        init_camera(world, &dimensions, &map.dimensions);
        world.insert(map.dimensions.clone());
        let terrain = load_terrain_pack(map.terrain_file.clone()).unwrap();
        let tile_sprites = load_terrain_textures(world, &terrain);
        init_map(world, &mut map, &terrain, &tile_sprites, &dimensions);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Window Resizing?


            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions, map_dimensions: &MapDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let (offset_x, offset_y) = map_to_world_iso(map_dimensions.width as f32 / 2., map_dimensions.height as f32 / 2.);
    let mut transform = Transform::default();
    transform.set_translation_xyz(offset_x, -offset_y, 1.);

    world.create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

pub fn load_terrain_textures(world: &mut World, terrain: &TerrainSet) -> Vec<SpriteRender> {
    // tile textures
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("terrain/{}.{}", terrain.texture_file, terrain.texture_format),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // spritesheet definition for textures
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("terrain/{}.ron", terrain.texture_file),
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    (0..terrain.num_tiles)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn init_map(world: &mut World, map: &mut Map, terrain: &TerrainSet, tile_sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    map.build_tiles(terrain);
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            let (world_x, world_y) = map_to_world_iso(x as f32, y as f32);

            let mut transform = Transform::default();
            let scalar = 64. / terrain.tile_size as f32;
            transform.set_scale(Vector3::new(scalar, scalar, 0.));
            transform.set_translation_xyz(world_x, -world_y, (y as f32 + x as f32) * 0.001);

            world
                .create_entity()
                .with(tile_sprites[t.index].clone())
                .with(transform)
                .build();
        }
    }
}

