use ggez::timer;
use ggez::graphics;
use ggez::event;
use ggez::event::{Keycode};
use ggez::{Context, GameResult};
use specs::{Builder, Dispatcher, DispatcherBuilder, RunNow, World};

use assets::Assets;
use camera::{Camera, ChaseCamera, SnapCamera, ChaseCameraSystem, SnapCameraSystem};
use input::{ControlableSystem, Controlable, Input};
use position::{PositionSystem, Position, Velocity};
use rendering::{RenderingSystem, Renderable, RenderableClass};

pub fn register_components(world: &mut World) {
    world.register::<SnapCamera>();
    world.register::<ChaseCamera>();
    world.register::<Controlable>();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Renderable>();
}

pub struct Game<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Game<'a, 'b>> {
        let mut world = World::new();

        register_components(&mut world);

        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .with(ControlableSystem, "controlable", &[])
            .with(PositionSystem, "position", &[])
            .with(ChaseCameraSystem, "chase_camera", &["position"])
            .with(SnapCameraSystem, "snap_camera", &["position"])
            .build();

        world.add_resource(Assets::new(ctx)?);
        world.add_resource(Input::new());

        let (w, h) = (ctx.conf.window_mode.width, ctx.conf.window_mode.height);
        let hc = h as f32 / w as f32;
        let fov = w as f32 * 1.5;
        world.add_resource(Camera::new(w, h, fov, hc * fov));

        world
            .create_entity()
            .with(Position { x: 400., y: 400. })
            .with(Renderable {
                layer: 0,
                class: RenderableClass::Image {
                    id: "warrior_attack_01"
                }
            })
            .build();

        world
            .create_entity()
            .with(Controlable)
            .with(Position { x: 100., y: 100. })
            .with(Velocity { x: 0., y: 0. })
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
