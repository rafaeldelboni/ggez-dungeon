use nalgebra::{Vector2};
use ncollide2d::shape::{Cuboid};
use nphysics2d::object::{BodyStatus};
use specs::{Builder, World};

use physics::component::{ShapeBase, ShapeCube, Velocity};
use enemy::resources::{enemy_idle, enemy_walk};
use rendering::component::{Renderable, Sprite};
use rendering::resources::{RenderableClass};
use states::component::{States};

pub fn spawn_enemy(world: &mut World, x: f32, y: f32) {
    let shape_cube = ShapeCube(Cuboid::new(Vector2::new(5., 5.)));

    let entity = world
        .create_entity()
        .with(shape_cube.clone())
        .with(Sprite {
            position: Vector2::new(x, y),
            direction: Vector2::new(1., 1.),
            scale: Vector2::new(1., 1.),
            offset: Vector2::new(0., 10.)
        })
        .with(Velocity::new(Vector2::new(0., 0.)))
        .with(States::new(Some(enemy_idle), Some(enemy_walk)))
        .with(Renderable {
            layer: 0,
            class: RenderableClass::Image {
                id: "warrior_attack_01"
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
