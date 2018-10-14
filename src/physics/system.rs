use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use physics::component::{EcsRigidBody, Velocity};
use physics::resources::{UpdateTime, PhysicWorld};
use rendering::component::{Sprite};
use states::component::{States};
use states::resources::{StateActions};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, EcsRigidBody>,
        WriteStorage<'a, States>,
        Write<'a, PhysicWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (vel, mut sprite, mut body, mut states, mut phy_world) = data;
        (&mut sprite, &vel, &mut states, &mut body).join().for_each(
            |(sprite, vel, states, body)| {
                let updated_position = body
                    .apply_velocity(&mut phy_world, vel.get())
                    .position()
                    .translation
                    .vector;
                sprite.pull(updated_position);
                if vel.is_stoping() || vel.is_moving() {
                    states.start_action(&StateActions::Walk);
                } else {
                    states.start_action(&StateActions::Idle);
                }
            }
        );
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
        physic_world.set_timestep(update_time.seconds);
        physic_world.step();
    }
}

