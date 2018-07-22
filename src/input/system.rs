use input::Input;
use position::{Velocity};
use specs::{Join, Read, System, WriteStorage};

pub struct ControlableSystem;

impl<'a> System<'a> for ControlableSystem {
    type SystemData = (Read<'a, Input>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (input, mut vel): Self::SystemData) {
        (&mut vel).join().for_each(|vel| {
            match *input {
                Input { left, .. } if left == true => vel.x = -10.,
                Input { right, .. } if right == true => vel.x = 10.,
                Input { up, .. } if up == true => vel.y = -10.,
                Input { down, .. } if down == true => vel.y = 10.,
                _ => { 
                    vel.x = 0.;
                    vel.y = 0.;
                },
            }
        });
    }
}
