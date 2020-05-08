use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::ferris::{Parallax, SCREEN_WIDTH};

#[derive(SystemDesc)]
pub struct ParallaxSystem;

impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Parallax>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut transforms, parallax, time): Self::SystemData) {
        for (e, transform, parallax) in (&*entities, &mut transforms, &parallax).join() {
            let new_x = transform.translation().x - time.delta_seconds() * parallax.factor;
            if new_x > (-SCREEN_WIDTH - parallax.width) * 0.5 {
                transform.set_translation_x(new_x);
            } else {
                match entities.delete(e) {
                    Ok(_v) => {}
                    Err(e) => println!("{} {:?}", "Wrong Generation Error", e),
                };
            }
        }
    }
}
