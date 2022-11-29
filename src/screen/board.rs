use bevy::prelude::*;
use std::collections::HashSet;
use crate::board::{GameBoard, BoardMove, MoveableComponent, BoardPutEntity};
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{BottomTextEvent};
use crate::player::CastFailed;
use crate::system::{self, Named, BelongsToPlayer};
use crate::cursor::{CURSOR_SPELL, CURSOR_BOX, CursorMovedEvent, CURSOR_FLY, PositionCursorOnEntity};

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
            .add_system_set(SystemSet::on_exit(GameState::GameMoveOnePlayer).with_system(move_one_finish))

            .add_system_set(SystemSet::on_enter(GameState::NextTurn).with_system(system::hide_board_entities))
            .add_system_set(SystemSet::on_update(GameState::NextTurn).with_system(next_turn));
    }
}

// Game -push-> GameCastSpell
//   | ^------pop------/
//  set
//    \-> GameMoveSetup -push-> GameMoveOnePlayer
//                 ^------pop------/
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
    mut query: Query<&mut Transform>,
) {
    println!("cast_spell_setup");
    g.cursor.set_type(CURSOR_SPELL);
    let player = g.get_player();
    let mut transform = query.get_mut(player.handle.unwrap()).unwrap();
    let spell = player.spells.get_chosen_spell();
    if spell.is_none() {
        return;
    }
    let pos = Vec2{ x: transform.translation.x, y: transform.translation.y };
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
    mut ev_board_put: EventWriter<BoardPutEntity>,
    mut query: Query<&mut Transform>,
) {
    let player = g.get_player();
    let mut transform = query.get_mut(player.handle.unwrap()).unwrap();
    let pos = Vec2{ x: transform.translation.x, y: transform.translation.y };
    let spell = player.spells.get_chosen_spell();
    if spell.is_none() {
        println!("STATE POP - no spell");
        state.pop().unwrap();
        return;
    }
    let tah = g.tah();
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        let to = g.cursor.get_pos_v();
        let player = g.get_player_mut();
        match player.cast(pos, to, &mut commands, tah) {
            Ok(e) => {
                ev_board_put.send(BoardPutEntity{
                    entity: e.unwrap(),
                    pos
                });
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
    mut ev_cursor_pos: EventWriter<PositionCursorOnEntity>,
) {
    println!("Move one for player {}", g.player_turn);
    let player = g.get_player();
    let mut s = player.name.clone();
    ev_cursor_pos.send(PositionCursorOnEntity(player.handle.unwrap()));
    s.push_str("'s turn");
    ev_text.send(BottomTextEvent::from(&s));
    moving.has_moved = HashSet::new();
}

fn move_one_keyboard(
    mut g: ResMut<Game>,
    board: Res<GameBoard>,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut query: Query<(&Named, &MoveableComponent, Option<&BelongsToPlayer>, &mut Transform,)>,
    mut playername: Query<&Named>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut moving: ResMut<Moving>,
    mut ev_move: EventWriter<BoardMove>,
) {
    if let Some(entity) = moving.entity {
        if moving.flying {
            if keys.just_pressed(KeyCode::S) {
                keys.reset(KeyCode::S);
                let cursor_pos;
                {
                    let cursor = &mut g.cursor;
                    cursor.set_type(CURSOR_BOX);
                    cursor_pos = cursor.get_pos_v();
                }
                ev_move.send(BoardMove{
                    from: moving.pos,
                    to: cursor_pos,
                });
                moving.distance_left = 0;
                moving.entity = None;
            }
        } else {
            for cur in ev_cursor.iter() {
                println!("Got cursor moved event in move one");
                ev_text.send(BottomTextEvent::clear());
                ev_move.send(BoardMove{
                    from: moving.pos,
                    to: **cur,
                });
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
            state.pop().unwrap();
            println!("Next player turn");
        }
        if keys.just_pressed(KeyCode::S) {
            keys.reset(KeyCode::S);
            let pos = g.cursor.get_pos_v();
            println!("Find thing at {}, {} to move", pos.x, pos.y);
            if board.has_entity(pos) {
                let e = board.get_entity(pos).unwrap();
                let (_, moveable, belongs, _) = query.get_mut(e).unwrap();
                let belongs_entity;
                if let Some(belongs) = belongs {
                    belongs_entity = belongs.player_entity;
                } else {
                    belongs_entity = e;
                }
                if g.get_player().handle.unwrap() == belongs_entity && !moving.has_moved.contains(&e) {
                    moving.has_moved.insert(e);
                    println!("Does belong to this player");
                    moving.flying = moveable.flying;
                    moving.entity = Some(e);
                    moving.pos = pos;
                    moving.distance_left = moveable.movement;
                    if moving.flying {
                        g.cursor.set_type(CURSOR_FLY);
                    } else {
                        g.cursor.set_invisible();
                    }
                    let mut text = String::from("Movement range=");
                    text.push_str(&moveable.movement.to_string());
                    ev_text.send(BottomTextEvent::from(&text));
                }
            }
        }
        for cur in ev_cursor.iter() {
            println!("Got cursor moved event, clear");
            if board.has_entity(**cur) {
                let e = board.get_entity(**cur).unwrap();
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

fn move_one_finish(mut g: ResMut<Game>) {
    println!("Finish move one, increment player turn");
    g.player_turn += 1;
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