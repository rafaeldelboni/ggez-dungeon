use specs::{Join, Read, System, WriteStorage};

use physics::resources::{UpdateTime};
use states::component::{States};

pub struct StatesSystem;

impl<'a> System<'a> for StatesSystem {
    type SystemData = (
        Read<'a, UpdateTime>,
        WriteStorage<'a, States>,
    );

    fn run(&mut self, (delta, mut states): Self::SystemData) {
        (&mut states).join().for_each(|state| {
            if !state.active.is_empty() {
                if state.current_is_finished() {
                    state.stop()
                }

                if let Some (current) = state.current_mut() { 
                    if current.duration_secs.is_some() {
                        current.executed_secs += delta.seconds;
                    }
                }
            }
        })
    }
}

