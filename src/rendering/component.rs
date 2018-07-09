use specs::{Component, VecStorage};

#[derive(Debug, Copy, Clone)]
pub enum RenderableClass {
    Animation {
        id: &'static str,
        frame: f32,
        speed: f32,
        length: f32,
    },
    Image { id: &'static str },
}

#[derive(Debug)]
pub struct Renderable {
    pub layer: usize,
    pub class: RenderableClass,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}
