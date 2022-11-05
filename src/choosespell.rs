use bevy::prelude::*;

use super::{GameState, despawn_screen, Game};
use crate::display::*;

pub struct ChooseSpellPlugin;

impl Plugin for ChooseSpellPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::PlayerMenu).with_system(player_menu_setup))
            .add_system_set(
                SystemSet::on_update(GameState::PlayerMenu)
                    .with_system(player_menu_keyboard)
                )
            .add_system_set(
                SystemSet::on_exit(GameState::PlayerMenu)
                    .with_system(despawn_screen::<PlayerMenu>),
            )
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuTransition).with_system(player_menu_transition));
    }
}

#[derive(Component, Clone, Copy)]
struct PlayerMenu;

fn player_menu_setup(
    mut commands: Commands,
    g: Res<Game>,
) {
    print_text(&*g.player_info[g.player_turn as usize].name, &mut commands, g.fah.clone(), Vec2::new(2.0, 8.0), PlayerMenu);
    print_text("1. Examine Spells", &mut commands, g.fah.clone(), Vec2::new(2.0, 6.0), PlayerMenu);
    print_text("2. Select Spell", &mut commands, g.fah.clone(), Vec2::new(2.0, 5.0), PlayerMenu);
    print_text("3. Examine Board", &mut commands, g.fah.clone(), Vec2::new(2.0, 4.0), PlayerMenu);
    print_text("4. Continue with Game", &mut commands, g.fah.clone(), Vec2::new(2.0, 3.0), PlayerMenu);
}

fn player_menu_keyboard(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Key4) {
        keys.reset(KeyCode::Key4);
        state.set(GameState::PlayerMenuTransition).unwrap();
    }
}

fn player_menu_transition(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
) {
    g.player_turn = g.player_turn+1;
    if g.player_turn >= g.players {
        g.player_turn = 0;
        state.set(GameState::Game).unwrap();
    } else {
        state.set(GameState::PlayerMenu).unwrap();
    }
}

#[derive(Component, Clone, Copy)]
struct ExamineSpellScreen;

#[derive(Component, Clone, Copy)]
struct SelectSpellScreen;