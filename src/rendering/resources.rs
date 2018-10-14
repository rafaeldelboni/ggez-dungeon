use ggez::graphics::{draw_ex, DrawParam, Point2, Rect};
use ggez::{Context};
use spritesheet_generator::spritesheet::Screen;
use specs::{Write};

use assets::Assets;
use camera::Camera;
use rendering::component::{Sprite};

#[derive(Debug, Copy, Clone)]
pub enum RenderableType {
    Animation,
    Image
}

#[derive(Debug, Copy, Clone)]
pub struct RenderableClass {
    pub render_type: RenderableType,
    pub id: &'static str,
    pub frame: f32,
    pub speed: f32,
    pub length: f32,
}

impl RenderableClass {
    pub fn new_animation(
        id: &'static str,
        speed: f32,
        length: f32
    ) -> RenderableClass {
        RenderableClass{
            render_type: RenderableType::Animation,
            id,
            frame: 0.,
            speed,
            length
        }
    }
}

fn generate_draw_param (
    camera: &Camera,
    frame: &Screen,
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
        ..Default::default()
    }
}

pub fn draw_image(
    camera: &Camera,
    context: &mut Context,
    spritesheet: &Write<Assets>,
    sprite: &Sprite,
    id: &str
) {
    if let Some(frame_data) = &spritesheet.spritesheet_data.frames.get(id) {
        let frame = frame_data.screen.clone();
        draw_ex(
            context,
            &spritesheet.spritesheet_image,
            generate_draw_param(&camera, &frame, *sprite)
        ).expect("Unable do render image.");
    }
}
