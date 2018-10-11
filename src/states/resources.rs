#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateActions {
    Idle,
    Walk,
    Attack
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateCommandTypes {
    Push,
    Pop,
    Replace
}

#[derive(Clone, Debug)]
pub struct State {
    pub action: StateActions,
    pub duration_ticks: Option<i64>,
    pub executed_ticks: i64,
    pub onstart_cmd_type: StateCommandTypes,
    pub interruptible: bool
}

impl State {
    fn new(
        action: StateActions,
        duration_ticks: Option<i64>,
        onstart_cmd_type: StateCommandTypes,
        interruptible: bool
    ) -> State {
        State {
            action,
            duration_ticks,
            executed_ticks: 0,
            onstart_cmd_type,
            interruptible
        }
    }
}
