use ggez::graphics::{draw_ex, DrawParam, Rect, Point2, Vector2};
use ggez::{Context};

use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use spritesheet_generator::spritesheet::Screen;

use assets::Assets;
use camera::Camera;
use position::Position;
use rendering::{Renderable, RenderableClass};

const TARGET_FPS: f32 = 60.;

fn generate_draw_param (
    camera: &Camera,
    frame: Screen,
    position: Position
) -> DrawParam {

    let cam_dest = camera.calculate_dest_point(
        Vector2::new(position.x, position.y)
    );
    let cam_scale = camera.draw_scale();

    DrawParam {
        src: Rect {
            x: frame.x as f32,
            y: frame.y as f32,
            w: frame.w as f32,
            h: frame.h as f32,
        },
        dest: cam_dest,
        scale: cam_scale,
        offset: Point2::new(0.5, 0.5),
        shear: Point2::new(1./1e4, 1./1e4),
        ..Default::default()
    }
}

fn draw_image(
    camera: &Camera,
    context: &mut Context,
    spritesheet: &Write<Assets>,
    position: &Position,
    id: String
) {
    if let Some(frame_data) = &spritesheet.spritesheet_data.frames.get(&id) {
        let frame = frame_data.screen.clone();

        draw_ex(
            context,
            &spritesheet.spritesheet_image,
            generate_draw_param(&camera, frame, *position)
        ).unwrap();
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
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (assets_sd, camera, mut renderable_sd, position_sd) = data;
        let spritesheet = assets_sd.unwrap();

        for (mut renderable, position) in (&mut renderable_sd, &position_sd).join() {
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
                    draw_image(&*camera, self.ctx, &spritesheet, position, id);
                },
                RenderableClass::Image { id } => {
                    draw_image(&*camera, self.ctx, &spritesheet, position, String::from(id));
                },
            }
        }
    }
}
