use std::time;

use ggez::{Context, GameResult};
use ggez::graphics::{
    spritebatch::SpriteBatch, Rect, Image, Vector2, Point2, draw_ex, DrawParam
};

use controls::Controls;

pub struct Player {
    pub position: Point2,
    pub velocity: Vector2,
    pub scale: Point2,
    pub sprite: SpriteBatch,
    pub controls: Controls
}

impl Player {
    pub fn new(context: &mut Context) -> Player {
        let image = Image::new(context, "/warrior.png").unwrap();
        let controls = Controls {
            direction: Vector2::new(0., 0.),
            attack: 0.
        };

        Player {
            position: Point2::new(0.25, 0.25),
            velocity: Vector2::new(20., 20.),
            scale: Point2::new(4.0, 4.0),
            sprite: SpriteBatch::new(image),
            controls: controls
        }
    }

    pub fn update(&mut self, delta_time: time::Duration) -> GameResult<()> {
        let delta = delta_time.subsec_nanos() as f32/1e8;

        let velocity = Vector2::new(
            delta * self.velocity.x * self.controls.direction.x,
            delta * self.velocity.y * self.controls.direction.y
        );

        self.position += velocity;

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let batch_param = DrawParam {
            src: Rect::new(0.0, 0.0, 0.1, 0.1),
            dest: self.position,
            scale: self.scale,
            ..Default::default()
        };
        self.sprite.add(batch_param);

        let image_param = DrawParam {
            ..Default::default()
        };
        draw_ex(ctx, &self.sprite, image_param)?;
        self.sprite.clear();

        Ok(())
    }
}
