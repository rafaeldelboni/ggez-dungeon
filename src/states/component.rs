use specs::{Component, VecStorage};

use rendering::resources::{RenderableState};
use states::resources::{State, StateCommandTypes};

#[derive(Clone, Debug, Default)]
pub struct States {
    pub list: Vec<State>,
    pub idle: Option<fn()->RenderableState>,
    pub walk: Option<fn()->RenderableState>,
}

impl Component for States {
    type Storage = VecStorage<Self>;
}

impl States {
    pub fn new(
        idle: Option<fn()->RenderableState>,
        walk: Option<fn()->RenderableState>,
    ) -> States {
        States {
            list: vec!(idle.unwrap()().state),
            idle,
            walk,
        }
    }

    pub fn current(&self) -> Option<&State> {
        self.list.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut State> {
        self.list.last_mut()
    }

    fn current_is_finished(&self) -> bool {
        match self.current() {
            Some (current) => {
                match current.duration_secs {
                    Some (duration) => duration >= current.executed_secs,
                    None => false
                }
            }
            None => true
        }
    }

    pub fn start(&mut self, state: State) {
        let should_start =
            if let Some(current) = self.current() {
                current.duration_secs.is_none()
                    && current.action != state.action
            } else {
                true
            };

        if should_start {
            let current_interruptible =
                if let Some(current) = self.current() {
                    current.interruptible
                } else {
                    true
                };

            match (&state.onstart_cmd_type, current_interruptible) {
                (StateCommandTypes::Push, false) => { 
                    self.list.push(state);
                },
                (StateCommandTypes::Replace, false) => {
                    self.list.pop();
                    self.list.push(state);
                },
                _ => {},
            }
        }
    }

    pub fn stop(&mut self) {
        self.list.pop();
    }

    pub fn update(&mut self, delta_seconds: f32) {
        if !self.list.is_empty() {
            if self.current_is_finished() {
                self.stop()
            }

            if let Some (current) = self.current_mut() { 
                if current.duration_secs.is_some() {
                    current.executed_secs += delta_seconds;
                }
                println!("{:?}", current);
            }
        }
    }
}
