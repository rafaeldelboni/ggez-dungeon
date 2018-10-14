use rendering::resources::{RenderableClass};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateActions {
    Idle,
    Walk,
    Attack
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateCommandTypes {
    Push,
    Replace
}

#[derive(Clone, Debug)]
pub struct State {
    pub action: StateActions,
    pub duration_secs: Option<f32>,
    pub executed_secs: f32,
    pub onstart_cmd_type: StateCommandTypes,
    pub interruptible: bool
}

impl State {
    pub fn new(
        action: StateActions,
        duration_secs: Option<f32>,
        onstart_cmd_type: StateCommandTypes,
        interruptible: bool
    ) -> State {
        State {
            action,
            duration_secs,
            executed_secs: 0.0,
            onstart_cmd_type,
            interruptible
        }
    }
}

#[derive(Clone, Debug)]
pub struct StateRenderable {
    pub state: State,
    pub renderable: RenderableClass
}
