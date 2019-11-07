use amethyst::{
    core::{math::Vector3, Transform, Time},
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
    window::ScreenDimensions,
};
use crate::{
    resources::MapDimensions,
    util::{world_to_map_iso_simple, closest_point_in_map_iso}
};

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, MapDimensions>,
        Read<'s, Time>,
    );

    fn run(&mut self, (screen_dimensions, cameras, mut transforms, input_handler, map_dimensions, time): Self::SystemData) {
        // will be an option
        let mouse_scroll_sensitivity = 50.;
        let delta_time = time.delta_real_seconds();
        // will be an option
        let move_factor = 480. * delta_time;
        // for the only camera
        for (_, transform) in (&cameras, &mut transforms).join() {
            // find mouse coords
            let (mouse_x, mouse_y) = match input_handler.mouse_position() {
                Some((x, y)) => (x, y),
                _ => (screen_dimensions.width() * 0.5, screen_dimensions.height() * 0.5),
            };
            // move camera if mouse is at edge of screen or keys are pressed
            if input_handler.action_is_down("CameraMoveUp").unwrap()
                || mouse_y - mouse_scroll_sensitivity < 0. {
                transform.move_up(move_factor);
            }
            if input_handler.action_is_down("CameraMoveDown").unwrap()
                || mouse_y + mouse_scroll_sensitivity > screen_dimensions.height() {
                transform.move_down(move_factor);
            }
            if input_handler.action_is_down("CameraMoveLeft").unwrap()
                || mouse_x - mouse_scroll_sensitivity < 0.  {
                transform.move_left(move_factor);
            }
            if input_handler.action_is_down("CameraMoveRight").unwrap()
                || mouse_x + mouse_scroll_sensitivity > screen_dimensions.width()  {
                transform.move_right(move_factor);
            }
            // snap camera to edge of map
            let cam_coords: Vector3<f32> = *transform.translation();
            let (map_x, map_y) = world_to_map_iso_simple(cam_coords[0], -cam_coords[1]);
            let (width, height) = (map_dimensions.width as f32, map_dimensions.height as f32);
            if map_x < 0. || map_x > width
                || map_y < 0. || map_y > height {
                // need to figure out how to get highest elevation in this system
                let (new_x, new_y) = closest_point_in_map_iso(map_x, map_y, width, height, 0.);
                transform.set_translation_x(new_x);
                transform.set_translation_y(-new_y);
            }
        }
    }
}

pub struct WindowResizeSystem {
    last_dimensions: ScreenDimensions,
}

impl WindowResizeSystem {
    pub fn new() -> Self {
        Self {
            last_dimensions: ScreenDimensions::new(0, 0, 0.0),
        }
    }
}

impl<'s> System<'s> for WindowResizeSystem {
    type SystemData = (ReadExpect<'s, ScreenDimensions>, WriteStorage<'s, Camera>);

    fn run(&mut self, (screen_dimensions, mut cameras): Self::SystemData) {
//        if screen_dimensions.width() < 720. || screen_dimensions.height() < 480. {
//            screen_dimensions.update(720., 480.);
//        }
        // prevents the contents of the window from scaling with screen size
        if self.last_dimensions.width() == 0. || self.last_dimensions.height() == 0. {
            self.last_dimensions = screen_dimensions.clone();
        } else if self.last_dimensions != *screen_dimensions {
            for camera in (&mut cameras).join() {
                if let Some(ortho) = camera.projection_mut().as_orthographic_mut() {
                    ortho.set_bottom_and_top(screen_dimensions.height() * -0.5, screen_dimensions.height() * 0.5);
                    ortho.set_left_and_right(screen_dimensions.width() * -0.5, screen_dimensions.width() * 0.5);
                }
            }

            self.last_dimensions = screen_dimensions.clone();
        }
    }
}