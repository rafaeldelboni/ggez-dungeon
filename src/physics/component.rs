use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::math::{Point, Inertia, Isometry};
use nphysics2d::object::{BodyHandle, BodyStatus, Material, RigidBody};
use nphysics2d::volumetric::Volumetric;
use specs::{Component, Entity, VecStorage, World, WriteStorage};

use physics::retained_storage::{RetainedStorage};
use physics::{PhysicWorld, BodiesMap};

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub vector: Vector2<f32>,
    pub direction: Vector2<f32>,
    pub scale: Vector2<f32>,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn set(&mut self, new: Vector2<f32>) {
        println!("old{} new{}", self.vector, new);
        self.vector = new;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Velocity {
    pub vector: Vector2<f32>
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct ShapeCube(pub Cuboid<f32>);

impl Component for ShapeCube {
    type Storage = VecStorage<Self>;
}

pub trait ShapeBase {
    fn create_rigid_body(
        &self,
        world: &mut World,
        entity: Entity,
        body_status: BodyStatus,
        position: Vector2<f32>
    );
}

macro_rules! impl_ShapeBase {
    (for $($type:ty),+) => {
        $(impl ShapeBase for $type {
            fn create_rigid_body(
                &self,
                world: &mut World,
                entity: Entity,
                body_status: BodyStatus,
                position: Vector2<f32>
            ) {
                let mut physic_world = world.write_resource::<PhysicWorld>();

                let shape = ShapeHandle::new(self.0.clone());
                let mut inertia = shape.inertia(1.0);
                inertia.angular = 0.0;

                let body_handle = EcsRigidBody::safe_insert(
                    entity,
                    Isometry2::new(position, 0.0),
                    inertia,
                    shape.center_of_mass(),
                    body_status,
                    &mut world.write_storage(),
                    &mut physic_world,
                    &mut world.write_resource(),
                );

                let margin = 0.01;
                physic_world.add_collider(
                    margin,
                    shape,
                    body_handle.handle(),
                    Isometry2::identity(),
                    Material::default(),
                );
            }
        })*
    }
}

impl_ShapeBase!(for ShapeCube);

#[derive(Clone)]
pub struct EcsRigidBody(BodyHandle);

impl Component for EcsRigidBody {
    type Storage = RetainedStorage<Self, VecStorage<Self>>;
}

impl EcsRigidBody {
    pub fn safe_insert<'a>(
        entity: Entity,
        position: Isometry<f32>,
        local_inertia: Inertia<f32>,
        local_center_of_mass: Point<f32>,
        status: BodyStatus,
        bodies_handle: &mut WriteStorage<'a, EcsRigidBody>,
        physic_world: &mut PhysicWorld,
        bodies_map: &mut BodiesMap,
    ) -> Self {
        let body_handle =
            physic_world.add_rigid_body(position, local_inertia, local_center_of_mass);
        {
            let rigid_body = physic_world.rigid_body_mut(body_handle).unwrap();
            rigid_body.set_status(status);
            rigid_body
                .activation_status_mut()
                .set_deactivation_threshold(None);
        }
        bodies_map.insert(body_handle, entity);

        bodies_handle
            .insert(entity, EcsRigidBody(body_handle))
            .expect("Error inserting bodies.");

        EcsRigidBody(body_handle)
    }

    pub fn handle(&self) -> BodyHandle {
        self.0
    }

    #[inline]
    pub fn get<'a>(
        &'a self,
        physic_world: &'a PhysicWorld,
    ) -> &'a RigidBody<f32> {
        physic_world
            .rigid_body(self.0)
            .expect("Rigid body in specs does not exist in physic world")
    }

    #[inline]
    pub fn get_mut<'a>(
        &self,
        physic_world: &'a mut PhysicWorld,
    ) -> &'a mut RigidBody<f32> {
        physic_world
            .rigid_body_mut(self.0)
            .expect("Rigid body in specs does not exist in physic world")
    }
}
