use bevy::prelude::*;

use crate::cursor::{CURSOR_BOX, Cursor, PositionCursorOnEntity};
use crate::display::*;
use crate::game::Game;
use crate::gamestate::GameState;
use crate::system;
use super::board;

pub struct TurnMenuPlugin;

impl Plugin for TurnMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TurnMenuEvent>()

            .add_systems((turn_menu_setup, system::hide_board_entities).in_schedule(OnEnter(GameState::TurnMenu)))
            .add_system(turn_menu_keyboard.in_set(OnUpdate(GameState::TurnMenu)))
            .add_system(system::despawn_screen::<TurnMenu>.in_schedule(OnExit(GameState::TurnMenu)))

            // Specific transition/setup when going to next player
            .add_system(turn_menu_transition.in_set(OnUpdate(GameState::TurnMenuTransition)))

            .add_system(turn_menu_examine_spell_setup.in_schedule(OnEnter(GameState::TurnMenuExamineSpell)))
            .add_system(turn_menu_examine_spell_keyboard.in_set(OnUpdate(GameState::TurnMenuExamineSpell)))
            .add_system(turn_menu_choose_spell_keyboard.in_set(OnUpdate(GameState::TurnMenuExamineSpell)))
            .add_system(system::despawn_screen::<ExamineSpellScreen>.in_schedule(OnExit(GameState::TurnMenuExamineSpell)))

            .add_system(turn_menu_examine_one_spell_setup.in_schedule(OnEnter(GameState::TurnMenuExamineOneSpell)))
            .add_system(turn_menu_examine_one_spell_keyboard.in_set(OnUpdate(GameState::TurnMenuExamineOneSpell)))
            .add_system(system::despawn_screen::<ExamineOneSpellScreen>.in_schedule(OnExit(GameState::TurnMenuExamineOneSpell)))

            .add_system(turn_menu_select_spell_setup.in_schedule(OnEnter(GameState::TurnMenuSelectSpell)))
            .add_system(turn_menu_select_spell_keyboard.in_set(OnUpdate(GameState::TurnMenuSelectSpell)))
            .add_system(turn_menu_choose_spell_keyboard.in_set(OnUpdate(GameState::TurnMenuSelectSpell)))
            .add_system(system::despawn_screen::<SelectSpellScreen>.in_schedule(OnExit(GameState::TurnMenuSelectSpell)))

            .add_system(turn_menu_examine_board_setup.in_schedule(OnEnter(GameState::TurnMenuExamineBoard)))
            .add_system(system::show_board_entities.in_schedule(OnEnter(GameState::TurnMenuExamineBoard)))
            .add_system(turn_menu_examine_board_keyboard.in_set(OnUpdate(GameState::TurnMenuExamineBoard)))
            .add_system(board::board_describe_piece.in_set(OnUpdate(GameState::TurnMenuExamineBoard)))
            .add_system(turn_menu_examine_board_exit.in_schedule(OnExit(GameState::TurnMenuExamineBoard)))
            .add_system(system::hide_board_entities.in_schedule(OnExit(GameState::TurnMenuExamineBoard)))
            ;
    }
}

#[derive(Component, Clone, Copy)]
struct TurnMenu;

fn turn_menu_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    keys.clear();
    print_text(&g.get_player().name, &mut commands, g.fah(), Vec2::new(1.0, 7.0), WHITE, TurnMenu);
    print_text("1. Examine Spells", &mut commands, g.fah(), Vec2::new(1.0, 5.0), WHITE, TurnMenu);
    print_text("2. Select Spell", &mut commands, g.fah(), Vec2::new(1.0, 4.0), WHITE, TurnMenu);
    print_text("3. Examine Board", &mut commands, g.fah(), Vec2::new(1.0, 3.0), WHITE, TurnMenu);
    print_text("4. Continue with Game", &mut commands, g.fah(), Vec2::new(1.0, 2.0), WHITE, TurnMenu);
}

fn turn_menu_keyboard(
    mut state: ResMut<NextState<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Key1) {
        keys.reset(KeyCode::Key4);
        state.set(GameState::TurnMenuExamineSpell);
    }
    if keys.just_pressed(KeyCode::Key2) {
        keys.reset(KeyCode::Key2);
        state.set(GameState::TurnMenuSelectSpell);
    }
    if keys.just_pressed(KeyCode::Key3) {
        keys.reset(KeyCode::Key3);
        state.set(GameState::TurnMenuExamineBoard);
    }
    if keys.just_pressed(KeyCode::Key4) {
        keys.reset(KeyCode::Key4);
        state.set(GameState::TurnMenuTransition);
    }
}

fn turn_menu_transition(
    mut state: ResMut<NextState<GameState>>,
    mut g: ResMut<Game>,
    mut cursor: ResMut<Cursor>,
) {
    g.player_turn += 1;
    if g.player_turn >= g.players {
        g.player_turn = 0;
        cursor.set_visible();
        state.set(GameState::CastSpellSetup);
    } else {
        state.set(GameState::TurnMenu);
    }
}

fn turn_menu_choose_spell_setup(
    mut commands: Commands,
    g: Res<Game>,
    screen: impl Component + std::marker::Copy,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    let mut n_player = (g.player_info.len()+1).to_string();
    n_player.push_str("'s spells");
    print_text(&n_player, &mut commands, g.fah(), Vec2::new(0.5, 9.0), WHITE, screen);
    let player = g.get_player();
    for (i, spell) in (0_u8..).zip(player.spells.spells.iter()) {
        let x = if 1 == i % 2 { 7.0 } else { 0.5 };
        let mut name_str = ((i+65) as char).to_string();
        name_str.push_str(spell.get_sep());
        name_str.push_str(&spell.name());
        print_text(&name_str, &mut commands, g.fah(), Vec2::new(x, 8.0-f32::from(i/2)), spell.casting_chance_color(), screen);
    }
    ev_text.send(BottomTextEvent::from("      Press 0 to exit"));
}

#[derive(Component, Clone, Copy)]
struct ExamineSpellScreen;

fn turn_menu_examine_spell_setup(
    commands: Commands,
    g: Res<Game>,
    ev_text: EventWriter<BottomTextEvent>,
) {
    turn_menu_choose_spell_setup(commands, g, ExamineSpellScreen, ev_text);
}

fn turn_menu_choose_spell_keyboard(
    mut state: ResMut<NextState<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
    mut char_evr: ResMut<Events<ReceivedCharacter>>,
    g: Res<Game>,
    mut ev_choose_spell: EventWriter<TurnMenuEvent>,
) {
    let player = g.get_player();
    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        state.set(GameState::TurnMenu);
    }
    for ev in char_evr.drain() {
        let c = ev.char as usize;
        if c >= 65 && c <= 65 + player.spells.len() {
            let choice = c-65;
            info!("Chosen spell {choice}");
            ev_choose_spell.send(TurnMenuEvent(choice));
        }
        if c >= 97 && c <= 97 + player.spells.len() {
            let choice = c-97;
            info!("Chosen spell {choice}");
            ev_choose_spell.send(TurnMenuEvent(choice));
        }
    }
}

struct TurnMenuEvent(usize);

fn turn_menu_examine_spell_keyboard(
    mut state: ResMut<NextState<GameState>>,
    mut ev_choose_spell: EventReader<TurnMenuEvent>,
) {
    for _ in ev_choose_spell.iter() {
        print!("LEAVE examine spell, set state TurnMenuExamineOneSpell");
        state.set(GameState::TurnMenuExamineOneSpell);
    }
}

#[derive(Component, Clone, Copy)]
struct ExamineOneSpellScreen;

fn turn_menu_examine_one_spell_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_choose_spell: EventReader<TurnMenuEvent>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    for ev in ev_choose_spell.iter() {
        let spell_id = ev.0;
        // FIXME
        let spell = g.get_player().spells.get_spell(spell_id);
        print_text(&spell.name(), &mut commands, g.fah(), Vec2::new(1.0, 9.0), WHITE, ExamineOneSpellScreen);
        for (i, line) in spell.get_description().iter().enumerate() {
            print_text(line, &mut commands, g.fah(), Vec2::new(1.0, 8.0-i as f32), WHITE, ExamineOneSpellScreen);
        }
    }
    ev_text.send(BottomTextEvent::from("      Any key to exit"));
}

fn turn_menu_examine_one_spell_keyboard(
    mut state: ResMut<NextState<GameState>>,
    mut char_evr: ResMut<Events<ReceivedCharacter>>,
) {
    for _ in char_evr.drain() {
        state.set(GameState::TurnMenuExamineSpell);
    }
}

#[derive(Component, Clone, Copy)]
struct SelectSpellScreen;

fn turn_menu_select_spell_setup(
    commands: Commands,
    g: Res<Game>,
    ev_text: EventWriter<BottomTextEvent>,
) {
    turn_menu_choose_spell_setup(commands, g, SelectSpellScreen, ev_text);
}

#[derive(Default)]
struct PickIllusion(bool);

fn turn_menu_select_spell_keyboard(
    mut state: ResMut<NextState<GameState>>,
    mut ev_choose_spell: EventReader<TurnMenuEvent>,
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
            state.set(GameState::TurnMenu);
        }
        if keys.just_pressed(KeyCode::N) {
            keys.reset(KeyCode::N);
            g.get_player_mut().spells.illusion = false;
            (*pickillusion).0 = false;
            state.set(GameState::TurnMenu);
        }
    } else {
        for ev in ev_choose_spell.iter() {
            g.get_player_mut().spells.set_chosen(ev.0);
            let can_be_illusion = g.get_player_mut().spells.get_chosen_spell().unwrap().can_be_illusion();
            if can_be_illusion {
                (*pickillusion).0 = true;
                ev_text.send(BottomTextEvent::from("Illusion? (Y/N)"));
            } else {
                state.set(GameState::TurnMenu);
            }
        }
    }
}

fn turn_menu_examine_board_setup(
    mut ev_text: EventWriter<BottomTextEvent>,
    mut cursor: ResMut<Cursor>,
    g: Res<Game>,
    mut ev_cursor_pos: EventWriter<PositionCursorOnEntity>,
) {
    ev_text.send(BottomTextEvent::from("      Press 0 to exit"));
    cursor.set_type(CURSOR_BOX);
    cursor.set_visible();
    cursor.hide_till_moved();
    let player = g.get_player();
    ev_cursor_pos.send(PositionCursorOnEntity(player.handle.unwrap()));
}

fn turn_menu_examine_board_keyboard(
    mut state: ResMut<NextState<GameState>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        state.set(GameState::TurnMenu);
    }
}

fn turn_menu_examine_board_exit(
    mut ev_text: EventWriter<BottomTextEvent>,
    mut cursor: ResMut<Cursor>,
) {
    ev_text.send(BottomTextEvent::clear());
    cursor.set_invisible();
}
