use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, timing::Time, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use collision::Aabb2;

use rand::{Rng, ThreadRng};

pub const SCREEN_HEIGHT: f32 = 100.0;
pub const SCREEN_WIDTH: f32 = 200.0;
pub const OBJECT_SIZE: f32 = 16.0;

#[derive(Default)]
pub struct Rustacean;

impl Component for Rustacean {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct BadTouch {
    pub bounding: Aabb2,
}

impl Component for BadTouch {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Parallax {
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

    let mut left_transform = Transform::default();
    left_transform.set_scale(Vector3::new(10.0, 0.5, 0.5));
    left_transform.set_translation_xyz(0.0, -OBJECT_SIZE * 0.5, 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(left_transform)
        .build();
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
    left_transform.set_translation_xyz(SCREEN_WIDTH * 0.5 - 16.0, 0.0, 0.0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Parallax {
            factor: 80.0,
            width: 16.0,
        })
        .with(left_transform)
        .with(BadTouch {})
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

impl SimpleState for Ferris {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // let bubbles_spritesheet =
        //     load_sprite_sheet(world, "texture/bubbles.png", "texture/bubbles.ron");

        world.register::<Rustacean>();

        self.coral_timer.replace(1.6);
        self.object_spritesheet.replace(load_sprite_sheet(
            world,
            "texture/objects.png",
            "texture/objects.ron",
        ));

        initialise_camera(world);
        initialise_rustacean(world, self.object_spritesheet.clone().unwrap());
        initialise_ground(world, self.object_spritesheet.clone().unwrap());
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.coral_timer.take() {
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                initialise_coral(
                    data.world,
                    &mut self.rng,
                    self.object_spritesheet.clone().unwrap(),
                );
                self.coral_timer.replace(self.rng.gen_range(0.8, 2.0));
            } else {
                self.coral_timer.replace(timer);
            }
        }

        Trans::None
    }
}
