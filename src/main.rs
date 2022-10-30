use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode, math::vec3};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
pub mod constants;
pub mod cursor;
pub use self::constants::*;

#[derive(Default)]
struct Game {
    tah: Handle<TextureAtlas>
}

#[derive(Component)]
struct Position {x: u8, y: u8  }
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Mortal {
    is_alive: bool
}

#[derive(Component)]
struct RepeatAnimation {
    max: usize,
    init: usize,
}


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Debug, Deserialize, Serialize)]
struct Creature {
    name: String,
    sprite_index: usize,
}

impl Creature {
    fn to_entity(
        &self,
        v: Vec2,
        commands: &mut Commands,
        texture_atlas_handle: Handle<TextureAtlas>
    ) -> Entity {
        return spawn_anim(commands, texture_atlas_handle.clone(), v, self.sprite_index, 4)
    }
}

pub fn get_anim(
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
) -> SpriteSheetBundle {
    let actual_v = v.mul_add(Vec2::splat(SPRITE_SIZE as f32), Vec2::new(0.0, 0.0));
    let mut sprite = TextureAtlasSprite::new(init);
    sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    return SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_xyz(actual_v.x, actual_v.y, 0.0),
        sprite: sprite,
        ..default()
    };
}

fn get_border(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>
) {
    commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(0.0, 0.0), BORDER_BOTTOMLEFT));
    commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(0.0, (HEIGHT-1) as f32), BORDER_TOPLEFT));
    commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, 0.0), BORDER_BOTTOMRIGHT));
    commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, (HEIGHT-1) as f32), BORDER_TOPRIGHT));
    for n in 1..HEIGHT-1 {
        commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(0.0, n as f32), BORDER_LEFT));
        commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new((WIDTH-1) as f32, n as f32), BORDER_RIGHT));
    }
    for n in 1..WIDTH-1 {
        commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(n as f32, 0.0), BORDER_BOTTOM));
        commands.spawn_bundle(get_anim(texture_atlas_handle.clone(), Vec2::new(n as f32, (HEIGHT-1) as f32), BORDER_TOP));
    }
}

fn spawn_anim(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    v: Vec2,
    init: usize,
    num: usize
) -> Entity {
    return commands
        .spawn_bundle(get_anim(texture_atlas_handle, v, init))
        .insert(AnimationTimer(Timer::from_seconds(ANIMATION_TICK, true)))
        .insert(RepeatAnimation {max: init+num-1, init: init}).id();
}

#[derive(Default)]
struct AtlasHandle(Handle<TextureAtlas>);

fn setup_initial(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_SIZE as f32, SPRITE_SIZE as f32), 10, 41);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_scale(vec3(1.0/SCALE, 1.0/SCALE, 1.0))
            .with_translation(vec3(((SPRITE_SIZE*WIDTH/2) as f32)-HALF_SPRITE, ((SPRITE_SIZE*HEIGHT/2) as f32)-HALF_SPRITE, 0.0)),
        ..default()
    });
    commands.insert_resource(Game{tah: texture_atlas_handle});
}
/* 
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut atlas: ResMut<AtlasHandle>,
) {

    
    get_border(&mut commands, atlas.clone());
    spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(2.0), 120, 8);
    spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(1.0), 180, 4);
    let creature = spawn_anim(&mut commands, texture_atlas_handle.clone(), Vec2::splat(3.0), 210, 4);
    commands.entity(creature).insert(Mortal{is_alive: false});

    let creature_map = load_creatures();
    creature_map.get("Pegasus").unwrap().to_entity(Vec2::splat(4.0), &mut commands, texture_atlas_handle.clone());

    Cursor::new(texture_atlas_handle.clone(), &mut commands);
}*/

fn load_creatures() -> HashMap<String, Creature> {
    let f = File::open("assets/creatures.ron").unwrap();
    return ron::de::from_reader(f).unwrap();
}

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Game,
}

fn main() {
    println!("WINDOW SIZE IS {} x {}", SCREEN_WIDTH, SCREEN_HEIGHT);


    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Menu)
        .add_startup_system(setup_initial)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

mod menu;
mod game;