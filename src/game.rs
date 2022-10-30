use bevy::{prelude::*, math::vec3};
use super::cursor::{Cursor, animate_cursor};

pub struct GamePlugin;

use super::{GameState, despawn_screen, spawn_anim, Game, AnimationTimer, RepeatAnimation, Mortal, SCALED_SPRITE_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH, HALF_SPRITE};

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(animate_sprite)
                    .with_system(animate_cursor)
                    .with_system(keyboard_input)
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

fn game_setup(mut commands: Commands, g: Res<Game>) {
    spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(2.0), 120, 8);
    spawn_anim(&mut commands, g.tah.clone(), Vec2::splat(1.0), 180, 4);
    Cursor::new(g.tah.clone(), &mut commands);
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

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Transform,
        &mut Cursor,
    )>,
) {
    for (mut transform, _cursor) in &mut query {
        let mut new_cursor_position = transform.translation;
        if keys.just_pressed(KeyCode::Left) {
            new_cursor_position = transform.translation - vec3(SCALED_SPRITE_SIZE, 0.0, 0.0);
        }
        if keys.just_pressed(KeyCode::Right) {
            new_cursor_position = transform.translation + vec3(SCALED_SPRITE_SIZE, 0.0, 0.0);
        }
        if keys.just_pressed(KeyCode::Up) {
            new_cursor_position = transform.translation + vec3(0.0, SCALED_SPRITE_SIZE, 0.0);
        }
        if keys.just_pressed(KeyCode::Down) {
            new_cursor_position = transform.translation - vec3(0.0, SCALED_SPRITE_SIZE, 0.0);
        }
        transform.translation.x = new_cursor_position.x.clamp(-SCREEN_WIDTH/2.0+HALF_SPRITE+SCALED_SPRITE_SIZE, SCREEN_WIDTH/2.0-HALF_SPRITE-SCALED_SPRITE_SIZE);
        transform.translation.y = new_cursor_position.y.clamp(-SCREEN_HEIGHT/2.0+HALF_SPRITE+SCALED_SPRITE_SIZE, SCREEN_HEIGHT/2.0-HALF_SPRITE-SCALED_SPRITE_SIZE);
    }
}
