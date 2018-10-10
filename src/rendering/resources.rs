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
