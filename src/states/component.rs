use specs::{Component, VecStorage};

use states::resources::{State};

#[derive(Debug)]
pub struct States {
    pub list: Vec<State>,
}

impl Component for States {
    type Storage = VecStorage<Self>;
}
