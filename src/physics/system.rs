use nphysics2d::algebra::{Velocity2};
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use physics::component::{EcsRigidBody, Velocity};
use physics::resources::{UpdateTime, PhysicWorld};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, EcsRigidBody>,
        Write<'a, PhysicWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (vel, mut body, mut phy_world) = data;
        (&vel, &mut body).join().for_each(|(vel, body)| {
            let b = body.get_mut(&mut phy_world);
            if vel.vector.x != 0.0 && vel.vector.y != 0.0 {
                let pi_inverse = 1.0 / (2.0 as f32).sqrt();
                b.set_velocity(Velocity2::linear(vel.vector.x, vel.vector.y) * pi_inverse);
            } else {
                b.set_velocity(Velocity2::linear(vel.vector.x, vel.vector.y));
            }
        });
    }
}

pub struct PhysicSystem;

impl<'a> System<'a> for PhysicSystem {
    type SystemData = (
        Read<'a, UpdateTime>,
        Write<'a, PhysicWorld>,
    );

    fn run(
        &mut self,
        (
            update_time,
            mut physic_world,
        ): Self::SystemData,
    ) {
        physic_world.set_timestep(update_time.0);
        physic_world.step();
    }
}

