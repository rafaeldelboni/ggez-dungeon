use ggez::graphics::{draw_ex, Drawable, DrawMode, DrawParam, Mesh, Point2, Rect};
use ggez::{Context, GameResult};
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};
use spritesheet_generator::spritesheet::Screen;

use assets::Assets;
use camera::Camera;
use physics::component::{EcsRigidBody, ShapeCube};
use physics::resources::{PhysicWorld};
use rendering::component::{Renderable, RenderableClass, Sprite};

const TARGET_FPS: f32 = 60.;

fn generate_draw_param (
    camera: &Camera,
    frame: Screen,
    sprite: Sprite
) -> DrawParam {
    let cam_dest = camera.calculate_dest_point(
        Point2::new(
            sprite.position.x + sprite.offset.x,
            sprite.position.y + sprite.offset.y
        )
    );
    let cam_scale = camera.draw_scale();
    let sprite_scale = Point2::new(
        cam_scale.x * sprite.scale.x * sprite.direction.x,
        cam_scale.y * sprite.scale.y,
    );

    DrawParam {
        src: Rect {
            x: frame.x as f32,
            y: frame.y as f32,
            w: frame.w as f32,
            h: frame.h as f32,
        },
        dest: cam_dest,
        scale: sprite_scale,
        offset: Point2::new(0.5, 0.5),
        shear: Point2::new(1./1e4, 1./1e4),
        ..Default::default()
    }
}

fn draw_image(
    camera: &Camera,
    context: &mut Context,
    spritesheet: &Write<Assets>,
    sprite: &Sprite,
    id: String
) {
    if let Some(frame_data) = &spritesheet.spritesheet_data.frames.get(&id) {
        let frame = frame_data.screen.clone();
        draw_ex(
            context,
            &spritesheet.spritesheet_image,
            generate_draw_param(&camera, frame, *sprite)
        ).expect("Unable do render image.");
    }
}

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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (assets_sd, camera, mut renderables, sprites) = data;
        let spritesheet = assets_sd.unwrap();

        for (mut renderable, sprite) in (&mut renderables, &sprites).join() {
            match renderable.class {
                RenderableClass::Animation { id, frame, speed, length } => {
                    let next_frame = (frame + (1. / TARGET_FPS) * speed) % length;

                    renderable.class = RenderableClass::Animation {
                        id,
                        frame: next_frame,
                        speed,
                        length
                    };

                    let id = format!("{}_{:02}", id, frame as usize);
                    draw_image(&*camera, self.ctx, &spritesheet, sprite, id);
                },
                RenderableClass::Image { id } => {
                    draw_image(&*camera, self.ctx, &spritesheet, sprite, String::from(id));
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

    pub fn render(&mut self, points: &[Point2], cam_scale: Point2) -> GameResult<()> {
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
