#![feature(custom_inner_attributes)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::too_many_arguments, clippy::unused_self, clippy::needless_pass_by_value, clippy::module_name_repetitions, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode, math::vec3};
use crate::display::*;
use crate::spell::{load_all_spells, AllSpells};
use crate::game::Game;
use crate::constants::*;


fn setup_initial(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game: ResMut<Game>,
) {
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(SPRITE_SIZE as f32, SPRITE_SIZE as f32), 10, 41);
    game.tah = texture_atlases.add(texture_atlas);
    let font_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new((SPRITE_SIZE/2) as f32, SPRITE_SIZE as f32), 20, 41);
    game.fah = texture_atlases.add(font_atlas);

    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_scale(vec3(1.0/(SCALE*SPRITE_SIZE as f32), 1.0/(SCALE*SPRITE_SIZE as f32), 1.0))
            .with_translation(vec3((WIDTH/2) as f32-0.5, (HEIGHT/2) as f32-0.5, CAMERA_Z)),
        ..default()
    });
}

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    InitialMenu,
    PlayerNameMenu,
    PlayerNameMenuTransition,
    PlayerMenu,
    PlayerMenuExamineSpell,
    PlayerMenuExamineOneSpell,
    PlayerMenuSelectSpell,
    PlayerMenuExamineBoard,
    PlayerMenuTransition,
    Game,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(WindowDescriptor {
            title: "Mayhem!".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AllSpells(load_all_spells()))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::InitialMenu)
        .add_startup_system(setup_initial)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(choosespell::ChooseSpellPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(cursor::CursorPlugin)
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
mod choosespell;
mod display;
mod player;
mod spell;
mod cursor;
mod creature;
mod constants;