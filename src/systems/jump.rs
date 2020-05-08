use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::ferris::{Mass, Rustacean};

#[derive(SystemDesc)]
pub struct JumpSystem;

impl<'s> System<'s> for JumpSystem {
    type SystemData = (
        ReadStorage<'s, Rustacean>,
        WriteStorage<'s, Mass>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (rustacean, mut mass, transform, input): Self::SystemData) {
        for (_rustacean, transform, mass) in (&rustacean, &transform, &mut mass).join() {
            if let Some(jump) = input.action_is_down("jump") {
                let height = transform.translation().y;
                if jump && height == 0.0 {
                    mass.velocity = 3.0;
                }
            }
        }
    }
}
