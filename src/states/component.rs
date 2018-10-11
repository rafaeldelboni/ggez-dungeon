use specs::{Component, VecStorage};

use states::resources::{State, StateCommandTypes};

#[derive(Debug, Default)]
pub struct States {
    pub list: Vec<State>,
}

impl Component for States {
    type Storage = VecStorage<Self>;
}

impl States {
    pub fn current(&self) -> Option<&State> {
        self.list.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut State> {
        self.list.last_mut()
    }

    fn current_is_finished(&self) -> bool {
        match self.current() {
            Some (current) => {
                match current.duration_ticks {
                    Some (duration) => duration >= current.executed_ticks,
                    None => false
                }
            }
            None => true
        }
    }

    pub fn start(&mut self, state: State) {
        let current_interruptible =
            if let Some(current) = self.current() {
                current.interruptible
            } else {
                true
            };

        match (&state.onstart_cmd_type, current_interruptible) {
            (StateCommandTypes::Push, true) => { 
                self.list.push(state);
            },
            (StateCommandTypes::Replace, true) => {
                self.list.pop();
                self.list.push(state);
            },
            _ => {},
        }
    }

    pub fn stop(&mut self) {
        self.list.pop();
    }

    pub fn update(&mut self) {
        if self.list.len() > 0 {
            if self.current_is_finished() {
                self.stop()
            }

            match self.current_mut() {
                Some (current) => { current.executed_ticks += 1; },
                None => {}
            }
        }
    }
}
