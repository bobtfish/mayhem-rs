use bevy::prelude::*;

mod menu;
mod board;
mod playermenu;
mod help;
mod spellcasting;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
	        .add_plugin(menu::MenuPlugin)
            .add_plugin(help::HelpPlugin)
            .add_plugin(playermenu::PlayerMenuPlugin)
            .add_plugin(spellcasting::SpellCastingPlugin)
            .add_plugin(board::BoardPlugin);
    }
}
