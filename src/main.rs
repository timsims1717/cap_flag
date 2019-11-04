use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use crate::systems::CameraSystem;

mod states;
mod map;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let config = app_root.join("config");
    let display_config = config.clone().join("display.ron");
    let input_config = config.clone().join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&input_config)?,
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0., 0., 0., 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(CameraSystem, "camera", &[]);

    let mut game = Application::new(resources, states::MapEditorState, game_data)?;
    game.run();

    Ok(())
}