use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, timing::Time, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, FontHandle, TtfFormat, UiText, UiTransform},
};

use rand::{rngs::ThreadRng, Rng};

pub const SCREEN_HEIGHT: f32 = 100.0;
pub const SCREEN_WIDTH: f32 = 200.0;
pub const OBJECT_SIZE: f32 = 16.0;

#[derive(Default)]
pub struct Rustacean;

impl Component for Rustacean {
    type Storage = DenseVecStorage<Self>;
}

pub struct BadTouch;

impl Component for BadTouch {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Scoreboard {
    pub winning: bool,
}

#[derive(Default)]
pub struct ScoreText;

impl Component for ScoreText {
    type Storage = DenseVecStorage<Self>;
}

pub enum OutOfScreenAction {
    REVIVE { vertical: bool, horizontal: bool },
    DELETE,
}

pub struct Parallax {
    pub on_out_of_screen: OutOfScreenAction,
    pub factor: f32,
    pub width: f32,
}

impl Component for Parallax {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Mass {
    pub velocity: f32,
}

impl Component for Mass {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ferris {
    pub rng: ThreadRng,
    pub coral_timer: Option<f32>,
    pub object_spritesheet: Option<Handle<SpriteSheet>>,
    pub winning: bool,
    pub score: f32,
    pub score_entity: Option<Entity>,
}

impl Ferris {
    pub fn new() -> Ferris {
        Ferris {
            rng: rand::thread_rng(),
            object_spritesheet: None,
            coral_timer: None,
            winning: false,
            score: 0.0,
            score_entity: None,
        }
    }
}

fn load_sprite_sheet(world: &mut World, image: &str, mapping: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(image, ImageFormat::default(), (), &texture_storage)
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        mapping,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_rustacean(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut left_transform = Transform::default();
    left_transform.set_scale(Vector3::new(0.5, 0.5, 0.5));
    left_transform.set_translation_xyz(16.0 - SCREEN_WIDTH * 0.5, 0.0, 0.0);

    world
        .create_entity()
        .with(Rustacean)
        .with(Mass { velocity: 0.0 })
        .with(sprite_render.clone())
        .with(left_transform)
        .build();
}

fn initialise_ground(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 7,
    };

    let num_grounds = 15;

    for n in 1..num_grounds {
        let mut left_transform = Transform::default();
        left_transform.set_scale(Vector3::new(0.5, 0.5, 0.5));
        left_transform.set_translation_xyz(
            OBJECT_SIZE * n as f32 - 100.0 - OBJECT_SIZE,
            -OBJECT_SIZE * 0.5,
            0.0,
        );

        world
            .create_entity()
            .with(sprite_render.clone())
            .with(Parallax {
                on_out_of_screen: OutOfScreenAction::REVIVE {
                    vertical: false,
                    horizontal: true,
                },
                factor: 80.0,
                width: 16.0,
            })
            .with(left_transform)
            .build();
    }
}

fn initialise_coral(
    world: &mut World,
    rng: &mut ThreadRng,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: rng.gen_range(4, 7),
    };

    let mut left_transform = Transform::default();
    left_transform.set_scale(Vector3::new(0.5, 0.5, 0.5));
    left_transform.set_translation_xyz(SCREEN_WIDTH * 0.5 + 16.0, 0.0, 0.0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Parallax {
            on_out_of_screen: OutOfScreenAction::DELETE,
            factor: 80.0,
            width: 16.0,
        })
        .with(left_transform)
        .with(BadTouch)
        .build();
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_scoreboard(world: &mut World, font: FontHandle) {
    let score_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        100.,
        0.,
        1.,
        500.,
        50.,
    );

    let score_text = world
        .create_entity()
        .with(score_transform)
        .with(ScoreText {})
        .with(UiText::new(
            font,
            "Score: 00000000".to_string(),
            [0.0, 0.0, 0.0, 1.0],
            40.,
        ))
        .build();

    world.insert(score_text);
}

fn initialise_you_lose(world: &mut World, font: FontHandle) {
    let score_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -30.,
        -80.,
        1.,
        500.,
        100.,
    );

    let score_text = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(
            font,
            "You Lose!".to_string(),
            [0.0, 0.0, 0.0, 0.0],
            40.,
        ))
        .build();

    world.insert(score_text);
}

impl SimpleState for Ferris {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // let bubbles_spritesheet =
        //     load_sprite_sheet(world, "texture/bubbles.png", "texture/bubbles.ron");

        world.register::<Rustacean>();
        world.insert(Scoreboard { winning: true });

        self.coral_timer.replace(1.6);
        self.object_spritesheet.replace(load_sprite_sheet(
            world,
            "texture/objects.png",
            "texture/objects.ron",
        ));

        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        initialise_camera(world);
        initialise_rustacean(world, self.object_spritesheet.clone().unwrap());
        initialise_ground(world, self.object_spritesheet.clone().unwrap());
        initialise_scoreboard(world, font.clone());
        initialise_you_lose(world, font);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.coral_timer.take() {
            let mut winning = false;
            {
                let scoreboard = data.world.fetch::<Scoreboard>();
                winning = scoreboard.winning;
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if winning {
                if timer <= 0.0 {
                    initialise_coral(
                        data.world,
                        &mut self.rng,
                        self.object_spritesheet.clone().unwrap(),
                    );
                    self.coral_timer.replace(self.rng.gen_range(0.5, 2.0));
                } else {
                    self.coral_timer.replace(timer);
                }
            }
        }

        Trans::None
    }
}
