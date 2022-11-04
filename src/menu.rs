use bevy::prelude::*;

use super::{GameState, Player, despawn_screen, Game, get_border};
use crate::display::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::InitialMenu).with_system(initial_menu_setup))
            .add_system_set(SystemSet::on_update(GameState::InitialMenu).with_system(initial_menu_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::InitialMenu).with_system(despawn_screen::<InitialMenuScreen>))
            .add_system_set(SystemSet::on_enter(GameState::PlayerNameMenu).with_system(player_name_menu_setup))
            .add_system_set(SystemSet::on_update(GameState::PlayerNameMenu).with_system(player_name_menu_keyboard_input))
            .add_system_set(SystemSet::on_exit(GameState::PlayerNameMenu).with_system(despawn_screen::<PlayerNameMenuScreen>));
    }
}

// Tag component used to tag entities added on the menu screen
#[derive(Component, Clone, Copy)]
struct InitialMenuScreen;

fn initial_menu_setup(mut commands: Commands, g: Res<Game>) {
    get_border(&mut commands, g.tah.clone());
    print_text("  MAYHEM - Remake of Chaos", &mut commands, g.fah.clone(), Vec2::new(1.0, 10.0), InitialMenuScreen);
    print_text("         By bobtfish", &mut commands, g.fah.clone(), Vec2::new(1.0, 9.0), InitialMenuScreen);
    print_text("How many wizards?", &mut commands, g.fah.clone(), Vec2::new(1.0, 7.0), InitialMenuScreen);
    print_text("(Press 2 to 8)", &mut commands, g.fah.clone(), Vec2::new(1.0, 6.0), InitialMenuScreen);
	//textBottom("       Press H for help", ss, win)

}


fn initial_menu_keyboard_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut state: ResMut<State<GameState>>,
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    for ev in char_evr.iter() {
        let c = ev.char as u32;
        if game.players == 0 {
            if c >= 50 && c <= 56 {
                game.players = (c-48) as u8;
                println!("Players {}", game.players);
                print_text(&*game.players.to_string(), &mut commands, game.fah.clone(), Vec2::new(8.5, 6.0), InitialMenuScreen);
                print_text("Level of computer wizards?", &mut commands, game.fah.clone(), Vec2::new(1.0, 4.0), InitialMenuScreen);
                print_text("(Press 1 to 8)", &mut commands, game.fah.clone(), Vec2::new(1.0, 3.0), InitialMenuScreen)
            }
        } else {
            if c >= 49 && c <= 56 {
                game.ai_level = (c-48) as u8;
                print_text(&*game.ai_level.to_string(), &mut commands, game.fah.clone(), Vec2::new(8.5, 3.0), InitialMenuScreen);
                // TODO - Do we want a pause here?
                state.set(GameState::PlayerNameMenu).unwrap();
            }
        }
    }
}

// Tag component used to tag entities added on the menu screen
#[derive(Component, Clone, Copy)]
struct PlayerNameMenuScreen;

fn player_name_menu_setup(
    mut commands: Commands,
    mut g: ResMut<Game>,
) {


}

#[derive(Default)]
struct CapturePlayer {
    init: Option<bool>,
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
    entities: Query<Entity, &PlayerNameMenuScreen>,
) {
    if player.init.is_none() {
        print_text("PLAYER", &mut commands, g.fah.clone(), Vec2::new(1.0, 10.0), PlayerNameMenuScreen);
        let n_player = g.player_info.len()+1;
        print_text(&*n_player.to_string(), &mut commands, g.fah.clone(), Vec2::new(4.5, 10.0), PlayerNameMenuScreen);
        print_text("Enter name (12 letters max.)", &mut commands, g.fah.clone(), Vec2::new(1.0, 9.0), PlayerNameMenuScreen);
        player.init = Some(true);
        return;
    }
    if player.name.is_none() {
        if keys.just_pressed(KeyCode::Return) {
            println!("Text input: {}", *string);
            player.name = Some(string.clone());
            *string = String::from("");
            print_text("Computer Controlled?", &mut commands, g.fah.clone(), Vec2::new(1.0, 6.0), PlayerNameMenuScreen);
            return;
        }
        for ev in char_evr.iter() {
            println!("Got char: '{}'", ev.char);
            string.push(ev.char);
            print_text(&*string, &mut commands, g.fah.clone(), Vec2::new(1.0, 8.0), PlayerNameMenuScreen);
        }
        return;
    }
    if player.computer_controlled.is_none() {
        if keys.just_pressed(KeyCode::Y) {
            player.computer_controlled = Some(true);
            print_text("YES", &mut commands, g.fah.clone(), Vec2::new(11.5, 6.0), PlayerNameMenuScreen);
        }
        if keys.just_pressed(KeyCode::N) {
            player.computer_controlled = Some(false);
            print_text("NO", &mut commands, g.fah.clone(), Vec2::new(11.5, 6.0), PlayerNameMenuScreen);
        }
        if player.computer_controlled.is_some() {
            print_text("Which character?", &mut commands, g.fah.clone(), Vec2::new(1.0, 5.0), PlayerNameMenuScreen);
            print_text("1  2  3  4  5  6  7  8", &mut commands, g.fah.clone(), Vec2::new(1.0, 4.0), PlayerNameMenuScreen);
        }
        return;
    }
    if player.character_icon.is_none() {
        for ev in char_evr.iter() {
            let c = ev.char as u32;
            if c >= 49 && c <= 56 {
                let choice = c-48;
                player.character_icon = Some(choice as u8);
                print_text(&*choice.to_string(), &mut commands, g.fah.clone(), Vec2::new(9.5, 5.0), PlayerNameMenuScreen);
                print_text("Which color?", &mut commands, g.fah.clone(), Vec2::new(1.0, 3.0), PlayerNameMenuScreen);
                print_text("1  2  3  4  5  6  7  8", &mut commands, g.fah.clone(), Vec2::new(1.0, 2.0), PlayerNameMenuScreen);
            }
        }
        return;
    }
    if player.color.is_none() {
        for ev in char_evr.iter() {
            let c = ev.char as u32;
            if c >= 49 && c <= 56 {
                let choice = c-48;
                player.color= Some(choice as u8);
                //print_text(&*choice.to_string(), &mut commands, g.fah.clone(), Vec2::new(7.5, 3.0), PlayerNameMenuScreen);
            }
        }
    }
    if player.color.is_some() {
        for ev in char_evr.iter() {
            let c = ev.char as u32;
            if c >= 49 && c <= 56 {
                let choice = c-48;
                player.color = Some(choice as u8);
            }
        }
        g.player_info.push(Player {
            name: player.name.clone().unwrap(),
            computer_controlled: player.computer_controlled.unwrap(),
            character_icon: player.character_icon.unwrap(),
            color: player.color.unwrap()
        });
        *player = CapturePlayer{..Default::default()};
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        if g.players == g.player_info.len() as u8 {
            state.set(GameState::Game).unwrap();
        }
    }
}


