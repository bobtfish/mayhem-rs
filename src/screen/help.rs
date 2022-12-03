use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{print_text, WHITE};
use crate::system;
pub struct HelpPlugin;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Help).with_system(help_setup))
            .add_system_set(SystemSet::on_update(GameState::Help).with_system(help_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::Help).with_system(system::despawn_screen::<HelpScreen>));
    }
}

#[derive(Component, Clone, Copy)]
struct HelpScreen;

fn help_setup(
    mut commands: Commands,
    g: Res<Game>,
//    mut ev_text: EventWriter<BottomTextEvent>,
) {
    print_text("  HELP SCREEN", &mut commands, g.fah(), Vec2::new(0.5, 8.0), WHITE, HelpScreen);
}

fn help_keyboard_input(
    mut state: ResMut<State<GameState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Return) {
        state.set(GameState::InitialMenu).unwrap();
        return
    }
}