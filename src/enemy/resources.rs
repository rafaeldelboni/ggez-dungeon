use states::resources::{State, StateActions, StateCommandTypes, StateRenderable};
use rendering::resources::{RenderableClass};

pub fn enemy_idle() -> StateRenderable {
    StateRenderable {
       state: State::new(
           StateActions::Idle, None, StateCommandTypes::Replace, false
       ),
       renderable: RenderableClass::new_animation("warrior_idle", 10., 10.)
    }
}

pub fn enemy_walk() -> StateRenderable {
    StateRenderable {
        state: State::new(
            StateActions::Walk, None, StateCommandTypes::Push, false
        ),
        renderable: RenderableClass::new_animation("warrior_walk", 10., 09.)
    }
}
