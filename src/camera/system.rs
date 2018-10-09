use ggez::graphics::{Point2};
use specs::{Join, Read, System, ReadStorage, Write, WriteStorage};

use camera::Camera;
use camera::component::{ChaseCamera, SnapCamera};
use rendering::component::{Sprite};

pub struct SnapCameraSystem ;

impl<'a> System<'a> for SnapCameraSystem {
    type SystemData = (
        Write<'a, Camera>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, SnapCamera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, sprite, snap) = data;

        for (sprite, _) in (&sprite, &snap).join() {
            camera.move_to(
                Point2::new(sprite.position.x as f32, sprite.position.y as f32)
            );
        }
    }
}

pub struct ChaseCameraSystem;

impl<'a> System<'a> for ChaseCameraSystem {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, ChaseCamera>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cam, chase, mut sprite) = data;

        for (sprite, _) in (&mut sprite, &chase).join() {
            let cam_location = cam.location();
            sprite.position.x = cam_location.x as f32;
            sprite.position.y = cam_location.y as f32;
        }
    }
}
