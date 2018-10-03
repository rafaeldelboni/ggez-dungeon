use ggez::graphics::{Point2};
use specs::{Join, Read, System, ReadStorage, Write, WriteStorage};

use camera::{Camera, ChaseCamera, SnapCamera};
use physics::{Position};

pub struct SnapCameraSystem ;

impl<'a> System<'a> for SnapCameraSystem {
    type SystemData = (
        Write<'a, Camera>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, SnapCamera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, position, snap) = data;

        for (pos, _) in (&position, &snap).join() {
            camera.move_to(Point2::new(pos.x as f32, pos.y as f32));
        }
    }
}

pub struct ChaseCameraSystem;

impl<'a> System<'a> for ChaseCameraSystem {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, ChaseCamera>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cam, chase, mut pos) = data;

        for (pos, _) in (&mut pos, &chase).join() {
            let loc = cam.location();
            pos.x = loc.x as f32;
            pos.y = loc.y as f32;
        }
    }
}
