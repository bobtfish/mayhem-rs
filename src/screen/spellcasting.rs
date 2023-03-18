use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::display::BottomTextEvent;
use crate::game::Game;
use crate::board::{BoardPutEntity, GameBoard};
use crate::player::CastFailed;
use crate::cursor::{CURSOR_SPELL, PositionCursorOnEntity, Cursor};
use crate::system;

pub struct SpellCastingPlugin;

impl Plugin for SpellCastingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<CastSpell>()
        .add_event::<CastSpellResult>()

        .add_system(system::show_board_entities.in_schedule(OnEnter(GameState::CastSpellSetup)))
        .add_system(spell_next.in_set(OnUpdate(GameState::CastSpellSetup)))

        .add_system(cast_spell_setup.in_schedule(OnEnter(GameState::CastSpell)))
        .add_system(cast_spell_keyboard.in_set(OnUpdate(GameState::CastSpell)))
        .add_system(cast_spell.in_set(OnUpdate(GameState::CastSpell)))
        .add_system(cast_spell_result.in_set(OnUpdate(GameState::CastSpell)))
        .add_system(super::board::board_describe_piece.in_set(OnUpdate(GameState::CastSpell)))
        .add_system(cast_spell_finish.in_schedule(OnExit(GameState::CastSpell)))
        ;
    }
}

fn spell_next(
    mut state: ResMut<NextState<GameState>>,
    mut g: ResMut<Game>,
) {
    println!("spell_next");
    if g.player_turn >= g.players {
        g.player_turn = 0;
        println!("Spell casting finished, do movement now");
        state.set(GameState::MoveSetup);
    } else {
        println!("Player turn to cast spell");
        // Next player's turn to cast a spell
        state.set(GameState::CastSpell);
    }
}

fn cast_spell_setup(
    g: Res<Game>,
    mut cursor: ResMut<Cursor>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_cursor_pos: EventWriter<PositionCursorOnEntity>,
) {
    println!("cast_spell_setup");
    cursor.set_type(CURSOR_SPELL);
    let player = g.get_player();
    if let Some(spell_name) = player.get_chosen_spell_name() {
        ev_text.send(BottomTextEvent::from(&spell_name));
        ev_cursor_pos.send(PositionCursorOnEntity(player.handle.unwrap()));
        cursor.hide_till_moved();
    }
}

struct CastSpell {
    target: Vec2
}

fn cast_spell(
    mut g: ResMut<Game>,
    mut ev_cast: EventReader<CastSpell>,
    mut state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Transform>,
    mut commands: Commands,
    mut ev_cast_res: EventWriter<CastSpellResult>,
    mut ev_board_put: EventWriter<BoardPutEntity>,
    board: Res<GameBoard>,
) {
    let player = g.get_player();
    let spell = player.spells.get_chosen_spell();
    if spell.is_none() {
        println!("STATE POP - no spell");
        state.set(GameState::CastSpellSetup);
        return;
    }
    let cast_range = spell.unwrap().cast_range();
    let transform = query.get_mut(player.handle.unwrap()).unwrap();
    let from = Vec2{ x: transform.translation.x, y: transform.translation.y };
    let tah = g.tah();
    for e in ev_cast.iter() {
        let to;
        if cast_range > 0 {
            to = e.target;
            if board.get_entity(to).is_some() {
                ev_cast_res.send(Err(CastFailed::NotThere));
                return
            }
        } else {
            to = from;
        }
        let player = g.get_player_mut();
        let res = player.cast(from, to, &mut commands, tah.clone());
        if let Ok(e) = res {
            if let Some(entity) = e {
                ev_board_put.send(BoardPutEntity{
                    entity,
                    pos: to,
                });
            }
            println!("State POP");
            state.set(GameState::CastSpellSetup);
        }
        ev_cast_res.send(res);
    }
}

type CastSpellResult = Result<Option<Entity>, CastFailed>;
fn cast_spell_result(
    mut ev_cast: EventReader<CastSpellResult>,
    mut cursor: ResMut<Cursor>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    for e in ev_cast.iter() {
        match e {
            Ok(_e) => {
            },
            Err(CastFailed::OutOfRange) => {
                ev_text.send(BottomTextEvent::from("Out of range"));
                cursor.hide_till_moved();
            }
            Err(CastFailed::NotThere) => {
            }
        }
    }
}

fn cast_spell_keyboard(
    mut keys: ResMut<Input<KeyCode>>,
    cursor: Res<Cursor>,
    mut ev_cast: EventWriter<CastSpell>,
    g: ResMut<Game>,
) {
    let spell = g.get_player().spells.get_chosen_spell();
    if spell.is_none() {
        return;
    }
    let range = spell.unwrap().cast_range();
    if range == 0 {
        let mut has_pressed = false;
        for _ in keys.get_just_pressed() {
            has_pressed = true;
        }
        if has_pressed {
            println!("Key pressed on distance 0 spell");
            keys.reset_all();
            ev_cast.send(CastSpell{target: Vec2::ZERO});
        }
        return;
    }
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        let to = cursor.get_pos_v();
        ev_cast.send(CastSpell{target: to});
    }
}

fn cast_spell_finish(mut g: ResMut<Game>) {
    println!("Finish cast spell, increment player turn");
    g.player_turn += 1;
}
