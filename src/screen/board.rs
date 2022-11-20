use bevy::prelude::*;
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{BottomTextEvent};
use crate::player::CastFailed;
use crate::system::{self, Named, BelongsToPlayer};
use crate::cursor::{CURSOR_SPELL, CURSOR_BOX, CursorMovedEvent};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(game_setup)
                    .with_system(system::show_board_entities)
            )
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game_next))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(system::despawn_screen::<OnGameScreen>))
            .add_system_set(SystemSet::on_enter(GameState::GameCastSpell).with_system(cast_spell_setup))
            .add_system_set(SystemSet::on_update(GameState::GameCastSpell).with_system(cast_spell_keyboard))
            .add_system_set(SystemSet::on_exit(GameState::GameCastSpell).with_system(cast_spell_finish))
            .add_system_set(SystemSet::on_enter(GameState::GameMoveSetup).with_system(move_setup))
            .add_system_set(SystemSet::on_update(GameState::GameMoveSetup).with_system(move_next))
            .add_system_set(SystemSet::on_enter(GameState::GameMoveOnePlayer).with_system(move_one_setup))
            .add_system_set(SystemSet::on_update(GameState::GameMoveOnePlayer).with_system(move_one_keyboard))
            .add_system_set(SystemSet::on_enter(GameState::NextTurn).with_system(system::hide_board_entities))
            .add_system_set(SystemSet::on_update(GameState::NextTurn).with_system(next_turn));
    }
}

// Tag component used to tag entities added on the menu screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(
    //mut commands: Commands,
    mut g: ResMut<Game>,
) {
    //let tah = g.tah();
    // Wizard with bow
    //spawn_anim(&mut commands, g.tah(), Vec2::splat(2.0), 120, 8);
    // Spell/splodey thing
    //spawn_anim(&mut commands, g.tah(), Vec2::splat(1.0), 180, 4);

    //let creature = spawn_anim(&mut commands, g.tah(), Vec2::splat(3.0), 210, 4);
    //commands.entity(creature).insert(Mortal{is_alive: false});

    //let creature_map = load_creatures();
    //creature_map.get("Pegasus").unwrap().to_entity(Vec2::splat(4.0), &mut commands, g.tah());
    g.player_turn = 0;
    g.cursor.set_visible();
}

fn game_next(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
) {
    if g.player_turn >= g.players {
        g.player_turn = 0;
        println!("Spell casting finished, do movement now");
        state.set(GameState::GameMoveSetup).unwrap();
    } else {
        println!("Player turn to cast spell");
        // Next player's turn to cast a spell
        state.push(GameState::GameCastSpell).unwrap();
    }
}

fn cast_spell_setup(
    mut g: ResMut<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    println!("cast_spell_setup");
    g.cursor.set_type(CURSOR_SPELL);
    let player = g.get_player();
    let spell = player.spells.get_chosen_spell();
    if spell.is_none() {
        return;
    }
    let pos = player.pos;
    let spell = spell.unwrap();
    let mut text = String::from(&player.name);
    text.push(' ');
    text.push_str(&spell.name());
    ev_text.send(BottomTextEvent::from(&text));
    g.cursor.set_pos(pos);
}

fn cast_spell_keyboard(
    mut keys: ResMut<Input<KeyCode>>,
    mut g: ResMut<Game>,
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
) {
    let player = g.get_player();
    let spell = player.spells.get_chosen_spell();
    if spell.is_none() {
        println!("STATE POP - no spell");
        state.pop().unwrap();
        return;
    }
    let tah = g.tah();
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        let pos = g.cursor.get_pos_v();
        let player = g.get_player_mut();
        match player.cast(pos, &mut commands, tah) {
            Ok(e) => {
                g.board_mut().put_entity(pos, e.unwrap());
                state.pop().unwrap();
            },
            Err(CastFailed::OutOfRange) => {
                ev_text.send(BottomTextEvent::from("Out of range"));
                g.cursor.hide_till_moved();
            }
        }
    }
    for _ in ev_cursor.iter() {
        println!("Got cursor moved event, clear");
        ev_text.send(BottomTextEvent::clear());
    }
}

fn cast_spell_finish(mut g: ResMut<Game>) {
    println!("Finish cast spell, increment player turn");
    g.player_turn += 1;
}

fn move_setup(
    mut g: ResMut<Game>,
) {
    g.cursor.set_type(CURSOR_BOX);
    g.player_turn = 0;
    println!("In move setup");
}

fn move_next(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
) {
    if g.player_turn >= g.players {
        g.player_turn = 0;
        println!("Moving finished, next turn now");
        state.set(GameState::NextTurn).unwrap();
    } else {
        println!("Player turn to move");
        state.push(GameState::GameMoveOnePlayer).unwrap();
    }
}

fn move_one_setup(
    mut g: ResMut<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    println!("Move one for player {}", g.player_turn);
    let player = g.get_player();
    let mut s = player.name.clone();
    let pos = player.pos;
    g.cursor.set_pos(pos);
    s.push_str("'s turn");
    ev_text.send(BottomTextEvent::from(&s));
}

fn move_one_keyboard(
    mut g: ResMut<Game>,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut query: Query<(&Named, Option<&BelongsToPlayer>)>,
    mut playername: Query<&Named>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        g.player_turn += 1;
        state.pop().unwrap();
        println!("Next player turn");
    }
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        let pos = g.cursor.get_pos_v();
        println!("Find thing at {}, {} to move", pos.x, pos.y);
    }
    for cur in ev_cursor.iter() {
        println!("Got cursor moved event, clear");
        if g.board().has_entity(**cur) {
            let e = g.board().get_entity(**cur).unwrap();
            let (named, belongs) = query.get_mut(e).unwrap();
            let mut text = named.name.clone();
            if belongs.is_some() {
                text.push('(');
                let player_named = playername.get_mut(belongs.unwrap().player_entity);
                text.push_str(&player_named.unwrap().name);
                text.push(')');
            }
            ev_text.send(BottomTextEvent::from(&text));
        }
        else {
            ev_text.send(BottomTextEvent::clear());
        }
    }
}

fn next_turn(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
) {
    println!("next_turn set state GameState::PlayerMenu");
    g.player_turn = 0;
    g.cursor.set_invisible();
    state.set(GameState::PlayerMenu).unwrap();
}