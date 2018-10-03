use input::Input;
use physics::{Velocity};
use specs::{Join, Read, System, WriteStorage};

pub struct ControlableSystem;

impl<'a> System<'a> for ControlableSystem {
    type SystemData = (Read<'a, Input>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (input, mut vel): Self::SystemData) {
        (&mut vel).join().for_each(|vel| {
            if input.left {
                vel.vector.x = -10.;
            } else if input.right {
                vel.vector.x = 10.;
            } else {
                vel.vector.x = 0.;
            }
            if input.up {
                vel.vector.y = 10.;
            } else if input.down {
                vel.vector.y = -10.;
            } else {
                vel.vector.y = 0.;
            }
        });
    }
}
