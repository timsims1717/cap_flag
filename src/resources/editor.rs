use crate::components::TerrainTile;

pub enum EditorMode {
    None,
    Terrain(TerrainTile)
}