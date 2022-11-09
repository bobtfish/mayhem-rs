#![feature(custom_inner_attributes)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::too_many_arguments, clippy::unused_self, clippy::needless_pass_by_value, clippy::module_name_repetitions, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode, math::vec3};

mod screen;
mod game;
mod display;
mod player;
mod spell;
mod cursor;
mod creature;
mod constants;
mod system;
mod gamestate;

use crate::spell::load_all_spells;
use crate::game::Game;
use crate::constants::*;
use crate::gamestate::GameState;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Mayhem!".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugin(game::GamePlugin)
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(load_all_spells())
        .add_plugins(DefaultPlugins)
        .add_state(GameState::InitialMenu)
        .add_startup_system(display::setup)
        .add_plugin(screen::ScreenPlugin)        
        .add_plugin(cursor::CursorPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}