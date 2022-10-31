use bevy::prelude::*;

use super::{GameState, despawn_screen, Game, get_border, get_sprite_sheet_bundle};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(keyboard_input_system))

            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(despawn_screen::<OnMenuScreen>),
            );
    }
}

    // Tag component used to tag entities added on the menu screen
    #[derive(Component)]
    struct OnMenuScreen;

fn menu_setup(mut commands: Commands, g: Res<Game>) {
    get_border(&mut commands, g.tah.clone());
    print_text("Hello", commands, g.fah.clone(), Vec2::new(1.0, 10.0))
    //commands.spawn_bundle(get_sprite_sheet_bundle(g.fah.clone(), Vec2::new(1.0, 10.0), 0));

}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
        state.set(GameState::Game).unwrap();
    }
}

fn print_text(str: &str, mut commands: Commands, fah: Handle<TextureAtlas>, v: Vec2) {
    for (i,ch) in str.chars().enumerate() {
        println!("{} {}", i, ch);
        let mut newV = v.clone();
        newV.x = newV.x + (i as f32/2.0);
        commands.spawn_bundle(get_sprite_sheet_bundle(fah.clone(), newV, char_to_pos(ch)))
        .insert(OnMenuScreen);
    }
}

fn char_to_pos(c: char) -> usize {
    println!("C IS {}", c);
    let d = c as u32;
    if d >=33 && d <= 126{
        return (d - 31) as usize;
    }
    if c == ' ' {
        return 1;
    }
    return 0;
}
