use bevy::{prelude::*, input::keyboard::KeyboardInput};

use super::{GameState, despawn_screen, Game};
use crate::display::*;

pub struct ChooseSpellPlugin;

impl Plugin for ChooseSpellPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::PlayerMenu).with_system(player_menu_setup))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenu).with_system(player_menu_keyboard))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenu).with_system(despawn_screen::<PlayerMenu>))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuTransition).with_system(player_menu_transition))
            .add_system_set(SystemSet::on_enter(GameState::PlayerMenuExamineSpell).with_system(player_menu_examine_spell_setup))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenuExamineSpell).with_system(despawn_screen::<ExamineSpellScreen>))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuExamineSpell).with_system(player_menu_examine_spell_keyboard))
            .add_system_set(SystemSet::on_enter(GameState::PlayerMenuExamineOneSpell).with_system(player_menu_examine_one_spell_setup))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenuExamineOneSpell).with_system(despawn_screen::<ExamineOneSpellScreen>))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuExamineOneSpell).with_system(player_menu_examine_one_spell_keyboard))
            .add_system_set(SystemSet::on_enter(GameState::PlayerMenuSelectSpell).with_system(player_menu_select_spell_setup))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuSelectSpell).with_system(player_menu_select_spell_keyboard))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenuSelectSpell).with_system(despawn_screen::<SelectSpellScreen>))
            .add_system_set(SystemSet::on_enter(GameState::PlayerMenuExamineBoard).with_system(player_menu_examine_board_setup))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuExamineBoard).with_system(player_menu_examine_board_keyboard))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenuExamineBoard).with_system(despawn_screen::<ExamineBoardScreen>))
            ;
    }
}

#[derive(Component, Clone, Copy)]
struct PlayerMenu;

fn player_menu_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    keys.clear();
    print_text(&g.get_player().name, &mut commands, g.fah.clone(), Vec2::new(2.0, 8.0), PlayerMenu);
    print_text("1. Examine Spells", &mut commands, g.fah.clone(), Vec2::new(2.0, 6.0), PlayerMenu);
    print_text("2. Select Spell", &mut commands, g.fah.clone(), Vec2::new(2.0, 5.0), PlayerMenu);
    print_text("3. Examine Board", &mut commands, g.fah.clone(), Vec2::new(2.0, 4.0), PlayerMenu);
    print_text("4. Continue with Game", &mut commands, g.fah.clone(), Vec2::new(2.0, 3.0), PlayerMenu);
}

fn player_menu_keyboard(
    mut state: ResMut<State<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Key1) {
        keys.reset(KeyCode::Key4);
        state.set(GameState::PlayerMenuExamineSpell).unwrap();
    }
    if keys.just_pressed(KeyCode::Key2) {
        keys.reset(KeyCode::Key2);
        state.set(GameState::PlayerMenuSelectSpell).unwrap();
    }
    if keys.just_pressed(KeyCode::Key3) {
        keys.reset(KeyCode::Key3);
        state.set(GameState::PlayerMenuExamineBoard).unwrap();
    }
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

fn player_menu_choose_spell_setup(
    mut commands: Commands,
    g: Res<Game>,
    screen: impl Component + std::marker::Copy,
) {
    let mut n_player = (g.player_info.len()+1).to_string();
    n_player.push_str("'s spells");
    print_text(&*n_player, &mut commands, g.fah.clone(), Vec2::new(0.5, 10.0), screen);
    let player = g.get_player();
    let mut i: u8 = 0;
    for spell in player.spells() {
        let mut x = 0.5;
        if i % 2 == 1 {
            x = 7.0;
        }
        let mut name_str = ((i+65) as char).to_string();
        name_str.push_str(spell.get_sep());
        name_str.push_str(&*spell.name);
        print_text(&*name_str, &mut commands, g.fah.clone(), Vec2::new(x, 9.0-((i/2) as f32)), screen);
        i = i + 1;
    }
    print_text("Press 0 to exit", &mut commands, g.fah.clone(), Vec2::new(4.0, 0.0), screen);
}

#[derive(Component, Clone, Copy)]
struct ExamineSpellScreen;

fn player_menu_examine_spell_setup(
    commands: Commands,
    g: Res<Game>,
) {
    player_menu_choose_spell_setup(commands, g, ExamineSpellScreen);
}

fn player_menu_choose_spell_keyboard(
    state: &mut ResMut<State<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    g: Res<Game>,
) -> Option<usize> {
    let player = g.get_player();
    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        (*state).set(GameState::PlayerMenu).unwrap();
    }
    let mut ret: Option<usize> = None;
    for ev in char_evr.iter() {
        println!("player_menu_choose_spell_keyboard - iter");
        let c = ev.char as usize;
        if c >= 65 && c <= 65 + player.spells().len() {
            let choice = c-65;
            println!("Chosen spell {}", choice);
            ret = Some(choice);
        }
        if c >= 97 && c <= 97 + player.spells().len() {
            let choice = c-97;
            println!("Chosen spell {}", choice);
            ret = Some(choice);
        }
    }
    return ret;
}

fn player_menu_examine_spell_keyboard(
    mut state: ResMut<State<GameState>>,
    keys: ResMut<Input<KeyCode>>,
    char_evr: EventReader<ReceivedCharacter>,
    g: Res<Game>,
) {
    //println!("player_menu_examine_spell_keyboard");
    let choice = player_menu_choose_spell_keyboard(&mut state, keys, char_evr, g);
    if choice.is_some() {
        println!("player_menu_examine_spell_keyboard - state change to PlayerMenuExamineOneSpell");
        state.set(GameState::PlayerMenuExamineOneSpell).unwrap();
    }
}

#[derive(Component, Clone, Copy)]
struct ExamineOneSpellScreen;

fn player_menu_examine_one_spell_setup(
    mut commands: Commands,
    g: Res<Game>,
) {
    println!("player_menu_examine_one_spell_setup");
    print_text(&g.get_player().spells()[0].name, &mut commands, g.fah.clone(), Vec2::new(4.0, 10.0), ExamineOneSpellScreen);
}

fn player_menu_examine_one_spell_keyboard(
    mut state: ResMut<State<GameState>>,
    mut key_evr: EventReader<KeyboardInput>,
    g: Res<Game>,
) {
    println!("player_menu_examine_one_spell_keyboard");

    use bevy::input::ButtonState;

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("player_menu_examine_one_spell_keyboard - state to PlayerMenuExamineSpell");
                state.set(GameState::PlayerMenuExamineSpell).unwrap();
            }
            ButtonState::Released => {}
        }
    }
}

#[derive(Component, Clone, Copy)]
struct SelectSpellScreen;

fn player_menu_select_spell_setup(
    commands: Commands,
    g: Res<Game>,
) {
    player_menu_choose_spell_setup(commands, g, SelectSpellScreen);
}

fn player_menu_select_spell_keyboard(
    mut state: ResMut<State<GameState>>,
    keys: ResMut<Input<KeyCode>>,
    char_evr: EventReader<ReceivedCharacter>,
    g: Res<Game>,
) {
    let choice = player_menu_choose_spell_keyboard(&mut state, keys, char_evr, g);
    if choice.is_some() {
        state.set(GameState::PlayerMenu).unwrap();
    }
}

#[derive(Component, Clone, Copy)]
struct ExamineBoardScreen;

fn player_menu_examine_board_setup(
    mut commands: Commands,
    g: Res<Game>,
) {
    print_text("Press 0 to exit", &mut commands, g.fah.clone(), Vec2::new(2.0, 6.0), ExamineBoardScreen);
}

fn player_menu_examine_board_keyboard(
    mut state: ResMut<State<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        state.set(GameState::PlayerMenu).unwrap();
    }
}