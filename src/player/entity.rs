use nalgebra::{Vector2};
use ncollide2d::shape::{Cuboid};
use nphysics2d::object::{BodyStatus};
use specs::{Builder, World};

use camera::{SnapCamera};
use input::{Controlable};
use physics::{Position, ShapeBase, ShapeCube, Velocity};
use rendering::{Renderable, RenderableClass};

pub fn spawn_player(world: &mut World, x: f32, y: f32) {
    let shape_cube = ShapeCube(Cuboid::new(Vector2::new(5., 5.)));

    let entity = world
        .create_entity()
        .with(Controlable)
        .with(SnapCamera)
        .with(shape_cube.clone())
        .with(Position { x, y })
        .with(Velocity {
            vector: Vector2::new(0., 0.)
        })
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
