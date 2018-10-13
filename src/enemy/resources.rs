use states::resources::{State, StateActions, StateCommandTypes};
use rendering::resources::{RenderableClass, RenderableState};

pub fn enemy_idle() -> RenderableState {
    RenderableState {
       state: State::new(
           StateActions::Idle, None, StateCommandTypes::Replace, false
       ),
       renderable: RenderableClass::new_animation("warrior_idle", 10., 10.)
    }
}

pub fn enemy_walk() -> RenderableState {
    RenderableState {
        state: State::new(
            StateActions::Walk, None, StateCommandTypes::Push, false
        ),
        renderable: RenderableClass::new_animation("warrior_walk", 10., 10.)
    }
}
