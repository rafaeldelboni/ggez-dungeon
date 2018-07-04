use serde_json;

use ggez::graphics::{Image};
use ggez::{Context, GameResult};
use spritesheet_generator::spritesheet::Spritesheet;

pub struct Assets {
    pub spritesheet_image: Image,
    pub spritesheet_data: Spritesheet,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let spritesheet_data_file = ctx.filesystem.open("spritesheet.json")
            .unwrap();
        let spritesheet_data: Spritesheet = serde_json::from_reader(
            spritesheet_data_file
        ).unwrap();

        Ok(Assets {
            spritesheet_image: Image::new(ctx, "/spritesheet.png").unwrap(),
            spritesheet_data
        })
    }
}
