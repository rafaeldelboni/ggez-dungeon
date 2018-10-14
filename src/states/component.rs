use std::collections::{HashMap};
use specs::{Component, VecStorage};

use states::resources::{State, StateActions, StateCommandTypes, StateRenderable};

#[derive(Clone, Debug, Default)]
pub struct States {
    pub active: Vec<State>,
    pub actions: HashMap<StateActions, fn()->StateRenderable>,
}

impl Component for States {
    type Storage = VecStorage<Self>;
}

impl States {
    pub fn new(
        active: &StateActions,
        actions: HashMap<StateActions, fn()->StateRenderable>,
    ) -> States {
        let active_state = actions
            .get(active)
            .expect("Active not found in actions argument.")
            ();
        States {
            active: vec!{active_state.state},
            actions,
        }
    }

    pub fn get_action(&self, action: &StateActions) -> StateRenderable {
        self.actions
            .get(action)
            .expect("Action not found in actions list.")
            ()
    }

    pub fn current_action(&self) -> StateRenderable {
        let current = self.current().expect("No current state found.");
        self.actions
            .get(&current.action)
            .expect("Current action not found in actions list.")
            ()
    }

    pub fn start_action(&mut self, action: &StateActions) {
        let state_renderer = self.get_action(action);
        self.start(state_renderer.state)
    }

    pub fn current(&self) -> Option<&State> {
        self.active.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut State> {
        self.active.last_mut()
    }

    pub fn current_is_finished(&self) -> bool {
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
                    self.active.push(state);
                },
                (StateCommandTypes::Replace, false) => {
                    self.active.pop();
                    self.active.push(state);
                },
                _ => {},
            }
        }
    }

    pub fn stop(&mut self) {
        self.active.pop();
    }
}
