use amethyst::{
    ecs::Entity,
    renderer::SpriteRender,
};

pub struct UISprites {
    pub set: Vec<SpriteRender>
}

pub struct CameraHandle {
    pub camera: Entity
}