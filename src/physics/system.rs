use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use physics::component::{EcsRigidBody, Position, Velocity};
use physics::resources::{UpdateTime, PhysicWorld};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, EcsRigidBody>,
        Write<'a, PhysicWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (vel, mut pos, mut body, mut phy_world) = data;
        (&mut pos, &vel, &mut body).join().for_each(|(pos, vel, body)| {
            if vel.is_moving() {
                let updated_position = body
                    .apply_velocity(&mut phy_world, vel.get())
                    .position()
                    .translation
                    .vector;

                pos.pull(updated_position);
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

