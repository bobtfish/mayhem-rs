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
        .add_system_set(
            SystemSet::on_enter(GameState::CastSpellSetup)
                .with_system(spell_setup)
                .with_system(system::show_board_entities)
        )
        .add_system_set(SystemSet::on_update(GameState::CastSpellSetup).with_system(spell_next))
        .add_system_set(SystemSet::on_enter(GameState::CastSpell).with_system(cast_spell_setup))
        .add_system_set(
            SystemSet::on_update(GameState::CastSpell)
                .with_system(cast_spell_keyboard)
                .with_system(cast_spell)
                .with_system(cast_spell_result)
                .with_system(super::board::board_describe_piece)
        )
        .add_system_set(SystemSet::on_exit(GameState::CastSpell).with_system(cast_spell_finish));
    }
}

fn spell_setup(
    mut g: ResMut<Game>,
    mut cursor: ResMut<Cursor>,
) {
    println!("spell_setup");
    g.player_turn = 0;
    cursor.set_visible();
}

fn spell_next(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
) {
    println!("game_next");
    if g.player_turn >= g.players {
        g.player_turn = 0;
        println!("Spell casting finished, do movement now");
        state.set(GameState::MoveSetup).unwrap();
    } else {
        println!("Player turn to cast spell");
        // Next player's turn to cast a spell
        state.push(GameState::CastSpell).unwrap();
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
    mut state: ResMut<State<GameState>>,
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
        state.pop().unwrap();
        return;
    }
    let transform = query.get_mut(player.handle.unwrap()).unwrap();
    let from = Vec2{ x: transform.translation.x, y: transform.translation.y };
    let tah = g.tah();
    for e in ev_cast.iter() {
        let to = e.target;
        if board.get_entity(to).is_some() {
            ev_cast_res.send(Err(CastFailed::NotThere));
            return
        }
        let player = g.get_player_mut();
        let res = player.cast(from, to, &mut commands, tah.clone());
        if let Ok(e) = res {
            ev_board_put.send(BoardPutEntity{
                entity: e.unwrap(),
                pos: to,
            });
            println!("State POP");
            state.pop().unwrap();
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
) {
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