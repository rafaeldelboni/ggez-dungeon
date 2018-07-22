use position::{Position, Velocity};
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct PositionSystem;

impl<'a> System<'a> for PositionSystem {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        (&vel, &mut pos).join().for_each(|(vel, pos)| {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
        });
    }
}

