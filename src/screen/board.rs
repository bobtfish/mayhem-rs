use bevy::prelude::*;
use std::collections::HashSet;
use crate::creature::CreatureComponent;
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{BottomTextEvent};
use crate::player::CastFailed;
use crate::system::{self, Named, BelongsToPlayer};
use crate::cursor::{CURSOR_SPELL, CURSOR_BOX, CursorMovedEvent, CURSOR_FLY};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Moving>()
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(game_setup)
                    .with_system(system::show_board_entities)
            )
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game_next))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(game_exit))
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

// Game -push-> GameCastSpell
//   | ^------pop------/
//  set
//    \-> GameMoveSetup
//
#[derive(Resource, Default)]
struct Moving {
    entity: Option<Entity>,
    distance_left: u8,
    flying: bool,
    pos: Vec2,
    has_moved: HashSet<Entity>,
}

fn game_setup(
    mut g: ResMut<Game>,
) {
    println!("game_setup");
    g.player_turn = 0;
    g.cursor.set_visible();
}

fn game_next(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
) {
    println!("game_next");
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

fn game_exit(
) {
    println!("Exit Game state");
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
                println!("State POP");
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
    mut moving: ResMut<Moving>,
) {
    println!("Move one for player {}", g.player_turn);
    let player = g.get_player();
    let mut s = player.name.clone();
    let pos = player.pos;
    g.cursor.set_pos(pos);
    s.push_str("'s turn");
    ev_text.send(BottomTextEvent::from(&s));
    moving.has_moved = HashSet::new();
}

fn move_one_keyboard(
    mut g: ResMut<Game>,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut query: Query<(&Named, &CreatureComponent, Option<&BelongsToPlayer>, &mut Transform,)>,
    mut playername: Query<&Named>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut moving: ResMut<Moving>,
) {
    if let Some(entity) = moving.entity {
        let (_, _, _, mut transform) = query.get_mut(entity).unwrap();
        if moving.flying {
            if keys.just_pressed(KeyCode::S) {
                keys.reset(KeyCode::S);
                let cursor_pos;
                {
                    let cursor = &mut g.cursor;
                    cursor.set_type(CURSOR_BOX);
                    cursor_pos = cursor.get_pos_v();
                }
                let board = g.board_mut();
                board.pop_entity(moving.pos);
                board.put_entity(cursor_pos, entity);
                *transform = transform.with_translation(cursor_pos.extend(1.0));
                moving.distance_left = 0;
                moving.entity = None;
            }
        } else {
            for cur in ev_cursor.iter() {
                println!("Got cursor moved event in move one");
                ev_text.send(BottomTextEvent::clear());
                g.board_mut().pop_entity(moving.pos);
                g.board_mut().put_entity(**cur, entity);
                *transform = transform.with_translation(cur.extend(1.0));
                moving.distance_left -= 1;
                if moving.distance_left == 0 {
                    println!("No movement left, clear entity");
                    moving.entity = None;
                    g.cursor.set_visible();
                }
            }
        }
    } else {
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
            if g.board().has_entity(pos) {
                let e = g.board().get_entity(pos).unwrap();
                let (_, creature, belongs, _) = query.get_mut(e).unwrap();
                if let Some(belongs) = belongs {
                    if g.get_player().handle.unwrap() == belongs.player_entity && !moving.has_moved.contains(&e) {
                        moving.has_moved.insert(e);
                        println!("Does belong to this player");
                        moving.flying = creature.flying;
                        moving.entity = Some(e);
                        moving.pos = pos;
                        moving.distance_left = creature.movement;
                        if moving.flying {
                            g.cursor.set_type(CURSOR_FLY);
                        } else {
                            g.cursor.set_invisible();
                        }
                        ev_text.send(BottomTextEvent::from("Movement range=xxx"));
                    }
                }
            }
        }
        for cur in ev_cursor.iter() {
            println!("Got cursor moved event, clear");
            if g.board().has_entity(**cur) {
                let e = g.board().get_entity(**cur).unwrap();
                let (named, _, belongs, _) = query.get_mut(e).unwrap();
                let mut text = named.name.clone();
                if let Some(belongs) = belongs {
                    text.push('(');
                    let player_named = playername.get_mut(belongs.player_entity);
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