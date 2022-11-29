use bevy::prelude::*;

use crate::board::BoardPutEntity;
use crate::display::*;
use crate::game::Game;
use crate::gamestate::GameState;
use crate::system;

pub struct PlayerMenuPlugin;

impl Plugin for PlayerMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerMenuEvent>()
            .add_system_set(SystemSet::on_enter(GameState::PlayerMenu).with_system(player_menu_setup))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenu).with_system(player_menu_keyboard))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenu).with_system(system::despawn_screen::<PlayerMenu>))
            // Specific transition/setup when going to next player
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuTransition).with_system(player_menu_transition))

            .add_system_set(SystemSet::on_enter(GameState::PlayerMenuExamineSpell).with_system(player_menu_examine_spell_setup))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenuExamineSpell).with_system(system::despawn_screen::<ExamineSpellScreen>))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuExamineSpell).with_system(player_menu_examine_spell_keyboard))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuExamineSpell).with_system(player_menu_choose_spell_keyboard))

            .add_system_set(SystemSet::on_enter(GameState::PlayerMenuExamineOneSpell).with_system(player_menu_examine_one_spell_setup))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenuExamineOneSpell).with_system(system::despawn_screen::<ExamineOneSpellScreen>))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuExamineOneSpell).with_system(player_menu_examine_one_spell_keyboard))

            .add_system_set(SystemSet::on_enter(GameState::PlayerMenuSelectSpell).with_system(player_menu_select_spell_setup))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuSelectSpell).with_system(player_menu_select_spell_keyboard))
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuSelectSpell).with_system(player_menu_choose_spell_keyboard))
            .add_system_set(SystemSet::on_exit(GameState::PlayerMenuSelectSpell).with_system(system::despawn_screen::<SelectSpellScreen>))

            .add_system_set(SystemSet::on_enter(
                GameState::PlayerMenuExamineBoard)
                .with_system(player_menu_examine_board_setup)
                .with_system(system::show_board_entities)
            )
            .add_system_set(SystemSet::on_update(GameState::PlayerMenuExamineBoard).with_system(player_menu_examine_board_keyboard))
            .add_system_set(
                SystemSet::on_exit(GameState::PlayerMenuExamineBoard)
                .with_system(system::hide_board_entities)
                .with_system(player_menu_examine_board_exit)
            )
            ;
    }
}

#[derive(Component, Clone, Copy)]
struct PlayerMenu;

fn player_menu_setup(
    mut commands: Commands,
    mut g: ResMut<Game>,
    mut keys: ResMut<Input<KeyCode>>,
    mut ev_board_put: EventWriter<BoardPutEntity>,
) {
    let tah = g.tah();
    keys.clear();
    let positions = crate::player::get_start_positions(g.players as usize).unwrap();
    for (i, p) in &mut g.player_info.iter_mut().enumerate() {
        let pos = positions[i];
        p.spawn(&mut commands, tah.clone(), pos);
        ev_board_put.send(BoardPutEntity { entity: p.handle.unwrap(), pos });
    }
    print_text(&g.get_player().name, &mut commands, g.fah(), Vec2::new(1.0, 7.0), PlayerMenu);
    print_text("1. Examine Spells", &mut commands, g.fah(), Vec2::new(1.0, 5.0), PlayerMenu);
    print_text("2. Select Spell", &mut commands, g.fah(), Vec2::new(1.0, 4.0), PlayerMenu);
    print_text("3. Examine Board", &mut commands, g.fah(), Vec2::new(1.0, 3.0), PlayerMenu);
    print_text("4. Continue with Game", &mut commands, g.fah(), Vec2::new(1.0, 2.0), PlayerMenu);
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
    g.player_turn += 1;
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
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    let mut n_player = (g.player_info.len()+1).to_string();
    n_player.push_str("'s spells");
    print_text(&n_player, &mut commands, g.fah(), Vec2::new(0.5, 9.0), screen);
    let player = g.get_player();
    for (i, spell) in (0_u8..).zip(player.spells.spells.iter()) {
        let x = if 1 == i % 2 { 7.0 } else { 0.5 };
        let mut name_str = ((i+65) as char).to_string();
        name_str.push_str(spell.get_sep());
        name_str.push_str(&spell.name());
        print_text(&name_str, &mut commands, g.fah(), Vec2::new(x, 8.0-f32::from(i/2)), screen);
    }
    ev_text.send(BottomTextEvent::from("      Press 0 to exit"));
}

#[derive(Component, Clone, Copy)]
struct ExamineSpellScreen;

fn player_menu_examine_spell_setup(
    commands: Commands,
    g: Res<Game>,
    ev_text: EventWriter<BottomTextEvent>,
) {
    player_menu_choose_spell_setup(commands, g, ExamineSpellScreen, ev_text);
}

fn player_menu_choose_spell_keyboard(
    mut state: ResMut<State<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
    mut char_evr: ResMut<Events<ReceivedCharacter>>,
    g: Res<Game>,
    mut ev_choose_spell: EventWriter<PlayerMenuEvent>,
) {
    let player = g.get_player();
    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        (*state).set(GameState::PlayerMenu).unwrap();
    }
    for ev in char_evr.drain() {
        let c = ev.char as usize;
        if c >= 65 && c <= 65 + player.spells.len() {
            let choice = c-65;
            println!("Chosen spell {choice}");
            ev_choose_spell.send(PlayerMenuEvent(choice));
        }
        if c >= 97 && c <= 97 + player.spells.len() {
            let choice = c-97;
            println!("Chosen spell {choice}");
            ev_choose_spell.send(PlayerMenuEvent(choice));
        }
    }
}

struct PlayerMenuEvent(usize);

fn player_menu_examine_spell_keyboard(
    mut state: ResMut<State<GameState>>,
    mut ev_choose_spell: EventReader<PlayerMenuEvent>,
) {
    for _ in ev_choose_spell.iter() {
        state.set(GameState::PlayerMenuExamineOneSpell).unwrap();
    }
}

#[derive(Component, Clone, Copy)]
struct ExamineOneSpellScreen;

fn player_menu_examine_one_spell_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_choose_spell: EventReader<PlayerMenuEvent>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    for ev in ev_choose_spell.iter() {
        let spell_id = ev.0;
        // FIXME
        print_text(&g.get_player().spells.get_spell(spell_id).name(), &mut commands, g.fah(), Vec2::new(1.0, 10.0), ExamineOneSpellScreen);
    }
    // FIXME - add more spell details
    ev_text.send(BottomTextEvent::from("    Any key to exit"));
}

fn player_menu_examine_one_spell_keyboard(
    mut state: ResMut<State<GameState>>,
    mut char_evr: ResMut<Events<ReceivedCharacter>>,
) {
    for _ in char_evr.drain() {
        state.set(GameState::PlayerMenuExamineSpell).unwrap();
    }
}

#[derive(Component, Clone, Copy)]
struct SelectSpellScreen;

fn player_menu_select_spell_setup(
    commands: Commands,
    g: Res<Game>,
    ev_text: EventWriter<BottomTextEvent>,
) {
    player_menu_choose_spell_setup(commands, g, SelectSpellScreen, ev_text);
}

#[derive(Default)]
struct PickIllusion(bool);

fn player_menu_select_spell_keyboard(
    mut state: ResMut<State<GameState>>,
    mut ev_choose_spell: EventReader<PlayerMenuEvent>,
    mut g: ResMut<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut pickillusion: Local<PickIllusion>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if (*pickillusion).0 {
        if keys.just_pressed(KeyCode::Y) {
            keys.reset(KeyCode::Y);
            g.get_player_mut().spells.illusion = true;
            (*pickillusion).0 = false;
            state.set(GameState::PlayerMenu).unwrap();
        }
        if keys.just_pressed(KeyCode::N) {
            keys.reset(KeyCode::N);
            g.get_player_mut().spells.illusion = false;
            (*pickillusion).0 = false;
            state.set(GameState::PlayerMenu).unwrap();
        }
    } else {
        for ev in ev_choose_spell.iter() {
            g.get_player_mut().spells.set_chosen(ev.0);
            let can_be_illusion = g.get_player_mut().spells.get_chosen_spell().unwrap().can_be_illusion();
            if can_be_illusion {
                (*pickillusion).0 = true;
                ev_text.send(BottomTextEvent::from("Illusion? (Y/N)"));
            } else {
                state.set(GameState::PlayerMenu).unwrap();
            }
        }
    }
}

fn player_menu_examine_board_setup(
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    ev_text.send(BottomTextEvent::from("      Press 0 to exit"));
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

fn player_menu_examine_board_exit(
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    ev_text.send(BottomTextEvent::clear());
}