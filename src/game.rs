use ggez::timer;
use ggez::graphics;
use ggez::event; use ggez::event::{Keycode};
use ggez::{Context, GameResult};
use nphysics2d::math::Vector;
use specs::{Dispatcher, DispatcherBuilder, RunNow, World};

use assets::Assets;
use camera::Camera;
use camera::system::{ChaseCameraSystem, SnapCameraSystem};
use camera::component::{ChaseCamera, SnapCamera};
use enemy::entity::{spawn_enemy};
use input::system::{ControlableSystem};
use input::component::{Controlable};
use input::resources::{Input};
use player::entity::{spawn_player};
use physics::system::{MoveSystem, PhysicSystem};
use physics::component::{EcsRigidBody, ShapeCube, Velocity};
use physics::retained_storage::{Retained};
use physics::resources::{BodiesMap, PhysicWorld, UpdateTime};
use rendering::component::{Sprite, Renderable};
use rendering::system::{DebugRenderingSystem, RenderingSystem};

pub fn register_components(world: &mut World) {
    world.register::<ChaseCamera>();
    world.register::<Controlable>();
    world.register::<EcsRigidBody>();
    world.register::<Renderable>();
    world.register::<ShapeCube>();
    world.register::<SnapCamera>();
    world.register::<Sprite>();
    world.register::<Velocity>();
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
            .with(PhysicSystem, "physic_system", &[])
            .with(MoveSystem, "move", &[])
            .with(ChaseCameraSystem, "chase_camera", &["move"])
            .with(SnapCameraSystem, "snap_camera", &["move"])
            .build();

        let mut physic_world = PhysicWorld::new();
        physic_world.set_gravity(Vector::new(0.0, 0.0));
        world.add_resource(physic_world);
        world.add_resource(BodiesMap::new());
        world.add_resource(UpdateTime(0.0));

        world.add_resource(Assets::new(ctx)?);
        world.add_resource(Input::new());

        world.add_resource(
            Camera::new(
                ctx.conf.window_mode.width,
                ctx.conf.window_mode.height,
                ctx.conf.window_mode.width as f32 / 3.,
                ctx.conf.window_mode.height as f32  / 3.
            )
        );

        spawn_enemy(&mut world, 200., 200.);
        spawn_player(&mut world, 100., 100.);

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

        let dt = timer::get_delta(ctx);
        let seconds = dt.subsec_nanos() as f32 / 1_000_000_000.0;
        self.world.write_resource::<UpdateTime>().0 = seconds;

        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

        let mut physic_world = self.world.write_resource::<PhysicWorld>();
        let mut bodies_map = self.world.write_resource::<BodiesMap>();

        let retained = self.world
            .write_storage::<EcsRigidBody>()
            .retained()
            .iter()
            .map(|r| r.handle())
            .collect::<Vec<_>>();

        physic_world.remove_bodies(&retained);

        for handle in &retained {
            bodies_map.remove(handle);
        }

        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        {
            let mut rs = RenderingSystem::new(ctx);
            rs.run_now(&mut self.world.res);
        }
        {
            let mut ds = DebugRenderingSystem::new(ctx);
            ds.run_now(&mut self.world.res);
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
