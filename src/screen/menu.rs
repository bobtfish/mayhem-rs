use bevy::prelude::*;

use crate::{display::*, spell::AllSpells};
use crate::player::Player;
use crate::game::Game;
use crate::system;
use crate::gamestate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::InitialMenu).with_system(initial_menu_setup))
            .add_system_set(SystemSet::on_update(GameState::InitialMenu).with_system(initial_menu_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::InitialMenu).with_system(system::despawn_screen::<InitialMenuScreen>))
            .add_system_set(SystemSet::on_enter(GameState::PlayerNameMenu).with_system(player_name_menu_setup))
            .add_system_set(SystemSet::on_update(GameState::PlayerNameMenu).with_system(player_name_menu_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::PlayerNameMenu).with_system(system::despawn_screen::<PlayerNameMenuScreen>))
            .add_system_set(SystemSet::on_update(GameState::PlayerNameMenuTransition).with_system(player_name_menu_transition));
    }
}

// Tag component used to tag entities added on the menu screen
#[derive(Component, Clone, Copy)]
struct InitialMenuScreen;

fn initial_menu_setup(
    mut commands: Commands,
    g: Res<Game>,
    mut ev_text: EventWriter<BottomTextEvent>,
) {
    get_border(&mut commands, g.tah());
    print_text("  MAYHEM - Remake of Chaos", &mut commands, g.fah(), Vec2::new(0.5, 8.0), InitialMenuScreen);
    print_text("         By bobtfish", &mut commands, g.fah(), Vec2::new(0.5, 7.0), InitialMenuScreen);
    print_text("How many wizards?", &mut commands, g.fah(), Vec2::new(0.5, 5.0), InitialMenuScreen);
    print_text("(Press 2 to 8)", &mut commands, g.fah(), Vec2::new(0.5, 4.0), InitialMenuScreen);
    ev_text.send(BottomTextEvent::from("      Press H for help"));
}


fn initial_menu_keyboard_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut state: ResMut<State<GameState>>,
    mut game: ResMut<Game>,
    mut commands: Commands,
    mut ev_text: EventWriter<BottomTextEvent>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::H) {
        state.set(GameState::Help).unwrap();
        return
    }
    for ev in char_evr.iter() {
        let c = ev.char as u32;
        if game.players == 0 {
            if (50..=56).contains(&c) {
                game.players = (c-48) as u8;
                println!("Players {}", game.players);
                print_text(&game.players.to_string(), &mut commands, game.fah(), Vec2::new(8.0, 4.0), InitialMenuScreen);
                print_text("Level of computer wizards?", &mut commands, game.fah(), Vec2::new(0.5, 2.0), InitialMenuScreen);
                print_text("(Press 1 to 8)", &mut commands, game.fah(), Vec2::new(0.5, 1.0), InitialMenuScreen);
            }
        } else if (49..=56).contains(&c) {
            game.ai_level = (c-48) as u8;
            print_text(&game.ai_level.to_string(), &mut commands, game.fah(), Vec2::new(8.0, 1.0), InitialMenuScreen);
            // TODO - Do we want a pause here?
            state.set(GameState::PlayerNameMenu).unwrap();
            ev_text.send(BottomTextEvent::clear());
        }
    }
}

// Tag component used to tag entities added on the menu screen
#[derive(Component, Clone, Copy)]
struct PlayerNameMenuScreen;

fn player_name_menu_setup(
    mut commands: Commands,
    g: Res<Game>,
) {
    print_text("PLAYER", &mut commands, g.fah(), Vec2::new(0.5, 9.0), PlayerNameMenuScreen);
    let n_player = g.player_info.len()+1;
    print_text(&n_player.to_string(), &mut commands, g.fah(), Vec2::new(4.0, 9.0), PlayerNameMenuScreen);
    print_text("Enter name (12 letters max.)", &mut commands, g.fah(), Vec2::new(0.5, 8.0), PlayerNameMenuScreen);
}

#[derive(Default)]
struct CapturePlayer {
    init: bool,
    name: Option<String>,
    computer_controlled: Option<bool>,
    character_icon: Option<u8>,
    color: Option<u8>,
}

fn player_name_menu_keyboard_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
    mut commands: Commands,
    mut string: Local<String>,
    mut player: Local<CapturePlayer>,
    keys: Res<Input<KeyCode>>,
    allspells: Res<AllSpells>,
) {
    if !player.init { // This is to force a frame advance and stop us re-capturing the keyboard input
        player.init = true;
        return;
    }
    if player.name.is_none() {
        if keys.just_pressed(KeyCode::Return) {
            println!("Text input: {}", *string);
            player.name = Some(string.clone());
            *string = String::new();
            print_text("Computer Controlled?", &mut commands, g.fah(), Vec2::new(0.5, 5.0), PlayerNameMenuScreen);
            return;
        }
        for ev in char_evr.iter() {
            println!("Got char: '{}'", ev.char);
            string.push(ev.char);
            print_text(&string, &mut commands, g.fah(), Vec2::new(0.5, 7.0), PlayerNameMenuScreen);
        }
        return;
    }
    if player.computer_controlled.is_none() {
        if keys.just_pressed(KeyCode::Y) {
            player.computer_controlled = Some(true);
            print_text("YES", &mut commands, g.fah(), Vec2::new(11.0, 5.0), PlayerNameMenuScreen);
        }
        if keys.just_pressed(KeyCode::N) {
            player.computer_controlled = Some(false);
            print_text("NO", &mut commands, g.fah(), Vec2::new(11.0, 5.0), PlayerNameMenuScreen);
        }
        if player.computer_controlled.is_some() {
            print_text("Which character?", &mut commands, g.fah(), Vec2::new(0.5, 4.0), PlayerNameMenuScreen);
            show_wizards(g.fah(), g.tah(), &mut commands, 3.0);
        }
        return;
    }
    if player.character_icon.is_none() {
        for ev in char_evr.iter() {
            let c = ev.char as u32;
            if (49..=56).contains(&c) {
                let choice = c-48;
                player.character_icon = Some(choice as u8);
                print_text(&choice.to_string(), &mut commands, g.fah(), Vec2::new(9.0, 4.0), PlayerNameMenuScreen);
                print_text("Which color?", &mut commands, g.fah(), Vec2::new(0.5, 2.0), PlayerNameMenuScreen);
                show_wizards(g.fah(), g.tah(), &mut commands, 1.0);
            }
        }
        return;
    }
    if player.color.is_none() {
        for ev in char_evr.iter() {
            let c = ev.char as u32;
            if (49..=56).contains(&c) {
                let choice = c-48;
                player.color= Some(choice as u8);
                //print_text(&*choice.to_string(), &mut commands, g.fah(), Vec2::new(7.5, 3.0), PlayerNameMenuScreen);
            }
        }
    }
    if player.color.is_some() {
        let mut p = Player::new(
            player.name.clone().unwrap(),
            player.computer_controlled.unwrap(),
            player.character_icon.unwrap(),
            player.color.unwrap(),
        );
        p.pick_spells(&allspells);
        g.player_info.push(p);
        *player = CapturePlayer{..Default::default()};
        state.set(GameState::PlayerNameMenuTransition).unwrap();
    }
}

fn show_wizards(fah: Handle<TextureAtlas>, tah: Handle<TextureAtlas>, commands: &mut Commands, y: f32) {
    for i in 0..8 {
        print_text(&(i+1).to_string(), commands, fah.clone(), Vec2::new((i as f32).mul_add(1.5, 0.5), y), PlayerNameMenuScreen);
        print_wizard(commands, tah.clone(), Vec2::new((i as f32).mul_add(1.5, 1.25), y), i, PlayerNameMenuScreen);
    }
}

fn player_name_menu_transition(
    mut state: ResMut<State<GameState>>,
    mut g: ResMut<Game>,
) {
    if g.players == g.player_info.len() as u8 {
        let positions = crate::player::get_start_positions(g.players as usize).unwrap();
        for (i, pos) in positions.iter().enumerate() {
            g.player_info[i].pos = *pos;
        }
        state.set(GameState::PlayerMenu).unwrap();
    } else {
        state.set(GameState::PlayerNameMenu).unwrap();
    }
}
