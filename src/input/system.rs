use input::{Controlable, Input};
use physics::{Velocity};
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct ControlableSystem;

impl<'a> System<'a> for ControlableSystem {
    type SystemData = (
        Read<'a, Input>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Controlable>
    );

    fn run(&mut self, (input, mut vel, ctrled): Self::SystemData) {
        (&mut vel, &ctrled).join().for_each(|(vel, _ctrled)| {
            if input.left {
                vel.vector.x = -50.;
            } else if input.right {
                vel.vector.x = 50.;
            } else {
                vel.vector.x = 0.;
            }
            if input.up {
                vel.vector.y = 50.;
            } else if input.down {
                vel.vector.y = -50.;
            } else {
                vel.vector.y = 0.;
            }
        });
    }
}
