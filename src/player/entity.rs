use nalgebra::{Vector2};
use ncollide2d::shape::{Cuboid};
use nphysics2d::object::{BodyStatus};
use specs::{Builder, World};

use camera::component::SnapCamera;
use input::component::Controlable;
use physics::component::{ShapeBase, ShapeCube, Velocity};
use player::resources::{player_idle, player_walk};
use rendering::component::{Renderable, Sprite};
use rendering::resources::{RenderableClass};
use states::component::{States};

pub fn spawn_player(world: &mut World, x: f32, y: f32) {
    let shape_cube = ShapeCube(Cuboid::new(Vector2::new(5., 5.)));

    let entity = world
        .create_entity()
        .with(Controlable)
        .with(SnapCamera)
        .with(shape_cube.clone())
        .with(Sprite {
            position: Vector2::new(x, y),
            direction: Vector2::new(1., 1.),
            scale: Vector2::new(1., 1.),
            offset: Vector2::new(0., 10.)
        })
        .with(Velocity::new(Vector2::new(0., 0.)))
        .with(States::new(Some(player_idle), Some(player_walk)))
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

    shape_cube.create_rigid_body(
        world,
        entity,
        BodyStatus::Dynamic,
        Vector2::new(x, y)
    );
}
