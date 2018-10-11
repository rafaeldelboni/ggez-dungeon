use specs::{Join, Read, System, WriteStorage};

use physics::resources::{UpdateTime};
use states::component::{States};

pub struct StatesSystem;

impl<'a> System<'a> for StatesSystem {
    type SystemData = (
        Read<'a, UpdateTime>,
        WriteStorage<'a, States>,
    );

    fn run(&mut self, (_delta, mut states): Self::SystemData) {
        (&mut states).join().for_each(|state| {
            state.update();
        })
    }
}

