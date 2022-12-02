use bevy::prelude::*;
use crate::board::{GameBoard, BoardMove, MoveableComponent};
use crate::creature::RangedCombat;
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{BottomTextEvent};
use crate::system::{self, Named, BelongsToPlayer};
use crate::cursor::{CURSOR_BOX, CursorMovedEvent, CURSOR_FLY, PositionCursorOnEntity, Cursor, CURSOR_TARGET};
use crate::vec::Vec2I;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::MoveSetup).with_system(move_setup))
            .add_system_set(SystemSet::on_update(GameState::MoveSetup).with_system(move_next))
            .add_system_set(SystemSet::on_enter(GameState::MoveChoose).with_system(move_choose_setup))
            .add_system_set(
                SystemSet::on_update(GameState::MoveChoose)
                    .with_system(move_choose_keyboard)
                    .with_system(board_describe_piece)
            )
            .add_system_set(SystemSet::on_exit(GameState::MoveChoose).with_system(move_choose_finish))
            .add_system_set(SystemSet::on_update(GameState::MoveMoving).with_system(move_moving_keyboard))
            .add_system_set(SystemSet::on_enter(GameState::NextTurn).with_system(system::hide_board_entities))
            .add_system_set(SystemSet::on_update(GameState::NextTurn).with_system(next_turn))
            .add_system_set(SystemSet::on_enter(GameState::RangedAttackChoose).with_system(ranged_attack_setup))
            .add_system_set(
                SystemSet::on_update(GameState::RangedAttackChoose)
                    .with_system(ranged_attack_keyboard)
                    .with_system(board_describe_piece)
            )
            .add_system_set(SystemSet::on_exit(GameState::RangedAttackChoose).with_system(ranged_attack_exit));
    }
}

// Game -push-> GameCastSpell
//   | ^------pop------/
//  set
//    \-> GameMoveSetup -push-> GameMoveChoose
//                 ^------pop------/

#[derive(Component)]
struct MovingComponent {
    start_pos: Vec2,
    steps: u8
}

#[derive(Component)]
struct HasMoved;

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
    mut q: Query<Entity, With<HasMoved>>,
    mut commands: Commands,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    for has_moved_entity in q.iter_mut() {
        commands.entity(has_moved_entity).remove::<HasMoved>();
    }
    if g.player_turn >= g.players {
        g.player_turn = 0;
        println!("Moving finished, next turn now");
        ev_text.send(BottomTextEvent::clear());
        state.set(GameState::NextTurn).unwrap();
    } else {
        println!("Player turn to move");
        state.push(GameState::MoveChoose).unwrap();
    }
}

fn move_choose_setup(
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_cursor_pos: EventWriter<PositionCursorOnEntity>,
) {
    println!("Move one for player {}", g.player_turn);
    let player = g.get_player();
    let mut s = player.name.clone();
    ev_cursor_pos.send(PositionCursorOnEntity(player.handle.unwrap()));
    s.push_str("'s turn");
    ev_text.send(BottomTextEvent::from(&s));
}

#[derive(Component)]
struct RangedAttackComponent;

fn move_choose_keyboard(
    g: Res<Game>,
    mut cursor: ResMut<Cursor>,
    board: Res<GameBoard>,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut query: Query<(&Named, &MoveableComponent, Option<&BelongsToPlayer>, &mut Transform, Option<&HasMoved>)>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut commands: Commands,
    moving_q: Query<(Entity, Option<&RangedCombat>), With<MovingComponent>>,
) {
    // We return here from MoveMoving with the entity that just moved still flagged with MovingComponent
    // remove that component, but at the same time check if this entity has ranged combat, as if so we need to do that now.
    for (e, ranged_combat) in moving_q.iter() {
        println!("Movement finished, remove MovingComponent");
        commands.entity(e).remove::<MovingComponent>();
        if ranged_combat.is_some() {
            println!("Do ranged attack now");
            commands.entity(e).insert(RangedAttackComponent);
            state.push(GameState::RangedAttackChoose).unwrap();
        }
    }

    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        state.pop().unwrap();
        println!("Next player turn");
    }
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        let pos = cursor.get_pos_v();
        println!("Find thing at {}, {} to move", pos.x, pos.y);
        if board.has_entity_at(pos) {
            let e = board.get_entity(pos).unwrap();
            let (_, moveable, belongs, _, has_moved) = query.get_mut(e).unwrap();
            let belongs_entity;
            if let Some(belongs) = belongs {
                belongs_entity = belongs.player_entity;
            } else {
                belongs_entity = e;
            }
            if g.get_player().handle.unwrap() == belongs_entity && has_moved.is_none() {
                commands.entity(e).insert(HasMoved);
                println!("Does belong to this player");
                let mut text = String::from("Movement range=");
                text.push_str(&moveable.movement.to_string());
                if moveable.flying {
                    cursor.set_type(CURSOR_FLY);
                    cursor.hide_till_moved();
                    text.push_str(" (flying)");
                } else {
                    cursor.set_invisible();
                }
                commands.entity(e).insert(MovingComponent{
                    start_pos: pos,
                    steps: 0,
                });
                ev_text.send(BottomTextEvent::from(&text));
                println!("State to MoveMoving");
                state.push(GameState::MoveMoving).unwrap();
            }
        }
    }

}

fn move_moving_keyboard(
    mut cursor: ResMut<Cursor>,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_move: EventWriter<BoardMove>,
    mut moving_q: Query<(Entity, &MoveableComponent, &MovingComponent)>,
) {
    let (entity, movable, moving) = moving_q.single_mut();
    if movable.flying {
        if keys.just_pressed(KeyCode::S) {
            keys.reset(KeyCode::S);
            let cursor_pos = cursor.get_pos_v();
            let distance = Vec2I::from(cursor_pos).distance(Vec2I::from(moving.start_pos));
            if distance > movable.movement as i8 {
                ev_text.send(BottomTextEvent::from("Out of range"));
                cursor.hide_till_moved();
            } else {
                ev_text.send(BottomTextEvent::clear());
                cursor.set_type(CURSOR_BOX);
                ev_move.send(BoardMove{
                    entity,
                    to: cursor_pos,
                });
                state.pop().unwrap();
                println!("Finished move");
            }
        }
    } else {
        for cur in ev_cursor.iter() {
            println!("Got cursor moved event in move one from {} to {}", moving.start_pos, **cur);
            ev_text.send(BottomTextEvent::clear());
            ev_move.send(BoardMove{
                entity,
                to: **cur,
            });
            let distance = Vec2I::from(**cur).distance(Vec2I::from(moving.start_pos));
            if movable.movement as i8 - distance <= 0 || moving.steps >= movable.movement {
                println!("No movement left, clear entity");
                cursor.set_visible();
                state.pop().unwrap();
                println!("Finished move");
            }
        }
    }
}

fn ranged_attack_setup(
    moving_q: Query<(Entity, &RangedCombat), With<RangedAttackComponent>>,
    mut cursor: ResMut<Cursor>,
    board: Res<GameBoard>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    cursor.set_type(CURSOR_TARGET);
    cursor.hide_till_moved();
    let (entity, ranged) = moving_q.single();
    cursor.set_pos(Vec2::from(board.get_entity_pos(entity)));
    let mut text = String::from("Ranged attack, range=");
    text.push_str(&ranged.range.to_string());
    ev_text.send(BottomTextEvent::from(&text));
}

fn ranged_attack_keyboard(
    mut keys: ResMut<Input<KeyCode>>,
    mut cursor: ResMut<Cursor>,
    moving_q: Query<(Entity, &RangedCombat), With<RangedAttackComponent>>,
    board: Res<GameBoard>,
    mut state: ResMut<State<GameState>>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        ev_text.send(BottomTextEvent::clear());
        let cursor_pos = cursor.get_pos_v();
        let (entity, ranged) = moving_q.single();
        let from = board.get_entity_pos(entity);
        let distance = Vec2I::from(cursor_pos).distance(from);
        if distance <= ranged.range as i8 {
            println!("CAN TARGET WITH RANGED");
            state.pop().unwrap();
        } else {
            ev_text.send(BottomTextEvent::from("Out of range"));
            cursor.hide_till_moved();
        }
    }
}
fn ranged_attack_exit(
    mut commands: Commands,
    moving_q: Query<Entity, With<RangedAttackComponent>>,
    mut cursor: ResMut<Cursor>,
) {
    commands.entity(moving_q.single()).remove::<RangedAttackComponent>();
    cursor.set_type(CURSOR_BOX);
}

pub fn board_describe_piece(
    board: Res<GameBoard>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut query: Query<(&Named, &MoveableComponent, Option<&BelongsToPlayer>, &mut Transform,)>,
    mut playername: Query<&Named>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    for cur in ev_cursor.iter() {
        if board.has_entity_at(**cur) {
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

fn move_choose_finish(mut g: ResMut<Game>) {
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
