use input::Input;
use position::{Velocity};
use specs::{Join, Read, System, WriteStorage};

pub struct ControlableSystem;

impl<'a> System<'a> for ControlableSystem {
    type SystemData = (Read<'a, Input>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (input, mut vel): Self::SystemData) {
        (&mut vel).join().for_each(|vel| {
            if input.left {
                vel.x = -10.;
            } else if input.right {
                vel.x = 10.;
            } else {
                vel.x = 0.;
            }
            if input.up {
                vel.y = 10.;
            } else if input.down {
                vel.y = -10.;
            } else {
                vel.y = 0.;
            }
        });
    }
}
