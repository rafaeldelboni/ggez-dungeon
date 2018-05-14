use std::time;

use ggez::{Context, GameResult};
use ggez::graphics::{Rect, Image, Vector2, Point2, draw_ex};

use actors::states::ActorStates;
use animations::*;
use controls::Controls;

pub struct Player {
    pub state: ActorStates,
    pub position: Point2,
    pub direction: Point2,
    pub velocity: Vector2,
    pub scale: Point2,
    pub controls: Controls,
    pub animations: Animations,
}

impl Player {
    pub fn new(_context: &mut Context) -> GameResult<Player> {
        let controls = Controls {
            direction: Vector2::new(0., 0.),
            attack: 0.
        };
        let animations: Animations = vec!(
            (ActorStates::Idle,
             Animation::new(
                 10.,
                 Point2::new(0.5, 0.5),
                 vec!(
                     Rect::new(0.0, 0.0, 0.1, 0.1),
                     Rect::new(0.1, 0.0, 0.1, 0.1),
                     Rect::new(0.2, 0.0, 0.1, 0.1),
                     Rect::new(0.3, 0.0, 0.1, 0.1),
                     Rect::new(0.4, 0.0, 0.1, 0.1),
                     Rect::new(0.5, 0.0, 0.1, 0.1),
                     Rect::new(0.6, 0.0, 0.1, 0.1),
                     Rect::new(0.7, 0.0, 0.1, 0.1),
                     Rect::new(0.8, 0.0, 0.1, 0.1),
                     Rect::new(0.9, 0.0, 0.1, 0.1),
                     )
                 )
            ),
            (ActorStates::Walk,
             Animation::new(
                 10.,
                 Point2::new(0.5, 0.5),
                 vec!(
                     Rect::new(0.0, 0.2, 0.1, 0.1),
                     Rect::new(0.1, 0.2, 0.1, 0.1),
                     Rect::new(0.2, 0.2, 0.1, 0.1),
                     Rect::new(0.3, 0.2, 0.1, 0.1),
                     Rect::new(0.4, 0.2, 0.1, 0.1),
                     Rect::new(0.5, 0.2, 0.1, 0.1),
                     Rect::new(0.6, 0.2, 0.1, 0.1),
                     Rect::new(0.7, 0.2, 0.1, 0.1),
                     Rect::new(0.8, 0.2, 0.1, 0.1),
                     Rect::new(0.9, 0.2, 0.1, 0.1),
                     )
                 )
            ))
            .into_iter()
            .collect();

        Ok(Player {
            state: ActorStates::Idle,
            position: Point2::new(100., 100.),
            direction: Point2::new(1., 1.),
            velocity: Vector2::new(20., 20.),
            scale: Point2::new(4.0, 4.0),
            controls: controls,
            animations: animations,
        })
    }


    pub fn update(&mut self, delta_time: time::Duration) -> GameResult<()> {
        let delta = delta_time.subsec_nanos() as f32/1e8;

        let velocity = Vector2::new(
            delta * self.velocity.x * self.controls.direction.x,
            delta * self.velocity.y * self.controls.direction.y
        );

        if self.controls.direction.x < 0. {
            self.direction.x = -1.;
        }
        if self.controls.direction.x > 0. {
            self.direction.x = 1.;
        }

        if velocity != Vector2::new(0., 0.) {
            self.state = ActorStates::Walk;
        } else {
            self.state = ActorStates::Idle;
        }

        self.position += velocity;

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Image) -> GameResult<()> {
        let current = current_frame(
            &self.state,
            &mut self.animations);

        let mut image_param = current;
        image_param.dest = self.position;
        image_param.scale = Point2::new(
            self.scale.x * self.direction.x,
            self.scale.y);

        draw_ex(ctx, assets, image_param)?;

        Ok(())
    }
}
