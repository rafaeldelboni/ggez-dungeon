use ggez::graphics::{Drawable, DrawMode, DrawParam, Mesh, Point2};
use ggez::{Context, GameResult};
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use assets::Assets;
use camera::Camera;
use physics::component::{EcsRigidBody, ShapeCube};
use physics::resources::{PhysicWorld};
use rendering::component::{Renderable, Sprite};
use rendering::resources::{draw_image, RenderableClass, RenderableType};
use states::component::{States};

const TARGET_FPS: f32 = 60.;

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
        Read<'a, Camera>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, States>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (assets_sd, camera, mut renderables, sprites, states) = data;
        let spritesheet = assets_sd.unwrap();

        for (mut renderable, sprite, states) 
            in (&mut renderables, &sprites, &states).join() {
                let anim = renderable.class;
                match anim.render_type {
                    RenderableType::Animation => {
                        let state_anim = states.current_action().renderable;

                        let next_frame = if anim.id == state_anim.id {
                            (anim.frame + (1. / TARGET_FPS) * anim.speed)
                                % anim.length
                        } else {
                            0.0
                        };

                        renderable.class = RenderableClass {
                            render_type: RenderableType::Animation,
                            id: state_anim.id,
                            frame: next_frame,
                            speed: state_anim.speed,
                            length: state_anim.length
                        };

                        let sheet_id = format!(
                            "{}_{:02}", state_anim.id, next_frame as usize
                        );
                        draw_image(&*camera, self.ctx, &spritesheet, sprite, &sheet_id);
                    },
                    RenderableType::Image => {
                        draw_image(&*camera, self.ctx, &spritesheet, sprite, anim.id);
                    },
                }
            }
    }
}

pub struct DebugRenderingSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> DebugRenderingSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> DebugRenderingSystem<'c> {
        DebugRenderingSystem { ctx }
    }

    pub fn render(&mut self, points: &[Point2], cam_scale: Point2)
        -> GameResult<()> {
            let mesh = Mesh::new_polygon(
                self.ctx,
                DrawMode::Line(1.),
                points
            ).expect("Error creating polygon.");

            mesh.draw_ex(
                self.ctx,
                DrawParam {
                    dest: Point2::origin(),
                    rotation: 0.0,
                    scale: cam_scale,
                    offset: Point2::new(0.5, 0.5),
                    ..Default::default()
                },
                )
        }
}

impl<'a, 'c> System<'a> for DebugRenderingSystem<'c> {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, EcsRigidBody>,
        ReadStorage<'a, ShapeCube>,
        Read<'a, PhysicWorld>,
    );

    fn run(&mut self, (camera, bodies, cube, world): Self::SystemData) {
        (&bodies, &cube).join().for_each(|(body, cube)| {
            let rbody = body.get(&world);
            let cam_position = camera.calculate_dest_point(
                Point2::new(
                    rbody.position().translation.vector.x,
                    rbody.position().translation.vector.y
                )
            );
            let cam_scale = camera.draw_scale();

            let rect_x = cam_position.x;
            let rect_y = cam_position.y;
            let rect_w = cube.0.half_extents().x * cam_scale.x;
            let rect_h = cube.0.half_extents().y * cam_scale.y;

            let x1 = rect_x - rect_w;
            let x2 = rect_x + rect_w;
            let y1 = rect_y - rect_h;
            let y2 = rect_y + rect_h;

            let points = [
                Point2::new(x1, y1),
                Point2::new(x2, y1),
                Point2::new(x2, y2),
                Point2::new(x1, y2),
            ];

            self.render(&points, Point2::new(1., 1.))
                .expect("Error drawing cube bounds.")
        });
    }
}
