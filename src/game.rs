use bevy::{prelude::*, math::vec3};
use super::cursor::{Cursor};

pub struct GamePlugin;

use super::{GameState, despawn_screen, spawn_anim, Game, AnimationTimer, RepeatAnimation, Mortal, SCREEN_HEIGHT, SCREEN_WIDTH, HALF_SPRITE, SPRITE_SIZE};

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(animate_sprite)
                )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(despawn_screen::<OnGameScreen>),
            );
    }
}

    // Tag component used to tag entities added on the menu screen
    #[derive(Component)]
    struct OnGameScreen;

fn game_setup(
    mut commands: Commands,
    g: Res<Game>,
) {
    spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(2.0), 120, 8);
    spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(1.0), 180, 4);
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut RepeatAnimation,
        Option<&Mortal>,
    )>,
) {
    for (mut timer, mut sprite, repeater, mortal) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let alive = match mortal {
                // The division was valid
                Some(x) => x.is_alive,
                None    => true,
            };
            if alive {
                let mut index = sprite.index + 1;
                if index > repeater.max {
                    index = repeater.init;
                }
                sprite.index = index;
            } else {
                sprite.index = repeater.max + 1;
            }
        }
    }
}

