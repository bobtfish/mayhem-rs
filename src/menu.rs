use bevy::prelude::*;

use super::{GameState, despawn_screen, Game, get_border, get_sprite_sheet_bundle};

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
    print_text("How many wizards?", &mut commands, g.fah.clone(), Vec2::new(1.0, 8.0), InitialMenuScreen);
    print_text("(Press 2 to 8)", &mut commands, g.fah.clone(), Vec2::new(1.0, 7.0), InitialMenuScreen);
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
                print_text(&*game.players.to_string(), &mut commands, game.fah.clone(), Vec2::new(8.5, 7.0), InitialMenuScreen);
                print_text("Level of computer wizards?", &mut commands, game.fah.clone(), Vec2::new(1.0, 6.0), InitialMenuScreen);
                print_text("(Press 1 to 8)", &mut commands, game.fah.clone(), Vec2::new(1.0, 5.0), InitialMenuScreen)
            }
        } else {
            if c >= 49 && c <= 56 {
                game.ai_level = (c-48) as u8;
                print_text(&*game.ai_level.to_string(), &mut commands, game.fah.clone(), Vec2::new(8.5, 5.0), InitialMenuScreen);
                // TODO - Do we want a pause here?
                state.set(GameState::PlayerNameMenu).unwrap();
            }
        }
    }
}

// Tag component used to tag entities added on the menu screen
#[derive(Component, Clone, Copy)]
struct PlayerNameMenuScreen;

fn player_name_menu_setup(mut commands: Commands, g: Res<Game>) {
}

fn player_name_menu_keyboard_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut state: ResMut<State<GameState>>,
    mut game: ResMut<Game>,
    mut commands: Commands,
) {}

fn print_text(str: &str, commands: &mut Commands, fah: Handle<TextureAtlas>, v: Vec2, c: impl Component + std::marker::Copy) {
    for (i,ch) in str.chars().enumerate() {
        println!("{} {}", i, ch);
        let mut new_v = v.clone();
        new_v.x = new_v.x + (i as f32/2.0);
        commands.spawn_bundle(get_sprite_sheet_bundle(fah.clone(), new_v, char_to_pos(ch)))
        .insert(c);
    }
}

fn char_to_pos(c: char) -> usize {
    let d = c as u32;
    if d >=33 && d <= 126{
        return (d - 31) as usize;
    }
    if c == ' ' {
        return 1;
    }
    return 0;
}
