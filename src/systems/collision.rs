use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write},
};

use crate::ferris::{BadTouch, Rustacean, Scoreboard};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, BadTouch>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Rustacean>,
        Write<'s, Scoreboard>,
    );

    fn run(&mut self, (bads, transforms, rustaceans, mut scoreboard): Self::SystemData) {
        for (_bad, bad_transform) in (&bads, &transforms).join() {
            for (_rustacean, transform) in (&rustaceans, &transforms).join() {
                if scoreboard.winning && transform.translation().y < 10.0
                    && (bad_transform.translation().x) < (transform.translation().x + 10.0)
                    && (bad_transform.translation().x + 8.0) > (transform.translation().x)
                {
                    scoreboard.winning = false;
                }
            }
        }
    }
}
