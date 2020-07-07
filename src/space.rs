use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude as Pre,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use Pre::Builder;
use Pre::WorldExt;

pub const ARENA_WIDTH: f32 = 160.0;
pub const ARENA_HEIGHT: f32 = 90.0;

pub struct Game;

impl Pre::SimpleState for Game {
    fn on_start(&mut self, data: Pre::StateData<'_, Pre::GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Boi>();
        world.register::<Witch>();

        initialise_witch(world, sprite_sheet_handle.clone());
        initialise_boi(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

pub struct Boi {}

impl Boi {
    fn new() -> Boi {
        Boi {}
    }
}

impl Component for Boi {
    type Storage = DenseVecStorage<Self>;
}

pub struct Witch {}

impl Component for Witch {
    type Storage = DenseVecStorage<Self>;
}

fn load_sprite_sheet(world: &mut Pre::World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "Tilemap.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "tilemap.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut Pre::World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_boi(world: &mut Pre::World, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Boi::new())
        .with(transform)
        .build();
}

fn initialise_witch(world: &mut Pre::World, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(10.0, 10.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 4,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Witch {})
        .with(transform)
        .build();
}
