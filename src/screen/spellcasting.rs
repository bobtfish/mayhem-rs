use bevy::prelude::*;

use crate::{gamestate::GameState, display::BottomTextEvent, game::Game, board::BoardPutEntity, player::CastFailed, cursor::{CursorMovedEvent, CURSOR_SPELL, PositionCursorOnEntity}};

pub struct SpellCastingPlugin;

impl Plugin for SpellCastingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::GameCastSpell).with_system(cast_spell_setup))
        .add_system_set(SystemSet::on_update(GameState::GameCastSpell).with_system(cast_spell_keyboard))
        .add_system_set(SystemSet::on_exit(GameState::GameCastSpell).with_system(cast_spell_finish))
        .add_event::<CastSpell>()
        .add_system_set(SystemSet::on_update(GameState::GameCastSpell).with_system(cast_spell))
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
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_board_put: EventWriter<BoardPutEntity>,
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
        let player = g.get_player_mut();
        match player.cast(from, to, &mut commands, tah.clone()) {
            Ok(e) => {
                ev_board_put.send(BoardPutEntity{
                    entity: e.unwrap(),
                    pos: to,
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
}

fn cast_spell_keyboard(
    mut keys: ResMut<Input<KeyCode>>,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut ev_cursor: EventReader<CursorMovedEvent>,
    mut ev_cast: EventWriter<CastSpell>,
) {
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        let to = g.cursor.get_pos_v();
        ev_cast.send(CastSpell{target: to});
    }
    for _ in ev_cursor.iter() {
        ev_text.send(BottomTextEvent::clear());
    }
}

fn cast_spell_finish(mut g: ResMut<Game>) {
    println!("Finish cast spell, increment player turn");
    g.player_turn += 1;
}