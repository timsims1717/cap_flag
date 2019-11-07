use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::base::Vector3, transform::Transform},
    ecs::prelude::Entity,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
//    ui::{Anchor, UiTransform},
    window::ScreenDimensions,
};

use crate::{
    components::Tile,
    resources::{
        CameraHandle,
        load_test_map,
        load_terrain_pack,
        Map, MapDimensions,
        TerrainSet,
        TerrainSprites,
        TileMap,
        UISprites
    },
    util::{map_to_world_iso_simple, map_to_world_iso, TILE_SIZE, TileLayer, z_value_iso},
};
use log::info;

pub struct MapEditorState;

impl SimpleState for MapEditorState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // load map from disc (test map right now, will be loaded in loading state I think)
        let mut map = load_test_map().unwrap();
        // get map dimensions, save as resource
        world.insert(map.dimensions.clone());
        let camera = init_camera(world, &dimensions, &map.dimensions);
        // save camera as resource
        world.insert(CameraHandle{camera});
        let terrain = load_terrain_pack(map.terrain_file.clone()).unwrap();
        // save terrain pack as resource
        world.insert(terrain.clone());
        let terrain_sprites = load_terrain_textures(world, &terrain);
        // save terrain sprites as resource
        world.insert(TerrainSprites{ set: terrain_sprites.clone() });
        let ui_sprites = load_ui_textures(world);
        // save ui sprites as resource
        world.insert(UISprites { set: ui_sprites.clone() });
        let tile_map = init_map(world, &mut map, &terrain, &terrain_sprites, &dimensions);
        // save set of tiles as resource
        world.insert(tile_map);

//        init_editor_panel(world, &ui_sprites);
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

fn init_camera(world: &mut World, dimensions: &ScreenDimensions, map_dimensions: &MapDimensions) -> Entity {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let (offset_x, offset_y) = map_to_world_iso_simple(map_dimensions.width as f32 / 2., map_dimensions.height as f32 / 2.);
    let mut transform = Transform::default();
    // reverse y to put origin at top of map
    transform.set_translation_xyz(offset_x, -offset_y, 1.);

    world.create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build()
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

pub fn load_ui_textures(world: &mut World) -> Vec<SpriteRender> {
    // ui textures
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/ui_sprites.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/ui_sprites.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    (0..1)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn init_map(world: &mut World, map: &mut Map, terrain: &TerrainSet, tile_sprites: &[SpriteRender], dimensions: &ScreenDimensions) -> TileMap {
    // initialize tiles (this will be done in loading state)
    map.build_tiles(terrain);
    let mut tile_map = TileMap { v: vec![] };
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            let (world_x, world_y) = map_to_world_iso(x as f32, y as f32, t.elevation as f32, t.height as f32);

            let mut transform = Transform::default();
            let scalar = TILE_SIZE / terrain.tile_size as f32;
            transform.set_scale(Vector3::new(scalar, scalar, 0.));
            transform.set_translation_xyz(world_x, -world_y, z_value_iso(x as f32, y as f32, 0., TileLayer::Base));

            tile_map.v.insert((y * map.dimensions.width) + x, world
                .create_entity()
                .with(t.clone())
                .with(tile_sprites[t.sprite_index].clone())
                .with(transform)
                .build()
            );
        }
    }
    tile_map
}

//fn init_editor_panel(world: &mut World, ui_sprites: &[SpriteRender]) {
//    let transform = UiTransform::new(
//        format!("terrain_panel_{}", 0),
//        Anchor::TopLeft, Anchor::TopLeft,
//        0.,0.,1.,32.,32.
//    );
//    world
//        .create_entity()
//        .with(ui_sprites[1].clone())
//        .with(transform)
//        .build();
//}
