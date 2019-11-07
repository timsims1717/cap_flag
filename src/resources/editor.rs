use amethyst::{
    ecs::Entity,
    renderer::SpriteRender,
};
use crate::components::TerrainTile;

pub enum EditorMode {
    None,
    Terrain(TerrainTile)
}

pub const EDITOR_WIDTH: f32 = 160.;
//pub const MODE_SELECT_HEIGHT: f32 = 256.;

pub struct EditorPanel {
    pub terrain_panel: TerrainPanel,
}

pub struct TerrainPanel {
    pub background: Vec<Entity>,
    pub border: Vec<Entity>,
}