use bevy::prelude::*;
use std::collections::HashSet;
use crate::board::{GameBoard, BoardMove, MoveableComponent};
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{BottomTextEvent};
use crate::system::{self, Named, BelongsToPlayer};
use crate::cursor::{CURSOR_BOX, CursorMovedEvent, CURSOR_FLY, PositionCursorOnEntity, Cursor};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Moving>()
            .add_system_set(SystemSet::on_enter(GameState::MoveSetup).with_system(move_setup))
            .add_system_set(SystemSet::on_update(GameState::MoveSetup).with_system(move_next))
            .add_system_set(SystemSet::on_enter(GameState::MoveOnePlayer).with_system(move_one_setup))
            .add_system_set(SystemSet::on_update(GameState::MoveOnePlayer).with_system(move_one_keyboard))
            .add_system_set(SystemSet::on_update(GameState::MoveOnePlayer).with_system(board_describe_piece))
            .add_system_set(SystemSet::on_exit(GameState::MoveOnePlayer).with_system(move_one_finish))

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

fn move_setup(
    mut g: ResMut<Game>,
    mut cursor: ResMut<Cursor>,
) {
    cursor.set_type(CURSOR_BOX);
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
        state.push(GameState::MoveOnePlayer).unwrap();
    }
}

fn move_one_setup(
    g: Res<Game>,
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
    mut cursor: ResMut<Cursor>,
    board: Res<GameBoard>,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut query: Query<(&Named, &MoveableComponent, Option<&BelongsToPlayer>, &mut Transform,)>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut moving: ResMut<Moving>,
    mut ev_move: EventWriter<BoardMove>,
) {
    if let Some(entity) = moving.entity {
        if moving.flying {
            if keys.just_pressed(KeyCode::S) {
                keys.reset(KeyCode::S);
                cursor.set_type(CURSOR_BOX);
                let cursor_pos = cursor.get_pos_v();
                ev_move.send(BoardMove{
                    from: moving.pos,
                    to: cursor_pos,
                });
                moving.distance_left = 0;
                moving.entity = None;
            }
        } else {
            for cur in ev_cursor.iter() {
                println!("Got cursor moved event in move one from {} to {}", moving.pos, **cur);
                ev_text.send(BottomTextEvent::clear());
                ev_move.send(BoardMove{
                    from: moving.pos,
                    to: **cur,
                });
                moving.distance_left -= 1;
                if moving.distance_left == 0 {
                    println!("No movement left, clear entity");
                    moving.entity = None;
                    cursor.set_visible();
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
            let pos = cursor.get_pos_v();
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
                        cursor.set_type(CURSOR_FLY);
                    } else {
                        cursor.set_invisible();
                    }
                    let mut text = String::from("Movement range=");
                    text.push_str(&moveable.movement.to_string());
                    ev_text.send(BottomTextEvent::from(&text));
                }
            }
        }
    }
}

pub fn board_describe_piece(
    board: Res<GameBoard>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut query: Query<(&Named, &MoveableComponent, Option<&BelongsToPlayer>, &mut Transform,)>,
    mut playername: Query<&Named>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    for cur in ev_cursor.iter() {
        if board.has_entity(**cur) {
            println!("HAs entity here");
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

fn move_one_finish(mut g: ResMut<Game>) {
    println!("Finish move one, increment player turn");
    g.player_turn += 1;
}

fn next_turn(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
    mut cursor: ResMut<Cursor>,
) {
    println!("next_turn set state GameState::PlayerMenu");
    g.player_turn = 0;
    cursor.set_invisible();
    state.set(GameState::PlayerMenu).unwrap();
}
