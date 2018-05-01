use std::collections::HashMap;

use ggez::graphics::{DrawParam, Image, Point2, Rect};

#[derive(Debug)]
pub struct Animation {
    pub speed: f64,
    pub source_image: Image,
    pub draw_params: Vec<DrawParam>,
    pub frame: f64,
}

impl Animation {
    pub fn new (
        speed: f64, image :Image, offset: Point2, origins: Vec<Rect>
        ) -> Animation {
        let params: Vec<DrawParam> = origins
            .iter()
            .map(|&origin| DrawParam {
                src: origin,
                offset: offset,
                shear: Point2::new(1./1e4, 1./1e4),
                ..Default::default()
            })
        .collect();

        Animation {
            speed: speed,
            source_image: image,
            draw_params: params,
            frame: 0.,
        }
    }
}

pub type Animations = HashMap<&'static str, Animation>;

pub fn current_frame (
    fps: f64,
    state: &'static str,
    animations: &mut Animations)
-> (Image, DrawParam) {
    let animation = animations.get_mut(state).unwrap();

    animation.frame = (animation.frame + (1. / fps) * animation.speed)
        % animation.draw_params.len() as f64;

    (
        animation.source_image.clone(),
        animation.draw_params[animation.frame as usize]
    )
}
