use bevy::prelude::*;

use super::{GameState, despawn_screen, Game, get_border};

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
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
        state.set(GameState::Game).unwrap();
    }
}
