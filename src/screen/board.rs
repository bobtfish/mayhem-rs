use bevy::prelude::*;
use crate::gamestate::GameState;
use crate::game::Game;
use crate::display;
use crate::system;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(display::animate_sprite)
                )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(system::despawn_screen::<OnGameScreen>),
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
    for p in g.player_info.iter_mut() {
        p.spawn(&mut commands, tah.clone());
    }
}

