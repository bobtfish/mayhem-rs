use bevy::prelude::*;

use crate::{gamestate::GameState, display::BottomTextEvent, game::Game, board::{BoardPutEntity, GameBoard}, player::CastFailed, cursor::{CursorMovedEvent, CURSOR_SPELL, PositionCursorOnEntity}};

pub struct SpellCastingPlugin;

impl Plugin for SpellCastingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::GameCastSpell).with_system(cast_spell_setup))
        .add_system_set(SystemSet::on_update(GameState::GameCastSpell).with_system(cast_spell_keyboard))
        .add_system_set(SystemSet::on_exit(GameState::GameCastSpell).with_system(cast_spell_finish))
        .add_event::<CastSpell>()
        .add_system_set(SystemSet::on_update(GameState::GameCastSpell).with_system(cast_spell))
        .add_event::<CastSpellResult>()
        .add_system_set(SystemSet::on_update(GameState::GameCastSpell).with_system(cast_spell_result))
        .add_system_set(SystemSet::on_update(GameState::GameCastSpell).with_system(super::board::board_describe_piece));
    }
}

fn cast_spell_setup(
    mut g: ResMut<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_cursor_pos: EventWriter<PositionCursorOnEntity>,
) {
    println!("cast_spell_setup");
    g.cursor.set_type(CURSOR_SPELL);
    let player = g.get_player();
    if let Some(spell_name) = player.get_chosen_spell_name() {
        ev_text.send(BottomTextEvent::from(&spell_name));
        ev_cursor_pos.send(PositionCursorOnEntity(player.handle.unwrap()));
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
    mut g: ResMut<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    for e in ev_cast.iter() {
        match e {
            Ok(_e) => {
            },
            Err(CastFailed::OutOfRange) => {
                ev_text.send(BottomTextEvent::from("Out of range"));
                g.cursor.hide_till_moved();
            }
            Err(CastFailed::NotThere) => {
            }
        }
    }
}

fn cast_spell_keyboard(
    mut keys: ResMut<Input<KeyCode>>,
    g: Res<Game>,
    mut ev_cast: EventWriter<CastSpell>,
) {
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        let to = g.cursor.get_pos_v();
        ev_cast.send(CastSpell{target: to});
    }
}

fn cast_spell_finish(mut g: ResMut<Game>) {
    println!("Finish cast spell, increment player turn");
    g.player_turn += 1;
}