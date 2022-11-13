use bevy::prelude::*;
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display::{BottomTextEvent};
use crate::system;
use crate::cursor::{self, CURSOR_SPELL};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(game_setup)
                    .with_system(cursor::set_visible)
            )
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game_next))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(system::despawn_screen::<OnGameScreen>))
            .add_system_set(SystemSet::on_enter(GameState::GameCastSpell).with_system(cast_spell_setup))
            .add_system_set(SystemSet::on_update(GameState::GameCastSpell).with_system(cast_spell_keyboard))
            .add_system_set(SystemSet::on_exit(GameState::GameCastSpell).with_system(cast_spell_finish))
            .add_system_set(SystemSet::on_enter(GameState::GameMove).with_system(move_setup));
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
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
) {
    if g.player_turn >= g.players {
        g.player_turn = 0;
        println!("Spell casting finished, do movement now");
        state.push(GameState::GameMove).unwrap();
    } else {
        println!("Player turn to cast spell");
        // Next player's turn to cast a spell
        state.push(GameState::GameCastSpell).unwrap();
    }
}

fn cast_spell_setup(
    mut g: ResMut<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
    mut state: ResMut<State<GameState>>
) {
    g.cursor.set_type(CURSOR_SPELL);
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
    mut keys: ResMut<Input<KeyCode>>,
    mut g: ResMut<Game>,
    mut commands: Commands,
    mut state: ResMut<State<GameState>>
) {
    let tah = g.tah();
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        let pos = g.cursor.get_pos_v();
        let player = g.get_player_mut();
        player.cast(pos, &mut commands, tah);
        state.pop().unwrap();
    }
}

fn cast_spell_finish(mut g: ResMut<Game>) {
    println!("Finish cast spell, increment player turn");
    g.player_turn += 1;
}

fn move_setup() {}