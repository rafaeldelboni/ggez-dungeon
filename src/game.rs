use std::time;

use ggez::timer;
use ggez::graphics;
use ggez::event;
use ggez::event::{Keycode};
use ggez::{Context, GameResult};
use specs::{Builder, Dispatcher, DispatcherBuilder, RunNow, World};

use input::Input;
use assets::Assets;
use position::Position;
use rendering::{RenderingSystem, Renderable, RenderableClass};

#[derive(Clone)]
pub struct DeltaTime {
    pub delta: time::Duration,
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
}

pub struct Game<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Game<'a, 'b>> {
        let mut world = World::new();
        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new().build();

        register_components(&mut world);

        world.add_resource(DeltaTime { delta: time::Duration::new(0, 0) });
        world.add_resource(Assets::new(ctx)?);
        world.add_resource(Input::new());

        world
            .create_entity()
            .with(Position { x: 100., y: 100. })
            .with(Renderable {
                layer: 0,
                class: RenderableClass::Animation {
                    id: "warrior_attack",
                    frame: 0.,
                    speed: 10.,
                    length: 10.,
                }
            })
            .build();

        Ok(Game {
            world,
            dispatcher,
        })
    }
}

impl<'a, 'b> event::EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::get_ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::get_delta(ctx));
            println!("Average FPS: {}", timer::get_fps(ctx));
        }

        self.world.write_resource::<DeltaTime>().delta = timer::get_delta(ctx);

        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        {
            let mut rs = RenderingSystem::new(ctx);
            rs.run_now(&mut self.world.res);
        }
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
        let mut input = self.world.write_resource::<Input>();

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
        let mut input = self.world.write_resource::<Input>();
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
