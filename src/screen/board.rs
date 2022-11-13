use bevy::prelude::*;
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{BottomTextEvent};
use crate::system;
use crate::cursor;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(game_setup)
                    .with_system(cursor::set_visible)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(game_next)
                )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(system::despawn_screen::<OnGameScreen>),
            )
            .add_system_set(SystemSet::on_enter(GameState::GameCastSpell).with_system(cast_spell_setup))
            .add_system_set(
                SystemSet::on_update(GameState::GameCastSpell)
                    .with_system(cast_spell_keyboard)
                );
    }
}

// Tag component used to tag entities added on the menu screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(
    mut commands: Commands,
    mut g: ResMut<Game>,
) {
    let tah = g.tah();
    // Wizard with bow
    //spawn_anim(&mut commands, g.tah(), Vec2::splat(2.0), 120, 8);
    // Spell/splodey thing
    //spawn_anim(&mut commands, g.tah(), Vec2::splat(1.0), 180, 4);

    //let creature = spawn_anim(&mut commands, g.tah(), Vec2::splat(3.0), 210, 4);
    //commands.entity(creature).insert(Mortal{is_alive: false});

    //let creature_map = load_creatures();
    //creature_map.get("Pegasus").unwrap().to_entity(Vec2::splat(4.0), &mut commands, g.tah());
    g.player_turn = 0;

    for p in &mut g.player_info {
        p.spawn(&mut commands, tah.clone());
    }
}

fn game_next(
    mut state: ResMut<State<GameState>>
) {
    state.push(GameState::GameCastSpell).unwrap();
}

fn cast_spell_setup(
    mut g: ResMut<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut state: ResMut<State<GameState>>
) {
    let player = g.get_player();
    let spell = player.get_chosen_spell();
    if spell.is_none() {
        state.pop().unwrap();
    }
    let spell = spell.unwrap();
    let mut text = String::from(&player.name);
    text.push(' ');
    text.push_str(&spell.name());
    ev_text.send(BottomTextEvent::from(&text));
    let x = player.x;
    let y = player.y;
    g.cursor.set_pos(x, y);
    println!("SET CURSOR TO {x} {y}");
}

fn cast_spell_keyboard(
    keys: Res<Input<KeyCode>>,
    mut g: ResMut<Game>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::S) {
        let pos = g.cursor.get_pos_v();
        let player = g.get_player_mut();
        let spell = player.get_chosen_spell().unwrap();
        //spell.cast(pos, commands);
    }
}