use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use physics::component::{EcsRigidBody, Velocity};
use physics::resources::{UpdateTime, PhysicWorld};
use rendering::component::{Sprite};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, EcsRigidBody>,
        Write<'a, PhysicWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (vel, mut sprite, mut body, mut phy_world) = data;
        (&mut sprite, &vel, &mut body).join().for_each(|(sprite, vel, body)| {
            if vel.is_moving() {
                let updated_position = body
                    .apply_velocity(&mut phy_world, vel.get())
                    .position()
                    .translation
                    .vector;

                sprite.pull(updated_position);
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

