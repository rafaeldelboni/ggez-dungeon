use specs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct SnapCamera;

impl Component for SnapCamera {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default)]
pub struct ChaseCamera;

impl Component for ChaseCamera {
    type Storage = NullStorage<Self>;
}
