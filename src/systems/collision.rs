use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::ferris::{BadTouch, Rustacean};

#[derive(SystemDesc)]
pub struct ParallaxSystem;

impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (ReadStorage<'s, BadTouch>, ReadStorage<'s, Rustacean>);

    fn run(&mut self, (mut transforms, parallax, time): Self::SystemData) {
        for (transform, parallax) in (&mut transforms, &parallax).join() {
            
        }
    }
}
