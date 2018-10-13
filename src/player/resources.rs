use states::resources::{State, StateActions, StateCommandTypes};

pub fn player_idle() -> State {
    State::new(
        StateActions::Idle, None, StateCommandTypes::Replace, false
    )
}

pub fn player_walk() -> State {
    State::new(
        StateActions::Walk, None, StateCommandTypes::Push, false
    )
}
