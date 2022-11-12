use bevy::prelude::*;

mod menu;
mod board;
mod playermenu;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
	        .add_plugin(menu::MenuPlugin)
            .add_plugin(playermenu::PlayerMenuPlugin)
            .add_plugin(board::BoardPlugin);
    }
}
