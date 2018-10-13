use states::resources::{State, StateActions, StateCommandTypes};

pub fn enemy_idle() -> State {
    State::new(
        StateActions::Idle, None, StateCommandTypes::Replace, true
    )
}

pub fn enemy_walk() -> State {
    State::new(
        StateActions::Walk, None, StateCommandTypes::Push, false
    )
}
