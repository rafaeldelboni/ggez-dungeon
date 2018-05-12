use std::collections::HashMap;

use ggez::graphics::{DrawParam, Point2, Rect};

#[derive(Debug)]
pub struct Animation {
    pub speed: f64,
    pub draw_params: Vec<DrawParam>,
    pub frame: f64,
}

impl Animation {
    pub fn new (
        speed: f64, offset: Point2, origins: Vec<Rect>
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
-> DrawParam {
    let animation = animations.get_mut(state).unwrap();

    animation.frame = (animation.frame + (1. / fps) * animation.speed)
        % animation.draw_params.len() as f64;

    animation.draw_params[animation.frame as usize]
}
