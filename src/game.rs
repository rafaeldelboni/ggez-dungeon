use std::time;

use ggez::timer;
use ggez::graphics;
use ggez::graphics::{draw_ex, DrawParam, Point2, Rect};
use ggez::event;
use ggez::event::{Keycode};
use ggez::{Context, GameResult};
use specs::{World, Dispatcher, DispatcherBuilder};

use assets::Assets;

#[derive(Clone)]
pub struct DeltaTime {
    pub delta: time::Duration,
}

#[derive(Clone)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub slide: bool,
    pub jump: bool,
    pub attack: bool,
}

impl PlayerInput {
    pub fn new() -> PlayerInput {
        PlayerInput {
            up: false,
            down: false,
            left: false,
            right: false,
            slide: false,
            jump: false,
            attack: false,
        }
    }
}

pub struct Game<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Game<'a, 'b>> {
        let mut world = World::new();
        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .build();

        world.add_resource(Assets::new(ctx)?);
        world.add_resource(DeltaTime { delta: time::Duration::new(0, 0) });
        world.add_resource(PlayerInput::new());

        Ok(Game {
            world,
            dispatcher,
        })
    }
}

impl<'a, 'b> event::EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::get_ticks(ctx) % 100 == 0 {
            //println!("Delta frame time: {:?} ", timer::get_delta(ctx));
            //println!("Average FPS: {}", timer::get_fps(ctx));
        }

        self.world.write_resource::<DeltaTime>().delta = timer::get_delta(ctx);

        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let assets = self.world.read_resource::<Assets>();
        let spritesheet = &assets.spritesheet_data;
        let frame = spritesheet.frames.get("warrior_die_08").unwrap().screen.clone();

        let image_param = DrawParam {
            src: Rect::new(frame.x, frame.y, frame.w, frame.h),
            dest: Point2::new(100., 100.),
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(4.0, 4.0),
            shear: Point2::new(1./1e4, 1./1e4),
            ..Default::default()
        };
        draw_ex(ctx, &assets.spritesheet_image, image_param).unwrap();

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        repeat: bool
    ) {
        let mut input = self.world.write_resource::<PlayerInput>();

        if !repeat {
            match keycode {
                Keycode::Left => input.left = true,
                Keycode::Right => input.right = true,
                Keycode::Up => input.up = true,
                Keycode::Down => input.down = true,
                Keycode::LCtrl => input.slide = true,
                Keycode::Space => input.jump = true,
                Keycode::LShift => input.attack = true,
                _ => (),
            }
        }
    }

    fn key_up_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        repeat: bool
    ) {
        let mut input = self.world.write_resource::<PlayerInput>();
        if !repeat {
            match keycode {
                Keycode::Left => input.left = false,
                Keycode::Right => input.right = false,
                Keycode::Up => input.up = false,
                Keycode::Down => input.down = false,
                _ => (),
            }
        }
    }
}
