use ggez::graphics::{draw_ex, DrawParam, Rect, Point2};
use ggez::{Context};

use specs::{System, Write};

use assets::Assets;

pub struct RenderingSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderingSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> RenderingSystem<'c> {
        RenderingSystem { ctx }
    }
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = (
        Option<Write<'a, Assets>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let assets = data;
        let spritesheet = assets.0.unwrap();
        let frame = &spritesheet
            .spritesheet_data
            .frames
            .get("warrior_die_08")
            .unwrap()
            .screen
            .clone();

        let image_param = DrawParam {
            src: Rect::new(frame.x, frame.y, frame.w, frame.h),
            dest: Point2::new(100., 100.),
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(4.0, 4.0),
            shear: Point2::new(1./1e4, 1./1e4),
            ..Default::default()
        };
        draw_ex(self.ctx, &spritesheet.spritesheet_image, image_param).unwrap();
    }
}
