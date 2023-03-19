use bevy::prelude::*;

mod board;
mod help;
mod menu;
mod playermenu;
mod spellcasting;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(board::BoardPlugin)
            .add_plugin(help::HelpPlugin)
	        .add_plugin(menu::MenuPlugin)
            .add_plugin(playermenu::PlayerMenuPlugin)
            .add_plugin(spellcasting::SpellCastingPlugin)
            ;
    }
}
