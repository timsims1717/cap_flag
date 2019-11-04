use amethyst::{
    core::{Transform, Time},
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
};

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut transforms, input_handler, time): Self::SystemData) {
        let delta_time = time.delta_real_seconds();
        let move_factor = 480. * delta_time;
        for (_, transform) in (&cameras, &mut transforms).join() {
            if input_handler.action_is_down("CameraMoveUp").unwrap() {
                transform.move_up(move_factor);
            }
            if input_handler.action_is_down("CameraMoveDown").unwrap() {
                transform.move_down(move_factor);
            }
            if input_handler.action_is_down("CameraMoveLeft").unwrap() {
                transform.move_left(move_factor);
            }
            if input_handler.action_is_down("CameraMoveRight").unwrap() {
                transform.move_right(move_factor);
            }
        }
    }
}