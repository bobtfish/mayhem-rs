use bevy::prelude::*;
use rand::Rng;
use crate::board::{GameBoard, BoardMove, MoveableComponent, BoardKill};
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{BottomTextEvent, StartExplosion, FinishedExplosion};
use crate::system::{Named, BelongsToPlayer, RangedCombat, CanDefend, CanAttack};
use crate::cursor::{CURSOR_BOX, CursorMovedEvent, CURSOR_FLY, PositionCursorOnEntity, Cursor, CURSOR_TARGET};
use crate::vec::Vec2I;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_next.in_set(OnUpdate(GameState::MoveSetup)))

            .add_system(move_choose_setup.in_schedule(OnEnter(GameState::MoveChoose)))
            .add_systems((move_choose_keyboard, board_describe_piece).in_set(OnUpdate(GameState::MoveChoose)))

            .add_system(move_moving_keyboard.in_set(OnUpdate(GameState::MoveMoving)))

            .add_system(ranged_attack_setup.in_schedule(OnEnter(GameState::RangedAttackChoose)))
            .add_systems((
                    board_describe_piece,
                    ranged_attack_keyboard,
                ).in_set(OnUpdate(GameState::RangedAttackChoose)))
            .add_system(ranged_attack_exit.in_schedule(OnExit(GameState::RangedAttackChoose)))

            .add_system(attack_start.in_schedule(OnEnter(GameState::AttackDo)))
            .add_system(attack_do.in_set(OnUpdate(GameState::AttackDo)))
            ;
    }
}

#[derive(Component)]
struct MovingComponent {
    start_pos: Vec2,
    steps: u8
}

#[derive(Component)]
struct HasMoved;

fn move_next(
    mut state: ResMut<NextState<GameState>>,
    mut g: ResMut<Game>,
    mut q: Query<Entity, With<HasMoved>>,
    mut commands: Commands,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_cursor_pos: EventWriter<PositionCursorOnEntity>,
    mut cursor: ResMut<Cursor>,
) {
    for has_moved_entity in q.iter_mut() {
        commands.entity(has_moved_entity).remove::<HasMoved>();
    }
    if g.player_turn >= g.players {
        println!("Moving finished, next turn now");
        ev_text.send(BottomTextEvent::clear());
        println!("next_turn set state GameState::TurnMenu");
        g.player_turn = 0;
        cursor.set_invisible();
        state.set(GameState::TurnMenu);
    } else {
        println!("Player turn to move");
        let player = g.get_player();
        let mut s = player.name.clone();
        ev_cursor_pos.send(PositionCursorOnEntity(player.handle.unwrap()));
        s.push_str("'s turn");
        ev_text.send(BottomTextEvent::from(&s));
        state.set(GameState::MoveChoose);
    }
}

#[derive(Component)]
struct RangedAttackComponent;

fn move_choose_setup(
    mut cursor: ResMut<Cursor>,
) {
    cursor.set_visible();
    cursor.set_type(CURSOR_BOX);
    println!("In move choose setup");
}

fn move_choose_keyboard(
    mut g: ResMut<Game>,
    mut cursor: ResMut<Cursor>,
    board: Res<GameBoard>,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
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
            state.set(GameState::RangedAttackChoose);
        }
    }

    if keys.just_pressed(KeyCode::Key0) {
        keys.reset(KeyCode::Key0);
        println!("Finish move one, increment player turn");
        g.player_turn += 1;
        state.set(GameState::MoveSetup);
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
                state.set(GameState::MoveMoving);
            }
        }
    }

}

fn move_moving_keyboard(
    mut cursor: ResMut<Cursor>,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_move: EventWriter<BoardMove>,
    mut moving_q: Query<(Entity, &MoveableComponent, &MovingComponent)>,
    mut other_q: Query<&BelongsToPlayer>,
    board: Res<GameBoard>,
    game: Res<Game>,
    mut commands: Commands,
) {
    let (entity, movable, moving) = moving_q.single_mut();
    if movable.flying {
        if keys.just_pressed(KeyCode::K) {
            // Cancel movement
            ev_text.send(BottomTextEvent::clear());
            cursor.set_type(CURSOR_BOX);
            state.set(GameState::MoveChoose);
            info!("cancelled move");
        }
        if keys.just_pressed(KeyCode::S) {
            keys.reset(KeyCode::S);
            let cursor_pos = cursor.get_pos_v();
            let distance = Vec2I::from(cursor_pos).distance(Vec2I::from(moving.start_pos));
            if distance > movable.movement {
                ev_text.send(BottomTextEvent::from("Out of range"));
                cursor.hide_till_moved();
            } else if board.has_entity_at(cursor_pos) {
                let other_entity = board.get_entity(cursor_pos).unwrap();
                let belongs_to = match other_q.get_mut(other_entity) {
                    Ok(b) => b.player_entity,
                    Err(_e) => other_entity,
                };
                let current_player_entity = game.get_player().handle.unwrap();
                info!("Current player is {:?}", current_player_entity);
                info!("Belongs to {:?}", belongs_to);
                if current_player_entity == belongs_to {
                    ev_text.send(BottomTextEvent::from("Cannot move to occupied square"));
                } else {
                    info!("Can attack");
                    commands.entity(entity).insert(AttackingComponent{
                        attackee: other_entity,
                    });
                    state.set(GameState::AttackDo);
                }
            } else {
                ev_text.send(BottomTextEvent::clear());
                ev_move.send(BoardMove{
                    entity,
                    to: cursor_pos,
                });
                state.set(GameState::MoveChoose);
                info!("Finished move");
            }
        }
    } else {
        if keys.just_pressed(KeyCode::K) {
            // Cancel movement
            ev_text.send(BottomTextEvent::clear());
            cursor.set_visible();
            state.set(GameState::MoveChoose);
            info!("cancelled move");
        }
        for cur in ev_cursor.iter() {
            println!("Got cursor moved event in move one from {} to {}", moving.start_pos, cur.0);
            if board.has_entity_at(cur.0) {
                let other_entity = board.get_entity(cur.0).unwrap();
                let belongs_to = match other_q.get_mut(other_entity) {
                    Ok(b) => b.player_entity,
                    Err(_e) => other_entity,
                };
                let current_player_entity = game.get_player().handle.unwrap();
                info!("Current player is {:?}", current_player_entity);
                info!("Belongs to {:?}", belongs_to);
                if current_player_entity == belongs_to {
                    ev_text.send(BottomTextEvent::from("Cannot move to occupied square"));
                    cursor.set_pos(cur.1);
                } else {
                    info!("Can attack");
                    commands.entity(entity).insert(AttackingComponent{
                        attackee: other_entity,
                    });
                    state.set(GameState::AttackDo);
                }
            } else {
                ev_text.send(BottomTextEvent::clear());
                ev_move.send(BoardMove{
                    entity,
                    to: cur.0,
                });
                let distance = Vec2I::from(cur.0).distance(Vec2I::from(moving.start_pos));
                info!("Moved distance {} has movement {}", distance, movable.movement);
                let distance_left = movable.movement.checked_sub(distance);
                if distance_left.unwrap_or(0) == 0 || moving.steps >= movable.movement {
                    info!("No movement left, clear entity");
                    cursor.set_visible();
                    state.set(GameState::MoveChoose);
                    info!("Finished move of this piece, choose next");
                }
            }
        }
    }
}

#[derive(Component)]
struct AttackingComponent {
    attackee: Entity
}

fn attack_start(
    board: Res<GameBoard>,
    attacking_q: Query<(Entity, &AttackingComponent)>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_explosion: EventWriter<StartExplosion>,
    mut cursor: ResMut<Cursor>,
    defender_q: Query<&CanDefend>,
    mut state: ResMut<NextState<GameState>>,
) {

    for (_e, ac) in attacking_q.iter() {
        let def = defender_q.get(ac.attackee);
        // Things which cannot defend cannot be attacked, abort and just go back to moving
        if def.is_err() {
            debug!("Cannot be attacked as no defence, going back to MoveMoving state");
            state.set(GameState::MoveMoving);
        } else {
            ev_text.send(BottomTextEvent::clear());
            cursor.set_invisible();
            let v = board.get_entity_pos(ac.attackee);
            info!("Spawn animation at {:?}", v);
            ev_explosion.send(StartExplosion {
                at: v,
                idx: 0,
            });
        }
    }
}

fn attack_do(
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut ev_explosion: EventReader<FinishedExplosion>,
    attacking_q: Query<(Entity, &AttackingComponent, &CanAttack)>,
    defender_q: Query<&CanDefend>,
    mut ev_kill: EventWriter<BoardKill>,
) {
    for _e in ev_explosion.iter() {
        let (e, ac, at) = attacking_q.single();
        let combat = at.combat;
        let defender_entity = ac.attackee;
        let candefend = defender_q.get(defender_entity).unwrap();
        let defence = candefend.defence;
        commands.entity(e).remove::<AttackingComponent>();
        let attack = combat + rand::thread_rng().gen_range(1..10);
        let def = defence + rand::thread_rng().gen_range(1..10);
        info!("Doing combat, base attack is {} base defence is {}. This attack is {} this defence is {}", combat, defence, attack, def);
        if attack >= def {
            info!("ATTACK SUCCESSFUL, KILLED");
            ev_kill.send(BoardKill {
                killer: e,
                killed: defender_entity,
            });
        } else {
            info!("Attack not successful");
        }
        info!("Finished attack, next move");
        state.set(GameState::MoveChoose);
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
    mut state: ResMut<NextState<GameState>>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    if keys.just_pressed(KeyCode::K) {
        ev_text.send(BottomTextEvent::clear());
        state.set(GameState::MoveChoose);
        info!("cancelled attack");
    }
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        ev_text.send(BottomTextEvent::clear());
        let cursor_pos = cursor.get_pos_v();
        let (entity, ranged) = moving_q.single();
        let from = board.get_entity_pos(entity);
        let distance = Vec2I::from(cursor_pos).distance(from);
        if distance <= ranged.range {
            println!("CAN TARGET WITH RANGED");
            state.set(GameState::MoveChoose);
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
        if board.has_entity_at(cur.0) {
            let e = board.get_entity(cur.0).unwrap();
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
