use chrono::prelude::{DateTime, Local};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StateActions {
    Idle,
    Walk,
    Attack
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StateCommandTypes {
    Push,
    Pop,
    Replace
}

#[derive(Debug)]
pub struct State {
    pub action: StateActions,
    pub duration_millis: i64,
    pub executed: DateTime<Local>,
    pub onstop_cmd_type: StateCommandTypes,
    pub onstart_cmd_type: StateCommandTypes,
    pub interruptible: bool
}

pub trait StateCommands<TYPE=Self> {
    fn start(&mut self, current_state: TYPE);
    fn stop(&mut self, next_state: TYPE);
    fn update(&mut self);
    fn is_finished(&self) -> bool;
}
