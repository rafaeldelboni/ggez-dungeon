//! A camera object for ggez.
//! Currently ggez has no actual global camera state to use,
//! so this really just does the coordinate transforms for you.
//!
//! Basically it translates ggez's coordinate system with the origin
//! at the top-left and Y increasing downward to a coordinate system
//! with the origin at the center of the screen and Y increasing
//! upward.
//!
//! Because that makes sense, darn it.
//!
//! However, does not yet do any actual camera movements like
//! easing, pinning, etc.
//! But a great source for how such things work is this:
//! http://www.gamasutra.com/blogs/ItayKeren/20150511/243083/Scroll_Back_The_Theory_and_Practice_of_Cameras_in_SideScrollers.php

// TODO: Debug functions to draw world and camera grid!

use ggez;
use ggez::GameResult;
use ggez::graphics;
use ggez::graphics::{DrawParam, Vector2};

// Hmm.  Could, instead, use a 2d transformation
// matrix, or create one of such.
pub struct Camera {
    screen_size: Vector2,
    view_size: Vector2,
    view_center: Vector2,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            screen_size: Vector2::new(0., 0.),
            view_size: Vector2::new(0., 0.),
            view_center: Vector2::new(0., 0.)
        }
    }
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, view_width: f32, view_height: f32) -> Self {
        let screen_size = Vector2::new(screen_width as f32, screen_height as f32);
        let view_size = Vector2::new(view_width as f32, view_height as f32);
        Camera {
            screen_size: screen_size,
            view_size: view_size,
            view_center: Vector2::new(0., 0.),
        }
    }

    pub fn move_by(&mut self, by: Vector2) {
        self.view_center += by;
    }

    pub fn move_to(&mut self, to: Vector2) {
        self.view_center = to;
    }

    pub fn draw_scale(&self) -> graphics::Point2 {
        graphics::Point2::new(
            (self.screen_size.x / self.view_size.x) as f32,
            (self.screen_size.y / self.view_size.y) as f32,
        )
    }

    /// Translates a point in world-space to a point in
    /// screen-space.
    ///
    /// Does not do any clipping or anything, since it does
    /// not know how large the thing that might be drawn is;
    /// that's not its job.
    pub fn world_to_screen_coords(&self, from: Vector2) -> (i32, i32) {
        let pixels_per_unit = self.screen_size.component_div(&self.view_size);
        let view_offset = from - self.view_center;
        let view_scale = view_offset.component_mul(&pixels_per_unit);


        let x = (*view_scale).x + (*self.screen_size).x / 2.0;
        let y = (*self.screen_size).y - ((*view_scale).y + (*self.screen_size).y / 2.0);
        (x as i32, y as i32)
    }


    // p_screen = max_p - p + max_p/2
    // p_screen - max_p/2 = max_p - p
    // p_screen - max_p/2 + max_p = -p
    // -p_screen - max_p/2 + max_p = p
    pub fn screen_to_world_coords(&self, from: (i32, i32)) -> Vector2 {
        let (sx, sy) = from;
        let sx = sx as f32;
        let sy = sy as f32;
        let flipped_x = sx - ((*self.screen_size).x / 2.0);
        let flipped_y = -sy + (*self.screen_size).y / 2.0;
        let screen_coords = Vector2::new(flipped_x, flipped_y);
        let units_per_pixel = self.view_size.component_div(&self.screen_size);
        let view_scale = screen_coords.component_mul(&units_per_pixel);
        let view_offset = self.view_center + view_scale;

        view_offset

    }

    pub fn location(&self) -> Vector2 {
        self.view_center
    }

    pub fn size(&self) -> Vector2 {
        self.view_size
    }

    pub fn calculate_dest_point(&self, location: Vector2) -> graphics::Point2 {
        let (sx, sy) = self.world_to_screen_coords(location);
        graphics::Point2::new(sx as f32, sy as f32)
    }
}

pub trait CameraDraw
where
    Self: graphics::Drawable, {
    fn draw_ex_camera(
        &self,
        camera: &Camera,
        ctx: &mut ggez::Context,
        p: ggez::graphics::DrawParam,
    ) -> GameResult<()> {
        let dest = Vector2::new(p.dest.x as f32, p.dest.y as f32);
        let dest = camera.calculate_dest_point(dest);
        let scale = camera.draw_scale();
        let orig_scale = p.scale.clone();
        let mut my_p = p;
        my_p.dest = dest;
        my_p.scale = graphics::Point2::new(orig_scale.x * scale.x, orig_scale.y * scale.y);
        self.draw_ex(ctx, my_p)
    }

    fn draw_camera(
        &self,
        camera: &Camera,
        ctx: &mut ggez::Context,
        dest: ggez::graphics::Point2,
        rotation: f32,
    ) -> GameResult<()> {
        let dest = Vector2::new(dest.x as f32, dest.y as f32);
        let dest = camera.calculate_dest_point(dest);
        let scale = camera.draw_scale();
        self.draw_ex(
            ctx,
            DrawParam {
                dest,
                scale,
                rotation,
                ..Default::default()
            },
        )
    }
}


impl<T> CameraDraw for T
where
    T: graphics::Drawable,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use Vector2;

    #[test]
    fn test_coord_round_trip() {
        let mut c = Camera::new(640, 480, 40.0, 30.0);
        let p1 = (200, 300);
        {
            let p1_world = c.screen_to_world_coords(p1);
            assert_eq!(p1_world, Vector2::new(-7.5, -3.75));
            let p1_screen = c.world_to_screen_coords(p1_world);
            assert_eq!(p1, p1_screen);
        }


        let p2 = Vector2::new(20.0, 10.0);
        {
            let p2_screen = c.world_to_screen_coords(p2);
            assert_eq!(p2_screen, (640, 80));
            let p2_world = c.screen_to_world_coords(p2_screen);
            assert_eq!(p2_world, p2);
        }

        c.move_to(Vector2::new(5.0, 5.0));

        {
            let p1_world = c.screen_to_world_coords(p1);
            assert_eq!(p1_world, Vector2::new(-2.5, 1.25));
            let p1_screen = c.world_to_screen_coords(p1_world);
            assert_eq!(p1, p1_screen);
        }
        {
            let p2_screen = c.world_to_screen_coords(p2);
            assert_eq!(p2_screen, (560, 160));
            let p2_world = c.screen_to_world_coords(p2_screen);
            assert_eq!(p2_world, p2);
        }
    }
}
