use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    ui::UiText,
};

use crate::ferris::{ScoreText, Scoreboard};

#[derive(SystemDesc)]
pub struct ScoreCheckSystem;

impl<'s> System<'s> for ScoreCheckSystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        Read<'s, Time>,
        Read<'s, Scoreboard>,
        ReadStorage<'s, ScoreText>,
    );

    fn run(&mut self, (mut ui_texts, time, scoreboard, score): Self::SystemData) {
        if (scoreboard.winning) {
            for (ui_text, _score) in (&mut ui_texts, &score).join() {
                ui_text.text = format!(
                    "{}",
                    format!("Score: {:08}", time.absolute_time().as_millis() / 100)
                );
            }
        } else {
            for (ui_text, _score) in (&mut ui_texts, !&score).join() {
                ui_text.color = [0.0, 0.0, 0.0, (time.absolute_time_seconds() as f32 * 2.0).sin().abs()];
            }
        }
    }
}
