use bevy::math::vec2;
use bevy::{prelude::*};
use super::constants::{ANIMATION_TICK, WIDTH, HEIGHT, CURSOR_Z};
use super::{GameState, Game};
use crate::display;

const CURSOR_SPRITE_ID: usize = 165;
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(cursor_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(keyboard_input)
                    .with_system(animate_cursor));
    }
}

#[derive(Default)]
pub struct Cursor {
    visible: bool,
    flash: bool,
    x: u8,
    y: u8,
    entity: Option<Entity>,
    flash_timer: Timer,
}

fn cursor_setup(
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    let x = 0;
    let y = 0;
    let entity = commands.spawn_bundle(display::get_sprite_sheet_bundle_z(game.tah(), Vec2::new(f32::from(x), f32::from(y)), CURSOR_SPRITE_ID, CURSOR_Z));
    game.cursor = Cursor{
        visible: true,
        flash: true,
        x,
        y,
        flash_timer: Timer::from_seconds(ANIMATION_TICK/2.0, true),
        entity: Some(entity.id()),
    };
}

#[allow(clippy::useless_let_if_seq)]
fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut moved = false;
    if keys.just_pressed(KeyCode::Left) && game.cursor.x > 0 {
        game.cursor.x -= 1;
        moved = true;
    }
    if keys.just_pressed(KeyCode::Right) && game.cursor.x < WIDTH as u8 {
        game.cursor.x += 1;
        moved = true;
    }
    if keys.just_pressed(KeyCode::Up) && game.cursor.y < HEIGHT as u8 {
        game.cursor.y += 1;
        moved = true;
    }
    if keys.just_pressed(KeyCode::Down) && game.cursor.y > 0 {
        game.cursor.y -= 1;
        moved = true;
    }
    if moved {
        let cursor = game.cursor.entity.unwrap();
        *transforms.get_mut(cursor).unwrap() = transforms.get(cursor).unwrap().with_translation(vec2(f32::from(game.cursor.x), f32::from(game.cursor.y)).extend(CURSOR_Z));
    }
}


fn animate_cursor(
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut textures: Query<&mut TextureAtlasSprite>,
) {
    game.cursor.flash_timer.tick(time.delta());
    if game.cursor.flash_timer.just_finished() {
        if game.cursor.flash || !game.cursor.visible {
            game.cursor.flash = false;
            textures.get_mut(game.cursor.entity.unwrap()).unwrap().color.set_a(0.0);
        } else {
            game.cursor.flash = true;
            textures.get_mut(game.cursor.entity.unwrap()).unwrap().color.set_a(1.0);
        }
    }
    
}
