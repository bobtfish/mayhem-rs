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
            .add_startup_system(cursor_setup.at_end())
            .add_system(keyboard_input)
            .add_system(animate_cursor);
    }
}

#[derive(Component)]
pub struct CursorEntity;

#[derive(Default)]
pub struct Cursor {
    visible: bool,
    x: f32,
    y: f32,
    flash_timer: Timer,
    moved: bool,
}

impl Cursor {
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn set_visible(&mut self) {
        self.visible = true;
        self.moved = true;
    }
    pub fn set_invisible(&mut self) {
        self.visible = false;
        self.moved = true;
    }
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.moved = true;
    }
    pub fn set_pos_v(&mut self, v: Vec2) {
        self.x = v.x;
        self.y = v.y;
        self.moved = true;
    }
    pub fn get_pos_v(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
}

fn cursor_setup(
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    let mut sprite = display::get_sprite_sheet_bundle_z(game.tah(), Vec2::new(0.0, 0.0), CURSOR_SPRITE_ID, display::WHITE, CURSOR_Z);
    sprite.visibility.is_visible = false;
    commands.spawn(sprite).insert(CursorEntity);
    game.cursor = Cursor{
        visible: false,
        x: 0.0,
        y: 0.0,
        flash_timer: Timer::from_seconds(ANIMATION_TICK/2.0, TimerMode::Repeating),
        moved: false,
    };
}

#[allow(clippy::useless_let_if_seq)]
fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
) {
    if !game.cursor.is_visible() {
        return;
    }
    if keys.just_pressed(KeyCode::Left) && game.cursor.x > 0.0 {
        game.cursor.x -= 1.0;
        game.cursor.moved = true;
    }
    if keys.just_pressed(KeyCode::Right) && game.cursor.x < WIDTH as f32 -2.0 {
        game.cursor.x += 1.0;
        game.cursor.moved = true;
    }
    if keys.just_pressed(KeyCode::Up) && game.cursor.y < HEIGHT as f32 -3.0 {
        game.cursor.y += 1.0;
        game.cursor.moved = true;
    }
    if keys.just_pressed(KeyCode::Down) && game.cursor.y > 0.0 {
        game.cursor.y -= 1.0;
        game.cursor.moved = true;
    }
}

pub fn set_visible(
    mut game: ResMut<Game>,
) {
    game.cursor.set_visible();
}
pub fn set_invisible(
    mut game: ResMut<Game>,
) {
    game.cursor.set_invisible();
}


fn animate_cursor(
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut transform: Query<&mut Transform, With<CursorEntity>>,
    mut query: Query<&mut Visibility, With<CursorEntity>>,
) {
    let mut vis = query.single_mut();
    if game.cursor.moved {
        vis.is_visible = game.cursor.is_visible();
        *transform.single_mut() = transform.single().with_translation(vec2(game.cursor.x, game.cursor.y).extend(CURSOR_Z));
        game.cursor.moved = false;
    }
    if !game.cursor.is_visible() {
        return;
    }
    game.cursor.flash_timer.tick(time.delta());
    if game.cursor.flash_timer.just_finished() {
        if vis.is_visible {
            vis.is_visible = false;
        } else {
            vis.is_visible = true;
        }
    }
    
}
