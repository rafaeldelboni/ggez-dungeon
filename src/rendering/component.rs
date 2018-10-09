use nalgebra::{Vector2};
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

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub position: Vector2<f32>,
    pub direction: Vector2<f32>,
    pub scale: Vector2<f32>,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

impl Sprite {
    pub fn pull(&mut self, new: Vector2<f32>) {
        self.direction.x = match new.x - self.position.x {
            diff_x if diff_x > 0. => 1.,
            diff_x if diff_x < 0. => -1.,
            _ => self.direction.x,
        };
        self.direction.y = match new.y - self.position.y {
            diff_y if diff_y > 0. => -1.,
            diff_y if diff_y < 0. => 1.,
            _ => self.direction.y,
        };
        self.position = new;
    }
}
