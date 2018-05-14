use ggez::graphics;
use ggez::graphics::{Image};
use ggez::{Context, GameResult};

pub struct Assets {
    pub warrior: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            warrior: Image::new(ctx, "/warrior.png").unwrap()
        })
    }
}
