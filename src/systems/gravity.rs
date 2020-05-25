use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
};

use crate::ferris::{Mass, Scoreboard};

#[derive(SystemDesc)]
pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Mass>,
        Read<'s, Time>,
        Read<'s, Scoreboard>,
    );

    fn run(&mut self, (mut transforms, mut masses, time, scoreboard): Self::SystemData) {
        if scoreboard.winning {
            for (transform, mass) in (&mut transforms, &mut masses).join() {
                mass.velocity -= 9.0 * time.delta_seconds();
                let height = transform.translation().y;
                transform.set_translation_y((height + mass.velocity).max(0.0));
            }
        }
    }
}
